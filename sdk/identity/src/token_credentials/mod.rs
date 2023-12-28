//! Access to token credentials through various means
//!
//! Supported means currently include:
//! * The environment
//! * Azure CLI credentials cache
//! * Managed identity
//! * Client secret
mod app_service_managed_identity_credential;
mod azure_cli_credentials;
#[cfg(feature = "azureauth-cli")]
mod azureauth_cli_credential;
mod cache;
#[cfg(feature = "client_certificate")]
mod client_certificate_credential;
mod client_secret_credential;
mod default_credentials;
mod environment_credentials;
mod imds_managed_identity_credentials;
mod options;
mod specific_azure_credential;
mod virtual_machine_managed_identity_credential;
mod workload_identity_credential;

pub use app_service_managed_identity_credential::*;
pub use azure_cli_credentials::*;
#[cfg(feature = "azureauth-cli")]
pub use azureauth_cli_credential::*;
#[cfg(feature = "client_certificate")]
pub use client_certificate_credential::*;
pub use client_secret_credential::*;
pub use default_credentials::*;
pub use environment_credentials::*;
pub use imds_managed_identity_credentials::*;
pub use options::*;
pub use specific_azure_credential::*;
pub use virtual_machine_managed_identity_credential::*;
pub use workload_identity_credential::*;
