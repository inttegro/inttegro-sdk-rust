//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{
    CustomData, FileMetadata, FileParty, FileResource, UploadRequestActor, UploadRequestAttempt,
    UploadRequestAttempts, UploadRequestConstraints, UploadRequestDisplay,
    UploadRequestLatestError, UploadRequestStatus,
};
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UploadRequest {
    pub id: String,
    pub purpose: String,
    pub status: UploadRequestStatus,
    pub active: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upload_url: Option<String>,
    pub constraints: UploadRequestConstraints,
    pub display: UploadRequestDisplay,
    pub subject: FileParty,
    pub recipient: FileParty,
    pub resource: FileResource,
    pub requester: UploadRequestActor,
    pub attempts: UploadRequestAttempts,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latest_error: Option<UploadRequestLatestError>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub canceled_by: Option<UploadRequestActor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<FileMetadata>,
    pub created_at: crate::Timestamp,
    pub updated_at: crate::Timestamp,
    pub expires_at: crate::Timestamp,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uploading_at: Option<crate::Timestamp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fulfilled_at: Option<crate::Timestamp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expired_at: Option<crate::Timestamp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<crate::Timestamp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attempt: Option<UploadRequestAttempt>,
}
