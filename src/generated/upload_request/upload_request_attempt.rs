//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{UploadRequestLatestError, UploadRequestReview};
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UploadRequestAttempt {
    pub attempted_at: crate::Timestamp,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub declared_size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<UploadRequestLatestError>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failed_at: Option<crate::Timestamp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    pub id: String,
    pub ordinal: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review: Option<UploadRequestReview>,
    pub status: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub succeeded_at: Option<crate::Timestamp>,
    pub upload_request_id: String,
}
