#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![doc = "generated by AutoRust 0.1.0"]
#[cfg(feature = "package-2017-04")]
pub mod package_2017_04;
#[cfg(all(feature = "package-2017-04", not(feature = "no-default-version")))]
pub use package_2017_04::{models, operations};
#[cfg(feature = "package-2015-08")]
pub mod package_2015_08;
#[cfg(all(feature = "package-2015-08", not(feature = "no-default-version")))]
pub use package_2015_08::{models, operations};
#[cfg(feature = "package-2014-09")]
pub mod package_2014_09;
#[cfg(all(feature = "package-2014-09", not(feature = "no-default-version")))]
pub use package_2014_09::{models, operations};
#[cfg(feature = "package-2021-01-preview")]
pub mod package_2021_01_preview;
#[cfg(all(feature = "package-2021-01-preview", not(feature = "no-default-version")))]
pub use package_2021_01_preview::{models, operations};
#[cfg(feature = "package-2021-06-preview")]
pub mod package_2021_06_preview;
#[cfg(all(feature = "package-2021-06-preview", not(feature = "no-default-version")))]
pub use package_2021_06_preview::{models, operations};
#[cfg(feature = "package-2018-01-preview")]
pub mod package_2018_01_preview;
#[cfg(all(feature = "package-2018-01-preview", not(feature = "no-default-version")))]
pub use package_2018_01_preview::{models, operations};
#[cfg(feature = "package-2021-11")]
pub mod package_2021_11;
#[cfg(all(feature = "package-2021-11", not(feature = "no-default-version")))]
pub use package_2021_11::{models, operations};
#[cfg(feature = "profile-hybrid-2020-09-01")]
pub mod profile_hybrid_2020_09_01;
use azure_core::setters;
#[cfg(all(feature = "profile-hybrid-2020-09-01", not(feature = "no-default-version")))]
pub use profile_hybrid_2020_09_01::{models, operations};
pub fn config(
    http_client: std::sync::Arc<dyn azure_core::HttpClient>,
    token_credential: Box<dyn azure_core::TokenCredential>,
) -> OperationConfigBuilder {
    OperationConfigBuilder {
        http_client,
        base_path: None,
        token_credential,
        token_credential_resource: None,
    }
}
pub struct OperationConfigBuilder {
    http_client: std::sync::Arc<dyn azure_core::HttpClient>,
    base_path: Option<String>,
    token_credential: Box<dyn azure_core::TokenCredential>,
    token_credential_resource: Option<String>,
}
impl OperationConfigBuilder {
    setters! { base_path : String => Some (base_path) , token_credential_resource : String => Some (token_credential_resource) , }
    pub fn build(self) -> OperationConfig {
        OperationConfig {
            http_client: self.http_client,
            base_path: self.base_path.unwrap_or_else(|| "https://management.azure.com".to_owned()),
            token_credential: Some(self.token_credential),
            token_credential_resource: self
                .token_credential_resource
                .unwrap_or_else(|| "https://management.azure.com/".to_owned()),
        }
    }
}
pub struct OperationConfig {
    http_client: std::sync::Arc<dyn azure_core::HttpClient>,
    base_path: String,
    token_credential: Option<Box<dyn azure_core::TokenCredential>>,
    token_credential_resource: String,
}
impl OperationConfig {
    pub fn http_client(&self) -> &dyn azure_core::HttpClient {
        self.http_client.as_ref()
    }
    pub fn base_path(&self) -> &str {
        self.base_path.as_str()
    }
    pub fn token_credential(&self) -> Option<&dyn azure_core::TokenCredential> {
        self.token_credential.as_deref()
    }
    pub fn token_credential_resource(&self) -> &str {
        self.token_credential_resource.as_str()
    }
}
