#[cfg(feature = "client_certificate")]
use crate::ClientCertificateCredential;
use crate::{
    AppServiceManagedIdentityCredential, AzureCliCredential, ClientSecretCredential,
    EnvironmentCredential, TokenCredentialOptions, UsernamePasswordCredential,
    VirtualMachineManagedIdentityCredential, WorkloadIdentityCredential,
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
    pub const CLIENT_SECRET: &str = "clientsecret";
    pub const WORKLOAD_IDENTITY: &str = "workloadidentity";
    #[cfg(feature = "client_certificate")]
    pub const CLIENT_CERTIFICATE: &str = "clientcertificate";
    pub const USERNAME_PASSWORD: &str = "usernamepassword";
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
    ClientSecret(ClientSecretCredential),
    WorkloadIdentity(WorkloadIdentityCredential),
    #[cfg(feature = "client_certificate")]
    ClientCertificate(ClientCertificateCredential),
    UsernamePassword(UsernamePasswordCredential),
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
            SpecificAzureCredentialEnum::ClientSecret(credential) => {
                credential.get_token(scopes).await
            }
            SpecificAzureCredentialEnum::WorkloadIdentity(credential) => {
                credential.get_token(scopes).await
            }
            #[cfg(feature = "client_certificate")]
            SpecificAzureCredentialEnum::ClientCertificate(credential) => {
                credential.get_token(scopes).await
            }
            SpecificAzureCredentialEnum::UsernamePassword(credential) => {
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
            SpecificAzureCredentialEnum::ClientSecret(credential) => credential.clear_cache().await,
            SpecificAzureCredentialEnum::WorkloadIdentity(credential) => {
                credential.clear_cache().await
            }
            #[cfg(feature = "client_certificate")]
            SpecificAzureCredentialEnum::ClientCertificate(credential) => {
                credential.clear_cache().await
            }
            SpecificAzureCredentialEnum::UsernamePassword(credential) => {
                credential.clear_cache().await
            }
        }
    }
}

#[derive(Debug)]
pub struct SpecificAzureCredential {
    source: SpecificAzureCredentialEnum,
}

impl SpecificAzureCredential {
    pub fn create(options: TokenCredentialOptions) -> azure_core::Result<SpecificAzureCredential> {
        let env = options.env();
        let credential_type = env.var(AZURE_CREDENTIAL_TYPE)?;
        let source: SpecificAzureCredentialEnum = match credential_type.to_lowercase().as_str() {
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
            azure_credential_types::CLIENT_SECRET => ClientSecretCredential::create(options)
                .map(SpecificAzureCredentialEnum::ClientSecret)?,
            azure_credential_types::WORKLOAD_IDENTITY => {
                WorkloadIdentityCredential::create(options)
                    .map(SpecificAzureCredentialEnum::WorkloadIdentity)?
            }
            #[cfg(feature = "client_certificate")]
            azure_credential_types::CLIENT_CERTIFICATE => {
                ClientCertificateCredential::create(options)
                    .map(SpecificAzureCredentialEnum::ClientCertificate)?
            }
            azure_credential_types::USERNAME_PASSWORD => {
                UsernamePasswordCredential::create(options)
                    .map(SpecificAzureCredentialEnum::UsernamePassword)?
            }
            _ => {
                return Err(Error::with_message(ErrorKind::Credential, || {
                    format!("Unknown credential type: {}", credential_type)
                }))
            }
        };
        Ok(Self { source })
    }

    #[cfg(test)]
    pub(crate) fn source(&self) -> &SpecificAzureCredentialEnum {
        &self.source
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl TokenCredential for SpecificAzureCredential {
    async fn get_token(&self, scopes: &[&str]) -> azure_core::Result<AccessToken> {
        self.source.get_token(scopes).await
    }

    async fn clear_cache(&self) -> azure_core::Result<()> {
        self.source.clear_cache().await
    }
}
