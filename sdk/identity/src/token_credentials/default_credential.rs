use crate::{
    token_credentials::cache::TokenCache,
    AppServiceManagedIdentityCredential, EnvironmentCredential, TokenCredentialOptions,
    VirtualMachineManagedIdentityCredential, {AzureCliCredential, ImdsManagedIdentityCredential},
};
use azure_core::{
    auth::{AccessToken, TokenCredential},
    error::{Error, ErrorKind, ResultExt},
};

/// Provides a mechanism of selectively disabling credentials used for a `DefaultAzureCredential` instance
pub struct DefaultAzureCredentialBuilder {
    options: TokenCredentialOptions,
    include_environment_credential: bool,
    include_app_service_managed_identity_credential: bool,
    include_virtual_machine_managed_identity_credential: bool,
    include_azure_cli_credential: bool,
}

impl Default for DefaultAzureCredentialBuilder {
    fn default() -> Self {
        Self {
            options: TokenCredentialOptions::default(),
            include_environment_credential: true,
            include_app_service_managed_identity_credential: true,
            // Unable to quickly detect if running in Azure VM, so it is disabled by default.
            include_virtual_machine_managed_identity_credential: false,
            include_azure_cli_credential: true,
        }
    }
}

impl DefaultAzureCredentialBuilder {
    /// Create a new `DefaultAzureCredentialBuilder`
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_options(&mut self, options: TokenCredentialOptions) -> &mut Self {
        self.options = options;
        self
    }

    /// Exclude using credentials from the environment
    pub fn exclude_environment_credential(&mut self) -> &mut Self {
        self.include_environment_credential = false;
        self
    }

    /// Exclude using any managed identity credential
    pub fn exclude_managed_identity_credential(&mut self) -> &mut Self {
        self.include_app_service_managed_identity_credential = false;
        self.include_virtual_machine_managed_identity_credential = false;
        self
    }

    /// Exclude using virtual machine managed identity credential
    pub fn exclude_virtual_machine_managed_identity_credential(&mut self) -> &mut Self {
        self.include_virtual_machine_managed_identity_credential = false;
        self
    }

    /// Include using virtual machine managed identity credential
    pub fn include_virtual_machine_managed_identity_credentials(&mut self) -> &mut Self {
        self.include_virtual_machine_managed_identity_credential = true;
        self
    }

    /// Inlucde using app service managed identity credential
    pub fn include_app_service_managed_identity_credentials(&mut self) -> &mut Self {
        self.include_app_service_managed_identity_credential = true;
        self
    }

    /// Exclude using credentials from the cli
    pub fn exclude_azure_cli_credential(&mut self) -> &mut Self {
        self.include_azure_cli_credential = false;
        self
    }

    /// Create a `DefaultAzureCredential` from this builder.
    pub fn build(&self) -> DefaultAzureCredential {
        let source_count = usize::from(self.include_environment_credential)
            + usize::from(self.include_azure_cli_credential)
            + usize::from(self.include_app_service_managed_identity_credential)
            + usize::from(self.include_virtual_machine_managed_identity_credential);
        let mut sources = Vec::<DefaultAzureCredentialEnum>::with_capacity(source_count);
        if self.include_environment_credential {
            if let Ok(credential) = EnvironmentCredential::create(self.options.clone()) {
                sources.push(DefaultAzureCredentialEnum::Environment(credential));
            }
        }
        if self.include_app_service_managed_identity_credential {
            if let Ok(credential) =
                AppServiceManagedIdentityCredential::create(self.options.clone())
            {
                sources.push(DefaultAzureCredentialEnum::AppService(credential));
            }
        }
        if self.include_virtual_machine_managed_identity_credential {
            sources.push(DefaultAzureCredentialEnum::VirtualMachine(
                VirtualMachineManagedIdentityCredential::new(self.options.clone()),
            ));
        }
        if self.include_azure_cli_credential {
            if let Ok(credential) = AzureCliCredential::create() {
                sources.push(DefaultAzureCredentialEnum::AzureCli(credential));
            }
        }
        DefaultAzureCredential::with_sources(sources)
    }
}

