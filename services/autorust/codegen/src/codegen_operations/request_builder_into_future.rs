use crate::Result;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use super::response_code::ResponseCode;

pub struct RequestBuilderIntoFutureCode {
    response_code: ResponseCode,
    lro: bool,
}

impl RequestBuilderIntoFutureCode {
    pub fn new(response_code: ResponseCode, lro: bool) -> Result<Self> {
        Ok(Self { response_code, lro })
    }
}

/// Adds the `IntoFuture` implementation to the `RequestBuilder` struct.
impl ToTokens for RequestBuilderIntoFutureCode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        // Do not emit IntoFuture for pageable operations; users should call `.pager()`.
        if self.response_code.pageable.is_some() {
            return;
        }
        // Do not emit IntoFuture for LROs; users should use `.poller()` or handle LRO explicitly.
        if self.lro {
            return;
        }

        if let Some(response_type) = self.response_code.response_type() {
            tokens.extend(quote! {
                impl std::future::IntoFuture for RequestBuilder {
                    type Output = azure_core::Result<#response_type>;
                    type IntoFuture = BoxFuture<'static, azure_core::Result<#response_type>>;
                    #[doc = "Returns a future that sends the request and returns the parsed response body."]
                    #[doc = ""]
                    #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
                    #[doc = ""]
                    #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
                    fn into_future(self) -> Self::IntoFuture {
                        Box::pin(async move { self.send().await?.into_body().await })
                    }
                }
            });
        }
    }
}
