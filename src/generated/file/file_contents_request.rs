//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{FileDelivery, FileDisposition};
use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileContentsRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disposition: Option<FileDisposition>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delivery: Option<FileDelivery>,
    pub file_id: String,
}
