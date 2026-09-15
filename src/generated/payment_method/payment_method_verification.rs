//! Generated, typed Inttegro domain or request value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentMethodVerification {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<crate::Timestamp>,
    pub initiated_at: crate::Timestamp,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mechanism: Option<String>,
    pub request_id: String,
    #[serde(rename = "type")]
    pub r#type: String,
}
