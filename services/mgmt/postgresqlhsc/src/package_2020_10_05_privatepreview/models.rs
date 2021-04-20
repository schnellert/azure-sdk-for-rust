#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PostgreSqlVersion {
    #[serde(rename = "11")]
    _11,
    #[serde(rename = "12")]
    _12,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum CitusVersion {
    #[serde(rename = "8.3")]
    _8_3,
    #[serde(rename = "9.0")]
    _9_0,
    #[serde(rename = "9.1")]
    _9_1,
    #[serde(rename = "9.2")]
    _9_2,
    #[serde(rename = "9.3")]
    _9_3,
    #[serde(rename = "9.4")]
    _9_4,
    #[serde(rename = "9.5")]
    _9_5,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ServerState {
    Ready,
    Dropping,
    Disabled,
    Starting,
    Stopping,
    Stopped,
    Updating,
    Provisioning,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ServerHaState {
    NotEnabled,
    CreatingStandby,
    ReplicatingData,
    FailingOver,
    Healthy,
    RemovingStandby,
    NotSync,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ServerRole {
    Coordinator,
    Worker,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FullyQualifiedDomainName {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerProperties {
    #[serde(rename = "serverEdition", default, skip_serializing_if = "Option::is_none")]
    pub server_edition: Option<server_properties::ServerEdition>,
    #[serde(rename = "storageQuotaInMb", default, skip_serializing_if = "Option::is_none")]
    pub storage_quota_in_mb: Option<i64>,
    #[serde(rename = "vCores", default, skip_serializing_if = "Option::is_none")]
    pub v_cores: Option<i64>,
    #[serde(rename = "enableHa", default, skip_serializing_if = "Option::is_none")]
    pub enable_ha: Option<bool>,
    #[serde(rename = "enablePublicIp", skip_serializing)]
    pub enable_public_ip: Option<bool>,
}
pub mod server_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ServerEdition {
        GeneralPurpose,
        MemoryOptimized,
    }
}
pub type ServerRoleGroupList = Vec<ServerRoleGroup>;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerRoleGroup {
    #[serde(flatten)]
    pub server_properties: ServerProperties,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<ServerRole>,
    #[serde(rename = "serverCount", default, skip_serializing_if = "Option::is_none")]
    pub server_count: Option<i32>,
    #[serde(rename = "serverNames", skip_serializing)]
    pub server_names: Vec<ServerNameItem>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerNameItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "fullyQualifiedDomainName", skip_serializing)]
    pub fully_qualified_domain_name: Option<FullyQualifiedDomainName>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerGroupServerProperties {
    #[serde(flatten)]
    pub server_properties: ServerProperties,
    #[serde(rename = "fullyQualifiedDomainName", skip_serializing)]
    pub fully_qualified_domain_name: Option<FullyQualifiedDomainName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<ServerRole>,
    #[serde(skip_serializing)]
    pub state: Option<ServerState>,
    #[serde(rename = "haState", skip_serializing)]
    pub ha_state: Option<ServerHaState>,
    #[serde(rename = "administratorLogin", default, skip_serializing_if = "Option::is_none")]
    pub administrator_login: Option<String>,
    #[serde(rename = "postgresqlVersion", default, skip_serializing_if = "Option::is_none")]
    pub postgresql_version: Option<PostgreSqlVersion>,
    #[serde(rename = "citusVersion", default, skip_serializing_if = "Option::is_none")]
    pub citus_version: Option<CitusVersion>,
    #[serde(rename = "availabilityZone", default, skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "standbyAvailabilityZone", default, skip_serializing_if = "Option::is_none")]
    pub standby_availability_zone: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerGroupServer {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(rename = "systemData", skip_serializing)]
    pub system_data: Option<SystemData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServerGroupServerProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerGroupServerListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ServerGroupServer>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerGroupForUpdate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServerGroupPropertiesForUpdate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerGroupPropertiesForUpdate {
    #[serde(rename = "administratorLoginPassword", default, skip_serializing_if = "Option::is_none")]
    pub administrator_login_password: Option<String>,
    #[serde(rename = "backupRetentionDays", default, skip_serializing_if = "Option::is_none")]
    pub backup_retention_days: Option<i32>,
    #[serde(rename = "postgresqlVersion", default, skip_serializing_if = "Option::is_none")]
    pub postgresql_version: Option<PostgreSqlVersion>,
    #[serde(rename = "citusVersion", default, skip_serializing_if = "Option::is_none")]
    pub citus_version: Option<CitusVersion>,
    #[serde(rename = "enableShardsOnCoordinator", default, skip_serializing_if = "Option::is_none")]
    pub enable_shards_on_coordinator: Option<bool>,
    #[serde(rename = "serverRoleGroups", default, skip_serializing_if = "Option::is_none")]
    pub server_role_groups: Option<ServerRoleGroupList>,
    #[serde(rename = "maintenanceWindow", default, skip_serializing_if = "Option::is_none")]
    pub maintenance_window: Option<MaintenanceWindow>,
    #[serde(rename = "availabilityZone", default, skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "standbyAvailabilityZone", default, skip_serializing_if = "Option::is_none")]
    pub standby_availability_zone: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerGroupListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ServerGroup>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerGroup {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(rename = "systemData", skip_serializing)]
    pub system_data: Option<SystemData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServerGroupProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerGroupProperties {
    #[serde(rename = "createMode", default, skip_serializing_if = "Option::is_none")]
    pub create_mode: Option<server_group_properties::CreateMode>,
    #[serde(rename = "administratorLogin", default, skip_serializing_if = "Option::is_none")]
    pub administrator_login: Option<String>,
    #[serde(rename = "administratorLoginPassword", default, skip_serializing_if = "Option::is_none")]
    pub administrator_login_password: Option<String>,
    #[serde(rename = "backupRetentionDays", default, skip_serializing_if = "Option::is_none")]
    pub backup_retention_days: Option<i32>,
    #[serde(rename = "postgresqlVersion", default, skip_serializing_if = "Option::is_none")]
    pub postgresql_version: Option<PostgreSqlVersion>,
    #[serde(rename = "citusVersion", default, skip_serializing_if = "Option::is_none")]
    pub citus_version: Option<CitusVersion>,
    #[serde(rename = "enableMx", default, skip_serializing_if = "Option::is_none")]
    pub enable_mx: Option<bool>,
    #[serde(rename = "enableZfs", default, skip_serializing_if = "Option::is_none")]
    pub enable_zfs: Option<bool>,
    #[serde(rename = "enableShardsOnCoordinator", default, skip_serializing_if = "Option::is_none")]
    pub enable_shards_on_coordinator: Option<bool>,
    #[serde(skip_serializing)]
    pub state: Option<ServerState>,
    #[serde(rename = "earliestRestoreTime", skip_serializing)]
    pub earliest_restore_time: Option<String>,
    #[serde(rename = "resourceProviderType", skip_serializing)]
    pub resource_provider_type: Option<server_group_properties::ResourceProviderType>,
    #[serde(rename = "serverRoleGroups", default, skip_serializing_if = "Option::is_none")]
    pub server_role_groups: Option<ServerRoleGroupList>,
    #[serde(rename = "maintenanceWindow", default, skip_serializing_if = "Option::is_none")]
    pub maintenance_window: Option<MaintenanceWindow>,
    #[serde(rename = "availabilityZone", default, skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "standbyAvailabilityZone", default, skip_serializing_if = "Option::is_none")]
    pub standby_availability_zone: Option<String>,
    #[serde(rename = "delegatedSubnetArguments", default, skip_serializing_if = "Option::is_none")]
    pub delegated_subnet_arguments: Option<server_group_properties::DelegatedSubnetArguments>,
    #[serde(rename = "readReplicas", skip_serializing)]
    pub read_replicas: Vec<String>,
    #[serde(rename = "sourceServerGroup", skip_serializing)]
    pub source_server_group: Option<String>,
    #[serde(rename = "sourceSubscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub source_subscription_id: Option<String>,
    #[serde(rename = "sourceResourceGroupName", default, skip_serializing_if = "Option::is_none")]
    pub source_resource_group_name: Option<String>,
    #[serde(rename = "sourceServerGroupName", default, skip_serializing_if = "Option::is_none")]
    pub source_server_group_name: Option<String>,
    #[serde(rename = "sourceLocation", default, skip_serializing_if = "Option::is_none")]
    pub source_location: Option<String>,
    #[serde(rename = "pointInTimeUTC", default, skip_serializing_if = "Option::is_none")]
    pub point_in_time_utc: Option<String>,
}
pub mod server_group_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CreateMode {
        Default,
        PointInTimeRestore,
        ReadReplica,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ResourceProviderType {
        Meru,
        Marlin,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct DelegatedSubnetArguments {
        #[serde(rename = "subnetArmResourceId", default, skip_serializing_if = "Option::is_none")]
        pub subnet_arm_resource_id: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MaintenanceWindow {
    #[serde(rename = "customWindow", default, skip_serializing_if = "Option::is_none")]
    pub custom_window: Option<String>,
    #[serde(rename = "startHour", default, skip_serializing_if = "Option::is_none")]
    pub start_hour: Option<i32>,
    #[serde(rename = "startMinute", default, skip_serializing_if = "Option::is_none")]
    pub start_minute: Option<i32>,
    #[serde(rename = "dayOfWeek", default, skip_serializing_if = "Option::is_none")]
    pub day_of_week: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerGroupConfiguration {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(rename = "systemData", skip_serializing)]
    pub system_data: Option<SystemData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServerGroupConfigurationProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerGroupConfigurationProperties {
    #[serde(skip_serializing)]
    pub description: Option<String>,
    #[serde(rename = "dataType", skip_serializing)]
    pub data_type: Option<server_group_configuration_properties::DataType>,
    #[serde(rename = "allowedValues", skip_serializing)]
    pub allowed_values: Option<String>,
    #[serde(rename = "serverRoleGroupConfigurations")]
    pub server_role_group_configurations: Vec<ServerRoleGroupConfiguration>,
}
pub mod server_group_configuration_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DataType {
        Boolean,
        Numeric,
        Integer,
        Enumeration,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerRoleGroupConfiguration {
    pub role: ServerRole,
    pub value: String,
    #[serde(rename = "defaultValue", skip_serializing)]
    pub default_value: Option<String>,
    #[serde(skip_serializing)]
    pub source: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerGroupConfigurationListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ServerGroupConfiguration>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerConfigurationProperties {
    pub value: String,
    #[serde(skip_serializing)]
    pub source: Option<String>,
    #[serde(skip_serializing)]
    pub description: Option<String>,
    #[serde(rename = "defaultValue", skip_serializing)]
    pub default_value: Option<String>,
    #[serde(rename = "dataType", skip_serializing)]
    pub data_type: Option<server_configuration_properties::DataType>,
    #[serde(rename = "allowedValues", skip_serializing)]
    pub allowed_values: Option<String>,
}
pub mod server_configuration_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DataType {
        Boolean,
        Numeric,
        Integer,
        Enumeration,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerConfiguration {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(rename = "systemData", skip_serializing)]
    pub system_data: Option<SystemData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServerConfigurationProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerConfigurationListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ServerConfiguration>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FirewallRuleProperties {
    #[serde(rename = "startIpAddress")]
    pub start_ip_address: String,
    #[serde(rename = "endIpAddress")]
    pub end_ip_address: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FirewallRule {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(rename = "systemData", skip_serializing)]
    pub system_data: Option<SystemData>,
    pub properties: FirewallRuleProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FirewallRuleListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<FirewallRule>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleProperties {
    pub password: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Role {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(rename = "systemData", skip_serializing)]
    pub system_data: Option<SystemData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RoleProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Role>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationDisplay {
    #[serde(skip_serializing)]
    pub provider: Option<String>,
    #[serde(skip_serializing)]
    pub resource: Option<String>,
    #[serde(skip_serializing)]
    pub operation: Option<String>,
    #[serde(skip_serializing)]
    pub description: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplay>,
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[serde(skip_serializing)]
    pub origin: Option<operation::Origin>,
    #[serde(skip_serializing)]
    pub properties: Option<serde_json::Value>,
}
pub mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Origin {
        NotSpecified,
        #[serde(rename = "user")]
        User,
        #[serde(rename = "system")]
        System,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NameAvailabilityRequest {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: name_availability_request::Type,
}
pub mod name_availability_request {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "Microsoft.DBforPostgreSQL/serverGroupsv2")]
        MicrosoftDBforPostgreSqlServerGroupsv2,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NameAvailability {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<CloudErrorBody>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudErrorBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<CloudErrorBody>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SystemData {
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "createdByType", default, skip_serializing_if = "Option::is_none")]
    pub created_by_type: Option<system_data::CreatedByType>,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_data::LastModifiedByType>,
    #[serde(rename = "lastModifiedAt", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_at: Option<String>,
}
pub mod system_data {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CreatedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum LastModifiedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProxyResource {
    #[serde(flatten)]
    pub resource: Resource,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    pub location: String,
}
