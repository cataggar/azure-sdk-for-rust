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
use std::{env, fs, path::PathBuf};
use typespec_codegen_types as model;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: typespec-codegen <code-model.json> [output-dir]");
        eprintln!();
        eprintln!("The code model JSON is produced by the TCGC WASI component.");
        std::process::exit(1);
    }

    let model_path = PathBuf::from(&args[1]);
    let output_dir = if args.len() > 2 {
        PathBuf::from(&args[2])
    } else {
        PathBuf::from(".")
    };

    let json = fs::read_to_string(&model_path)
        .with_context(|| format!("failed to read {}", model_path.display()))?;
    let crate_model: model::Crate =
        serde_json::from_str(&json).context("failed to parse code model JSON")?;

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
