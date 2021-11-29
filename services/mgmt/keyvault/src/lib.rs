#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![doc = "generated by AutoRust 0.1.0"]
#[cfg(feature = "package-preview-2021-06")]
pub mod package_preview_2021_06;
#[cfg(all(feature = "package-preview-2021-06", not(feature = "no-default-version")))]
pub use package_preview_2021_06::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-preview-2021-04")]
pub mod package_preview_2021_04;
#[cfg(all(feature = "package-preview-2021-04", not(feature = "no-default-version")))]
pub use package_preview_2021_04::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-preview-2021-04-full")]
pub mod package_preview_2021_04_full;
#[cfg(all(feature = "package-preview-2021-04-full", not(feature = "no-default-version")))]
pub use package_preview_2021_04_full::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-preview-2020-04")]
pub mod package_preview_2020_04;
#[cfg(all(feature = "package-preview-2020-04", not(feature = "no-default-version")))]
pub use package_preview_2020_04::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-preview-2020-04-full")]
pub mod package_preview_2020_04_full;
#[cfg(all(feature = "package-preview-2020-04-full", not(feature = "no-default-version")))]
pub use package_preview_2020_04_full::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2019-09")]
pub mod package_2019_09;
#[cfg(all(feature = "package-2019-09", not(feature = "no-default-version")))]
pub use package_2019_09::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2018-02-14-preview")]
pub mod package_2018_02_14_preview;
#[cfg(all(feature = "package-2018-02-14-preview", not(feature = "no-default-version")))]
pub use package_2018_02_14_preview::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2018-02")]
pub mod package_2018_02;
#[cfg(all(feature = "package-2018-02", not(feature = "no-default-version")))]
pub use package_2018_02::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2016-10")]
pub mod package_2016_10;
#[cfg(all(feature = "package-2016-10", not(feature = "no-default-version")))]
pub use package_2016_10::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2015-06")]
pub mod package_2015_06;
#[cfg(all(feature = "package-2015-06", not(feature = "no-default-version")))]
pub use package_2015_06::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "profile-hybrid-2020-09-01")]
pub mod profile_hybrid_2020_09_01;
#[cfg(all(feature = "profile-hybrid-2020-09-01", not(feature = "no-default-version")))]
pub use profile_hybrid_2020_09_01::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
