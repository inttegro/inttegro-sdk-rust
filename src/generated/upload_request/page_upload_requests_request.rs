//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{FileResourceInput, UploadRequestStatus};
use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PageUploadRequestsRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<UploadRequestStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<FileResourceInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}
