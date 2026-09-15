//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::OTPVerificationVerdict;
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OTPVerificationAttemptResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    pub verdict: OTPVerificationVerdict,
}
