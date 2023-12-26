use crate::{
    token_credentials::{
        ClientSecretCredential, TokenCredentialOptions, WorkloadIdentityCredential,
    },
    TokenCredentialCreator,
};
use azure_core::{
    auth::{AccessToken, TokenCredential},
    error::{Error, ErrorKind, ResultExt},
    Url,
};

const AZURE_TENANT_ID_ENV_KEY: &str = "AZURE_TENANT_ID";
const AZURE_CLIENT_ID_ENV_KEY: &str = "AZURE_CLIENT_ID";
const AZURE_CLIENT_SECRET_ENV_KEY: &str = "AZURE_CLIENT_SECRET";
// const AZURE_USERNAME_ENV_KEY: &str = "AZURE_USERNAME";
// const AZURE_PASSWORD_ENV_KEY: &str = "AZURE_PASSWORD";
// const AZURE_CLIENT_CERTIFICATE_PATH_ENV_KEY: &str = "AZURE_CLIENT_CERTIFICATE_PATH";
const AZURE_FEDERATED_TOKEN_FILE: &str = "AZURE_FEDERATED_TOKEN_FILE";
const AZURE_FEDERATED_TOKEN: &str = "AZURE_FEDERATED_TOKEN";
const AZURE_AUTHORITY_HOST: &str = "AZURE_AUTHORITY_HOST";

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
/// This credential ultimately uses a or `WorkloadIdentityCredential` a`ClientSecretCredential` to perform the authentication using
/// these details.
/// Please consult the documentation of that class for more details.
#[derive(Debug)]
pub struct EnvironmentCredential {
    credential: Box<dyn TokenCredential>,
}

impl EnvironmentCredential {
    pub fn create(options: TokenCredentialOptions) -> azure_core::Result<EnvironmentCredential> {
        let env = options.env();
        let tenant_id = env
            .var(AZURE_TENANT_ID_ENV_KEY)
            .with_context(ErrorKind::Credential, || {
                format!("missing tenant id set in {AZURE_TENANT_ID_ENV_KEY} environment variable")
            })?;
        let client_id = env
            .var(AZURE_CLIENT_ID_ENV_KEY)
            .with_context(ErrorKind::Credential, || {
                format!("missing client id set in {AZURE_CLIENT_ID_ENV_KEY} environment variable")
            })?;

        let client_secret = env.var(AZURE_CLIENT_SECRET_ENV_KEY);
        // let username = env.var(AZURE_USERNAME_ENV_KEY);
        // let password = env.var(AZURE_PASSWORD_ENV_KEY);
        // let client_certificate_path = env.var(AZURE_CLIENT_CERTIFICATE_PATH_ENV_KEY);
        let federated_token_file = env.var(AZURE_FEDERATED_TOKEN_FILE);
        let federated_token = env.var(AZURE_FEDERATED_TOKEN);
        let authority_host = env.var(AZURE_AUTHORITY_HOST);

        let mut options = options.clone();
        if let Ok(authority_host) = authority_host {
            options.set_authority_host(Url::parse(&authority_host)?);
        }

        let credential: Box<dyn TokenCredential> = if let Ok(token) = federated_token {
            let credential: WorkloadIdentityCredential =
                WorkloadIdentityCredential::new(options.clone(), tenant_id, client_id, token);
            Box::new(credential)
        } else if let Ok(file) = federated_token_file {
            let token = std::fs::read_to_string(file.clone())
                .with_context(ErrorKind::Credential, || {
                    format!("failed to read federated token from file {}", file.as_str())
                })?;
            let credential: WorkloadIdentityCredential =
                WorkloadIdentityCredential::new(options, tenant_id, client_id, token);
            Box::new(credential)
        } else if let Ok(client_secret) = client_secret {
            let credential =
                ClientSecretCredential::new(options, tenant_id, client_id, client_secret);
            Box::new(credential)
        // } else if username.is_ok() && password.is_ok() {
        //     // Could use multiple if-let with #![feature(let_chains)] once stabilised - see https://github.com/rust-lang/rust/issues/53667
        //     // TODO: username & password credential
        // } else if let Ok(_path) = client_certificate_path {
        // TODO: client certificate credential
        } else {
            return Err(Error::message(
                ErrorKind::Credential,
                "no valid environment credential providers",
            ));
        };
        Ok(Self { credential })
    }
}

#[derive(Debug, Default)]
struct EnvironmentCredentialCreator {}

impl TokenCredentialCreator for EnvironmentCredentialCreator {
    fn create(
        &self,
        options: TokenCredentialOptions,
    ) -> azure_core::Result<Box<dyn TokenCredential>> {
        EnvironmentCredential::create(options)
            .map(|credential| Box::new(credential) as Box<dyn TokenCredential>)
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl TokenCredential for EnvironmentCredential {
    async fn get_token(&self, scopes: &[&str]) -> azure_core::Result<AccessToken> {
        self.credential.as_ref().get_token(scopes).await
    }

    async fn clear_cache(&self) -> azure_core::Result<()> {
        self.credential.as_ref().clear_cache().await
    }
}
