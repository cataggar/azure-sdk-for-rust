// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use serde::Deserialize;
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};

const MAX_SOURCE_BYTES: u64 = 256 * 1024 * 1024;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ResourceManifest {
    schema_version: u32,
    root: String,
    files: Vec<String>,
}

pub(crate) fn collect(
    project: &Path,
    resources: Option<&Path>,
    project_relative: Option<&Path>,
) -> Result<BTreeMap<String, String>, String> {
    let mut sources = BTreeMap::new();
    let mut bytes = 0;
    collect_dir(project, project, "/spec", false, &mut sources, &mut bytes)?;
    check_relative_imports(&sources)?;
    let resources = resources.ok_or_else(|| {
        "TypeSpec component resources are required; pass --resources <component/dist/resources>"
            .to_string()
    })?;
    let packages = resources.join("node_modules");
    if !packages.is_dir() {
        return Err(format!(
            "Missing TypeSpec resources: {}",
            packages.display()
        ));
    }
    collect_dir(
        &packages,
        &packages,
        "/node_modules",
        true,
        &mut sources,
        &mut bytes,
    )?;
    let manifest_path = resources
        .parent()
        .ok_or_else(|| format!("Resource directory has no parent: {}", resources.display()))?
        .join("resources.json");
    let manifest: ResourceManifest = serde_json::from_slice(
        &fs::read(&manifest_path)
            .map_err(|error| format!("{}: {error}", manifest_path.display()))?,
    )
    .map_err(|error| format!("{}: {error}", manifest_path.display()))?;
    if manifest.schema_version != 1 || manifest.root != "resources/node_modules" {
        return Err(format!(
            "Unsupported resource manifest in {}",
            manifest_path.display()
        ));
    }
    let expected: BTreeSet<_> = manifest.files.iter().collect();
    let actual: BTreeSet<_> = sources
        .keys()
        .filter(|path| path.starts_with("/node_modules/"))
        .collect();
    if expected.len() != manifest.files.len() || expected != actual {
        return Err(format!(
            "TypeSpec resource files differ from {}",
            manifest_path.display()
        ));
    }
    let virtual_project = if let Some(path) = project_relative {
        let segments = path
            .iter()
            .map(|part| {
                part.to_str()
                    .ok_or_else(|| format!("Non-UTF-8 TypeSpec project path: {}", path.display()))
            })
            .collect::<Result<Vec<_>, _>>()?;
        format!("/spec/{}", segments.join("/"))
    } else {
        "/spec".to_string()
    };
    if !sources.contains_key(&format!("{virtual_project}/main.tsp"))
        && !sources.contains_key(&format!("{virtual_project}/client.tsp"))
    {
        return Err(format!(
            "TypeSpec project has no main.tsp or client.tsp: {}",
            project.display()
        ));
    }
    Ok(sources)
}

fn check_relative_imports(sources: &BTreeMap<String, String>) -> Result<(), String> {
    for (file, content) in sources.iter().filter(|(path, _)| path.ends_with(".tsp")) {
        for import in relative_imports(content) {
            if !import.starts_with("./") && !import.starts_with("../") {
                continue;
            }
            let mut segments: Vec<_> = file
                .rsplit_once('/')
                .expect("virtual source path has a parent")
                .0
                .trim_start_matches('/')
                .split('/')
                .collect();
            for part in import.split('/') {
                match part {
                    "" | "." => {}
                    ".." if segments.len() > 1 => {
                        segments.pop();
                    }
                    ".." => {
                        return Err(format!("TypeSpec import {import} in {file} escapes /spec"))
                    }
                    part => segments.push(part),
                }
            }
            let target = format!("/{}", segments.join("/"));
            let present = if target.ends_with(".tsp") || target.ends_with(".json") {
                sources.contains_key(&target)
            } else {
                sources.contains_key(&format!("{target}.tsp"))
                    || sources.contains_key(&format!("{target}/main.tsp"))
                    || sources.contains_key(&format!("{target}/index.tsp"))
            };
            if !present {
                return Err(format!(
                    "Missing relative TypeSpec import {import} in {file} (resolved to {target}); include its directory in additionalDirectories for a sparse pinned checkout"
                ));
            }
        }
    }
    Ok(())
}

