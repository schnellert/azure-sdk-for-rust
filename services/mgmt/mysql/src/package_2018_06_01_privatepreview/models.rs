#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ServerVersion {
    #[serde(rename = "5.6")]
    _5_6,
    #[serde(rename = "5.7")]
    _5_7,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SslEnforcement {
    Enabled,
    Disabled,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum MinimalTlsVersion {
    #[serde(rename = "TLS1_0")]
    Tls10,
    #[serde(rename = "TLS1_1")]
    Tls11,
    #[serde(rename = "TLS1_2")]
    Tls12,
    #[serde(rename = "TLSEnforcementDisabled")]
    TlsEnforcementDisabled,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerPrivateEndpointConnection {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServerPrivateEndpointConnectionProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerPrivateEndpointConnectionProperties {
    #[serde(rename = "privateEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<PrivateEndpointProperty>,
    #[serde(rename = "privateLinkServiceConnectionState", default, skip_serializing_if = "Option::is_none")]
    pub private_link_service_connection_state: Option<ServerPrivateLinkServiceConnectionStateProperty>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<server_private_endpoint_connection_properties::ProvisioningState>,
}
pub mod server_private_endpoint_connection_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Approving,
        Ready,
        Dropping,
        Failed,
        Rejecting,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerPrivateLinkServiceConnectionStateProperty {
    pub status: server_private_link_service_connection_state_property::Status,
    pub description: String,
    #[serde(rename = "actionsRequired", skip_serializing)]
    pub actions_required: Option<server_private_link_service_connection_state_property::ActionsRequired>,
}
pub mod server_private_link_service_connection_state_property {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Approved,
        Pending,
        Rejected,
        Disconnected,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ActionsRequired {
        None,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerProperties {
    #[serde(rename = "administratorLogin", default, skip_serializing_if = "Option::is_none")]
    pub administrator_login: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<ServerVersion>,
    #[serde(rename = "sslEnforcement", default, skip_serializing_if = "Option::is_none")]
    pub ssl_enforcement: Option<SslEnforcement>,
    #[serde(rename = "minimalTlsVersion", default, skip_serializing_if = "Option::is_none")]
    pub minimal_tls_version: Option<MinimalTlsVersion>,
    #[serde(rename = "byokEnforcement", skip_serializing)]
    pub byok_enforcement: Option<String>,
    #[serde(rename = "infrastructureEncryption", default, skip_serializing_if = "Option::is_none")]
    pub infrastructure_encryption: Option<server_properties::InfrastructureEncryption>,
    #[serde(rename = "userVisibleState", default, skip_serializing_if = "Option::is_none")]
    pub user_visible_state: Option<server_properties::UserVisibleState>,
    #[serde(rename = "fullyQualifiedDomainName", default, skip_serializing_if = "Option::is_none")]
    pub fully_qualified_domain_name: Option<String>,
    #[serde(rename = "earliestRestoreDate", default, skip_serializing_if = "Option::is_none")]
    pub earliest_restore_date: Option<String>,
    #[serde(rename = "storageProfile", default, skip_serializing_if = "Option::is_none")]
    pub storage_profile: Option<StorageProfile>,
    #[serde(rename = "replicationRole", default, skip_serializing_if = "Option::is_none")]
    pub replication_role: Option<String>,
    #[serde(rename = "masterServerId", default, skip_serializing_if = "Option::is_none")]
    pub master_server_id: Option<String>,
    #[serde(rename = "replicaCapacity", default, skip_serializing_if = "Option::is_none")]
    pub replica_capacity: Option<i32>,
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<server_properties::PublicNetworkAccess>,
    #[serde(rename = "privateEndpointConnections", skip_serializing)]
    pub private_endpoint_connections: Vec<ServerPrivateEndpointConnection>,
}
pub mod server_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum InfrastructureEncryption {
        Enabled,
        Disabled,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum UserVisibleState {
        Ready,
        Dropping,
        Disabled,
        Inaccessible,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PublicNetworkAccess {
        Enabled,
        Disabled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageProfile {
    #[serde(rename = "backupRetentionDays", default, skip_serializing_if = "Option::is_none")]
    pub backup_retention_days: Option<i64>,
    #[serde(rename = "geoRedundantBackup", default, skip_serializing_if = "Option::is_none")]
    pub geo_redundant_backup: Option<storage_profile::GeoRedundantBackup>,
    #[serde(rename = "storageMB", default, skip_serializing_if = "Option::is_none")]
    pub storage_mb: Option<i32>,
    #[serde(rename = "storageAutogrow", default, skip_serializing_if = "Option::is_none")]
    pub storage_autogrow: Option<storage_profile::StorageAutogrow>,
}
pub mod storage_profile {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum GeoRedundantBackup {
        Enabled,
        Disabled,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum StorageAutogrow {
        Enabled,
        Disabled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerPropertiesForCreate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<ServerVersion>,
    #[serde(rename = "sslEnforcement", default, skip_serializing_if = "Option::is_none")]
    pub ssl_enforcement: Option<SslEnforcement>,
    #[serde(rename = "storageProfile", default, skip_serializing_if = "Option::is_none")]
    pub storage_profile: Option<StorageProfile>,
    #[serde(rename = "createMode")]
    pub create_mode: server_properties_for_create::CreateMode,
}
pub mod server_properties_for_create {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CreateMode {
        Default,
        PointInTimeRestore,
        GeoRestore,
        Replica,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerPropertiesForDefaultCreate {
    #[serde(flatten)]
    pub server_properties_for_create: ServerPropertiesForCreate,
    #[serde(rename = "administratorLogin")]
    pub administrator_login: String,
    #[serde(rename = "administratorLoginPassword")]
    pub administrator_login_password: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerPropertiesForRestore {
    #[serde(flatten)]
    pub server_properties_for_create: ServerPropertiesForCreate,
    #[serde(rename = "sourceServerId")]
    pub source_server_id: String,
    #[serde(rename = "restorePointInTime")]
    pub restore_point_in_time: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerPropertiesForGeoRestore {
    #[serde(flatten)]
    pub server_properties_for_create: ServerPropertiesForCreate,
    #[serde(rename = "sourceServerId")]
    pub source_server_id: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerPropertiesForReplica {
    #[serde(flatten)]
    pub server_properties_for_create: ServerPropertiesForCreate,
    #[serde(rename = "sourceServerId")]
    pub source_server_id: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<sku::Tier>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
}
pub mod sku {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Tier {
        Basic,
        GeneralPurpose,
        MemoryOptimized,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceIdentity {
    #[serde(rename = "principalId", skip_serializing)]
    pub principal_id: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<resource_identity::Type>,
    #[serde(rename = "tenantId", skip_serializing)]
    pub tenant_id: Option<String>,
}
pub mod resource_identity {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        SystemAssigned,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Server {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ResourceIdentity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServerProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerForCreate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    pub properties: ServerPropertiesForCreate,
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerUpdateParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<server_update_parameters::Properties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
pub mod server_update_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[serde(rename = "storageProfile", default, skip_serializing_if = "Option::is_none")]
        pub storage_profile: Option<StorageProfile>,
        #[serde(rename = "administratorLoginPassword", default, skip_serializing_if = "Option::is_none")]
        pub administrator_login_password: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub version: Option<ServerVersion>,
        #[serde(rename = "sslEnforcement", default, skip_serializing_if = "Option::is_none")]
        pub ssl_enforcement: Option<SslEnforcement>,
        #[serde(rename = "minimalTlsVersion", default, skip_serializing_if = "Option::is_none")]
        pub minimal_tls_version: Option<MinimalTlsVersion>,
        #[serde(rename = "replicationRole", default, skip_serializing_if = "Option::is_none")]
        pub replication_role: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Server>,
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
    pub properties: FirewallRuleProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FirewallRuleListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<FirewallRule>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualNetworkRuleProperties {
    #[serde(rename = "virtualNetworkSubnetId")]
    pub virtual_network_subnet_id: String,
    #[serde(rename = "ignoreMissingVnetServiceEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub ignore_missing_vnet_service_endpoint: Option<bool>,
    #[serde(skip_serializing)]
    pub state: Option<virtual_network_rule_properties::State>,
}
pub mod virtual_network_rule_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Initializing,
        InProgress,
        Ready,
        Deleting,
        Unknown,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualNetworkRule {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualNetworkRuleProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualNetworkRuleListResult {
    #[serde(skip_serializing)]
    pub value: Vec<VirtualNetworkRule>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatabaseProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub charset: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collation: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Database {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DatabaseProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatabaseListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Database>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigurationProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing)]
    pub description: Option<String>,
    #[serde(rename = "defaultValue", skip_serializing)]
    pub default_value: Option<String>,
    #[serde(rename = "dataType", skip_serializing)]
    pub data_type: Option<String>,
    #[serde(rename = "allowedValues", skip_serializing)]
    pub allowed_values: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Configuration {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ConfigurationProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigurationListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Configuration>,
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
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogFileProperties {
    #[serde(rename = "sizeInKB", default, skip_serializing_if = "Option::is_none")]
    pub size_in_kb: Option<i64>,
    #[serde(rename = "createdTime", skip_serializing)]
    pub created_time: Option<String>,
    #[serde(rename = "lastModifiedTime", skip_serializing)]
    pub last_modified_time: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogFile {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<LogFileProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogFileListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<LogFile>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PerformanceTierServiceLevelObjectives {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edition: Option<String>,
    #[serde(rename = "vCore", default, skip_serializing_if = "Option::is_none")]
    pub v_core: Option<i64>,
    #[serde(rename = "hardwareGeneration", default, skip_serializing_if = "Option::is_none")]
    pub hardware_generation: Option<String>,
    #[serde(rename = "maxBackupRetentionDays", default, skip_serializing_if = "Option::is_none")]
    pub max_backup_retention_days: Option<i64>,
    #[serde(rename = "minBackupRetentionDays", default, skip_serializing_if = "Option::is_none")]
    pub min_backup_retention_days: Option<i64>,
    #[serde(rename = "maxStorageMB", default, skip_serializing_if = "Option::is_none")]
    pub max_storage_mb: Option<i32>,
    #[serde(rename = "minStorageMB", default, skip_serializing_if = "Option::is_none")]
    pub min_storage_mb: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PerformanceTierProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "serviceLevelObjectives", default, skip_serializing_if = "Vec::is_empty")]
    pub service_level_objectives: Vec<PerformanceTierServiceLevelObjectives>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PerformanceTierListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PerformanceTierProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NameAvailabilityRequest {
    pub name: String,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NameAvailability {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityAlertPolicyProperties {
    pub state: security_alert_policy_properties::State,
    #[serde(rename = "disabledAlerts", default, skip_serializing_if = "Vec::is_empty")]
    pub disabled_alerts: Vec<String>,
    #[serde(rename = "emailAddresses", default, skip_serializing_if = "Vec::is_empty")]
    pub email_addresses: Vec<String>,
    #[serde(rename = "emailAccountAdmins", default, skip_serializing_if = "Option::is_none")]
    pub email_account_admins: Option<bool>,
    #[serde(rename = "storageEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub storage_endpoint: Option<String>,
    #[serde(rename = "storageAccountAccessKey", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_access_key: Option<String>,
    #[serde(rename = "retentionDays", default, skip_serializing_if = "Option::is_none")]
    pub retention_days: Option<i32>,
}
pub mod security_alert_policy_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Enabled,
        Disabled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerSecurityAlertPolicy {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SecurityAlertPolicyProperties>,
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
pub struct ServerAdministratorProperties {
    #[serde(rename = "administratorType")]
    pub administrator_type: server_administrator_properties::AdministratorType,
    pub login: String,
    pub sid: String,
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
}
pub mod server_administrator_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AdministratorType {
        ActiveDirectory,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerAdministratorResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServerAdministratorProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerAdministratorResourceListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ServerAdministratorResource>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryTextProperties {
    #[serde(rename = "queryId", default, skip_serializing_if = "Option::is_none")]
    pub query_id: Option<String>,
    #[serde(rename = "queryText", default, skip_serializing_if = "Option::is_none")]
    pub query_text: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryText {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<QueryTextProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryTextsResultList {
    #[serde(skip_serializing)]
    pub value: Vec<QueryText>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TopQueryStatisticsInputProperties {
    #[serde(rename = "numberOfTopQueries")]
    pub number_of_top_queries: i32,
    #[serde(rename = "aggregationFunction")]
    pub aggregation_function: String,
    #[serde(rename = "observedMetric")]
    pub observed_metric: String,
    #[serde(rename = "observationStartTime")]
    pub observation_start_time: String,
    #[serde(rename = "observationEndTime")]
    pub observation_end_time: String,
    #[serde(rename = "aggregationWindow")]
    pub aggregation_window: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TopQueryStatisticsInput {
    pub properties: TopQueryStatisticsInputProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryStatisticProperties {
    #[serde(rename = "queryId", default, skip_serializing_if = "Option::is_none")]
    pub query_id: Option<String>,
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(rename = "aggregationFunction", default, skip_serializing_if = "Option::is_none")]
    pub aggregation_function: Option<String>,
    #[serde(rename = "databaseNames", default, skip_serializing_if = "Vec::is_empty")]
    pub database_names: Vec<String>,
    #[serde(rename = "queryExecutionCount", default, skip_serializing_if = "Option::is_none")]
    pub query_execution_count: Option<i64>,
    #[serde(rename = "metricName", default, skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    #[serde(rename = "metricDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub metric_display_name: Option<String>,
    #[serde(rename = "metricValue", default, skip_serializing_if = "Option::is_none")]
    pub metric_value: Option<f64>,
    #[serde(rename = "metricValueUnit", default, skip_serializing_if = "Option::is_none")]
    pub metric_value_unit: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryStatistic {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<QueryStatisticProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TopQueryStatisticsResultList {
    #[serde(skip_serializing)]
    pub value: Vec<QueryStatistic>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WaitStatisticsInputProperties {
    #[serde(rename = "observationStartTime")]
    pub observation_start_time: String,
    #[serde(rename = "observationEndTime")]
    pub observation_end_time: String,
    #[serde(rename = "aggregationWindow")]
    pub aggregation_window: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WaitStatisticsInput {
    pub properties: WaitStatisticsInputProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WaitStatisticProperties {
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(rename = "eventName", default, skip_serializing_if = "Option::is_none")]
    pub event_name: Option<String>,
    #[serde(rename = "eventTypeName", default, skip_serializing_if = "Option::is_none")]
    pub event_type_name: Option<String>,
    #[serde(rename = "queryId", default, skip_serializing_if = "Option::is_none")]
    pub query_id: Option<i64>,
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(rename = "userId", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(rename = "totalTimeInMs", default, skip_serializing_if = "Option::is_none")]
    pub total_time_in_ms: Option<f64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WaitStatistic {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WaitStatisticProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WaitStatisticsResultList {
    #[serde(skip_serializing)]
    pub value: Vec<WaitStatistic>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdvisorProperties {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Advisor {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AdvisorProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdvisorsResultList {
    #[serde(skip_serializing)]
    pub value: Vec<Advisor>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecommendationActionProperties {
    #[serde(rename = "advisorName", default, skip_serializing_if = "Option::is_none")]
    pub advisor_name: Option<String>,
    #[serde(rename = "sessionId", default, skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(rename = "actionId", default, skip_serializing_if = "Option::is_none")]
    pub action_id: Option<i32>,
    #[serde(rename = "createdTime", default, skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(rename = "expirationTime", default, skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "recommendationType", default, skip_serializing_if = "Option::is_none")]
    pub recommendation_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecommendationAction {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RecommendationActionProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecommendationActionsResultList {
    #[serde(skip_serializing)]
    pub value: Vec<RecommendationAction>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecommendedActionSessionsOperationStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointConnectionProperties {
    #[serde(rename = "privateEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<PrivateEndpointProperty>,
    #[serde(rename = "privateLinkServiceConnectionState", default, skip_serializing_if = "Option::is_none")]
    pub private_link_service_connection_state: Option<PrivateLinkServiceConnectionStateProperty>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkServiceConnectionStateProperty {
    pub status: String,
    pub description: String,
    #[serde(rename = "actionsRequired", skip_serializing)]
    pub actions_required: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointConnection {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateEndpointConnectionProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointConnectionListResult {
    #[serde(skip_serializing)]
    pub value: Vec<PrivateEndpointConnection>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TagsObject {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkResourceListResult {
    #[serde(skip_serializing)]
    pub value: Vec<PrivateLinkResource>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkResourceProperties {
    #[serde(rename = "groupId", skip_serializing)]
    pub group_id: Option<String>,
    #[serde(rename = "requiredMembers", skip_serializing)]
    pub required_members: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateLinkResourceProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    pub location: String,
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
pub struct ProxyResource {
    #[serde(flatten)]
    pub resource: Resource,
}
