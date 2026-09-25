// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    emit::{field_ident, ident, types},
    model::{
        Authentication, Client, OAuth2Flow, Operation, OperationKind, Package, ParameterLocation,
        Type,
    },
};
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashSet;

pub(super) fn render(package: &Package) -> Result<TokenStream, String> {
    let mut clients = Vec::new();
    let mut client_names = HashSet::new();
    let mut roots: Vec<_> = package.clients.iter().collect();
    roots.sort_by(|a, b| a.name.cmp(&b.name));
    for client in roots {
        render_client(client, &mut clients, &mut client_names)?;
    }
    fn kinds(client: &Client) -> (bool, bool) {
        let mut basic = client
            .operations
            .iter()
            .any(|operation| matches!(operation.kind, OperationKind::Basic));
        let mut paging = client
            .operations
            .iter()
            .any(|operation| matches!(operation.kind, OperationKind::Paging));
        for child in &client.children {
            let (child_basic, child_paging) = kinds(child);
            basic |= child_basic;
            paging |= child_paging;
        }
        (basic, paging)
    }
    let (basic, paging) = package
        .clients
        .iter()
        .fold((false, false), |(basic, paging), client| {
            let (client_basic, client_paging) = kinds(client);
            (basic || client_basic, paging || client_paging)
        });
    let basic_imports = basic.then(|| {
        quote!(
            use azure_core::http::{Context, Response};
        )
    });
    let paging_import = paging.then(|| {
        quote!(
            use azure_core::http::UrlExt;
        )
    });
    Ok(quote! {
        use azure_core::{
            error::CheckSuccessOptions,
            http::{Method, Pipeline, PipelineSendOptions, Request, Url},
        };
        #basic_imports
        #paging_import

        #(#clients)*
    })
}

fn render_client(
    client: &Client,
    clients: &mut Vec<TokenStream>,
    client_names: &mut HashSet<String>,
) -> Result<(), String> {
    let scope = format!("client {}", client.name);
    if client.operations.is_empty() && client.children.is_empty() {
        return Err(format!("{scope}: client has no operations or children"));
    }
    let name = client_ident(&client.name)?;
    let client_doc = client.doc.as_deref().unwrap_or(&client.name);
    let authentication_doc = match &client.authentication {
        Some(Authentication::Bearer) => {
            "The caller must supply a pipeline configured with bearer-token authentication.".to_string()
        }
        Some(Authentication::OAuth2 {
            flow,
            authorization_url,
            scopes,
        }) => match flow {
            OAuth2Flow::Implicit => format!(
                "The caller must supply a pipeline configured with OAuth2 implicit-flow token authentication (authorization URL: {authorization_url}) for the scopes: {}.",
                scopes.join(", ")
            ),
        },
        None => "The caller must supply a pipeline configured for this service.".to_string(),
    };
    let api_version_field = client
        .api_version
        .as_ref()
        .map(|_| quote!(api_version: String,));
    let api_version_init = client.api_version.as_ref().map(|version| {
        let default = &version.default;
        quote!(api_version: #default.to_string(),)
    });
    let api_version_override = client.api_version.as_ref().map(|_| quote! {
        /// Overrides the API version used in operation requests.
        ///
        /// # Errors
        ///
        /// Returns an error for an empty or invalid version.
        pub fn with_api_version(mut self, api_version: impl Into<String>) -> azure_core::Result<Self> {
            let api_version = api_version.into();
            if api_version.is_empty() || api_version.chars().any(char::is_control) {
                return Err(azure_core::Error::with_message(
                    azure_core::error::ErrorKind::Other,
                    "invalid API version",
                ));
            }
            self.api_version = api_version;
            Ok(self)
        }
    });
    if !client_names.insert(name.to_string()) {
        return Err(format!("{scope}: duplicate Rust client name"));
    }
    let mut methods = HashSet::from(["new".to_string(), "endpoint".to_string()]);
    if client.api_version.is_some() {
        methods.insert("with_api_version".to_string());
    }
    let mut operations = Vec::new();
    let mut sorted_operations: Vec<_> = client.operations.iter().collect();
    sorted_operations.sort_by(|a, b| a.name.cmp(&b.name));
    for operation in sorted_operations {
        let method = field_ident(&operation.name, &scope)?;
        if !methods.insert(method.to_string()) {
            return Err(format!(
                "{scope}: duplicate or reserved Rust method {method}"
            ));
        }
        operations.push(render_operation(client, operation, &method)?);
    }
    let mut accessors = Vec::new();
    let mut children: Vec<_> = client.children.iter().collect();
    children.sort_by(|a, b| a.name.cmp(&b.name));
    for child in children {
        if child.api_version.is_some()
            && client.api_version.as_ref().map(|v| &v.name)
                != child.api_version.as_ref().map(|v| &v.name)
        {
            return Err(format!(
                "{scope}: child {} has an unrelated API version",
                child.name
            ));
        }
        let child_api_version = child
            .api_version
            .as_ref()
            .map(|_| quote!(api_version: self.api_version.clone(),));
        let child_name = client_ident(&child.name)?;
        let method = field_ident(
            &format!("get{}Client", child.name.trim_end_matches("Client")),
            &scope,
        )?;
        if !methods.insert(method.to_string()) {
            return Err(format!(
                "{scope}: duplicate or reserved Rust method {method}"
            ));
        }
        let accessor_doc = format!("Returns the {} subclient.", child.name);
        accessors.push(quote! {
            #[doc = #accessor_doc]
            pub fn #method(&self) -> #child_name {
                #child_name {
                    endpoint: self.endpoint.clone(),
                    pipeline: self.pipeline.clone(),
                    #child_api_version
                }
            }
        });
        render_client(child, clients, client_names)?;
    }
    clients.push(quote! {
            #[doc = #client_doc]
            ///
            #[doc = #authentication_doc]
            pub struct #name {
                endpoint: Url,
                pipeline: Pipeline,
                #api_version_field
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
                    Ok(Self { endpoint, pipeline, #api_version_init })
                }

                /// Returns the URL associated with this client.
                pub fn endpoint(&self) -> &Url {
                    &self.endpoint
                }

                #api_version_override
                #(#operations)*
                #(#accessors)*
            }
    });
    Ok(())
}

