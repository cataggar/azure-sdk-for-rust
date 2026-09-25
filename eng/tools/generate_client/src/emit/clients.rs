// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    emit::{field_ident, ident, types},
    model::{Client, Operation, Package, ParameterLocation, Type},
};
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashSet;

pub(super) fn render(package: &Package) -> Result<TokenStream, String> {
    let mut clients = Vec::new();
    let mut client_names = HashSet::new();
    for client in &package.clients {
        let scope = format!("client {}", client.name);
        if !client.children.is_empty() {
            return Err(format!(
                "{scope}: child clients are not supported in --preview-basic"
            ));
        }
        if client.operations.is_empty() {
            return Err(format!(
                "{scope}: clients without operations are not supported"
            ));
        }
        let name = ident(&client.name, &scope)?;
        let client_doc = client.doc.as_deref().unwrap_or(&client.name);
        if !client_names.insert(name.to_string()) {
            return Err(format!("{scope}: duplicate Rust client name"));
        }
        let mut operations = Vec::new();
        let mut methods = HashSet::new();
        for operation in &client.operations {
            let method = field_ident(&operation.name, &scope)?;
            if !methods.insert(method.to_string())
                || matches!(method.to_string().as_str(), "new" | "endpoint")
            {
                return Err(format!(
                    "{scope}: duplicate or reserved Rust method {}",
                    method
                ));
            }
            operations.push(render_operation(client, operation, &method)?);
        }
        clients.push(quote! {
            #[doc = #client_doc]
            ///
            /// The caller must supply a pipeline configured with the required authentication.
            pub struct #name {
                endpoint: Url,
                pipeline: Pipeline,
            }

            impl #name {
                /// Creates a client with the provided endpoint and pipeline.
                ///
                /// # Errors
                ///
                /// Returns an error if the endpoint has a query or fragment, or cannot be used as a base URL.
                pub fn new(endpoint: Url, pipeline: Pipeline) -> azure_core::Result<Self> {
                    if endpoint.query().is_some() || endpoint.fragment().is_some() || endpoint.cannot_be_a_base() {
                        return Err(azure_core::Error::with_message(
                            azure_core::error::ErrorKind::Other,
                            "client endpoint must be a base URL without a query or fragment",
                        ));
                    }
                    Ok(Self { endpoint, pipeline })
                }

                /// Returns the URL associated with this client.
                pub fn endpoint(&self) -> &Url {
                    &self.endpoint
                }

                #(#operations)*
            }
        });
    }
    Ok(quote! {
        use azure_core::{
            error::CheckSuccessOptions,
            http::{Context, Method, Pipeline, PipelineSendOptions, Request, Response, Url},
        };

        #(#clients)*
    })
}

