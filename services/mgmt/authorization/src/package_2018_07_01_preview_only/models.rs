#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DenyAssignmentFilter {
    #[serde(rename = "denyAssignmentName", default, skip_serializing_if = "Option::is_none")]
    pub deny_assignment_name: Option<String>,
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(rename = "gdprExportPrincipalId", default, skip_serializing_if = "Option::is_none")]
    pub gdpr_export_principal_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DenyAssignmentProperties {
    #[serde(rename = "denyAssignmentName", default, skip_serializing_if = "Option::is_none")]
    pub deny_assignment_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub permissions: Vec<DenyAssignmentPermission>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(rename = "doNotApplyToChildScopes", default, skip_serializing_if = "Option::is_none")]
    pub do_not_apply_to_child_scopes: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub principals: Vec<Principal>,
    #[serde(rename = "excludePrincipals", default, skip_serializing_if = "Vec::is_empty")]
    pub exclude_principals: Vec<Principal>,
    #[serde(rename = "isSystemProtected", default, skip_serializing_if = "Option::is_none")]
    pub is_system_protected: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DenyAssignment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DenyAssignmentProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DenyAssignmentListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DenyAssignment>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DenyAssignmentPermission {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<String>,
    #[serde(rename = "notActions", default, skip_serializing_if = "Vec::is_empty")]
    pub not_actions: Vec<String>,
    #[serde(rename = "dataActions", default, skip_serializing_if = "Vec::is_empty")]
    pub data_actions: Vec<String>,
    #[serde(rename = "notDataActions", default, skip_serializing_if = "Vec::is_empty")]
    pub not_data_actions: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorDetail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDetail>,
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorAdditionalInfo {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Principal {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}
