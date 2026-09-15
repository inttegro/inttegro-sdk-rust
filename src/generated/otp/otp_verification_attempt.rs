//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::OTPVerificationAttemptResult;
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OTPVerificationAttempt {
    pub attempted_at: crate::Timestamp,
    pub id: String,
    pub presented_token: String,
    pub recipient: String,
    pub result: OTPVerificationAttemptResult,
}
