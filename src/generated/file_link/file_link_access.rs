//! Generated, typed Inttegro domain or request value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileLinkAccess {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_accesses: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_accessed_at: Option<crate::Timestamp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_download: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_origins: Option<Vec<String>>,
}
