//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{SecretKeyStatus, SecretKeyTokenType};
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecretKey {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub token_type: SecretKeyTokenType,
    pub issued_at: crate::Timestamp,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<crate::Timestamp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<crate::Timestamp>,
    pub status: SecretKeyStatus,
    pub active: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revoked_at: Option<crate::Timestamp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_used_at: Option<crate::Timestamp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage_count: Option<i64>,
}
