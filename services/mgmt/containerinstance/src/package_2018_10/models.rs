#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Container {
    pub name: String,
    pub properties: ContainerProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerProperties {
    pub image: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub command: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ports: Vec<ContainerPort>,
    #[serde(rename = "environmentVariables", default, skip_serializing_if = "Vec::is_empty")]
    pub environment_variables: Vec<EnvironmentVariable>,
    #[serde(rename = "instanceView", skip_serializing)]
    pub instance_view: Option<container_properties::InstanceView>,
    pub resources: ResourceRequirements,
    #[serde(rename = "volumeMounts", default, skip_serializing_if = "Vec::is_empty")]
    pub volume_mounts: Vec<VolumeMount>,
    #[serde(rename = "livenessProbe", default, skip_serializing_if = "Option::is_none")]
    pub liveness_probe: Option<ContainerProbe>,
    #[serde(rename = "readinessProbe", default, skip_serializing_if = "Option::is_none")]
    pub readiness_probe: Option<ContainerProbe>,
}
pub mod container_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct InstanceView {
        #[serde(rename = "restartCount", skip_serializing)]
        pub restart_count: Option<i64>,
        #[serde(rename = "currentState", default, skip_serializing_if = "Option::is_none")]
        pub current_state: Option<ContainerState>,
        #[serde(rename = "previousState", default, skip_serializing_if = "Option::is_none")]
        pub previous_state: Option<ContainerState>,
        #[serde(skip_serializing)]
        pub events: Vec<Event>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerState {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "exitCode", default, skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i64>,
    #[serde(rename = "finishTime", default, skip_serializing_if = "Option::is_none")]
    pub finish_time: Option<String>,
    #[serde(rename = "detailStatus", default, skip_serializing_if = "Option::is_none")]
    pub detail_status: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Event {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(rename = "firstTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub first_timestamp: Option<String>,
    #[serde(rename = "lastTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub last_timestamp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceRequirements {
    pub requests: ResourceRequests,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<ResourceLimits>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceRequests {
    #[serde(rename = "memoryInGB")]
    pub memory_in_gb: f64,
    pub cpu: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gpu: Option<GpuResource>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceLimits {
    #[serde(rename = "memoryInGB", default, skip_serializing_if = "Option::is_none")]
    pub memory_in_gb: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpu: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gpu: Option<GpuResource>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GpuResource {
    pub count: i32,
    pub sku: gpu_resource::Sku,
}
pub mod gpu_resource {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Sku {
        K80,
        P100,
        V100,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureFileVolume {
    #[serde(rename = "shareName")]
    pub share_name: String,
    #[serde(rename = "readOnly", default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(rename = "storageAccountName")]
    pub storage_account_name: String,
    #[serde(rename = "storageAccountKey", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_key: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmptyDirVolume {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecretVolume {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GitRepoVolume {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub directory: Option<String>,
    pub repository: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Volume {
    pub name: String,
    #[serde(rename = "azureFile", default, skip_serializing_if = "Option::is_none")]
    pub azure_file: Option<AzureFileVolume>,
    #[serde(rename = "emptyDir", default, skip_serializing_if = "Option::is_none")]
    pub empty_dir: Option<EmptyDirVolume>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<SecretVolume>,
    #[serde(rename = "gitRepo", default, skip_serializing_if = "Option::is_none")]
    pub git_repo: Option<GitRepoVolume>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VolumeMount {
    pub name: String,
    #[serde(rename = "mountPath")]
    pub mount_path: String,
    #[serde(rename = "readOnly", default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerExec {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub command: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerHttpGet {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    pub port: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme: Option<container_http_get::Scheme>,
}
pub mod container_http_get {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Scheme {
        #[serde(rename = "http")]
        Http,
        #[serde(rename = "https")]
        Https,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerProbe {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exec: Option<ContainerExec>,
    #[serde(rename = "httpGet", default, skip_serializing_if = "Option::is_none")]
    pub http_get: Option<ContainerHttpGet>,
    #[serde(rename = "initialDelaySeconds", default, skip_serializing_if = "Option::is_none")]
    pub initial_delay_seconds: Option<i32>,
    #[serde(rename = "periodSeconds", default, skip_serializing_if = "Option::is_none")]
    pub period_seconds: Option<i32>,
    #[serde(rename = "failureThreshold", default, skip_serializing_if = "Option::is_none")]
    pub failure_threshold: Option<i32>,
    #[serde(rename = "successThreshold", default, skip_serializing_if = "Option::is_none")]
    pub success_threshold: Option<i32>,
    #[serde(rename = "timeoutSeconds", default, skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerGroup {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(flatten)]
    pub serde_json_value: serde_json::Value,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerGroupIdentity {
    #[serde(rename = "principalId", skip_serializing)]
    pub principal_id: Option<String>,
    #[serde(rename = "tenantId", skip_serializing)]
    pub tenant_id: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<container_group_identity::Type>,
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<serde_json::Value>,
}
pub mod container_group_identity {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        SystemAssigned,
        UserAssigned,
        #[serde(rename = "SystemAssigned, UserAssigned")]
        SystemAssignedUserAssigned,
        None,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageRegistryCredential {
    pub server: String,
    pub username: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerGroupDiagnostics {
    #[serde(rename = "logAnalytics", default, skip_serializing_if = "Option::is_none")]
    pub log_analytics: Option<LogAnalytics>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogAnalytics {
    #[serde(rename = "workspaceId")]
    pub workspace_id: String,
    #[serde(rename = "workspaceKey")]
    pub workspace_key: String,
    #[serde(rename = "logType", default, skip_serializing_if = "Option::is_none")]
    pub log_type: Option<log_analytics::LogType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}
pub mod log_analytics {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum LogType {
        ContainerInsights,
        ContainerInstanceLogs,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerGroupNetworkProfile {
    pub id: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IpAddress {
    pub ports: Vec<Port>,
    #[serde(rename = "type")]
    pub type_: ip_address::Type,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(rename = "dnsNameLabel", default, skip_serializing_if = "Option::is_none")]
    pub dns_name_label: Option<String>,
    #[serde(skip_serializing)]
    pub fqdn: Option<String>,
}
pub mod ip_address {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        Public,
        Private,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Port {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<port::Protocol>,
    pub port: i32,
}
pub mod port {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Protocol {
        #[serde(rename = "TCP")]
        Tcp,
        #[serde(rename = "UDP")]
        Udp,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerPort {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<container_port::Protocol>,
    pub port: i32,
}
pub mod container_port {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Protocol {
        #[serde(rename = "TCP")]
        Tcp,
        #[serde(rename = "UDP")]
        Udp,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentVariable {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "secureValue", default, skip_serializing_if = "Option::is_none")]
    pub secure_value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    pub name: String,
    pub display: operation::Display,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<operation::Origin>,
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
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Origin {
        User,
        System,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageListResult {
    #[serde(skip_serializing)]
    pub value: Vec<Usage>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Usage {
    #[serde(skip_serializing)]
    pub unit: Option<String>,
    #[serde(rename = "currentValue", skip_serializing)]
    pub current_value: Option<i64>,
    #[serde(skip_serializing)]
    pub limit: Option<i64>,
    #[serde(skip_serializing)]
    pub name: Option<usage::Name>,
}
pub mod usage {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Name {
        #[serde(skip_serializing)]
        pub value: Option<String>,
        #[serde(rename = "localizedValue", skip_serializing)]
        pub localized_value: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerGroupListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ContainerGroup>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Logs {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerExecRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    #[serde(rename = "terminalSize", default, skip_serializing_if = "Option::is_none")]
    pub terminal_size: Option<container_exec_request::TerminalSize>,
}
pub mod container_exec_request {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct TerminalSize {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub rows: Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub cols: Option<i64>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerExecResponse {
    #[serde(rename = "webSocketUri", default, skip_serializing_if = "Option::is_none")]
    pub web_socket_uri: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DnsConfiguration {
    #[serde(rename = "nameServers")]
    pub name_servers: Vec<String>,
    #[serde(rename = "searchDomains", default, skip_serializing_if = "Option::is_none")]
    pub search_domains: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<String>,
}
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
pub struct CachedImagesListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CachedImages>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CachedImages {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "osType")]
    pub os_type: String,
    pub image: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CapabilitiesListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Capabilities>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Capabilities {
    #[serde(rename = "resourceType", skip_serializing)]
    pub resource_type: Option<String>,
    #[serde(rename = "osType", skip_serializing)]
    pub os_type: Option<String>,
    #[serde(skip_serializing)]
    pub location: Option<String>,
    #[serde(rename = "ipAddressType", skip_serializing)]
    pub ip_address_type: Option<String>,
    #[serde(skip_serializing)]
    pub gpu: Option<String>,
    #[serde(skip_serializing)]
    pub capabilities: Option<capabilities::Capabilities>,
}
pub mod capabilities {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Capabilities {
        #[serde(rename = "maxMemoryInGB", skip_serializing)]
        pub max_memory_in_gb: Option<f64>,
        #[serde(rename = "maxCpu", skip_serializing)]
        pub max_cpu: Option<f64>,
        #[serde(rename = "maxGpuCount", skip_serializing)]
        pub max_gpu_count: Option<f64>,
    }
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
