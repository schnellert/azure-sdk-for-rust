#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![doc = "generated by AutoRust 0.1.0"]
#[cfg(feature = "package-2021-05-01")]
pub mod package_2021_05_01;
#[cfg(all(feature = "package-2021-05-01", not(feature = "no-default-version")))]
pub use package_2021_05_01::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2020-01-01-preview")]
pub mod package_2020_01_01_preview;
#[cfg(all(feature = "package-2020-01-01-preview", not(feature = "no-default-version")))]
pub use package_2020_01_01_preview::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
