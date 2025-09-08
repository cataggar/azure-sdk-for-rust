// LRO poller generation is currently disabled; LRO awaiting is handled in IntoFuture.
use heck::ToSnakeCase;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};

use crate::codegen::PARAM_RE;
use crate::Result;
use crate::{codegen::parse_path_params, identifier::SnakeCaseIdent};

use super::{new_request_code::NewRequestCode, response_code::ResponseCode, set_request_code::SetRequestCode};
/// The `send` function of the request builder.
pub struct RequestBuilderSendCode {
    new_request_code: NewRequestCode,
    request_builder: SetRequestCode,
    response_code: ResponseCode,
    url_args: Vec<Ident>,
    lro: bool,
}

impl RequestBuilderSendCode {
    pub fn new(new_request_code: NewRequestCode, request_builder: SetRequestCode, response_code: ResponseCode, lro: bool) -> Result<Self> {
        let params = parse_path_params(&new_request_code.path);
        let url_args: Result<Vec<_>> = params.iter().map(|s| s.to_snake_case_ident()).collect();
        let url_args = url_args?;
        Ok(Self {
            new_request_code,
            request_builder,
            response_code,
            url_args,
            lro,
        })
    }
}

impl ToTokens for RequestBuilderSendCode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let new_request_code = &self.new_request_code;
        let request_builder = &self.request_builder;

        let url_args = self.url_args.iter().map(|url_arg| {
            quote! { &self.#url_arg }
        });
        let url_str_args = quote! { #(#url_args),* };
        let fpath = PARAM_RE.replace_all(&new_request_code.path, "{}");
        let fpath_expr = if url_str_args.is_empty() {
            quote! { #fpath }
        } else {
            quote! { &format!(#fpath, #url_str_args) }
        };

        // match_status placeholder retained for compatibility; no longer used with custom Response removed
        let _match_status = TokenStream::new();

        let urlfn = if self.request_builder.has_param_api_version {
            let api_version = &self.request_builder.api_version;
            quote! {
                fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                    let mut url = self.client.endpoint().clone();
                    url.set_path(#fpath_expr);

                    let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                    if !has_api_version_already {
                        url.query_pairs_mut().append_pair(azure_core::http::headers::query_param::API_VERSION, #api_version);
                    }
                    Ok(url)
                }
            }
        } else {
            quote! {
                fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                    let mut url = self.client.endpoint().clone();
                    url.set_path(#fpath_expr);

                    Ok(url)
                }
            }
        };

        let send_future = if let Some(response_type) = self.response_code.response_type() {
            // Typed send(): returns azure_core::http::response::Response<T, JsonFormat>
            quote! {
                #[doc = "Returns a future that sends the request and returns a typed azure_core [`Response<T>`]."]
                #[doc = ""]
                #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
                pub fn send(self) -> BoxFuture<'static, azure_core::Result<azure_core::http::response::Response<#response_type, azure_core::http::JsonFormat>>> {
                    Box::pin({
                        let this = self.clone();
                        async move {
                            let url = this.url()?;
                            #new_request_code
                            #request_builder
                            let raw = this.client.send_raw(&mut req).await?;
                            let response: azure_core::http::response::Response<#response_type, azure_core::http::JsonFormat> = raw.into();
                            Ok(response)
                        }
                    })
                }
            }
        } else {
            // No typed body: return the raw response directly
            quote! {
                #[doc = "Returns a future that sends the request and returns the raw HTTP response."]
                pub fn send(self) -> BoxFuture<'static, azure_core::Result<()>> {
                    Box::pin({
                        let this = self.clone();
                        async move {
                            let url = this.url()?;
                            #new_request_code
                            #request_builder
                            this.client.send_raw(&mut req).await?;
                            Ok(())
                        }
                    })
                }
            }
        };

        let pager_tokens = if let Some(pageable) = &self.response_code.pageable {
            // TODO: Pageable requires the values to be part of the response schema,
            // however, some schemas do this via the header x-ms-continuation rather than
            // provide a next_link_name.  For now, those cases get documented that we don't
            // poll and move on.
            if let Some(next_link_name) = pageable.next_link_name.as_ref() {
                let mut stream_api_version = quote! {};

                // per discussion in SDK meeting, we should always set the
                // api-version on the request if we have a version.
                if request_builder.has_param_api_version {
                    let api_version = &request_builder.api_version;
                    stream_api_version.extend(quote! {
                        let has_api_version_already = req.url_mut().query_pairs().any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                        if !has_api_version_already {
                            req.url_mut().query_pairs_mut().append_pair(azure_core::http::headers::query_param::API_VERSION, #api_version);
                        }
                    });
                }

                if request_builder.has_param_x_ms_version {
                    let api_version = &request_builder.api_version;
                    stream_api_version.extend(quote! {
                        req.insert_header(azure_core::http::headers::VERSION, #api_version);
                    });
                }
                let response_type = self.response_code.response_type().expect("pageable response has a body");

                // some of the pageable requests specify the continuation token
                // as a parameter.  In this case, use the basic request builder,
                // but insert the continuation parameter
                if let Some(continuable_param) = get_continuable_param(next_link_name, request_builder) {
                    // Continuation token provided as a query parameter; build subsequent requests by reusing the builder and setting the token.
                    quote! {
                                #[doc = "Return a Pager over pages"]
                    pub fn pager(self) -> azure_core::Result<azure_core::http::pager::Pager<#response_type>> {
                                    let client = self.client.clone();
                                    let initial = self.clone();
                                    Ok(azure_core::http::pager::Pager::from_callback(move |state: azure_core::http::pager::PagerState<String>| {
                                        let client = client.clone();
                                        let initial = initial.clone();
                                        async move {
                                            let rsp = match state {
                                                azure_core::http::pager::PagerState::Initial => {
                                                    // Build the initial request and send it as raw to maintain access to bytes
                                                    let url = initial.url()?;
                                                    let mut req = typespec_client_core::http::request::Request::new(url, azure_core::http::Method::Get);
                                                    let bearer_token = client.bearer_token().await?;
                                                    req.insert_header(azure_core::http::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                                                    #stream_api_version
                                                    client.send_raw(&mut req).await?
                                                }
                                                azure_core::http::pager::PagerState::More(token) => {
                                                    let url = initial.url()?;
                                                    let mut req = typespec_client_core::http::request::Request::new(url, azure_core::http::Method::Get);
                                                    let bearer_token = client.bearer_token().await?;
                                                    req.insert_header(azure_core::http::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                                                    #stream_api_version
                                                    req.url_mut().query_pairs_mut().append_pair(#continuable_param, token.as_ref());
                                                    client.send_raw(&mut req).await?
                                                }
                                            };
                                            if !rsp.status().is_success() {
                                                return Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse { status: rsp.status(), error_code: None }));
                                            }
                                            let (status, headers, body) = rsp.deconstruct();
                                            let bytes = body.collect().await?;
                                            let page: #response_type = serde_json::from_slice(&bytes)?;
                                            // Clone bytes before moving into RawResponse so we can parse continuation after.
                                            let bytes_for_json = bytes.clone();
                                            let raw = azure_core::http::response::RawResponse::from_bytes(status, headers, bytes);
                                            let response: azure_core::http::response::Response<#response_type, azure_core::http::JsonFormat> = raw.into();
                                            // Extract continuation from the JSON field specified by x-ms-pageable.nextLinkName
                                            let continuation = serde_json::from_slice::<serde_json::Value>(&bytes_for_json)
                                                .ok()
                                                .and_then(|v| v.get(#next_link_name).and_then(|x| x.as_str()).map(|s| s.to_string()))
                                                .filter(|s| !s.is_empty());
                                            Ok(match continuation {
                                                Some(continuation) => azure_core::http::pager::PagerResult::More { response, continuation },
                                                None => azure_core::http::pager::PagerResult::Done { response },
                                            })
                                        }
                                    }))
                                }
                            }
                } else {
                    // Continuation link provided as a URL in the response body (next link); build subsequent requests from the absolute/relative URL.
                    quote! {
                                #[doc = "Return a Pager over pages"]
                    pub fn pager(self) -> azure_core::Result<azure_core::http::pager::Pager<#response_type>> {
                                    let client = self.client.clone();
                                    let initial = self.clone();
                                    Ok(azure_core::http::pager::Pager::from_callback(move |state: azure_core::http::pager::PagerState<String>| {
                                        let client = client.clone();
                                        let initial = initial.clone();
                                        async move {
                                            let rsp = match state {
                                                azure_core::http::pager::PagerState::Initial => {
                                                    // Build the initial request and send it as raw to maintain access to bytes
                                                    let url = initial.url()?;
                                                    let mut req = typespec_client_core::http::request::Request::new(url, azure_core::http::Method::Get);
                                                    let bearer_token = client.bearer_token().await?;
                                                    req.insert_header(azure_core::http::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                                                    #stream_api_version
                                                    client.send_raw(&mut req).await?
                                                }
                                                azure_core::http::pager::PagerState::More(next_url) => {
                                                    let mut url = client.endpoint().clone();
                                                    url.set_path("");
                                                    let url = url.join(next_url.as_ref())?;
                                                    let mut req = typespec_client_core::http::request::Request::new(url, azure_core::http::Method::Get);
                                                    let bearer_token = client.bearer_token().await?;
                                                    req.insert_header(azure_core::http::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                                                    #stream_api_version
                                                    client.send_raw(&mut req).await?
                                                }
                                            };
                                            if !rsp.status().is_success() {
                                                return Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse { status: rsp.status(), error_code: None }));
                                            }
                                            let (status, headers, body) = rsp.deconstruct();
                                            let bytes = body.collect().await?;
                                            let page: #response_type = serde_json::from_slice(&bytes)?;
                                            // Clone bytes before moving into RawResponse so we can parse continuation after.
                                            let bytes_for_json = bytes.clone();
                                            let raw = azure_core::http::response::RawResponse::from_bytes(status, headers, bytes);
                                            let response: azure_core::http::response::Response<#response_type, azure_core::http::JsonFormat> = raw.into();
                                            // Extract continuation from the JSON field specified by x-ms-pageable.nextLinkName
                                            let continuation = serde_json::from_slice::<serde_json::Value>(&bytes_for_json)
                                                .ok()
                                                .and_then(|v| v.get(#next_link_name).and_then(|x| x.as_str()).map(|s| s.to_string()))
                                                .filter(|s| !s.is_empty());
                                            Ok(match continuation {
                                                Some(continuation) => azure_core::http::pager::PagerResult::More { response, continuation },
                                                None => azure_core::http::pager::PagerResult::Done { response },
                                            })
                                        }
                                    }))
                                }
                            }
                }
            } else {
                // most often when this happens, the continuation token is provided
                // by an HTTP Header x-ms-continuation, which should be extracted
                // from the response.
                //
                // Note, this is only *sometimes* this is specified in the spec.
                //
                // Ref: https://github.com/Azure/azure-sdk-for-rust/issues/446
                quote! {}
            }
        } else {
            quote! {}
        };
        // Emit url(), send(), and optionally pager()
        tokens.extend(urlfn);
        tokens.extend(send_future);
        tokens.extend(pager_tokens);

        // Emit a generic LRO poller that returns an Operation wrapper implementing StatusMonitor.
        if self.lro {
            // Only emit when we can determine a response body type (to wrap inside Operation)
            if self.response_code.response_type().is_some() {
                let poller_tokens = quote! {
                    #[doc = "Return a Poller over an LRO"]
                    pub fn poller(self) -> azure_core::Result<azure_core::http::poller::Poller<Operation>> {
                        let client = self.client.clone();
                        let initial = self.clone();
                        Ok(azure_core::http::poller::Poller::from_callback(
                            move |state: azure_core::http::poller::PollerState<azure_core::http::Url>| {
                                let client = client.clone();
                                let initial = initial.clone();
                                async move {
                                    use azure_core::http::poller::{PollerResult, PollerState, StatusMonitor as _};
                                    use azure_core::json;
                                               let (rsp, next_link) = match state {
                                                   PollerState::Initial => {
                                                       // Build and send the initial request as raw
                                                       let url = initial.url()?;
                                                       let mut req = typespec_client_core::http::request::Request::new(url, azure_core::http::Method::Get);
                                                       let bearer_token = client.bearer_token().await?;
                                                       req.insert_header(azure_core::http::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                                                       let rsp = client.send_raw(&mut req).await?;
                                                       let next = initial.clone().url()?;
                                                       (rsp, next)
                                                   }
                                        PollerState::More(next_url) => {
                                            let mut req = typespec_client_core::http::request::Request::new(
                                                next_url.clone(),
                                                azure_core::http::Method::Get,
                                            );
                                            let bearer_token = client.bearer_token().await?;
                                            req.insert_header(
                                                azure_core::http::headers::AUTHORIZATION,
                                                format!("Bearer {}", bearer_token.secret()),
                                            );
                                            let rsp = client.send_raw(&mut req).await?;
                                            (rsp, next_url.clone())
                                        }
                                    };
                                    if !rsp.status().is_success() {
                                        return Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse { status: rsp.status(), error_code: None }));
                                    }
                                    let (status, headers, body) = rsp.deconstruct();
                                    let retry_after = azure_core::http::poller::get_retry_after(
                                        &headers,
                                        &[
                                            azure_core::http::headers::RETRY_AFTER_MS,
                                            azure_core::http::headers::X_MS_RETRY_AFTER_MS,
                                            azure_core::http::headers::RETRY_AFTER,
                                        ],
                                        &azure_core::http::poller::PollerOptions::default(),
                                    );
                                    let bytes = body.collect().await?;
                                    // Parse into Operation to determine status
                                    let op: Operation = json::from_json(&bytes)?;
                                    let response = azure_core::http::response::RawResponse::from_bytes(status, headers, bytes).into();
                                    Ok(match op.status() {
                                        azure_core::http::poller::PollerStatus::InProgress => PollerResult::InProgress { response, retry_after, next: next_link },
                                        _ => PollerResult::Done { response },
                                    })
                                }
                            },
                            None,
                        ))
                    }
                };
                tokens.extend(poller_tokens);
            }
        }
    }
}

fn get_continuable_param(next_link_name: &str, request_builder: &SetRequestCode) -> Option<String> {
    let next_link_name = next_link_name.to_snake_case();
    let link_name = next_link_name.strip_prefix("next_");

    for param in request_builder.parameters.params() {
        let param_name = param.variable_name.to_string();
        if param_name == next_link_name {
            return Some(param_name);
        }
        if let Some(link_name) = link_name {
            if param_name == link_name {
                return Some(param_name);
            }
        }
    }
    None
}
