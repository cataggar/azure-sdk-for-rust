#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![doc = "generated by AutoRust"]
#[cfg(feature = "package-2017-04")]
pub mod package_2017_04;
#[cfg(all(feature = "package-2017-04", not(feature = "no-default-tag")))]
pub use package_2017_04::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2017-01")]
pub mod package_2017_01;
#[cfg(all(feature = "package-2017-01", not(feature = "no-default-tag")))]
pub use package_2017_01::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
