//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::SecretKeyTokenType;
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeneratedSecretKey {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub token_type: SecretKeyTokenType,
    pub issued_at: crate::Timestamp,
    pub token: String,
}
