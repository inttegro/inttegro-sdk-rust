//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::OTPTransmissionStatus;
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OTPTransmission {
    pub recipient: String,
    pub sender_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sent_at: Option<crate::Timestamp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sent_via: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<OTPTransmissionStatus>,
}
