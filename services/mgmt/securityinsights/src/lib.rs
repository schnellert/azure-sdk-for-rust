#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![doc = "generated by AutoRust 0.1.0"]
#[cfg(feature = "package-2021-04-01-only")]
pub mod package_2021_04_01_only;
#[cfg(all(feature = "package-2021-04-01-only", not(feature = "no-default-version")))]
pub use package_2021_04_01_only::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2020-01")]
pub mod package_2020_01;
#[cfg(all(feature = "package-2020-01", not(feature = "no-default-version")))]
pub use package_2020_01::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2021-03-preview-only")]
pub mod package_2021_03_preview_only;
#[cfg(all(feature = "package-2021-03-preview-only", not(feature = "no-default-version")))]
pub use package_2021_03_preview_only::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2019-01-preview")]
pub mod package_2019_01_preview;
#[cfg(all(feature = "package-2019-01-preview", not(feature = "no-default-version")))]
pub use package_2019_01_preview::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
