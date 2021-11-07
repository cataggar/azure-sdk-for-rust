#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![doc = "generated by AutoRust 0.1.0"]
#[cfg(feature = "package-2021-08")]
pub mod package_2021_08;
#[cfg(all(feature = "package-2021-08", not(feature = "no-default-version")))]
pub use package_2021_08::{models, operations};
#[cfg(feature = "package-2021-07")]
pub mod package_2021_07;
#[cfg(all(feature = "package-2021-07", not(feature = "no-default-version")))]
pub use package_2021_07::{models, operations};
#[cfg(feature = "package-2021-06")]
pub mod package_2021_06;
#[cfg(all(feature = "package-2021-06", not(feature = "no-default-version")))]
pub use package_2021_06::{models, operations};
#[cfg(feature = "package-2021-04")]
pub mod package_2021_04;
#[cfg(all(feature = "package-2021-04", not(feature = "no-default-version")))]
pub use package_2021_04::{models, operations};
#[cfg(feature = "package-2021-03")]
pub mod package_2021_03;
#[cfg(all(feature = "package-2021-03", not(feature = "no-default-version")))]
pub use package_2021_03::{models, operations};
#[cfg(feature = "package-2021-02")]
pub mod package_2021_02;
#[cfg(all(feature = "package-2021-02", not(feature = "no-default-version")))]
pub use package_2021_02::{models, operations};
#[cfg(feature = "package-2016-06")]
pub mod package_2016_06;
#[cfg(all(feature = "package-2016-06", not(feature = "no-default-version")))]
pub use package_2016_06::{models, operations};
#[cfg(feature = "package-2020-02")]
pub mod package_2020_02;
#[cfg(all(feature = "package-2020-02", not(feature = "no-default-version")))]
pub use package_2020_02::{models, operations};
#[cfg(feature = "package-2020-10")]
pub mod package_2020_10;
#[cfg(all(feature = "package-2020-10", not(feature = "no-default-version")))]
pub use package_2020_10::{models, operations};
#[cfg(feature = "package-2021-01")]
pub mod package_2021_01;
use azure_core::setters;
#[cfg(all(feature = "package-2021-01", not(feature = "no-default-version")))]
pub use package_2021_01::{models, operations};
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
