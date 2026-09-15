//! Generated, typed Inttegro domain or request value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UploadRequestAttempts {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_attempts: Option<i64>,
    pub attempt_count: i64,
    pub failed_attempt_count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_attempted_at: Option<crate::Timestamp>,
}
