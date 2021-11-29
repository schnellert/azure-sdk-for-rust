#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![doc = "generated by AutoRust 0.1.0"]
#[cfg(feature = "package-policy-2021-06")]
pub mod package_policy_2021_06;
#[cfg(all(feature = "package-policy-2021-06", not(feature = "no-default-version")))]
pub use package_policy_2021_06::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-privatelinks-2020-05")]
pub mod package_privatelinks_2020_05;
#[cfg(all(feature = "package-privatelinks-2020-05", not(feature = "no-default-version")))]
pub use package_privatelinks_2020_05::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-locks-2020-05")]
pub mod package_locks_2020_05;
#[cfg(all(feature = "package-locks-2020-05", not(feature = "no-default-version")))]
pub use package_locks_2020_05::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-resources-2021-04")]
pub mod package_resources_2021_04;
#[cfg(all(feature = "package-resources-2021-04", not(feature = "no-default-version")))]
pub use package_resources_2021_04::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-locks-2017-04")]
pub mod package_locks_2017_04;
#[cfg(all(feature = "package-locks-2017-04", not(feature = "no-default-version")))]
pub use package_locks_2017_04::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-preview-2020-08")]
pub mod package_preview_2020_08;
#[cfg(all(feature = "package-preview-2020-08", not(feature = "no-default-version")))]
pub use package_preview_2020_08::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-subscriptions-2021-01")]
pub mod package_subscriptions_2021_01;
#[cfg(all(feature = "package-subscriptions-2021-01", not(feature = "no-default-version")))]
pub use package_subscriptions_2021_01::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-deploymentscripts-2020-10")]
pub mod package_deploymentscripts_2020_10;
#[cfg(all(feature = "package-deploymentscripts-2020-10", not(feature = "no-default-version")))]
pub use package_deploymentscripts_2020_10::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-deploymentscripts-2019-10-preview")]
pub mod package_deploymentscripts_2019_10_preview;
#[cfg(all(feature = "package-deploymentscripts-2019-10-preview", not(feature = "no-default-version")))]
pub use package_deploymentscripts_2019_10_preview::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-features-2021-07")]
pub mod package_features_2021_07;
#[cfg(all(feature = "package-features-2021-07", not(feature = "no-default-version")))]
pub use package_features_2021_07::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-features-2015-12")]
pub mod package_features_2015_12;
#[cfg(all(feature = "package-features-2015-12", not(feature = "no-default-version")))]
pub use package_features_2015_12::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-locks-2016-09")]
pub mod package_locks_2016_09;
#[cfg(all(feature = "package-locks-2016-09", not(feature = "no-default-version")))]
pub use package_locks_2016_09::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-locks-2015-01")]
pub mod package_locks_2015_01;
#[cfg(all(feature = "package-locks-2015-01", not(feature = "no-default-version")))]
pub use package_locks_2015_01::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-policy-2019-09")]
pub mod package_policy_2019_09;
#[cfg(all(feature = "package-policy-2019-09", not(feature = "no-default-version")))]
pub use package_policy_2019_09::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-policy-2019-06")]
pub mod package_policy_2019_06;
#[cfg(all(feature = "package-policy-2019-06", not(feature = "no-default-version")))]
pub use package_policy_2019_06::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-policy-2019-01")]
pub mod package_policy_2019_01;
#[cfg(all(feature = "package-policy-2019-01", not(feature = "no-default-version")))]
pub use package_policy_2019_01::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-policy-2018-05")]
pub mod package_policy_2018_05;
#[cfg(all(feature = "package-policy-2018-05", not(feature = "no-default-version")))]
pub use package_policy_2018_05::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-policy-2018-03")]
pub mod package_policy_2018_03;
#[cfg(all(feature = "package-policy-2018-03", not(feature = "no-default-version")))]
pub use package_policy_2018_03::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-policy-2017-06")]
pub mod package_policy_2017_06;
#[cfg(all(feature = "package-policy-2017-06", not(feature = "no-default-version")))]
pub use package_policy_2017_06::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-pure-policy-2017-06")]
pub mod package_pure_policy_2017_06;
#[cfg(all(feature = "package-pure-policy-2017-06", not(feature = "no-default-version")))]
pub use package_pure_policy_2017_06::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-templatespecs-2021-05")]
pub mod package_templatespecs_2021_05;
#[cfg(all(feature = "package-templatespecs-2021-05", not(feature = "no-default-version")))]
pub use package_templatespecs_2021_05::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-templatespecs-2021-03-preview")]
pub mod package_templatespecs_2021_03_preview;
#[cfg(all(feature = "package-templatespecs-2021-03-preview", not(feature = "no-default-version")))]
pub use package_templatespecs_2021_03_preview::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-templatespecs-2019-06-preview")]
pub mod package_templatespecs_2019_06_preview;
#[cfg(all(feature = "package-templatespecs-2019-06-preview", not(feature = "no-default-version")))]
pub use package_templatespecs_2019_06_preview::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-policy-2016-12")]
pub mod package_policy_2016_12;
#[cfg(all(feature = "package-policy-2016-12", not(feature = "no-default-version")))]
pub use package_policy_2016_12::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-policy-2016-04")]
pub mod package_policy_2016_04;
#[cfg(all(feature = "package-policy-2016-04", not(feature = "no-default-version")))]
pub use package_policy_2016_04::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-policy-2015-10")]
pub mod package_policy_2015_10;
#[cfg(all(feature = "package-policy-2015-10", not(feature = "no-default-version")))]
pub use package_policy_2015_10::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-resources-2021-01")]
pub mod package_resources_2021_01;
#[cfg(all(feature = "package-resources-2021-01", not(feature = "no-default-version")))]
pub use package_resources_2021_01::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-resources-2020-10")]
pub mod package_resources_2020_10;
#[cfg(all(feature = "package-resources-2020-10", not(feature = "no-default-version")))]
pub use package_resources_2020_10::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-resources-2020-08")]
pub mod package_resources_2020_08;
#[cfg(all(feature = "package-resources-2020-08", not(feature = "no-default-version")))]
pub use package_resources_2020_08::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-resources-2020-06")]
pub mod package_resources_2020_06;
#[cfg(all(feature = "package-resources-2020-06", not(feature = "no-default-version")))]
pub use package_resources_2020_06::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-resources-2019-10")]
pub mod package_resources_2019_10;
#[cfg(all(feature = "package-resources-2019-10", not(feature = "no-default-version")))]
pub use package_resources_2019_10::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-resources-2019-08")]
pub mod package_resources_2019_08;
#[cfg(all(feature = "package-resources-2019-08", not(feature = "no-default-version")))]
pub use package_resources_2019_08::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-resources-2019-07")]
pub mod package_resources_2019_07;
#[cfg(all(feature = "package-resources-2019-07", not(feature = "no-default-version")))]
pub use package_resources_2019_07::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-resources-2019-0510")]
pub mod package_resources_2019_0510;
#[cfg(all(feature = "package-resources-2019-0510", not(feature = "no-default-version")))]
pub use package_resources_2019_0510::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-resources-2019-05")]
pub mod package_resources_2019_05;
#[cfg(all(feature = "package-resources-2019-05", not(feature = "no-default-version")))]
pub use package_resources_2019_05::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-resources-2019-03")]
pub mod package_resources_2019_03;
#[cfg(all(feature = "package-resources-2019-03", not(feature = "no-default-version")))]
pub use package_resources_2019_03::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-resources-2018-05")]
pub mod package_resources_2018_05;
#[cfg(all(feature = "package-resources-2018-05", not(feature = "no-default-version")))]
pub use package_resources_2018_05::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-resources-2018-02")]
pub mod package_resources_2018_02;
#[cfg(all(feature = "package-resources-2018-02", not(feature = "no-default-version")))]
pub use package_resources_2018_02::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-resources-2017-05")]
pub mod package_resources_2017_05;
#[cfg(all(feature = "package-resources-2017-05", not(feature = "no-default-version")))]
pub use package_resources_2017_05::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-resources-2016-09")]
pub mod package_resources_2016_09;
#[cfg(all(feature = "package-resources-2016-09", not(feature = "no-default-version")))]
pub use package_resources_2016_09::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-resources-2016-07")]
pub mod package_resources_2016_07;
#[cfg(all(feature = "package-resources-2016-07", not(feature = "no-default-version")))]
pub use package_resources_2016_07::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-resources-2016-02")]
pub mod package_resources_2016_02;
#[cfg(all(feature = "package-resources-2016-02", not(feature = "no-default-version")))]
pub use package_resources_2016_02::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-resources-2015-11")]
pub mod package_resources_2015_11;
#[cfg(all(feature = "package-resources-2015-11", not(feature = "no-default-version")))]
pub use package_resources_2015_11::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-subscriptions-2020-01")]
pub mod package_subscriptions_2020_01;
#[cfg(all(feature = "package-subscriptions-2020-01", not(feature = "no-default-version")))]
pub use package_subscriptions_2020_01::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-subscriptions-2019-11")]
pub mod package_subscriptions_2019_11;
#[cfg(all(feature = "package-subscriptions-2019-11", not(feature = "no-default-version")))]
pub use package_subscriptions_2019_11::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-subscriptions-2019-06")]
pub mod package_subscriptions_2019_06;
#[cfg(all(feature = "package-subscriptions-2019-06", not(feature = "no-default-version")))]
pub use package_subscriptions_2019_06::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-subscriptions-2018-06")]
pub mod package_subscriptions_2018_06;
#[cfg(all(feature = "package-subscriptions-2018-06", not(feature = "no-default-version")))]
pub use package_subscriptions_2018_06::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-subscriptions-2016-06")]
pub mod package_subscriptions_2016_06;
#[cfg(all(feature = "package-subscriptions-2016-06", not(feature = "no-default-version")))]
pub use package_subscriptions_2016_06::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-subscriptions-2015-11")]
pub mod package_subscriptions_2015_11;
#[cfg(all(feature = "package-subscriptions-2015-11", not(feature = "no-default-version")))]
pub use package_subscriptions_2015_11::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-links-2016-09")]
pub mod package_links_2016_09;
#[cfg(all(feature = "package-links-2016-09", not(feature = "no-default-version")))]
pub use package_links_2016_09::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-managedapplications-2019-07")]
pub mod package_managedapplications_2019_07;
#[cfg(all(feature = "package-managedapplications-2019-07", not(feature = "no-default-version")))]
pub use package_managedapplications_2019_07::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-managedapplications-2018-06")]
pub mod package_managedapplications_2018_06;
#[cfg(all(feature = "package-managedapplications-2018-06", not(feature = "no-default-version")))]
pub use package_managedapplications_2018_06::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-managedapplications-2017-09")]
pub mod package_managedapplications_2017_09;
#[cfg(all(feature = "package-managedapplications-2017-09", not(feature = "no-default-version")))]
pub use package_managedapplications_2017_09::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-managedapplications-2016-09")]
pub mod package_managedapplications_2016_09;
#[cfg(all(feature = "package-managedapplications-2016-09", not(feature = "no-default-version")))]
pub use package_managedapplications_2016_09::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
