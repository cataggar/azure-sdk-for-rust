#[cfg(feature = "client_certificate")]
pub use crate::token_credentials::ClientCertificateCredential;
use crate::token_credentials::{
    ClientSecretCredential, TokenCredentialOptions, UsernamePasswordCredential,
    WorkloadIdentityCredential,
};
use azure_core::{
    auth::{AccessToken, TokenCredential},
    error::{Error, ErrorKind},
    Url,
};

const AZURE_AUTHORITY_HOST: &str = "AZURE_AUTHORITY_HOST";

#[derive(Debug)]
pub enum EnvironmentCredentialEnum {
    ClientSecret(ClientSecretCredential),
    WorkloadIdentity(WorkloadIdentityCredential),
    #[cfg(feature = "client_certificate")]
    ClientCertificate(ClientCertificateCredential),
    UsernamePassword(UsernamePasswordCredential),
}

/// Enables authentication with Workflows Identity if either `AZURE_FEDERATED_TOKEN` or `AZURE_FEDERATED_TOKEN_FILE` is set,
/// otherwise enables authentication to Azure Active Directory using client secret, or a username and password.
///
///
/// Details configured in the following environment variables:
///
/// | Variable                            | Description                                      |
/// |-------------------------------------|--------------------------------------------------|
/// | `AZURE_TENANT_ID`                   | The Azure Active Directory tenant(directory) ID. |
/// | `AZURE_CLIENT_ID`                   | The client(application) ID of an App Registration in the tenant. |
/// | `AZURE_CLIENT_SECRET`               | A client secret that was generated for the App Registration. |
/// | `AZURE_FEDERATED_TOKEN_FILE`        | Path to an federated token file. Variable is present in pods with aks workload identities. |
/// | `AZURE_AUTHORITY_HOST`              | Url for the identity provider to exchange to federated token for an `access_token`. Variable is present in pods with aks workload identities. |
///
/// This credential ultimately uses a `WorkloadIdentityCredential` or a`ClientSecretCredential` to perform the authentication using
/// these details.
/// Please consult the documentation of that class for more details.
#[derive(Debug)]
pub struct EnvironmentCredential {
    source: EnvironmentCredentialEnum,
}

impl EnvironmentCredential {
    pub fn create(
        options: impl Into<TokenCredentialOptions>,
    ) -> azure_core::Result<EnvironmentCredential> {
        let options = options.into();
        let env = options.env();
        let authority_host = env.var(AZURE_AUTHORITY_HOST);
        let mut options = options.clone();
        if let Ok(authority_host) = authority_host {
            options.set_authority_host(Url::parse(&authority_host)?);
        }

        let source: EnvironmentCredentialEnum =
            if let Ok(credential) = WorkloadIdentityCredential::create(options.clone()) {
                EnvironmentCredentialEnum::WorkloadIdentity(credential)
            } else if let Ok(credential) = ClientSecretCredential::create(options.clone()) {
                EnvironmentCredentialEnum::ClientSecret(credential)
            } else if let Ok(credential) = UsernamePasswordCredential::create(options.clone()) {
                EnvironmentCredentialEnum::UsernamePassword(credential)
            } else {
                return Err(Error::message(
                    ErrorKind::Credential,
                    "no valid environment credential providers",
                ));
            };
        Ok(Self { source })
    }

    #[cfg(test)]
    pub(crate) fn source(&self) -> &EnvironmentCredentialEnum {
        &self.source
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl TokenCredential for EnvironmentCredential {
    async fn get_token(&self, scopes: &[&str]) -> azure_core::Result<AccessToken> {
        match &self.source {
            EnvironmentCredentialEnum::ClientSecret(credential) => {
                credential.get_token(scopes).await
            }
            EnvironmentCredentialEnum::WorkloadIdentity(credential) => {
                credential.get_token(scopes).await
            }
            #[cfg(feature = "client_certificate")]
            EnvironmentCredentialEnum::ClientCertificate(credential) => {
                credential.get_token(scopes).await
            }
            EnvironmentCredentialEnum::UsernamePassword(credential) => {
                credential.get_token(scopes).await
            }
        }
    }

    async fn clear_cache(&self) -> azure_core::Result<()> {
        match &self.source {
            EnvironmentCredentialEnum::ClientSecret(credential) => credential.clear_cache().await,
            EnvironmentCredentialEnum::WorkloadIdentity(credential) => {
                credential.clear_cache().await
            }
            #[cfg(feature = "client_certificate")]
            EnvironmentCredentialEnum::ClientCertificate(credential) => {
                credential.clear_cache().await
            }
            EnvironmentCredentialEnum::UsernamePassword(credential) => {
                credential.clear_cache().await
            }
        }
    }
}