fn relative_imports(content: &str) -> Vec<&str> {
    let bytes = content.as_bytes();
    let mut imports = Vec::new();
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index..].starts_with(b"//") {
            index = bytes[index..]
                .iter()
                .position(|byte| *byte == b'\n')
                .map_or(bytes.len(), |offset| index + offset);
        } else if bytes[index..].starts_with(b"/*") {
            index = bytes[index + 2..]
                .windows(2)
                .position(|pair| pair == b"*/")
                .map_or(bytes.len(), |offset| index + offset + 4);
        } else if matches!(bytes[index], b'"' | b'\'' | b'`') {
            let quote = bytes[index];
            index += 1;
            while index < bytes.len() {
                if bytes[index] == b'\\' {
                    index += 2;
                } else if bytes[index] == quote {
                    index += 1;
                    break;
                } else {
                    index += 1;
                }
            }
        } else if bytes[index..].starts_with(b"import")
            && (index == 0 || !is_identifier_byte(bytes[index - 1]))
            && bytes
                .get(index + 6)
                .is_some_and(|byte| byte.is_ascii_whitespace())
        {
            index += 6;
            while bytes
                .get(index)
                .is_some_and(|byte| byte.is_ascii_whitespace())
            {
                index += 1;
            }
            if let Some(&quote @ (b'"' | b'\'')) = bytes.get(index) {
                let start = index + 1;
                if let Some(end) = bytes[start..].iter().position(|byte| *byte == quote) {
                    imports.push(&content[start..start + end]);
                    index = start + end + 1;
                    continue;
                }
            }
        } else {
            index += 1;
        }
    }
    imports
}

fn is_identifier_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

fn collect_dir(
    root: &Path,
    dir: &Path,
    virtual_root: &str,
    packages_only: bool,
    sources: &mut BTreeMap<String, String>,
    bytes: &mut u64,
) -> Result<(), String> {
    let mut entries: Vec<_> = fs::read_dir(dir)
        .map_err(|error| format!("{}: {error}", dir.display()))?
        .collect::<Result<_, _>>()
        .map_err(|error| format!("{}: {error}", dir.display()))?;
    entries.sort_by_key(|entry| entry.file_name());
    for entry in entries {
        let path = entry.path();
        let file_type = entry
            .file_type()
            .map_err(|error| format!("{}: {error}", path.display()))?;
        if file_type.is_symlink() {
            return Err(format!(
                "Symlink in TypeSpec source tree: {}",
                path.display()
            ));
        }
        if file_type.is_dir() {
            if entry.file_name() != ".git" && (entry.file_name() != "node_modules" || packages_only)
            {
                collect_dir(root, &path, virtual_root, packages_only, sources, bytes)?;
            }
            continue;
        }
        if !file_type.is_file() {
            continue;
        }
        let filename = path
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| format!("Non-UTF-8 TypeSpec source filename: {}", path.display()))?;
        let include = if packages_only {
            filename.ends_with(".tsp") || filename == "package.json"
        } else {
            filename.ends_with(".tsp") || filename.ends_with(".yaml") || filename.ends_with(".json")
        };
        if !include {
            continue;
        }
        let metadata = entry
            .metadata()
            .map_err(|error| format!("{}: {error}", path.display()))?;
        *bytes = bytes
            .checked_add(metadata.len())
            .ok_or_else(|| "TypeSpec source size overflow".to_string())?;
        if *bytes > MAX_SOURCE_BYTES {
            return Err(format!(
                "TypeSpec source exceeds {} MiB",
                MAX_SOURCE_BYTES / (1024 * 1024)
            ));
        }
        let relative = path
            .strip_prefix(root)
            .map_err(|error| format!("{}: {error}", path.display()))?;
        let segments: Result<Vec<_>, _> = relative
            .iter()
            .map(|segment| {
                segment
                    .to_str()
                    .ok_or_else(|| format!("Non-UTF-8 TypeSpec path segment: {}", path.display()))
            })
            .collect();
        let virtual_path = format!("{virtual_root}/{}", segments?.join("/"));
        let content =
            fs::read_to_string(&path).map_err(|error| format!("{}: {error}", path.display()))?;
        if sources.insert(virtual_path.clone(), content).is_some() {
            return Err(format!("Duplicate TypeSpec virtual path: {virtual_path}"));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests;