/// Types of `TokenCredential` supported by `DefaultAzureCredential`
#[derive(Debug)]
pub enum DefaultAzureCredentialEnum {
    /// `TokenCredential` from environment variable.
    Environment(EnvironmentCredential),
    /// `TokenCredential` from managed identity that has been assigned to an App Service.
    AppService(AppServiceManagedIdentityCredential),
    /// `TokenCredential` from managed identity that has been assigned to a virtual machine.
    VirtualMachine(VirtualMachineManagedIdentityCredential),
    /// `TokenCredential` from Azure CLI.
    AzureCli(AzureCliCredential),
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl TokenCredential for DefaultAzureCredentialEnum {
    async fn get_token(&self, scopes: &[&str]) -> azure_core::Result<AccessToken> {
        match self {
            DefaultAzureCredentialEnum::Environment(credential) => {
                credential.get_token(scopes).await.context(
                    ErrorKind::Credential,
                    "error getting environment credential",
                )
            }
            DefaultAzureCredentialEnum::AppService(credential) => {
                credential.get_token(scopes).await.context(
                    ErrorKind::Credential,
                    "error getting managed identity credential for App Service",
                )
            }
            DefaultAzureCredentialEnum::VirtualMachine(credential) => {
                credential.get_token(scopes).await.context(
                    ErrorKind::Credential,
                    "error getting managed identity credential for virtual machine",
                )
            }
            DefaultAzureCredentialEnum::AzureCli(credential) => {
                credential.get_token(scopes).await.context(
                    ErrorKind::Credential,
                    "error getting token credential from Azure CLI",
                )
            }
        }
    }

    /// Clear the credential's cache.
    async fn clear_cache(&self) -> azure_core::Result<()> {
        match self {
            DefaultAzureCredentialEnum::Environment(credential) => credential.clear_cache().await,
            DefaultAzureCredentialEnum::AppService(credential) => credential.clear_cache().await,
            DefaultAzureCredentialEnum::VirtualMachine(credential) => {
                credential.clear_cache().await
            }
            DefaultAzureCredentialEnum::AzureCli(credential) => credential.clear_cache().await,
        }
    }
}

/// Provides a default `TokenCredential` authentication flow for applications that will be deployed to Azure.
///
/// The following credential types if enabled will be tried, in order:
/// - `EnvironmentCredential`
/// - `ManagedIdentityCredential`
/// - `AzureCliCredential`
/// Consult the documentation of these credential types for more information on how they attempt authentication.
#[derive(Debug)]
pub struct DefaultAzureCredential {
    sources: Vec<DefaultAzureCredentialEnum>,
    cache: TokenCache,
}

impl DefaultAzureCredential {
    /// Creates a `DefaultAzureCredential` with specified sources.
    ///
    /// These sources will be tried in the order provided in the `TokenCredential` authentication flow.
    pub fn with_sources(sources: Vec<DefaultAzureCredentialEnum>) -> Self {
        DefaultAzureCredential {
            sources,
            cache: TokenCache::new(),
        }
    }

