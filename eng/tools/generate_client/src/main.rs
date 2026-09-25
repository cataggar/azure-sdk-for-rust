// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

mod cli;
mod emit;
mod host;
mod location;
mod model;
mod output;
mod source;

use clap::Parser;
use std::{fs, path::Path};

fn main() {
    if let Err(error) = run(cli::Args::parse()) {
        eprintln!("generate_client: {error}");
        std::process::exit(1);
    }
}

fn run(args: cli::Args) -> Result<(), String> {
    let manifest = args
        .manifest_path
        .canonicalize()
        .map_err(|error| format!("{}: {error}", args.manifest_path.display()))?;
    if manifest.file_name().is_none_or(|name| name != "Cargo.toml") {
        return Err(format!(
            "Expected a Cargo.toml manifest: {}",
            manifest.display()
        ));
    }
    let crate_dir = manifest
        .parent()
        .ok_or_else(|| format!("Manifest has no parent: {}", manifest.display()))?;
    let output = args.output.as_deref().unwrap_or(crate_dir);
    if !Path::new(output).join("Cargo.toml").exists() {
        return Err(format!("Output is not an SDK crate: {}", output.display()));
    }

    let model = if let Some(model) = &args.model {
        fs::read_to_string(model).map_err(|error| format!("{}: {error}", model.display()))?
    } else {
        let location = location::Location::load(crate_dir)?;
        let project = if args.sync {
            location
                .as_ref()
                .ok_or_else(|| format!("Missing tsp-location.yaml in {}", crate_dir.display()))?
                .sync()?
        } else {
            let project = args
                .spec_dir
                .as_deref()
                .ok_or_else(|| "Specify --spec-dir, --sync, or --model".to_string())?;
            if let Some(location) = &location {
                location.verify(project)?;
            }
            project.to_path_buf()
        };
        let component = args
            .component
            .as_deref()
            .ok_or_else(|| "--component is required for TypeSpec input".to_string())?;
        let source_root = if let Some(location) = &location {
            location.root_for(&project)?
        } else {
            project.clone()
        };
        let project_relative = location.as_ref().map(location::Location::directory);
        host::compile(
            &source_root,
            project_relative,
            component,
            args.resources.as_deref(),
        )?
    };
    let parsed: model::Package = serde_json::from_str(&model)
        .map_err(|error| format!("Invalid TCGC code model: {error}"))?;
    parsed.validate()?;

    if args.preview_models {
        let output_root = output
            .canonicalize()
            .map_err(|error| format!("{}: {error}", output.display()))?;
        if output_root == crate_dir {
            return Err("--preview-models requires a separate scratch output crate".to_string());
        }
        let files = emit::render(&parsed)?;
        return output::reconcile(&output_root, &files, args.check);
    }

    Err(format!(
        "Emitter is not yet implemented; refusing to modify {} (check={})",
        output.join("src").join("generated").display(),
        args.check
    ))
}
