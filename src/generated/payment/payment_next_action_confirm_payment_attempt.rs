//! Generated, typed Inttegro domain or request value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentNextActionConfirmPaymentAttempt {
    pub status: String,
    pub confirmed: bool,
    pub reason: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub executed_at: Option<crate::Timestamp>,
    pub created_at: crate::Timestamp,
}
