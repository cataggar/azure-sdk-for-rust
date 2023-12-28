use crate::{ImdsId, ImdsManagedIdentityCredential, TokenCredentialOptions};
use azure_core::auth::{AccessToken, TokenCredential};
use azure_core::headers::HeaderName;
use azure_core::Url;

const ENDPOINT_ENV: &str = "IDENTITY_ENDPOINT";
const API_VERSION: &str = "2019-08-01";
const SECRET_HEADER: HeaderName = HeaderName::from_static("x-identity-header");
const SECRET_ENV: &str = "IDENTITY_HEADER";

#[derive(Debug)]
pub struct AppServiceManagedIdentityCredential {
    credential: ImdsManagedIdentityCredential,
}

impl AppServiceManagedIdentityCredential {
    pub fn create(options: TokenCredentialOptions) -> azure_core::Result<Self> {
        let env = options.env();
        let endpoint = Url::parse(&env.var(ENDPOINT_ENV)?)?;
        Ok(Self {
            credential: ImdsManagedIdentityCredential::new(
                options,
                endpoint,
                API_VERSION,
                SECRET_HEADER,
                SECRET_ENV,
                ImdsId::SystemAssigned,
            ),
        })
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl TokenCredential for AppServiceManagedIdentityCredential {
    async fn get_token(&self, scopes: &[&str]) -> azure_core::Result<AccessToken> {
        self.credential.get_token(scopes).await
    }

    async fn clear_cache(&self) -> azure_core::Result<()> {
        self.credential.clear_cache().await
    }
}
