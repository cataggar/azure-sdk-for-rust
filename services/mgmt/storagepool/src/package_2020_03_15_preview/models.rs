#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoragePoolOperationDisplay {
    pub provider: String,
    pub resource: String,
    pub operation: String,
    pub description: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoragePoolRpOperation {
    pub name: String,
    #[serde(rename = "isDataAction")]
    pub is_data_action: bool,
    #[serde(rename = "actionType", default, skip_serializing_if = "Option::is_none")]
    pub action_type: Option<String>,
    pub display: StoragePoolOperationDisplay,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoragePoolOperationListResult {
    pub value: Vec<StoragePoolRpOperation>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskPoolListResult {
    pub value: Vec<DiskPool>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskPool {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    pub properties: DiskPoolProperties,
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemMetadata>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskPoolCreate {
    pub properties: DiskPoolCreateProperties,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskPoolUpdate {
    pub properties: DiskPoolUpdateProperties,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
pub type AvailabilityZone = String;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskPoolProperties {
    #[serde(rename = "provisioningState")]
    pub provisioning_state: ProvisioningState,
    #[serde(rename = "availabilityZones")]
    pub availability_zones: Vec<AvailabilityZone>,
    pub status: OperationalStatus,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub disks: Vec<Disk>,
    #[serde(rename = "subnetId")]
    pub subnet_id: String,
    #[serde(rename = "additionalCapabilities", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_capabilities: Vec<AdditionalCapability>,
    pub tier: DiskPoolTier,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskPoolCreateProperties {
    #[serde(rename = "availabilityZones")]
    pub availability_zones: Vec<AvailabilityZone>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub disks: Vec<Disk>,
    #[serde(rename = "subnetId")]
    pub subnet_id: String,
    #[serde(rename = "additionalCapabilities", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_capabilities: Vec<AdditionalCapability>,
    pub tier: DiskPoolTier,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskPoolUpdateProperties {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub disks: Vec<Disk>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Disk {
    pub id: String,
}
pub type AdditionalCapability = String;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IscsiTargetList {
    pub value: Vec<IscsiTarget>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IscsiTarget {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    pub properties: IscsiTargetProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IscsiTargetCreate {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    pub properties: IscsiTargetCreateProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IscsiTargetUpdate {
    pub properties: IscsiTargetUpdateProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IscsiTargetProperties {
    #[serde(rename = "provisioningState")]
    pub provisioning_state: ProvisioningState,
    pub status: OperationalStatus,
    pub tpgs: Vec<TargetPortalGroup>,
    #[serde(rename = "targetIqn")]
    pub target_iqn: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IscsiTargetCreateProperties {
    pub tpgs: Vec<TargetPortalGroupCreate>,
    #[serde(rename = "targetIqn", default, skip_serializing_if = "Option::is_none")]
    pub target_iqn: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IscsiTargetUpdateProperties {
    pub tpgs: Vec<TargetPortalGroupUpdate>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TargetPortalGroup {
    pub luns: Vec<IscsiLun>,
    pub acls: Vec<Acl>,
    pub attributes: Attributes,
    pub endpoints: Vec<String>,
    pub tag: i32,
    pub port: i32,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TargetPortalGroupCreate {
    pub luns: Vec<IscsiLun>,
    pub acls: Vec<Acl>,
    pub attributes: Attributes,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TargetPortalGroupUpdate {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub luns: Vec<IscsiLun>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub acls: Vec<Acl>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Acl {
    #[serde(rename = "initiatorIqn")]
    pub initiator_iqn: String,
    #[serde(rename = "mappedLuns")]
    pub mapped_luns: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<IscsiTargetCredentials>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Attributes {
    pub authentication: bool,
    #[serde(rename = "prodModeWriteProtect")]
    pub prod_mode_write_protect: bool,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IscsiTargetCredentials {
    pub username: String,
    pub password: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IscsiLun {
    pub name: String,
    #[serde(rename = "managedDiskAzureResourceId")]
    pub managed_disk_azure_resource_id: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Error {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ProvisioningState {
    Invalid,
    Succeeded,
    Failed,
    Canceled,
    Pending,
    Creating,
    Updating,
    Deleting,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum OperationalStatus {
    Invalid,
    Unknown,
    Healthy,
    Unhealthy,
    Updating,
    Running,
    Stopped,
    #[serde(rename = "Stopped (deallocated)")]
    StoppedDeallocated,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DiskPoolTier {
    Basic,
    Standard,
    Premium,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SystemMetadata {
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "createdByType", default, skip_serializing_if = "Option::is_none")]
    pub created_by_type: Option<String>,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<String>,
    #[serde(rename = "lastModifiedAt", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_at: Option<String>,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProxyResource {
    #[serde(flatten)]
    pub resource: Resource,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorAdditionalInfo {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorResponse>,
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}
