#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![doc = "generated by AutoRust 0.1.0"]
#[cfg(feature = "package-2018-03-01-beta")]
pub mod package_2018_03_01_beta;
#[cfg(all(feature = "package-2018-03-01-beta", not(feature = "no-default-version")))]
pub use package_2018_03_01_beta::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
