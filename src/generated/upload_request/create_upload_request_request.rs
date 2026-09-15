//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{
    CustomData, FileActorInput, FilePartyInput, FileResourceInput, UploadRequestAttemptsRequest,
    UploadRequestConstraintsInput, UploadRequestDisplayInput,
};
use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateUploadRequestRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub constraints: Option<UploadRequestConstraintsInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<UploadRequestDisplayInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<FilePartyInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipient: Option<FilePartyInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<FileResourceInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requester: Option<FileActorInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attempts: Option<UploadRequestAttemptsRequest>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<crate::Timestamp>,
    pub purpose: String,
}
