//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::PayoutFailureReason;
use serde::{Deserialize, Serialize};

/// Caller-safe information about a terminal payout failure.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PayoutFailure {
    pub detail: String,
    pub reason: PayoutFailureReason,
    pub retryable: bool,
}
