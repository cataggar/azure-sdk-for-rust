#![allow(unused)]
use super::cache::TokenCache;
use crate::TokenCredentialOptions;
use azure_core::auth::{AccessToken, TokenCredential};
use azure_core::error::{Error, ErrorKind, ResultExt};
use azure_core::{auth::Secret, HttpClient};
use std::sync::Arc;
use url::Url;

/// Enables authentication to Microsoft Entra ID using a user's username and password.
/// This credential requires a high degree of trust and is not recommended outside of prototyping
/// when more secure credentials can be used.
#[derive(Debug)]
pub struct UsernamePasswordCredential {
    /// The Azure Active Directory tenant (directory) ID or name.
    tenant_id: String,
    /// The client (application) ID of an App Registration in the tenant.
    client_id: String,
    /// The user account's user name, UPN.
    username: String,
    /// The user account's password.
    password: Secret,
    /// The http client used to perform requests.
    http_client: Arc<dyn HttpClient>,
    /// The authority host to authenticate against.
    authority_host: Url,
    /// The token cache.
    cache: TokenCache,
}

const AZURE_TENANT_ID_ENV_KEY: &str = "AZURE_TENANT_ID";
const AZURE_CLIENT_ID_ENV_KEY: &str = "AZURE_CLIENT_ID";
const AZURE_USERNAME_ENV_KEY: &str = "AZURE_USERNAME";
const AZURE_PASSWORD_ENV_KEY: &str = "AZURE_PASSWORD";
const AZURE_AUTHORITY_HOST: &str = "AZURE_AUTHORITY_HOST";

impl UsernamePasswordCredential {
    /// Create a new `UsernamePasswordCredential`
    pub fn new(
        options: impl Into<TokenCredentialOptions>,
        tenant_id: String,
        client_id: String,
        username: String,
        password: Secret,
    ) -> azure_core::Result<UsernamePasswordCredential> {
        let options = options.into();
        let http_client = options.http_client().clone();
        let authority_host = options.authority_host().clone();
        Ok(UsernamePasswordCredential {
            client_id,
            username,
            password,
            tenant_id,
            authority_host,
            http_client,
            cache: TokenCache::new(),
        })
    }

    pub fn create(
        options: impl Into<TokenCredentialOptions>,
    ) -> Result<UsernamePasswordCredential, Error> {
        let mut options = options.into();
        let env = options.env();
        let tenant_id = env
            .var(AZURE_TENANT_ID_ENV_KEY)
            .map_kind(ErrorKind::Credential)?;
        let client_id = env
            .var(AZURE_CLIENT_ID_ENV_KEY)
            .map_kind(ErrorKind::Credential)?;
        let username = env
            .var(AZURE_USERNAME_ENV_KEY)
            .map_kind(ErrorKind::Credential)?;
        let password = env
            .var(AZURE_PASSWORD_ENV_KEY)
            .map_kind(ErrorKind::Credential)?;

        let authority_host = env.var(AZURE_AUTHORITY_HOST);
        if let Ok(authority_host) = authority_host {
            options.set_authority_host(Url::parse(&authority_host)?);
        }

        // TODO Add UsernamePasswordCredential
        // https://github.com/Azure/azure-sdk-for-rust/issues/1528
        Err(Error::message(
            ErrorKind::Credential,
            "no valid environment credential providers",
        ))

        // UsernamePasswordCredential::new(
        //     options,
        //     tenant_id,
        //     client_id,
        //     username,
        //     Secret::new(password),
        // )
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl TokenCredential for UsernamePasswordCredential {
    async fn get_token(&self, scopes: &[&str]) -> azure_core::Result<AccessToken> {
        self.cache.get_token(scopes, self.get_token(scopes)).await
    }

    async fn clear_cache(&self) -> azure_core::Result<()> {
        self.cache.clear().await
    }
}
