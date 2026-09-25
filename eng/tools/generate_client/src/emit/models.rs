// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    emit::{field_ident, ident, types::rust_type},
    model::{BytesEncoding, Client, DateTimeFormat, DateTimeWireType, Package, Type},
};
use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt};
use std::collections::{BTreeMap, BTreeSet};

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
    let mut pages = BTreeMap::new();
    fn collect_pages<'a>(
        clients: &'a [Client],
        pages: &mut BTreeMap<&'a str, (&'a str, &'a Type)>,
    ) -> Result<(), String> {
        for client in clients {
            for operation in &client.operations {
                if let Some(paging) = &operation.paging {
                    let Some(Type::Model { name }) = &operation.response_type else {
                        return Err(format!(
                            "{}.{}: invalid paging response",
                            client.name, operation.name
                        ));
                    };
                    if let Some((field, ty)) =
                        pages.insert(name.as_str(), (&paging.items.name, &paging.items.item_type))
                    {
                        if field != paging.items.name || ty != &paging.items.item_type {
                            return Err(format!("{name}: inconsistent paging item fields"));
                        }
                    }
                }
            }
            collect_pages(&client.children, pages)?;
        }
        Ok(())
    }
    collect_pages(&package.clients, &mut pages)?;
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
            let field_name = field_ident(&field.name, &scope)?;
            if !rust_names.insert(field_name.to_string().trim_start_matches("r#").to_owned()) {
                return Err(format!("{scope}: duplicate Rust field identifier"));
            }
            let is_nullable = matches!(field.field_type, Type::Nullable { .. });
            let page_items = pages
                .get(model.name.as_str())
                .is_some_and(|(name, _)| *name == field.name);
            let boxed = recursive.contains(&(model.name.clone(), field.name.clone()));
            let field_type = match (&field.field_type, field.optional && !page_items) {
                (Type::Nullable { value_type }, true) => {
                    // Match existing SDK models: missing and null both deserialize as None.
                    let inner = rust_type(value_type, &scope, boxed)?;
                    quote!(Option<#inner>)
                }
                (value, optional) => {
                    let inner = rust_type(value, &scope, boxed)?;
                    if optional {
                        quote!(Option<#inner>)
                    } else {
                        quote!(#inner)
                    }
                }
            };
            let field_doc = field.doc.as_deref().unwrap_or(&field.name);
            let wire_name = &field.wire_name;
            let mut attributes = quote!(rename = #wire_name);
            if page_items {
                attributes.append_all(quote!(, default));
                if field.read_only {
                    attributes.append_all(quote!(, skip_serializing));
                }
            } else if field.read_only {
                attributes.append_all(quote!(, skip_serializing));
                if field.optional {
                    attributes.append_all(quote!(, default));
                }
            } else if field.optional {
                attributes.append_all(quote!(, default, skip_serializing_if = "Option::is_none"));
            }
            if is_nullable && !field.optional {
                attributes.append_all(quote!(, deserialize_with = "Option::deserialize"));
            }
            if matches!(
                field.field_type,
                Type::Bytes {
                    encoding: BytesEncoding::Base64Url
                }
            ) {
                let (serialize, deserialize) = if field.optional {
                    (
                        "azure_core::base64::option::serialize_url_safe",
                        "azure_core::base64::option::deserialize_url_safe",
                    )
                } else {
                    (
                        "azure_core::base64::serialize_url_safe",
                        "azure_core::base64::deserialize_url_safe",
                    )
                };
                attributes.append_all(
                    quote!(, serialize_with = #serialize, deserialize_with = #deserialize),
                );
            }
            if matches!(
                field.field_type,
                Type::UtcDateTime {
                    format: DateTimeFormat::UnixTime,
                    wire_type: DateTimeWireType::Int32
                }
            ) {
                let with = if field.optional {
                    "azure_core::time::unix_time::option"
                } else {
                    "azure_core::time::unix_time"
                };
                attributes.append_all(quote!(, with = #with));
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
        if let Some((field, item_type)) = pages.get(model.name.as_str()) {
            let item_field = field_ident(field, &scope)?;
            let Type::Array { value_type } = item_type else {
                return Err(format!("{scope}: paging items must be an array"));
            };
            let item = rust_type(value_type, &scope, false)?;
            output.extend(quote! {
                #[async_trait::async_trait]
                impl azure_core::http::pager::Page for #name {
                    type Item = #item;
                    type IntoIter = <Vec<#item> as IntoIterator>::IntoIter;

                    async fn into_items(self) -> azure_core::Result<Self::IntoIter> {
                        Ok(self.#item_field.into_iter())
                    }
                }
            });
        }
    }
    Ok(output)
}
