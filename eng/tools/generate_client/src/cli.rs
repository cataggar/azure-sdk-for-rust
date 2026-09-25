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

    /// Emit the schema-1 model-only preview into a separate crate checkout.
    #[arg(long, requires_all = ["model", "output"])]
    pub(crate) preview_models: bool,
}
