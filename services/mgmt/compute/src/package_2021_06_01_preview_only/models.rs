#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[doc = "Contains metadata of a diagnostic type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComputeDiagnosticBase {
    #[doc = "Identifier for the type of diagnostic"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Friendly name for the type of diagnostic"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Type of the diagnostic"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Contains additional properties of a diagnostic"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DiagnosticProperties>,
}
impl ComputeDiagnosticBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Lists all available Compute diagnostics for a subscription in a location."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComputeDiagnosticsList {
    #[doc = "The collection of available Compute diagnostics returned by the listing operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ComputeDiagnosticBase>,
    #[doc = "The continuation token."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl ComputeDiagnosticsList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains additional properties of a diagnostic"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiagnosticProperties {
    #[doc = "Describes what are the supported resource types for a diagnostic."]
    #[serde(rename = "supportedResourceTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_resource_types: Vec<String>,
}
impl DiagnosticProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error Detail message."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDetail {
    #[doc = "The error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The target of the particular error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "User friendly error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The Api error details"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDetail>,
    #[doc = "Inner error details."]
    #[serde(rename = "innerError", default, skip_serializing_if = "Option::is_none")]
    pub inner_error: Option<InnerError>,
}
impl ErrorDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An error response from the Compute Diagnostic Resource Provider service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "Error Detail message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
}
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Inner error details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InnerError {
    #[doc = "The exception type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exceptiontype: Option<String>,
    #[doc = "The internal error message or exception dump."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errordetail: Option<String>,
}
impl InnerError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Api output result when disk inspection result is completed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RunDiskInspectionAsyncOperationResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    #[serde(rename = "resultStatus", default, skip_serializing_if = "Option::is_none")]
    pub result_status: Option<run_disk_inspection_async_operation_result::ResultStatus>,
    #[doc = "Error Detail message."]
    #[serde(rename = "errorDetail", default, skip_serializing_if = "Option::is_none")]
    pub error_detail: Option<ErrorDetail>,
    #[doc = "The time when the disk inspection was completed."]
    #[serde(rename = "createdUTC", default, skip_serializing_if = "Option::is_none")]
    pub created_utc: Option<String>,
}
impl RunDiskInspectionAsyncOperationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod run_disk_inspection_async_operation_result {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ResultStatus {
        Success,
        Failed,
    }
}
#[doc = "Data used for requesting a Disk Inspection execution."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RunDiskInspectionInput {
    #[doc = "Qualified name of the resource."]
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    #[doc = "Name of manifest in order to trigger Disk Inspection."]
    pub manifest: String,
    #[doc = "SAS uri to the blob where results will be uploaded."]
    #[serde(rename = "uploadSasUri")]
    pub upload_sas_uri: String,
}
impl RunDiskInspectionInput {
    pub fn new(resource_id: String, manifest: String, upload_sas_uri: String) -> Self {
        Self {
            resource_id,
            manifest,
            upload_sas_uri,
        }
    }
}
