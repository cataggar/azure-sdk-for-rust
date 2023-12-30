use crate::env::Env;
use azure_core::authority_hosts::AZURE_PUBLIC_CLOUD;
use std::sync::Arc;
use url::Url;

/// Provides options to configure how the Identity library makes authentication
/// requests to Azure Active Directory.
#[derive(Debug, Clone)]
pub struct TokenCredentialOptions {
    env: Env,
    http_client: Arc<dyn azure_core::HttpClient>,
    authority_host: Url,
}

impl Default for TokenCredentialOptions {
    fn default() -> Self {
        Self {
            env: Env::default(),
            http_client: azure_core::new_http_client(),
            authority_host: AZURE_PUBLIC_CLOUD.to_owned(),
        }
    }
}

impl TokenCredentialOptions {
    /// Create a new `TokenCredentialsOptions`. `default()` may also be used.
    pub fn new(
        env: Env,
        http_client: Arc<dyn azure_core::HttpClient>,
        authority_host: Url,
    ) -> Self {
        Self {
            env,
            http_client,
            authority_host,
        }
    }
    /// Set the authority host for authentication requests.
    pub fn set_authority_host(&mut self, authority_host: Url) {
        self.authority_host = authority_host;
    }

    /// The authority host to use for authentication requests.  The default is
    /// `https://login.microsoftonline.com`.
    pub fn authority_host(&self) -> &Url {
        &self.authority_host
    }

    pub fn http_client(&self) -> Arc<dyn azure_core::HttpClient> {
        self.http_client.clone()
    }

    pub fn env(&self) -> &Env {
        &self.env
    }
}

impl From<Arc<dyn azure_core::HttpClient>> for TokenCredentialOptions {
    fn from(http_client: Arc<dyn azure_core::HttpClient>) -> Self {
        Self {
            http_client,
            ..Default::default()
        }
    }
}
