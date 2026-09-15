//! Generated, typed Inttegro domain or request value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileLinkAccessRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_accesses: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_download: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_origins: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_ip_ranges: Option<Vec<String>>,
}
