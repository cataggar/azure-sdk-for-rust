use crate::{
    AppServiceManagedIdentityCredential, AzureCliCredential, EnvironmentCredential,
    TokenCredentialOptions, VirtualMachineManagedIdentityCredential,
};
use azure_core::{
    auth::{AccessToken, TokenCredential},
    error::ErrorKind,
    Error,
};

pub const AZURE_CREDENTIAL_TYPE: &str = "AZURE_CREDENTIAL_TYPE";

pub mod azure_credential_types {
    pub const ENVIRONMENT: &str = "environment";
    pub const AZURE_CLI: &str = "azurecli";
    pub const VIRTUAL_MACHINE: &str = "virtualmachine";
    pub const APP_SERVICE: &str = "appservice";
    // pub const AZUREAUTH_CLI: &str = "azureauthcli";
    // pub const SERVICE_FABRIC: &str = "servicefabric";
    // pub const CLOUD_SHELL: &str = "cloudshell";
    // pub const AZURE_ARC: &str = "azurearc";
}

#[derive(Debug)]
pub enum SpecificAzureCredentialEnum {
    Environment(EnvironmentCredential),
    AzureCli(AzureCliCredential),
    VirtualMachine(VirtualMachineManagedIdentityCredential),
    AppService(AppServiceManagedIdentityCredential),
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl TokenCredential for SpecificAzureCredentialEnum {
    async fn get_token(&self, scopes: &[&str]) -> azure_core::Result<AccessToken> {
        match self {
            SpecificAzureCredentialEnum::Environment(credential) => {
                credential.get_token(scopes).await
            }
            SpecificAzureCredentialEnum::AzureCli(credential) => credential.get_token(scopes).await,
            SpecificAzureCredentialEnum::VirtualMachine(credential) => {
                credential.get_token(scopes).await
            }
            SpecificAzureCredentialEnum::AppService(credential) => {
                credential.get_token(scopes).await
            }
        }
    }

    async fn clear_cache(&self) -> azure_core::Result<()> {
        match self {
            SpecificAzureCredentialEnum::Environment(credential) => credential.clear_cache().await,
            SpecificAzureCredentialEnum::AzureCli(credential) => credential.clear_cache().await,
            SpecificAzureCredentialEnum::VirtualMachine(credential) => {
                credential.clear_cache().await
            }
            SpecificAzureCredentialEnum::AppService(credential) => credential.clear_cache().await,
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
            azure_credential_types::ENVIRONMENT => EnvironmentCredential::create(options)
                .map(SpecificAzureCredentialEnum::Environment)?,
            azure_credential_types::APP_SERVICE => {
                AppServiceManagedIdentityCredential::create(options)
                    .map(SpecificAzureCredentialEnum::AppService)?
            }
            azure_credential_types::VIRTUAL_MACHINE => SpecificAzureCredentialEnum::VirtualMachine(
                VirtualMachineManagedIdentityCredential::new(options),
            ),
            azure_credential_types::AZURE_CLI => {
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
