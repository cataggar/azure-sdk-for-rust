// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::source;
use serde_json::{json, Value};
use std::path::Path;
use wasmtime::{
    component::{Component, Linker, ResourceTable},
    Config, Engine, Store, StoreLimits, StoreLimitsBuilder,
};
use wasmtime_wasi::{WasiCtx, WasiCtxView, WasiView};

struct HostState {
    ctx: WasiCtx,
    table: ResourceTable,
    limits: StoreLimits,
}

impl WasiView for HostState {
    fn ctx(&mut self) -> WasiCtxView<'_> {
        WasiCtxView {
            ctx: &mut self.ctx,
            table: &mut self.table,
        }
    }
}

pub(crate) fn compile(
    source_root: &Path,
    project_relative: Option<&Path>,
    component_path: &Path,
    resources: Option<&Path>,
) -> Result<String, String> {
    let sources = source::collect(source_root, resources, project_relative)?;
    let options = json!({"__spec_files": sources});
    let options = serde_json::to_string(&options)
        .map_err(|error| format!("Failed to serialize TypeSpec inputs: {error}"))?;
    let mut config = Config::new();
    config.wasm_component_model(true);
    let engine = Engine::new(&config).map_err(|error| format!("Wasmtime engine: {error:#}"))?;
    let component = Component::from_file(&engine, component_path)
        .map_err(|error| format!("{}: {error:#}", component_path.display()))?;
    let mut linker = Linker::<HostState>::new(&engine);
    wasmtime_wasi::p2::add_to_linker_sync(&mut linker)
        .map_err(|error| format!("Failed to link WASI Preview 2: {error:#}"))?;
    let state = HostState {
        ctx: WasiCtx::builder().build(),
        table: ResourceTable::new(),
        limits: StoreLimitsBuilder::new()
            .memory_size(4 * 1024 * 1024 * 1024)
            .build(),
    };
    let mut store = Store::new(&engine, state);
    store.limiter(|state| &mut state.limits);
    let instance = linker
        .instantiate(&mut store, &component)
        .map_err(|error| format!("Failed to instantiate TCGC component: {error:#}"))?;
    let interface = instance
        .get_export_index(&mut store, None, "azure:codegen/tcgc")
        .ok_or_else(|| "Component does not export azure:codegen/tcgc".to_string())?;
    let method = instance
        .get_export_index(&mut store, Some(&interface), "compile")
        .ok_or_else(|| "Component does not export tcgc.compile".to_string())?;
    let compile = instance
        .get_typed_func::<(String, String), (Result<String, String>,)>(&mut store, &method)
        .map_err(|error| format!("Invalid tcgc.compile WIT signature: {error:#}"))?;
    let virtual_project = match project_relative {
        Some(path) => format!(
            "/spec/{}",
            path.iter()
                .map(|part| part.to_str().ok_or_else(|| {
                    format!("Non-UTF-8 TypeSpec project path: {}", path.display())
                }))
                .collect::<Result<Vec<_>, _>>()?
                .join("/")
        ),
        None => "/spec".to_string(),
    };
    let (result,) = compile
        .call(&mut store, (virtual_project, options))
        .map_err(|error| format!("TCGC component trapped: {error:#}"))?;
    match result {
        Ok(output) => {
            let _: Value = serde_json::from_str(&output)
                .map_err(|error| format!("TCGC returned invalid JSON: {error}"))?;
            Ok(output)
        }
        Err(diagnostic) => Err(format!("TypeSpec/TCGC: {diagnostic}")),
    }
}
