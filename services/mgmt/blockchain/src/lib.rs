#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![doc = "generated by AutoRust 0.1.0"]
#[cfg(feature = "package-2018-06-01-preview")]
pub mod package_2018_06_01_preview;
#[cfg(all(feature = "package-2018-06-01-preview", not(feature = "no-default-version")))]
pub use package_2018_06_01_preview::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
