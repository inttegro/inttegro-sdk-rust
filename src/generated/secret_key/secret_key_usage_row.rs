//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::SecretKeyAuthResult;
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecretKeyUsageRow {
    pub secret_key_id: String,
    pub occurred_at: crate::Timestamp,
    pub auth_result: SecretKeyAuthResult,
}
