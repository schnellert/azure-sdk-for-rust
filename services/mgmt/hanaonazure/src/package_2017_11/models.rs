#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HanaInstancesListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<HanaInstance>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HanaInstance {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<HanaInstanceProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HanaInstanceProperties {
    #[serde(rename = "hardwareProfile", default, skip_serializing_if = "Option::is_none")]
    pub hardware_profile: Option<HardwareProfile>,
    #[serde(rename = "storageProfile", default, skip_serializing_if = "Option::is_none")]
    pub storage_profile: Option<StorageProfile>,
    #[serde(rename = "osProfile", default, skip_serializing_if = "Option::is_none")]
    pub os_profile: Option<OsProfile>,
    #[serde(rename = "networkProfile", default, skip_serializing_if = "Option::is_none")]
    pub network_profile: Option<NetworkProfile>,
    #[serde(rename = "hanaInstanceId", default, skip_serializing_if = "Option::is_none")]
    pub hana_instance_id: Option<String>,
    #[serde(rename = "powerState", default, skip_serializing_if = "Option::is_none")]
    pub power_state: Option<hana_instance_properties::PowerState>,
    #[serde(rename = "proximityPlacementGroup", default, skip_serializing_if = "Option::is_none")]
    pub proximity_placement_group: Option<String>,
    #[serde(rename = "hwRevision", default, skip_serializing_if = "Option::is_none")]
    pub hw_revision: Option<String>,
    #[serde(rename = "partnerNodeId", default, skip_serializing_if = "Option::is_none")]
    pub partner_node_id: Option<String>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<hana_instance_properties::ProvisioningState>,
}
pub mod hana_instance_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PowerState {
        #[serde(rename = "starting")]
        Starting,
        #[serde(rename = "started")]
        Started,
        #[serde(rename = "stopping")]
        Stopping,
        #[serde(rename = "stopped")]
        Stopped,
        #[serde(rename = "restarting")]
        Restarting,
        #[serde(rename = "unknown")]
        Unknown,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Accepted,
        Creating,
        Updating,
        Failed,
        Succeeded,
        Deleting,
        Migrating,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HardwareProfile {
    #[serde(rename = "hardwareType", default, skip_serializing_if = "Option::is_none")]
    pub hardware_type: Option<hardware_profile::HardwareType>,
    #[serde(rename = "hanaInstanceSize", default, skip_serializing_if = "Option::is_none")]
    pub hana_instance_size: Option<hardware_profile::HanaInstanceSize>,
}
pub mod hardware_profile {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum HardwareType {
        #[serde(rename = "Cisco_UCS")]
        CiscoUcs,
        #[serde(rename = "HPE")]
        Hpe,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum HanaInstanceSize {
        S72m,
        S144m,
        S72,
        S144,
        S192,
        S192m,
        S192xm,
        S96,
        S112,
        S224,
        S224m,
        S224om,
        S224oo,
        S224oom,
        S224ooo,
        S384,
        S384m,
        S384xm,
        S384xxm,
        S448,
        S448m,
        S448om,
        S448oo,
        S448oom,
        S448ooo,
        S576m,
        S576xm,
        S672,
        S672m,
        S672om,
        S672oo,
        S672oom,
        S672ooo,
        S768,
        S768m,
        S768xm,
        S896,
        S896m,
        S896om,
        S896oo,
        S896oom,
        S896ooo,
        S960m,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Disk {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "diskSizeGB", default, skip_serializing_if = "Option::is_none")]
    pub disk_size_gb: Option<i32>,
    #[serde(skip_serializing)]
    pub lun: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageProfile {
    #[serde(rename = "nfsIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub nfs_ip_address: Option<String>,
    #[serde(rename = "osDisks", default, skip_serializing_if = "Vec::is_empty")]
    pub os_disks: Vec<Disk>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OsProfile {
    #[serde(rename = "computerName", default, skip_serializing_if = "Option::is_none")]
    pub computer_name: Option<String>,
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "sshPublicKey", default, skip_serializing_if = "Option::is_none")]
    pub ssh_public_key: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkProfile {
    #[serde(rename = "networkInterfaces", default, skip_serializing_if = "Vec::is_empty")]
    pub network_interfaces: Vec<IpAddress>,
    #[serde(rename = "circuitId", default, skip_serializing_if = "Option::is_none")]
    pub circuit_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IpAddress {
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<Display>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Display {
    #[serde(skip_serializing)]
    pub provider: Option<String>,
    #[serde(skip_serializing)]
    pub resource: Option<String>,
    #[serde(skip_serializing)]
    pub operation: Option<String>,
    #[serde(skip_serializing)]
    pub description: Option<String>,
    #[serde(skip_serializing)]
    pub origin: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<error_response::Error>,
}
pub mod error_response {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Error {
        #[serde(skip_serializing)]
        pub code: Option<String>,
        #[serde(skip_serializing)]
        pub message: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Tags {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitoringDetails {
    #[serde(rename = "hanaSubnet", default, skip_serializing_if = "Option::is_none")]
    pub hana_subnet: Option<String>,
    #[serde(rename = "hanaHostname", default, skip_serializing_if = "Option::is_none")]
    pub hana_hostname: Option<String>,
    #[serde(rename = "hanaDbName", default, skip_serializing_if = "Option::is_none")]
    pub hana_db_name: Option<String>,
    #[serde(rename = "hanaDbSqlPort", default, skip_serializing_if = "Option::is_none")]
    pub hana_db_sql_port: Option<i64>,
    #[serde(rename = "hanaDbUsername", default, skip_serializing_if = "Option::is_none")]
    pub hana_db_username: Option<String>,
    #[serde(rename = "hanaDbPassword", default, skip_serializing_if = "Option::is_none")]
    pub hana_db_password: Option<String>,
}
