use crate::token_credentials::cache::TokenCache;
use crate::{oauth2_http_client::Oauth2HttpClient, TokenCredentialOptions};
use azure_core::{
    auth::{AccessToken, Secret, TokenCredential},
    error::{ErrorKind, ResultExt},
    HttpClient, Url,
};
use oauth2::{basic::BasicClient, AuthType, AuthUrl, Scope, TokenUrl};
use std::{str, sync::Arc};
use time::OffsetDateTime;

/// A list of tenant IDs
pub mod tenant_ids {
    /// The tenant ID for multi-tenant apps
    ///
    /// <https://docs.microsoft.com/azure/active-directory/develop/howto-convert-app-to-be-multi-tenant>
    pub const TENANT_ID_COMMON: &str = "common";
    /// The tenant ID for Active Directory Federated Services
    pub const TENANT_ID_ADFS: &str = "adfs";
}

/// Enables authentication to Azure Active Directory using a client secret that was generated for an App Registration.
///
/// More information on how to configure a client secret can be found here:
/// <https://docs.microsoft.com/azure/active-directory/develop/quickstart-configure-app-access-web-apis#add-credentials-to-your-web-application>
#[derive(Debug)]
pub struct ClientSecretCredential {
    http_client: Arc<dyn HttpClient>,
    authority_host: Url,
    tenant_id: String,
    client_id: oauth2::ClientId,
    client_secret: Option<oauth2::ClientSecret>,
    cache: TokenCache,
}

impl ClientSecretCredential {
    /// Create a new `ClientSecretCredential`
    pub fn new(
        options: TokenCredentialOptions,
        tenant_id: String,
        client_id: String,
        client_secret: String,
    ) -> ClientSecretCredential {
        ClientSecretCredential {
            http_client: options.http_client().clone(),
            authority_host: options.authority_host().clone(),
            tenant_id,
            client_id: oauth2::ClientId::new(client_id),
            client_secret: Some(oauth2::ClientSecret::new(client_secret)),
            cache: TokenCache::new(),
        }
    }

    async fn get_token(&self, scopes: &[&str]) -> azure_core::Result<AccessToken> {
        let authority_host = self.authority_host.to_string(); // TODO append URL

        let token_url = TokenUrl::from_url(
            Url::parse(&format!(
                "{}/{}/oauth2/v2.0/token",
                authority_host, self.tenant_id
            ))
            .with_context(ErrorKind::Credential, || {
                format!(
                    "failed to construct token endpoint with tenant id {}",
                    self.tenant_id
                )
            })?,
        );

        let auth_url = AuthUrl::from_url(
            Url::parse(&format!(
                "{}/{}/oauth2/v2.0/authorize",
                authority_host, self.tenant_id
            ))
            .with_context(ErrorKind::Credential, || {
                format!(
                    "failed to construct authorize endpoint with tenant id {}",
                    self.tenant_id
                )
            })?,
        );

        let client = BasicClient::new(
            self.client_id.clone(),
            self.client_secret.clone(),
            auth_url,
            Some(token_url),
        )
        .set_auth_type(AuthType::RequestBody);

        let scopes = scopes.iter().map(|x| Scope::new(x.to_string()));
        let oauth_http_client = Oauth2HttpClient::new(self.http_client.clone());
        let token_result = client
            .exchange_client_credentials()
            .add_scopes(scopes)
            .request_async(|request| oauth_http_client.request(request))
            .await
            .map(|r| {
                use oauth2::TokenResponse as _;
                AccessToken::new(
                    Secret::new(r.access_token().secret().to_owned()),
                    OffsetDateTime::now_utc() + r.expires_in().unwrap_or_default(),
                )
            })
            .context(ErrorKind::Credential, "request token error")?;

        Ok(token_result)
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl TokenCredential for ClientSecretCredential {
    async fn get_token(&self, scopes: &[&str]) -> azure_core::Result<AccessToken> {
        self.cache.get_token(scopes, self.get_token(scopes)).await
    }
    /// Clear the credential's cache.
    async fn clear_cache(&self) -> azure_core::Result<()> {
        self.cache.clear().await
    }
}
