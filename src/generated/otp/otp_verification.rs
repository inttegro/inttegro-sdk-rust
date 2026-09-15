//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{OTPTransaction, OTPVerificationAttempt};
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OTPVerification {
    pub transaction: OTPTransaction,
    pub verification_attempt: OTPVerificationAttempt,
}
