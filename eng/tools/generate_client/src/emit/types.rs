// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    emit::ident,
    model::{Model, Type},
};
use quote::quote;
use std::collections::{BTreeMap, BTreeSet};
use syn::{parse2, Type as RustType};

pub(super) fn rust_type(value: &Type, scope: &str, boxed: bool) -> Result<RustType, String> {
    let tokens = match value {
        Type::String => quote!(String),
        Type::Boolean => quote!(bool),
        Type::Int32 => quote!(i32),
        Type::Int64 => quote!(i64),
        Type::Float64 => quote!(f64),
        Type::Model { name } | Type::Enum { name } => {
            let name = ident(name, scope)?;
            if boxed {
                quote!(Box<#name>)
            } else {
                quote!(#name)
            }
        }
        Type::Array { value_type } => {
            let inner = rust_type(value_type, scope, false)?;
            quote!(Vec<#inner>)
        }
        Type::Dict { value_type } => {
            let inner = rust_type(value_type, scope, false)?;
            quote!(std::collections::HashMap<String, #inner>)
        }
        Type::Nullable { value_type } => {
            if matches!(**value_type, Type::Nullable { .. }) {
                return Err(format!("{scope}: nested nullable types are unsupported"));
            }
            let inner = rust_type(value_type, scope, boxed)?;
            quote!(Option<#inner>)
        }
        Type::Bytes => return Err(format!("{scope}: bytes require an explicit wire encoding")),
        Type::UtcDateTime => {
            return Err(format!(
                "{scope}: utcDateTime requires an explicit wire format"
            ))
        }
    };
    parse2(tokens).map_err(|error| format!("{scope}: invalid Rust type: {error}"))
}

/// Only direct, possibly nullable, model references create infinite-size cycles.
pub(super) fn recursive_fields(models: &[Model]) -> BTreeSet<(String, String)> {
    fn target(value: &Type) -> Option<&str> {
        match value {
            Type::Nullable { value_type } => target(value_type),
            Type::Model { name } => Some(name),
            _ => None,
        }
    }
    fn visit(
        name: &str,
        models: &BTreeMap<&str, &Model>,
        visited: &mut BTreeSet<String>,
        stack: &mut BTreeSet<String>,
        boxed: &mut BTreeSet<(String, String)>,
    ) {
        visited.insert(name.to_owned());
        stack.insert(name.to_owned());
        if let Some(model) = models.get(name) {
            let mut edges: Vec<_> = model
                .fields
                .iter()
                .filter_map(|field| target(&field.field_type).map(|to| (field.name.as_str(), to)))
                .collect();
            edges.sort_unstable();
            for (field, to) in edges {
                if stack.contains(to) {
                    boxed.insert((name.to_owned(), field.to_owned()));
                } else if !visited.contains(to) {
                    visit(to, models, visited, stack, boxed);
                }
            }
        }
        stack.remove(name);
    }
    let sorted: BTreeMap<_, _> = models
        .iter()
        .map(|model| (model.name.as_str(), model))
        .collect();
    let mut visited = BTreeSet::new();
    let mut stack = BTreeSet::new();
    let mut boxed = BTreeSet::new();
    for name in sorted.keys() {
        if !visited.contains(*name) {
            visit(name, &sorted, &mut visited, &mut stack, &mut boxed);
        }
    }
    boxed
}
