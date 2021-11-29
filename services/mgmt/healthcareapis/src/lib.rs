#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![doc = "generated by AutoRust 0.1.0"]
#[cfg(feature = "package-preview-2021-06")]
pub mod package_preview_2021_06;
#[cfg(all(feature = "package-preview-2021-06", not(feature = "no-default-version")))]
pub use package_preview_2021_06::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2021-01")]
pub mod package_2021_01;
#[cfg(all(feature = "package-2021-01", not(feature = "no-default-version")))]
pub use package_2021_01::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2020-03-30")]
pub mod package_2020_03_30;
#[cfg(all(feature = "package-2020-03-30", not(feature = "no-default-version")))]
pub use package_2020_03_30::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2020-03")]
pub mod package_2020_03;
#[cfg(all(feature = "package-2020-03", not(feature = "no-default-version")))]
pub use package_2020_03::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2019-09")]
pub mod package_2019_09;
#[cfg(all(feature = "package-2019-09", not(feature = "no-default-version")))]
pub use package_2019_09::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2018-08-preview")]
pub mod package_2018_08_preview;
#[cfg(all(feature = "package-2018-08-preview", not(feature = "no-default-version")))]
pub use package_2018_08_preview::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
