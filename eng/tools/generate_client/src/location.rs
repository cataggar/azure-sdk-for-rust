// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use serde::Deserialize;
use std::{
    fs,
    path::{Component, Path, PathBuf},
    process::Command,
};

const REPOSITORY: &str = "Azure/azure-rest-api-specs";
const URL: &str = "https://github.com/Azure/azure-rest-api-specs.git";

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Location {
    directory: PathBuf,
    commit: String,
    repo: String,
    #[serde(rename = "additionalDirectories")]
    additional_directories: Option<Vec<PathBuf>>,
}

impl Location {
    pub(crate) fn directory(&self) -> &Path {
        &self.directory
    }

    pub(crate) fn root_for(&self, project: &Path) -> Result<PathBuf, String> {
        let root = git_output(project, &["rev-parse", "--show-toplevel"])?;
        PathBuf::from(root.trim())
            .canonicalize()
            .map_err(|error| format!("Git root: {error}"))
    }

    pub(crate) fn load(crate_dir: &Path) -> Result<Option<Self>, String> {
        let path = crate_dir.join("tsp-location.yaml");
        let data = match fs::read_to_string(&path) {
            Ok(data) => data,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(error) => return Err(format!("{}: {error}", path.display())),
        };
        let location: Self =
            serde_yaml::from_str(&data).map_err(|error| format!("{}: {error}", path.display()))?;
        if location.repo != REPOSITORY {
            return Err(format!(
                "Unsupported TypeSpec repository: {}",
                location.repo
            ));
        }
        if location.commit.len() != 40
            || !location.commit.bytes().all(|byte| byte.is_ascii_hexdigit())
        {
            return Err(format!("Invalid TypeSpec commit: {}", location.commit));
        }
        if location.directory.as_os_str().is_empty()
            || !location
                .directory
                .components()
                .all(|part| matches!(part, Component::Normal(_)))
        {
            return Err(format!(
                "Invalid TypeSpec directory: {}",
                location.directory.display()
            ));
        }
        for directory in location.additional_directories.iter().flatten() {
            if directory.as_os_str().is_empty()
                || !directory
                    .components()
                    .all(|part| matches!(part, Component::Normal(_)))
            {
                return Err(format!(
                    "Invalid additional TypeSpec directory: {}",
                    directory.display()
                ));
            }
        }
        Ok(Some(location))
    }

    pub(crate) fn verify(&self, project: &Path) -> Result<(), String> {
        let actual = project
            .canonicalize()
            .map_err(|error| format!("{}: {error}", project.display()))?;
        let root = self.root_for(project)?;
        let expected = root
            .join(&self.directory)
            .canonicalize()
            .map_err(|error| format!("{}: {error}", root.join(&self.directory).display()))?;
        if actual != expected {
            return Err(format!(
                "TypeSpec project {} does not match pinned {}",
                actual.display(),
                expected.display()
            ));
        }
        let head = git_output(&root, &["rev-parse", "HEAD"])?;
        if head.trim() != self.commit {
            return Err(format!(
                "TypeSpec HEAD {} does not match pinned {}",
                head.trim(),
                self.commit
            ));
        }
        let status = git_output(&root, &["status", "--porcelain", "--untracked-files=all"])?;
        if !status.trim().is_empty() {
            return Err(format!(
                "TypeSpec checkout {} has uncommitted or untracked files",
                root.display()
            ));
        }
        Ok(())
    }

    pub(crate) fn sync(&self) -> Result<PathBuf, String> {
        let cache = std::env::var_os("LOCALAPPDATA")
            .or_else(|| std::env::var_os("XDG_CACHE_HOME"))
            .map(PathBuf::from)
            .ok_or_else(|| "Set LOCALAPPDATA or XDG_CACHE_HOME for --sync".to_string())?
            .join("azure-sdk-for-rust")
            .join("typespec")
            .join(&self.commit);
        fs::create_dir_all(&cache).map_err(|error| format!("{}: {error}", cache.display()))?;
        if !cache.join(".git").is_dir() {
            git_command(&cache, &["init", "-q", "."])?;
        }
        if git_output(&cache, &["rev-parse", "HEAD"])
            .ok()
            .as_deref()
            .map(str::trim)
            != Some(self.commit.as_str())
        {
            git_command(&cache, &["fetch", "--depth=1", URL, &self.commit])?;
            git_command(&cache, &["checkout", "--detach", "FETCH_HEAD"])?;
        }
        let project = cache.join(&self.directory);
        self.verify(&project)?;
        Ok(project)
    }
}

fn git_output(dir: &Path, args: &[&str]) -> Result<String, String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(args)
        .output()
        .map_err(|error| format!("Failed to execute git in {}: {error}", dir.display()))?;
    if !output.status.success() {
        return Err(format!(
            "git {} failed in {}: {}",
            args.join(" "),
            dir.display(),
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    String::from_utf8(output.stdout).map_err(|error| format!("git output is not UTF-8: {error}"))
}

fn git_command(dir: &Path, args: &[&str]) -> Result<(), String> {
    git_output(dir, args).map(|_| ())
}

#[cfg(test)]
mod tests;
