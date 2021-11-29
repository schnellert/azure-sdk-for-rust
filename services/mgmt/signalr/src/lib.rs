#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![doc = "generated by AutoRust 0.1.0"]
#[cfg(feature = "package-2021-09-01-preview")]
pub mod package_2021_09_01_preview;
#[cfg(all(feature = "package-2021-09-01-preview", not(feature = "no-default-version")))]
pub use package_2021_09_01_preview::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2021-10-01")]
pub mod package_2021_10_01;
#[cfg(all(feature = "package-2021-10-01", not(feature = "no-default-version")))]
pub use package_2021_10_01::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2021-06-01-preview")]
pub mod package_2021_06_01_preview;
#[cfg(all(feature = "package-2021-06-01-preview", not(feature = "no-default-version")))]
pub use package_2021_06_01_preview::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2021-04-01-preview")]
pub mod package_2021_04_01_preview;
#[cfg(all(feature = "package-2021-04-01-preview", not(feature = "no-default-version")))]
pub use package_2021_04_01_preview::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2020-07-01-preview")]
pub mod package_2020_07_01_preview;
#[cfg(all(feature = "package-2020-07-01-preview", not(feature = "no-default-version")))]
pub use package_2020_07_01_preview::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2020-05-01")]
pub mod package_2020_05_01;
#[cfg(all(feature = "package-2020-05-01", not(feature = "no-default-version")))]
pub use package_2020_05_01::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2018-10-01")]
pub mod package_2018_10_01;
#[cfg(all(feature = "package-2018-10-01", not(feature = "no-default-version")))]
pub use package_2018_10_01::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2018-03-01-preview")]
pub mod package_2018_03_01_preview;
#[cfg(all(feature = "package-2018-03-01-preview", not(feature = "no-default-version")))]
pub use package_2018_03_01_preview::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
