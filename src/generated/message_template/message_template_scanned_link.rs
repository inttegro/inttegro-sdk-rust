//! Generated, typed Inttegro domain or request value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageTemplateScannedLink {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    pub raw: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    pub scheme: String,
    pub status: String,
}