fn client_ident(name: &str) -> Result<syn::Ident, String> {
    let name_with_suffix = if name.ends_with("Client") {
        name.to_owned()
    } else {
        format!("{name}Client")
    };
    ident(&name_with_suffix, &format!("client {name}"))
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
            let ty = response_type(value, &scope)?;
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
    let mut has_content_type = false;
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
        let value = if parameter.client_owned {
            quote!(self.api_version.as_str())
        } else if let Some(constant) = &parameter.constant {
            validate_constant(constant, &parameter.parameter_type, &binding)?;
            quote!(#constant)
        } else {
            let ty = match &parameter.parameter_type {
                Type::String => quote!(&str),
                Type::Boolean => quote!(bool),
                Type::Int32 => quote!(i32),
                Type::Int64 => quote!(i64),
                Type::Float64 => quote!(f64),
                Type::Enum { name } => {
                    if parameter.location != ParameterLocation::Query {
                        return Err(format!(
                            "{binding}: enum parameters require a query binding"
                        ));
                    }
                    let name = ident(name, &binding)?;
                    quote!(&crate::generated::models::#name)
                }
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
                let value = if parameter.optional {
                    if !matches!(parameter.parameter_type, Type::String)
                        || !operation
                            .path
                            .ends_with(&format!("/{{{}}}", parameter.wire_name))
                    {
                        return Err(format!("{binding}: unsupported optional path parameter"));
                    }
                    quote!(#name.unwrap_or("").to_string())
                } else {
                    value
                };
                let finite = (parameter.constant.is_none() && matches!(parameter.parameter_type, Type::Float64)).then(|| quote! {
                    if !#name.is_finite() {
                        return Err(azure_core::Error::with_message(azure_core::error::ErrorKind::Other, "non-finite HTTP parameter"));
                    }
                });
                path_bindings.insert(
                    parameter.wire_name.as_str(),
                    (value, finite, parameter.optional),
                );
            }
            ParameterLocation::Query => {
                let wire = &parameter.wire_name;
                if parameter.client_owned || parameter.constant.is_some() {
                    queries
                        .push(quote!(_generated_url.query_pairs_mut().append_pair(#wire, #value);));
                } else {
                    let optional_finite = matches!(parameter.parameter_type, Type::Float64).then(|| quote! {
                        if !#name.is_finite() {
                            return Err(azure_core::Error::with_message(azure_core::error::ErrorKind::Other, "non-finite HTTP parameter"));
                        }
                    });
                    if parameter.optional {
                        let text = if matches!(parameter.parameter_type, Type::Enum { .. }) {
                            quote!(value.as_ref())
                        } else {
                            quote!(&value.to_string())
                        };
                        let finite = matches!(parameter.parameter_type, Type::Float64).then(|| quote! {
                            if !value.is_finite() {
                                return Err(azure_core::Error::with_message(azure_core::error::ErrorKind::Other, "non-finite HTTP parameter"));
                            }
                        });
                        queries.push(quote! {
                            if let Some(value) = #name {
                                #finite
                                _generated_url.query_pairs_mut().append_pair(#wire, #text);
                            }
                        });
                    } else {
                        let text = if matches!(parameter.parameter_type, Type::Enum { .. }) {
                            quote!(#name.as_ref())
                        } else {
                            quote!(&#name.to_string())
                        };
                        queries.push(quote! { #optional_finite _generated_url.query_pairs_mut().append_pair(#wire, #text); });
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
                if wire == "content-type" {
                    has_content_type = true;
                    if operation.body.is_some()
                        && parameter.constant.as_deref() != Some("application/json")
                    {
                        return Err(format!(
                            "{binding}: JSON body requires a constant application/json content type"
                        ));
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
    let body_request = if let Some(body) = &operation.body {
        let binding = format!("{scope}.{}", body.name);
        let name = field_ident(&body.name, &binding)?;
        if !names.insert(name.to_string()) || name == "context" {
            return Err(format!(
                "{binding}: duplicate or reserved Rust parameter name"
            ));
        }
        let Type::Model { name: model } = &body.body_type else {
            return Err(format!("{binding}: unsupported JSON request body"));
        };
        if body.optional {
            return Err(format!("{binding}: optional request body is not supported"));
        }
        let model = ident(model, &binding)?;
        args.push(quote!(#name: &crate::generated::models::#model));
        if !has_content_type {
            headers.push(
                quote!(_generated_request.insert_header("content-type", "application/json");),
            );
        }
        Some(quote!(_generated_request.set_json(#name)?;))
    } else {
        None
    };
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
            let (value, finite, optional) = path_bindings
                .remove(wire)
                .ok_or_else(|| format!("{scope}: missing path binding {wire}"))?;
            let nonempty = (!optional).then(|| quote!(value.is_empty() ||));
            paths.push(quote! {
                {
                    #finite
                    let value = #value;
                    if #nonempty value == "." || value == ".." {
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
    if let OperationKind::Paging = operation.kind {
        let Some(paging) = &operation.paging else {
            return Err(format!("{scope}: missing paging metadata"));
        };
        if operation
            .parameters
            .iter()
            .any(|param| param.location == ParameterLocation::Header && param.constant.is_none())
        {
            return Err(format!(
                "{scope}: variable paging headers are not supported"
            ));
        }
        let Some(Type::Model { name }) = &operation.response_type else {
            return Err(format!("{scope}: paging response must be a model"));
        };
        let page = ident(name, &scope)?;
        let next_wire = &paging.next_link.wire_name;
        return Ok(quote! {
            #[doc = #operation_doc]
            ///
            /// # Errors
            ///
            /// Returns an error for invalid request parameters or endpoint path.
            pub fn #method_name(&self, options: Option<azure_core::http::pager::PagerOptions<'static>>, #(#args),*) -> azure_core::Result<azure_core::http::Pager<crate::generated::models::#page>> {
                let mut _generated_url = self.endpoint.clone();
                {
                    let mut _generated_segments = _generated_url.path_segments_mut().map_err(|_| {
                        azure_core::Error::with_message(azure_core::error::ErrorKind::Other, "endpoint cannot contain path segments")
                    })?;
                    _generated_segments.pop_if_empty();
                    #(#paths)*
                }
                #(#queries)*
                let first_url = _generated_url;
                let pipeline = self.pipeline.clone();
                let api_version = self.api_version.clone();
                #[derive(serde::Deserialize)]
                struct PageLink {
                    #[serde(rename = #next_wire)]
                    next_link: Option<String>,
                }
                Ok(azure_core::http::Pager::new(
                    move |state, pager_options| {
                        let pipeline = pipeline.clone();
                        let first_url = first_url.clone();
                        let api_version = api_version.clone();
                        Box::pin(async move {
                            let url = match state {
                                azure_core::http::pager::PagerState::Initial => first_url.clone(),
                                azure_core::http::pager::PagerState::More(link) => {
                                    let mut url: Url = link.try_into()?;
                                    let mut query = url.query_builder();
                                    query.set_pair("api-version", &api_version);
                                    query.build();
                                    url
                                }
                            };
                            let mut _generated_request = Request::new(url, Method::Get);
                            #(#headers)*
                            let response = pipeline.send(
                                &pager_options.context,
                                &mut _generated_request,
                                Some(PipelineSendOptions {
                                    check_success: CheckSuccessOptions { success_codes: &[#(#statuses),*] },
                                    ..Default::default()
                                }),
                            ).await?;
                            let (status, headers, body) = response.deconstruct();
                            let link: PageLink = azure_core::json::from_json(&body)?;
                            let response = azure_core::http::RawResponse::from_bytes(status, headers, body).into();
                            Ok(match link.next_link {
                                Some(link) if !link.is_empty() => azure_core::http::pager::PagerResult::More {
                                    response,
                                    continuation: azure_core::http::pager::PagerContinuation::Link(first_url.join(&link)?),
                                },
                                _ => azure_core::http::pager::PagerResult::Done { response },
                            })
                        })
                    },
                    options,
                ))
            }
        });
    }
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
            #body_request
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

fn response_type(value: &Type, scope: &str) -> Result<TokenStream, String> {
    match value {
        Type::Model { name } | Type::Enum { name } => {
            let name = ident(name, scope)?;
            Ok(quote!(crate::generated::models::#name))
        }
        Type::Array { value_type } => {
            let inner = response_type(value_type, scope)?;
            Ok(quote!(Vec<#inner>))
        }
        Type::Dict { value_type } => {
            let inner = response_type(value_type, scope)?;
            Ok(quote!(std::collections::HashMap<String, #inner>))
        }
        Type::Nullable { value_type } => {
            let inner = response_type(value_type, scope)?;
            Ok(quote!(Option<#inner>))
        }
        _ => {
            let ty = types::rust_type(value, scope, false)?;
            Ok(quote!(#ty))
        }
    }
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
