// #![feature(return_position_impl_trait_in_trait)]
use crate::{AzureCliCredential, ImdsManagedIdentityCredential, TokenCredentialOptions};
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
pub enum SpecificAzureCredentialEnum {
    Imds(ImdsManagedIdentityCredential),
    AzureCli(AzureCliCredential),
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl TokenCredential for SpecificAzureCredentialEnum {
    async fn get_token(&self, scopes: &[&str]) -> azure_core::Result<AccessToken> {
        match self {
            SpecificAzureCredentialEnum::Imds(credential) => credential.get_token(scopes).await,
            SpecificAzureCredentialEnum::AzureCli(credential) => credential.get_token(scopes).await,
        }
    }

    async fn clear_cache(&self) -> azure_core::Result<()> {
        match self {
            SpecificAzureCredentialEnum::Imds(credential) => credential.clear_cache().await,
            SpecificAzureCredentialEnum::AzureCli(credential) => credential.clear_cache().await,
        }
    }
}

#[derive(Debug)]
pub struct SpecificAzureCredential {
    credential: SpecificAzureCredentialEnum,
}

impl SpecificAzureCredential {
    pub fn create(options: TokenCredentialOptions) -> azure_core::Result<SpecificAzureCredential> {
        let env = options.env();
        let credential_type = env.var(AZURE_CREDENTIAL_TYPE)?;
        let credential: SpecificAzureCredentialEnum = match credential_type.to_lowercase().as_str()
        {
            IMDS_MANAGED_IDENTITY_CREDENTIAL => {
                SpecificAzureCredentialEnum::Imds(ImdsManagedIdentityCredential::new(options))
            }
            AZURE_CLI_CREDENTIAL => {
                AzureCliCredential::create().map(SpecificAzureCredentialEnum::AzureCli)?
            }
            _ => {
                return Err(Error::with_message(ErrorKind::Credential, || {
                    format!("Unknown credential type: {}", credential_type)
                }))
            }
        };
        Ok(Self { credential })
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl TokenCredential for SpecificAzureCredential {
    async fn get_token(&self, scopes: &[&str]) -> azure_core::Result<AccessToken> {
        self.credential.get_token(scopes).await
    }

    async fn clear_cache(&self) -> azure_core::Result<()> {
        self.credential.clear_cache().await
    }
}
