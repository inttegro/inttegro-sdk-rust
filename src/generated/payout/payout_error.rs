//! Generated, typed Inttegro domain or request value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PayoutError {
    pub cause: String,
    pub message: String,
    pub occurred_at: crate::Timestamp,
    #[serde(rename = "type")]
    pub r#type: String,
}
