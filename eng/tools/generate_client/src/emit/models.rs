// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    emit::{ident, types::rust_type},
    model::{Package, Type},
};
use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt};
use std::collections::BTreeSet;

pub(super) fn render(
    package: &Package,
    recursive: &BTreeSet<(String, String)>,
) -> Result<TokenStream, String> {
    let mut output = if package.models.is_empty() {
        quote!()
    } else {
        quote! {
            use azure_core::fmt::SafeDebug;
            use serde::{Deserialize, Serialize};
        }
    };
    let mut enum_refs = BTreeSet::new();
    fn collect_enum_refs<'a>(value: &'a Type, names: &mut BTreeSet<&'a str>) {
        match value {
            Type::Enum { name } => {
                names.insert(name);
            }
            Type::Array { value_type }
            | Type::Dict { value_type }
            | Type::Nullable { value_type } => collect_enum_refs(value_type, names),
            _ => {}
        }
    }
    for field in package.models.iter().flat_map(|model| &model.fields) {
        collect_enum_refs(&field.field_type, &mut enum_refs);
    }
    if !enum_refs.is_empty() {
        let imports: Vec<_> = enum_refs
            .iter()
            .map(|name| ident(name, "enum reference"))
            .collect::<Result<_, _>>()?;
        output.extend(quote!(use super::{#(#imports),*};));
    }
    let mut sorted: Vec<_> = package.models.iter().collect();
    sorted.sort_by(|a, b| a.name.cmp(&b.name));
    for model in sorted {
        let scope = format!("model {}", model.name);
        let name = ident(&model.name, &scope)?;
        let documentation = model.doc.as_deref().unwrap_or(&model.name);
        let mut fields = quote!();
        let mut sorted_fields: Vec<_> = model.fields.iter().collect();
        sorted_fields.sort_by(|a, b| a.name.cmp(&b.name));
        let mut rust_names = BTreeSet::new();
        for field in sorted_fields {
            let scope = format!("{}.{}", model.name, field.name);
            let field_name = ident(&field.name, &scope)?;
            if !rust_names.insert(field_name.to_string().trim_start_matches("r#").to_owned()) {
                return Err(format!("{scope}: duplicate Rust field identifier"));
            }
            let is_nullable = matches!(field.field_type, Type::Nullable { .. });
            if field.optional && is_nullable {
                return Err(format!(
                    "{scope}: optional nullable fields need distinct missing/null semantics"
                ));
            }
            let boxed = recursive.contains(&(model.name.clone(), field.name.clone()));
            let field_type = rust_type(&field.field_type, &scope, boxed)?;
            let field_type = if field.optional {
                quote!(Option<#field_type>)
            } else {
                quote!(#field_type)
            };
            let field_doc = field.doc.as_deref().unwrap_or(&field.name);
            let wire_name = &field.wire_name;
            let mut attributes = quote!(rename = #wire_name);
            if field.read_only {
                attributes.append_all(quote!(, skip_serializing));
                if field.optional {
                    attributes.append_all(quote!(, default));
                }
            } else if field.optional {
                attributes.append_all(quote!(, default, skip_serializing_if = "Option::is_none"));
            }
            if is_nullable {
                attributes.append_all(quote!(, deserialize_with = "Option::deserialize"));
            }
            fields.extend(quote! {
                #[doc = #field_doc]
                #[serde(#attributes)]
                pub #field_name: #field_type,
            });
        }
        let derive_default = if model.fields.iter().all(|field| field.optional) {
            quote!(Default,)
        } else {
            quote!()
        };
        output.extend(quote! {
            #[doc = #documentation]
            #[derive(Clone, #derive_default Deserialize, SafeDebug, Serialize)]
            #[non_exhaustive]
            pub struct #name {
                #fields
            }
        });
    }
    Ok(output)
}
