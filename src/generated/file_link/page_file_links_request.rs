//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::FileLinkStatus;
use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PageFileLinksRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<FileLinkStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}
