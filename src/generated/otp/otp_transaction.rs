//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{OTPStatus, OTPTransmission};
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OTPTransaction {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancel_reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<crate::Timestamp>,
    pub expires_at: crate::Timestamp,
    pub full_message: String,
    pub id: String,
    pub initiated_at: crate::Timestamp,
    pub status: OTPStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transmission: Option<OTPTransmission>,
}
