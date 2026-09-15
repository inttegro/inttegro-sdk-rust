//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::OTPAlphabetType;
use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InitiateOTPRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub async_delivery: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_template: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sender: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token_alphabet: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token_alphabet_type: Option<OTPAlphabetType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validity_duration_in_minutes: Option<i64>,
    pub recipient: String,
    pub service_name: String,
    pub token_size: i64,
}
