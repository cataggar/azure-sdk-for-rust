use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::codegen::TypeNameCode;

use super::response_code::ResponseCode;

#[derive(Clone)]
pub struct OperationTypeCode {
    response_code: ResponseCode,
    lro: bool,
}

impl OperationTypeCode {
    pub fn new(response_code: ResponseCode, lro: bool) -> Self {
        Self { response_code, lro }
    }
}

impl ToTokens for OperationTypeCode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if !self.lro {
            return;
        }
        if let Some(response_type) = self.response_code.response_type() {
            // Generate an Operation type that is a transparent wrapper over the response model
            // and implements StatusMonitor by inspecting common ARM provisioning state fields.
            let response_ty: TypeNameCode = response_type;
            tokens.extend(quote! {
                #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
                #[serde(transparent)]
                pub struct Operation(pub #response_ty);

                impl azure_core::http::poller::StatusMonitor for Operation {
                    type Output = #response_ty;
                    fn status(&self) -> azure_core::http::poller::PollerStatus {
                        fn map_status(s: &str) -> azure_core::http::poller::PollerStatus {
                            match s.to_ascii_lowercase().as_str() {
                                "succeeded" => azure_core::http::poller::PollerStatus::Succeeded,
                                "failed" => azure_core::http::poller::PollerStatus::Failed,
                                "canceled" | "cancelled" => azure_core::http::poller::PollerStatus::Canceled,
                                // treat unknown/missing as in-progress
                                _ => azure_core::http::poller::PollerStatus::InProgress,
                            }
                        }
                        fn find_status(value: &serde_json::Value) -> Option<azure_core::http::poller::PollerStatus> {
                            // Common ARM shapes: properties.provisioningState or top-level provisioningState/status
                            if let Some(ps) = value
                                .get("properties")
                                .and_then(|p| p.get("provisioningState"))
                                .and_then(|v| v.as_str())
                            {
                                return Some(map_status(ps));
                            }
                            if let Some(ps) = value.get("provisioningState").and_then(|v| v.as_str()) {
                                return Some(map_status(ps));
                            }
                            if let Some(ps) = value.get("status").and_then(|v| v.as_str()) {
                                return Some(map_status(ps));
                            }
                            None
                        }
                        match serde_json::to_value(&self.0).ok().and_then(|v| find_status(&v)) {
                            Some(s) => s,
                            None => azure_core::http::poller::PollerStatus::InProgress,
                        }
                    }
                }
            });
        }
    }
}
