// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use clap::{ArgGroup, Parser};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about = "Generate Rust Azure clients from TypeSpec")]
#[command(group(ArgGroup::new("source").required(true).args(["spec_dir", "model", "sync"])))]
pub(crate) struct Args {
    /// Cargo manifest for the target SDK crate.
    #[arg(long)]
    pub(crate) manifest_path: PathBuf,

    /// Directory containing a local TypeSpec project.
    #[arg(long)]
    pub(crate) spec_dir: Option<PathBuf>,

    /// Use the exact repository and commit in tsp-location.yaml.
    #[arg(long)]
    pub(crate) sync: bool,

    /// Load a previously captured JSON code model without invoking TypeSpec.
    #[arg(long)]
    pub(crate) model: Option<PathBuf>,

    /// Path to the TCGC component, required for TypeSpec generation.
    #[arg(long, conflicts_with = "model")]
    pub(crate) component: Option<PathBuf>,

    /// Path to the pinned TypeSpec package resource tree.
    #[arg(long, requires = "component")]
    pub(crate) resources: Option<PathBuf>,

    /// Compare owned output without writing any files.
    #[arg(long)]
    pub(crate) check: bool,

    /// Target crate root; defaults to the parent of --manifest-path.
    #[arg(long)]
    pub(crate) output: Option<PathBuf>,

    /// Emit the schema-2 model-only preview into a separate scratch crate.
    #[arg(long, requires_all = ["model", "output"], conflicts_with = "preview_basic")]
    pub(crate) preview_models: bool,

    /// Emit the guarded schema-2 basic-client preview into a separate scratch crate.
    #[arg(long, requires = "output", conflicts_with_all = ["preview_models", "export_model"])]
    pub(crate) preview_basic: bool,

    /// Save the validated TCGC JSON model to a new file in a separate scratch crate.
    #[arg(long, requires = "output", conflicts_with_all = ["model", "preview_models", "preview_basic", "check"])]
    pub(crate) export_model: bool,
}

#[cfg(test)]
mod tests {
    use super::Args;
    use clap::Parser;

    #[test]
    fn basic_preview_accepts_typespec_source_with_scratch_output() {
        let args = Args::try_parse_from([
            "generate_client",
            "--manifest-path",
            "Cargo.toml",
            "--spec-dir",
            "spec",
            "--output",
            "scratch",
            "--preview-basic",
        ])
        .unwrap();
        assert!(args.preview_basic);
        assert!(args.model.is_none());
    }

    #[test]
    fn preview_requires_explicit_scratch_output() {
        assert!(Args::try_parse_from([
            "generate_client",
            "--manifest-path",
            "Cargo.toml",
            "--spec-dir",
            "spec",
            "--preview-basic",
        ])
        .is_err());
    }

    #[test]
    fn export_requires_scratch_output_and_typespec_source() {
        assert!(Args::try_parse_from([
            "generate_client",
            "--manifest-path",
            "Cargo.toml",
            "--sync",
            "--export-model"
        ])
        .is_err());
        assert!(Args::try_parse_from([
            "generate_client",
            "--manifest-path",
            "Cargo.toml",
            "--model",
            "model.json",
            "--output",
            "scratch",
            "--export-model"
        ])
        .is_err());
    }
}
