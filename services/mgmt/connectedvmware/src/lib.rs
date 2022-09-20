#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2022-01-10-preview")]
pub mod package_2022_01_10_preview;
#[cfg(all(feature = "package-2022-01-10-preview", not(feature = "no-default-tag")))]
pub use package_2022_01_10_preview::*;
#[cfg(feature = "package-2020-10-01-preview")]
pub mod package_2020_10_01_preview;
#[cfg(all(feature = "package-2020-10-01-preview", not(feature = "no-default-tag")))]
pub use package_2020_10_01_preview::*;
