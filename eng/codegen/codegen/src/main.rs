// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! TypeSpec Rust code generator.
//!
//! Generates Azure SDK Rust crates from TypeSpec specifications by:
//! 1. Loading TypeSpec + TCGC via a WASI component (built with jco)
//! 2. Transforming the TCGC code model into Rust source files
//!
//! This replaces the TypeScript-based emitter at Azure/typespec-rust.

mod emit;

use anyhow::{Context, Result};
use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};
use typespec_codegen_types as model;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage:");
        eprintln!("  typespec-codegen <code-model.json> [output-dir]");
        eprintln!("  typespec-codegen --tsp <project-path> [output-dir] [--crate-name NAME] [--crate-version VER]");
        std::process::exit(1);
    }

    let (crate_model, output_dir) = if args[1] == "--tsp" {
        // TypeSpec mode: invoke TCGC component via Node.js bridge
        let project_path = args.get(2).expect("missing project path after --tsp");
        let output_dir = args
            .get(3)
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("."));
        let mut emitter_opts = serde_json::Map::new();
        let mut i = 4;
        while i < args.len() {
            match args[i].as_str() {
                "--crate-name" => {
                    emitter_opts.insert("crate-name".into(), args[i + 1].clone().into());
                    i += 2;
                }
                "--crate-version" => {
                    emitter_opts.insert("crate-version".into(), args[i + 1].clone().into());
                    i += 2;
                }
                _ => i += 1,
            }
        }

        let component_dir = find_tcgc_component()?;
        let json = invoke_tcgc_via_node(&component_dir, project_path, &emitter_opts)?;
        let crate_model: model::Crate =
            serde_json::from_str(&json).context("failed to parse TCGC output")?;
        (crate_model, output_dir)
    } else {
        // JSON mode: read pre-generated code model
        let model_path = PathBuf::from(&args[1]);
        let output_dir = args
            .get(2)
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("."));
        let json = fs::read_to_string(&model_path)
            .with_context(|| format!("failed to read {}", model_path.display()))?;
        let crate_model: model::Crate =
            serde_json::from_str(&json).context("failed to parse code model JSON")?;
        (crate_model, output_dir)
    };

    println!(
        "Generating crate '{}' v{} ({:?})",
        crate_model.name, crate_model.version, crate_model.service_type
    );
    println!(
        "  {} client(s), {} model(s), {} enum(s)",
        crate_model.clients.len(),
        crate_model.models.len(),
        crate_model.enums.len(),
    );

    let generated_dir = output_dir.join("src").join("generated");
    fs::create_dir_all(generated_dir.join("clients"))?;
    fs::create_dir_all(generated_dir.join("models"))?;

    emit::emit_crate(&crate_model, &output_dir)?;

    println!("Generated to {}", output_dir.display());
    Ok(())
}

/// Find the tcgc-component directory relative to the binary.
fn find_tcgc_component() -> Result<PathBuf> {
    // Look relative to the codegen workspace
    let candidates = [
        PathBuf::from("eng/codegen/tcgc-component"),
        PathBuf::from("../tcgc-component"),
        PathBuf::from("tcgc-component"),
    ];
    for p in &candidates {
        if p.join("src/index.js").exists() {
            return Ok(p.clone());
        }
    }
    anyhow::bail!(
        "tcgc-component directory not found; run from the repo root or set TCGC_COMPONENT_DIR"
    )
}

/// Invoke the TCGC component via Node.js (bridge until wasmtime hosting is ready).
fn invoke_tcgc_via_node(
    component_dir: &Path,
    project_path: &str,
    emitter_opts: &serde_json::Map<String, serde_json::Value>,
) -> Result<String> {
    let opts_json = serde_json::to_string(emitter_opts)?;
    let output = Command::new("node")
        .arg(component_dir.join("src/index.js"))
        .arg(project_path)
        .arg(&opts_json)
        .output()
        .context("failed to run node")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("TCGC component failed: {stderr}");
    }

    Ok(String::from_utf8(output.stdout)?)
}
