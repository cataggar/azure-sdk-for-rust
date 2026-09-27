// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    emit::{clients::client_ident, field_ident, ident, variant_ident},
    model::{Client, Operation, OperationKind, Package, Type},
};
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashSet;

pub(super) fn name(client: &Client, operation: &Operation) -> Result<syn::Ident, String> {
    let client_name = client_ident(&client.name)?;
    let operation_name = variant_ident(&operation.name, &operation.name)?;
    ident(
        &format!("{client_name}{operation_name}Options"),
        &format!("{}.{}", client.name, operation.name),
    )
}

pub(super) fn render(package: &Package) -> Result<TokenStream, String> {
    let mut declarations = Vec::new();
    let mut basic = false;
    let mut paging = false;
    let mut names: HashSet<_> = package
        .models
        .iter()
        .map(|model| model.name.clone())
        .chain(
            package
                .enums
                .iter()
                .map(|enumeration| enumeration.name.clone()),
        )
        .collect();
    for client in &package.clients {
        render_client(
            client,
            &mut declarations,
            &mut names,
            &mut basic,
            &mut paging,
        )?;
    }
    let basic_import = basic.then(|| {
        quote!(
            use azure_core::http::ClientMethodOptions;
        )
    });
    let paging_import = paging.then(|| {
        quote!(
            use azure_core::http::pager::PagerOptions;
        )
    });
    Ok(quote! {
        use azure_core::fmt::SafeDebug;
        #basic_import
        #paging_import
        #(#declarations)*
    })
}

fn render_client(
    client: &Client,
    declarations: &mut Vec<TokenStream>,
    names: &mut HashSet<String>,
    basic: &mut bool,
    paging: &mut bool,
) -> Result<(), String> {
    let mut operations: Vec<_> = client.operations.iter().collect();
    operations.sort_by(|a, b| a.name.cmp(&b.name));
    for operation in operations {
        let scope = format!("{}.{}", client.name, operation.name);
        let name = name(client, operation)?;
        if !names.insert(name.to_string()) {
            return Err(format!("{scope}: duplicate Rust options type {name}"));
        }
        let mut fields = Vec::new();
        let mut owned_fields = Vec::new();
        let mut field_names = HashSet::from(["method_options".to_string()]);
        for parameter in &operation.parameters {
            if !parameter.optional || parameter.constant.is_some() || parameter.client_owned {
                continue;
            }
            let binding = format!("{scope}.{}", parameter.name);
            let field = field_ident(&parameter.name, &binding)?;
            if !field_names.insert(field.to_string()) {
                return Err(format!(
                    "{binding}: duplicate or reserved Rust options field"
                ));
            }
            let ty = match &parameter.parameter_type {
                Type::String => quote!(String),
                Type::Boolean => quote!(bool),
                Type::Int32 => quote!(i32),
                Type::Int64 => quote!(i64),
                Type::Float64 => quote!(f64),
                Type::Enum { name } => {
                    let enumeration = ident(name, &binding)?;
                    quote!(crate::generated::models::#enumeration)
                }
                _ => return Err(format!("{binding}: unsupported HTTP parameter")),
            };
            fields.push(quote!(pub #field: Option<#ty>,));
            owned_fields.push(quote!(#field: self.#field,));
        }
        let method_options = match operation.kind {
            OperationKind::Basic => {
                *basic = true;
                quote!(ClientMethodOptions<'a>)
            }
            OperationKind::Paging => {
                *paging = true;
                quote!(PagerOptions<'a>)
            }
        };
        let owned = if matches!(operation.kind, OperationKind::Paging) {
            Some(quote! {
                impl #name<'_> {
                    /// Converts the options to owned values for paging.
                    pub fn into_owned(self) -> #name<'static> {
                        #name {
                            #(#owned_fields)*
                            method_options: PagerOptions {
                                context: self.method_options.context.into_owned(),
                                ..self.method_options
                            },
                        }
                    }
                }
            })
        } else {
            None
        };
        declarations.push(quote! {
            #[doc = concat!("Options for ", stringify!(#name), ".")]
            #[derive(Clone, Default, SafeDebug)]
            pub struct #name<'a> {
                /// Allows customization of the method call.
                pub method_options: #method_options,
                #(#fields)*
            }
            #owned
        });
    }
    for child in &client.children {
        render_client(child, declarations, names, basic, paging)?;
    }
    Ok(())
}
