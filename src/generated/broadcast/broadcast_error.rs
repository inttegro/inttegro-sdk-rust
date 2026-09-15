//! Generated, typed Inttegro domain or request value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BroadcastError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipient: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fix_code: Option<String>,
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}
