//! Generated, typed Inttegro domain or request value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// Public error information for a payment attempt.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentAttemptError {
    pub message: String,
}
