#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![doc = "generated by AutoRust 0.1.0"]
#[cfg(feature = "package-2021-10")]
pub mod package_2021_10;
#[cfg(all(feature = "package-2021-10", not(feature = "no-default-version")))]
pub use package_2021_10::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2020-09")]
pub mod package_2020_09;
#[cfg(all(feature = "package-2020-09", not(feature = "no-default-version")))]
pub use package_2020_09::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2020-01")]
pub mod package_2020_01;
#[cfg(all(feature = "package-2020-01", not(feature = "no-default-version")))]
pub use package_2020_01::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2019-10-preview")]
pub mod package_2019_10_preview;
#[cfg(all(feature = "package-2019-10-preview", not(feature = "no-default-version")))]
pub use package_2019_10_preview::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2019-03-preview")]
pub mod package_2019_03_preview;
#[cfg(all(feature = "package-2019-03-preview", not(feature = "no-default-version")))]
pub use package_2019_03_preview::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2018-11-preview")]
pub mod package_2018_11_preview;
#[cfg(all(feature = "package-2018-11-preview", not(feature = "no-default-version")))]
pub use package_2018_11_preview::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2018-03-preview")]
pub mod package_2018_03_preview;
#[cfg(all(feature = "package-2018-03-preview", not(feature = "no-default-version")))]
pub use package_2018_03_preview::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2017-11-preview")]
pub mod package_2017_11_preview;
#[cfg(all(feature = "package-2017-11-preview", not(feature = "no-default-version")))]
pub use package_2017_11_preview::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
