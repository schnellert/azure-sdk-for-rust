#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![doc = "generated by AutoRust 0.1.0"]
#[cfg(feature = "package-2018-05")]
pub mod package_2018_05;
#[cfg(all(feature = "package-2018-05", not(feature = "no-default-version")))]
pub use package_2018_05::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2018-03-preview")]
pub mod package_2018_03_preview;
#[cfg(all(feature = "package-2018-03-preview", not(feature = "no-default-version")))]
pub use package_2018_03_preview::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2017-10")]
pub mod package_2017_10;
#[cfg(all(feature = "package-2017-10", not(feature = "no-default-version")))]
pub use package_2017_10::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2017-09")]
pub mod package_2017_09;
#[cfg(all(feature = "package-2017-09", not(feature = "no-default-version")))]
pub use package_2017_09::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2016-04")]
pub mod package_2016_04;
#[cfg(all(feature = "package-2016-04", not(feature = "no-default-version")))]
pub use package_2016_04::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2015-05-preview")]
pub mod package_2015_05_preview;
#[cfg(all(feature = "package-2015-05-preview", not(feature = "no-default-version")))]
pub use package_2015_05_preview::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "profile-hybrid-2020-09-01")]
pub mod profile_hybrid_2020_09_01;
#[cfg(all(feature = "profile-hybrid-2020-09-01", not(feature = "no-default-version")))]
pub use profile_hybrid_2020_09_01::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
