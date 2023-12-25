// #![feature(return_position_impl_trait_in_trait)]
use crate::{env::Env, AzureCliCredential, CreateTokenCredential, ImdsManagedIdentityCredential};
use azure_core::{
    auth::{AccessToken, TokenCredential},
    error::{Error, ErrorKind},
};

pub const AZURE_CREDENTIAL_TYPE: &str = "AZURE_CREDENTIAL_TYPE";

pub const IMDS_MANAGED_IDENTITY_CREDENTIAL: &str = "imds";
pub const AZURE_CLI_CREDENTIAL: &str = "azurecli";
// pub const AZUREAUTH_CLI_CREDENTIAL: &str = "azureauthcli";
// pub const SERVICE_FABRIC_CREDENTIAL: &str = "servicefabric";
// pub const APP_SERVICE_MANAGED_IDENTITY_CREDENTIAL: &str = "appservice";
// pub const CLOUD_SHELL_MANAGED_IDENTITY_CREDENTIAL: &str = "cloudshell";
// pub const AZURE_ARC_MANAGED_IDENTITY_CREDENTIAL: &str = "azurearc";

#[derive(Debug)]
pub struct SpecificAzureCredential {
    credential: Box<dyn TokenCredential>,
}

impl SpecificAzureCredential {
    pub fn new(credential: Box<dyn TokenCredential>) -> Self {
        Self { credential }
    }
}

impl CreateTokenCredential for SpecificAzureCredential {
    fn create_credential(
        &self,
        env: impl Env,
        _http_client: std::sync::Arc<dyn azure_core::HttpClient>,
        _options: &crate::TokenCredentialOptions,
    ) -> azure_core::Result<impl TokenCredential> {
        let credential_type = env.var(AZURE_CREDENTIAL_TYPE)?;
        let credential: Box<dyn TokenCredential> = match credential_type.to_lowercase().as_str() {
            IMDS_MANAGED_IDENTITY_CREDENTIAL => Box::new(ImdsManagedIdentityCredential::default()),
            AZURE_CLI_CREDENTIAL => Box::new(AzureCliCredential::default()),
            _ => {
                return Err(Error::with_message(ErrorKind::Credential, || {
                    format!("Unknown credential type: {}", credential_type)
                }))
            }
        };
        Ok(Self::new(credential))
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl TokenCredential for SpecificAzureCredential {
    async fn get_token(&self, scopes: &[&str]) -> azure_core::Result<AccessToken> {
        self.credential.as_ref().get_token(scopes).await
    }

    async fn clear_cache(&self) -> azure_core::Result<()> {
        self.credential.as_ref().clear_cache().await
    }
}
