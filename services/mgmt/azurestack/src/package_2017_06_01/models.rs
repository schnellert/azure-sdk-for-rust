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
    pub etag: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    pub location: tracked_resource::Location,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
pub mod tracked_resource {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Location {
        #[serde(rename = "global")]
        Global,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ProvisioningState {
    Creating,
    Failed,
    Succeeded,
    Canceled,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<Display>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}
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
pub struct OperationList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudManifestFileResponse {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CloudManifestFileProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudManifestFileProperties {
    #[serde(rename = "deploymentData", default, skip_serializing_if = "Option::is_none")]
    pub deployment_data: Option<CloudManifestFileDeploymentData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudManifestFileDeploymentData {
    #[serde(rename = "externalDsmsCertificates", default, skip_serializing_if = "Option::is_none")]
    pub external_dsms_certificates: Option<String>,
    #[serde(rename = "customCloudVerificationKey", default, skip_serializing_if = "Option::is_none")]
    pub custom_cloud_verification_key: Option<String>,
    #[serde(rename = "customEnvironmentEndpoints", default, skip_serializing_if = "Option::is_none")]
    pub custom_environment_endpoints: Option<CloudManifestFileEnvironmentEndpoints>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudManifestFileEnvironmentEndpoints {
    #[serde(rename = "customCloudArmEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub custom_cloud_arm_endpoint: Option<String>,
    #[serde(rename = "externalDsmsEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub external_dsms_endpoint: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorDetails {
    #[serde(skip_serializing)]
    pub code: Option<String>,
    #[serde(skip_serializing)]
    pub message: Option<String>,
    #[serde(skip_serializing)]
    pub target: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetails>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ComputeRole {
    None,
    IaaS,
    PaaS,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum OperatingSystem {
    None,
    Windows,
    Linux,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Uri {
    #[serde(skip_serializing)]
    pub uri: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExtendedProduct {
    #[serde(rename = "galleryPackageBlobSasUri", skip_serializing)]
    pub gallery_package_blob_sas_uri: Option<String>,
    #[serde(rename = "productKind", skip_serializing)]
    pub product_kind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ExtendedProductProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExtendedProductProperties {
    #[serde(flatten)]
    pub virtual_machine_extension_product_properties: VirtualMachineExtensionProductProperties,
    #[serde(flatten)]
    pub virtual_machine_product_properties: VirtualMachineProductProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineExtensionProductProperties {
    #[serde(rename = "computeRole", default, skip_serializing_if = "Option::is_none")]
    pub compute_role: Option<ComputeRole>,
    #[serde(rename = "isSystemExtension", skip_serializing)]
    pub is_system_extension: Option<bool>,
    #[serde(rename = "sourceBlob", default, skip_serializing_if = "Option::is_none")]
    pub source_blob: Option<Uri>,
    #[serde(rename = "supportMultipleExtensions", skip_serializing)]
    pub support_multiple_extensions: Option<bool>,
    #[serde(skip_serializing)]
    pub version: Option<String>,
    #[serde(rename = "vmOsType", default, skip_serializing_if = "Option::is_none")]
    pub vm_os_type: Option<OperatingSystem>,
    #[serde(rename = "vmScaleSetEnabled", skip_serializing)]
    pub vm_scale_set_enabled: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineProductProperties {
    #[serde(skip_serializing)]
    pub version: Option<String>,
    #[serde(rename = "osDiskImage", default, skip_serializing_if = "Option::is_none")]
    pub os_disk_image: Option<OsDiskImage>,
    #[serde(rename = "dataDiskImages", skip_serializing)]
    pub data_disk_images: Vec<DataDiskImage>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OsDiskImage {
    #[serde(rename = "operatingSystem", default, skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<OperatingSystem>,
    #[serde(rename = "sourceBlobSasUri", skip_serializing)]
    pub source_blob_sas_uri: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataDiskImage {
    #[serde(skip_serializing)]
    pub lun: Option<i32>,
    #[serde(rename = "sourceBlobSasUri", skip_serializing)]
    pub source_blob_sas_uri: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Product {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProductNestedProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProductNestedProperties {
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "publisherDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub publisher_display_name: Option<String>,
    #[serde(rename = "publisherIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub publisher_identifier: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    #[serde(rename = "offerVersion", default, skip_serializing_if = "Option::is_none")]
    pub offer_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[serde(rename = "billingPartNumber", default, skip_serializing_if = "Option::is_none")]
    pub billing_part_number: Option<String>,
    #[serde(rename = "vmExtensionType", default, skip_serializing_if = "Option::is_none")]
    pub vm_extension_type: Option<String>,
    #[serde(rename = "galleryItemIdentity", default, skip_serializing_if = "Option::is_none")]
    pub gallery_item_identity: Option<String>,
    #[serde(rename = "iconUris", default, skip_serializing_if = "Option::is_none")]
    pub icon_uris: Option<IconUris>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<ProductLink>,
    #[serde(rename = "legalTerms", default, skip_serializing_if = "Option::is_none")]
    pub legal_terms: Option<String>,
    #[serde(rename = "privacyPolicy", default, skip_serializing_if = "Option::is_none")]
    pub privacy_policy: Option<String>,
    #[serde(rename = "payloadLength", default, skip_serializing_if = "Option::is_none")]
    pub payload_length: Option<i64>,
    #[serde(rename = "productKind", default, skip_serializing_if = "Option::is_none")]
    pub product_kind: Option<String>,
    #[serde(rename = "productProperties", default, skip_serializing_if = "Option::is_none")]
    pub product_properties: Option<ProductProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compatibility: Option<Compatibility>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Compatibility {
    #[serde(rename = "isCompatible", default, skip_serializing_if = "Option::is_none")]
    pub is_compatible: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub issues: Vec<CompatibilityIssue>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum CompatibilityIssue {
    HigherDeviceVersionRequired,
    LowerDeviceVersionRequired,
    CapacityBillingModelRequired,
    PayAsYouGoBillingModelRequired,
    DevelopmentBillingModelRequired,
    #[serde(rename = "AzureADIdentitySystemRequired")]
    AzureAdIdentitySystemRequired,
    #[serde(rename = "ADFSIdentitySystemRequired")]
    AdfsIdentitySystemRequired,
    ConnectionToInternetRequired,
    ConnectionToAzureRequired,
    DisconnectedEnvironmentRequired,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IconUris {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub large: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wide: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub medium: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub small: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hero: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProductProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProductLink {
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProductList {
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Product>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceConfiguration {
    #[serde(rename = "deviceVersion", skip_serializing)]
    pub device_version: Option<String>,
    #[serde(rename = "identitySystem", skip_serializing)]
    pub identity_system: Option<device_configuration::IdentitySystem>,
}
pub mod device_configuration {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum IdentitySystem {
        #[serde(rename = "AzureAD")]
        AzureAd,
        #[serde(rename = "ADFS")]
        Adfs,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MarketplaceProductLogUpdate {
    #[serde(skip_serializing)]
    pub operation: Option<String>,
    #[serde(skip_serializing)]
    pub status: Option<String>,
    #[serde(skip_serializing)]
    pub error: Option<String>,
    #[serde(skip_serializing)]
    pub details: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProductLog {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(rename = "productId", skip_serializing)]
    pub product_id: Option<String>,
    #[serde(rename = "subscriptionId", skip_serializing)]
    pub subscription_id: Option<String>,
    #[serde(rename = "registrationName", skip_serializing)]
    pub registration_name: Option<String>,
    #[serde(rename = "resourceGroupName", skip_serializing)]
    pub resource_group_name: Option<String>,
    #[serde(skip_serializing)]
    pub operation: Option<String>,
    #[serde(rename = "startDate", skip_serializing)]
    pub start_date: Option<String>,
    #[serde(rename = "endDate", skip_serializing)]
    pub end_date: Option<String>,
    #[serde(skip_serializing)]
    pub status: Option<String>,
    #[serde(skip_serializing)]
    pub error: Option<String>,
    #[serde(skip_serializing)]
    pub details: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Registration {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RegistrationProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegistrationProperties {
    #[serde(rename = "objectId", default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[serde(rename = "cloudId", default, skip_serializing_if = "Option::is_none")]
    pub cloud_id: Option<String>,
    #[serde(rename = "billingModel", default, skip_serializing_if = "Option::is_none")]
    pub billing_model: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegistrationList {
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Registration>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActivationKeyResult {
    #[serde(rename = "activationKey", default, skip_serializing_if = "Option::is_none")]
    pub activation_key: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegistrationParameter {
    pub properties: RegistrationParameterProperties,
    pub location: registration_parameter::Location,
}
pub mod registration_parameter {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Location {
        #[serde(rename = "global")]
        Global,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegistrationParameterProperties {
    #[serde(rename = "registrationToken")]
    pub registration_token: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomerSubscription {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CustomerSubscriptionProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomerSubscriptionProperties {
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomerSubscriptionList {
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CustomerSubscription>,
}