    /// Try to fetch a token using each of the credential sources until one succeeds
    async fn get_token(&self, scopes: &[&str]) -> azure_core::Result<AccessToken> {
        let mut errors = Vec::new();
        for source in &self.sources {
            let token_res = source.get_token(scopes).await;

            match token_res {
                Ok(token) => return Ok(token),
                Err(error) => errors.push(error),
            }
        }
        Err(Error::with_message(ErrorKind::Credential, || {
            format!(
                "Multiple errors were encountered while attempting to authenticate:\n{}",
                format_aggregate_error(&errors)
            )
        }))
    }
}

impl Default for DefaultAzureCredential {
    fn default() -> Self {
        DefaultAzureCredentialBuilder::new().build()
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl TokenCredential for DefaultAzureCredential {
    async fn get_token(&self, scopes: &[&str]) -> azure_core::Result<AccessToken> {
        self.cache.get_token(scopes, self.get_token(scopes)).await
    }

    /// Clear the credential's cache.
    async fn clear_cache(&self) -> azure_core::Result<()> {
        // clear the internal cache as well as each of the underlying providers
        self.cache.clear().await?;

        for source in &self.sources {
            source.clear_cache().await?;
        }

        Ok(())
    }
}

fn format_aggregate_error(errors: &[Error]) -> String {
    errors
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<String>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::matches;

    #[test]
    fn test_builder_included_credential_flags() {
        let builder = DefaultAzureCredentialBuilder::new();
        assert!(builder.include_azure_cli_credential);
        assert!(builder.include_environment_credential);
        assert!(builder.include_app_service_managed_identity_credential);
        assert!(!builder.include_virtual_machine_managed_identity_credential);

        let mut builder = DefaultAzureCredentialBuilder::new();
        builder.exclude_azure_cli_credential();
        assert!(!builder.include_azure_cli_credential);
        assert!(builder.include_environment_credential);
        assert!(builder.include_app_service_managed_identity_credential);
        assert!(!builder.include_virtual_machine_managed_identity_credential);

        let mut builder = DefaultAzureCredentialBuilder::new();
        builder.exclude_environment_credential();
        assert!(builder.include_azure_cli_credential);
        assert!(!builder.include_environment_credential);
        assert!(builder.include_app_service_managed_identity_credential);
        assert!(!builder.include_virtual_machine_managed_identity_credential);

        let mut builder = DefaultAzureCredentialBuilder::new();
        builder.exclude_managed_identity_credential();
        assert!(builder.include_azure_cli_credential);
        assert!(builder.include_environment_credential);
        assert!(!builder.include_app_service_managed_identity_credential);
        assert!(!builder.include_virtual_machine_managed_identity_credential);
    }

    macro_rules! contains_credential {
        ($creds:expr, $p:pat) => {
            $creds.sources.iter().any(|x| matches!(x, $p))
        };
    }

    #[test]
    fn test_credential_sources() {
        let mut builder = DefaultAzureCredentialBuilder::new();

        // test with all sources

        let credential = builder.build();
        assert_eq!(credential.sources.len(), 3);

        assert!(contains_credential!(
            credential,
            DefaultAzureCredentialEnum::Environment(_)
        ));
        assert!(contains_credential!(
            credential,
            DefaultAzureCredentialEnum::AzureCli(_)
        ));
        // assert!(contains_credential!(
        //     credential,
        //     DefaultAzureCredentialEnum::ManagedIdentity(_)
        // ));

        // remove environment source

        builder.exclude_environment_credential();
        let credential = builder.build();

        assert_eq!(credential.sources.len(), 2);

        assert!(!contains_credential!(
            credential,
            DefaultAzureCredentialEnum::Environment(_)
        ));
        assert!(contains_credential!(
            credential,
            DefaultAzureCredentialEnum::AzureCli(_)
        ));
        // assert!(contains_credential!(
        //     credential,
        //     DefaultAzureCredentialEnum::ManagedIdentity(_)
        // ));

        // remove cli source

        builder.exclude_azure_cli_credential();
        let credential = builder.build();

        assert_eq!(credential.sources.len(), 1);

        assert!(!contains_credential!(
            credential,
            DefaultAzureCredentialEnum::Environment(_)
        ));
        assert!(!contains_credential!(
            credential,
            DefaultAzureCredentialEnum::AzureCli(_)
        ));
        // assert!(contains_credential!(
        //     credential,
        //     DefaultAzureCredentialEnum::ManagedIdentity(_)
        // ));

        // remove managed identity source

        builder.exclude_managed_identity_credential();
        let credential = builder.build();

        assert_eq!(credential.sources.len(), 0);
    }
}