fn render_operation(
    client: &Client,
    operation: &Operation,
    method_name: &syn::Ident,
) -> Result<TokenStream, String> {
    let scope = format!("{}.{}", client.name, operation.name);
    let method = match operation.http_method.as_str() {
        "GET" => quote!(Method::Get),
        "PUT" => quote!(Method::Put),
        "POST" => quote!(Method::Post),
        "PATCH" => quote!(Method::Patch),
        "DELETE" => quote!(Method::Delete),
        "HEAD" => quote!(Method::Head),
        _ => return Err(format!("{scope}: unsupported HTTP method")),
    };
    if operation
        .status_codes
        .iter()
        .any(|code| !matches!(code, 200 | 201 | 202 | 204 | 205))
    {
        return Err(format!(
            "{scope}: unsupported success status (response format is unknown)"
        ));
    }
    if operation.response_type.is_some()
        && operation
            .status_codes
            .iter()
            .any(|code| !matches!(code, 200 | 201))
    {
        return Err(format!(
            "{scope}: JSON response requires only 200/201 success statuses"
        ));
    }
    let response = match &operation.response_type {
        None => quote!(Response<(), azure_core::http::NoFormat>),
        Some(value) => {
            let ty = types::rust_type(value, &scope, false)?;
            let ty = match value {
                Type::Model { .. } | Type::Enum { .. } => quote!(crate::generated::models::#ty),
                Type::Array { .. } | Type::Dict { .. } | Type::Nullable { .. } => {
                    return Err(format!(
                        "{scope}: composite response types are not supported in --preview-basic"
                    ));
                }
                _ => quote!(#ty),
            };
            quote!(Response<#ty>)
        }
    };
    let mut args = Vec::new();
    let mut names = HashSet::new();
    let mut paths = Vec::new();
    let mut queries = Vec::new();
    let mut headers = Vec::new();
    let mut header_names = HashSet::new();
    let mut path_bindings = std::collections::HashMap::new();
    let mut has_accept = false;
    for parameter in &operation.parameters {
        let binding = format!("{scope}.{}", parameter.name);
        let name = field_ident(&parameter.name, &binding)?;
        if !names.insert(name.to_string()) || name == "context" {
            return Err(format!(
                "{binding}: duplicate or reserved Rust parameter name"
            ));
        }
        if parameter.constant.is_some() && parameter.optional {
            return Err(format!("{binding}: an optional constant is ambiguous"));
        }
        let value = if let Some(constant) = &parameter.constant {
            validate_constant(constant, &parameter.parameter_type, &binding)?;
            quote!(#constant)
        } else {
            let ty = match &parameter.parameter_type {
                Type::String => quote!(&str),
                Type::Boolean => quote!(bool),
                Type::Int32 => quote!(i32),
                Type::Int64 => quote!(i64),
                Type::Float64 => quote!(f64),
                _ => return Err(format!("{binding}: unsupported HTTP parameter")),
            };
            if parameter.optional {
                args.push(quote!(#name: Option<#ty>));
            } else {
                args.push(quote!(#name: #ty));
            }
            quote!(#name.to_string())
        };
        match parameter.location {
            ParameterLocation::Path => {
                if parameter.optional {
                    return Err(format!("{binding}: optional path parameter"));
                }
                let finite = (parameter.constant.is_none() && matches!(parameter.parameter_type, Type::Float64)).then(|| quote! {
                    if !#name.is_finite() {
                        return Err(azure_core::Error::with_message(azure_core::error::ErrorKind::Other, "non-finite HTTP parameter"));
                    }
                });
                path_bindings.insert(parameter.wire_name.as_str(), (value, finite));
            }
            ParameterLocation::Query => {
                let wire = &parameter.wire_name;
                if parameter.constant.is_some() {
                    queries
                        .push(quote!(_generated_url.query_pairs_mut().append_pair(#wire, #value);));
                } else {
                    let optional_finite = matches!(parameter.parameter_type, Type::Float64).then(|| quote! {
                        if !#name.is_finite() {
                            return Err(azure_core::Error::with_message(azure_core::error::ErrorKind::Other, "non-finite HTTP parameter"));
                        }
                    });
                    if parameter.optional {
                        let finite = matches!(parameter.parameter_type, Type::Float64).then(|| quote! {
                            if !value.is_finite() {
                                return Err(azure_core::Error::with_message(azure_core::error::ErrorKind::Other, "non-finite HTTP parameter"));
                            }
                        });
                        queries.push(quote! {
                            if let Some(value) = #name {
                                #finite
                                _generated_url.query_pairs_mut().append_pair(#wire, &value.to_string());
                            }
                        });
                    } else {
                        queries.push(quote! { #optional_finite _generated_url.query_pairs_mut().append_pair(#wire, &#name.to_string()); });
                    }
                }
            }
            ParameterLocation::Header => {
                let wire = parameter.wire_name.to_ascii_lowercase();
                if !header_names.insert(wire.clone()) {
                    return Err(format!(
                        "{binding}: duplicate case-insensitive header binding"
                    ));
                }
                if !valid_header_name(&wire)
                    || matches!(
                        wire.as_str(),
                        "host"
                            | "content-length"
                            | "transfer-encoding"
                            | "authorization"
                            | "proxy-authorization"
                    )
                {
                    return Err(format!("{binding}: unsupported or invalid header name"));
                }
                if wire == "accept" {
                    has_accept = true;
                    if operation.response_type.is_some()
                        && parameter.constant.as_deref() != Some("application/json")
                    {
                        return Err(format!("{binding}: JSON response requires a constant application/json accept header"));
                    }
                }
                if let Some(constant) = &parameter.constant {
                    if constant.chars().any(char::is_control) {
                        return Err(format!("{binding}: invalid constant header value"));
                    }
                    headers.push(quote!(_generated_request.insert_header(#wire, #constant);));
                } else {
                    let optional_finite = matches!(parameter.parameter_type, Type::Float64).then(|| quote! {
                        if !value.is_finite() {
                            return Err(azure_core::Error::with_message(azure_core::error::ErrorKind::Other, "non-finite HTTP parameter"));
                        }
                    });
                    if parameter.optional {
                        headers.push(quote! {
                            if let Some(value) = #name {
                                #optional_finite
                                let value = value.to_string();
                                if value.chars().any(char::is_control) {
                                    return Err(azure_core::Error::with_message(azure_core::error::ErrorKind::Other, "invalid header value"));
                                }
                                _generated_request.insert_header(#wire, value);
                            }
                        });
                    } else {
                        let finite = matches!(parameter.parameter_type, Type::Float64).then(|| quote! {
                            if !#name.is_finite() {
                                return Err(azure_core::Error::with_message(azure_core::error::ErrorKind::Other, "non-finite HTTP parameter"));
                            }
                        });
                        headers.push(quote! {
                            #finite
                            let value = #name.to_string();
                            if value.chars().any(char::is_control) {
                                return Err(azure_core::Error::with_message(azure_core::error::ErrorKind::Other, "invalid header value"));
                            }
                            _generated_request.insert_header(#wire, value);
                        });
                    }
                }
            }
        }
    }
    if operation.response_type.is_some() && !has_accept {
        headers.push(quote!(_generated_request.insert_header("accept", "application/json");));
    }
    if operation.http_method == "HEAD" && operation.response_type.is_some() {
        return Err(format!("{scope}: HEAD cannot return a JSON body"));
    }
    let segments: Vec<_> = operation
        .path
        .strip_prefix('/')
        .unwrap_or(&operation.path)
        .split('/')
        .collect();
    if segments[..segments.len() - 1]
        .iter()
        .any(|segment| segment.is_empty())
    {
        return Err(format!("{scope}: empty path segment is not supported"));
    }
    for segment in segments {
        if segment.is_empty() {
            paths.push(quote!(_generated_segments.push("");));
        } else if segment.starts_with('{') && segment.ends_with('}') && segment.len() > 2 {
            let wire = &segment[1..segment.len() - 1];
            let (value, finite) = path_bindings
                .remove(wire)
                .ok_or_else(|| format!("{scope}: missing path binding {wire}"))?;
            paths.push(quote! {
                {
                    #finite
                    let value = #value;
                    if value.is_empty() || value == "." || value == ".." {
                        return Err(azure_core::Error::with_message(azure_core::error::ErrorKind::Other, "invalid path segment"));
                    }
                    _generated_segments.push(&value);
                }
            });
        } else if segment.contains(['{', '}'])
            || segment == "."
            || segment == ".."
            || !segment
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || b"-._~".contains(&byte))
        {
            return Err(format!(
                "{scope}: unsupported path literal or composite binding '{segment}'"
            ));
        } else {
            paths.push(quote!(_generated_segments.push(#segment);));
        }
    }
    if !path_bindings.is_empty() {
        return Err(format!("{scope}: unconsumed path bindings"));
    }
    let statuses = &operation.status_codes;
    let operation_doc = operation.doc.as_deref().unwrap_or(&operation.name);
    Ok(quote! {
        #[doc = #operation_doc]
        ///
        /// # Errors
        ///
        /// Returns an error if a path or header value is invalid, the request fails, or the response has an unexpected status.
        pub async fn #method_name(&self, context: &Context<'_>, #(#args),*) -> azure_core::Result<#response> {
            let mut _generated_url = self.endpoint.clone();
            {
                let mut _generated_segments = _generated_url.path_segments_mut().map_err(|_| {
                    azure_core::Error::with_message(azure_core::error::ErrorKind::Other, "endpoint cannot contain path segments")
                })?;
                _generated_segments.pop_if_empty();
                #(#paths)*
            }
            #(#queries)*
            let mut _generated_request = Request::new(_generated_url, #method);
            #(#headers)*
            let _generated_response = self.pipeline.send(
                context,
                &mut _generated_request,
                Some(PipelineSendOptions {
                    check_success: CheckSuccessOptions { success_codes: &[#(#statuses),*] },
                    ..Default::default()
                }),
            ).await?;
            Ok(_generated_response.into())
        }
    })
}

fn valid_header_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || b"!#$%&'*+-.^_`|~".contains(&byte))
}

fn validate_constant(value: &str, ty: &Type, scope: &str) -> Result<(), String> {
    let valid = match ty {
        Type::String => true,
        Type::Boolean => value.parse::<bool>().is_ok(),
        Type::Int32 => value.parse::<i32>().is_ok(),
        Type::Int64 => value.parse::<i64>().is_ok(),
        Type::Float64 => value.parse::<f64>().is_ok_and(f64::is_finite),
        _ => false,
    };
    if valid {
        Ok(())
    } else {
        Err(format!(
            "{scope}: invalid or unsupported constant for parameter type"
        ))
    }
}
