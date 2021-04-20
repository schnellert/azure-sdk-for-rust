#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagementLockOwner {
    #[serde(rename = "applicationId", default, skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagementLockProperties {
    pub level: management_lock_properties::Level,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub owners: Vec<ManagementLockOwner>,
}
pub mod management_lock_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Level {
        NotSpecified,
        CanNotDelete,
        ReadOnly,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagementLockObject {
    pub properties: ManagementLockProperties,
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagementLockListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ManagementLockObject>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
}
pub mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Display {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
