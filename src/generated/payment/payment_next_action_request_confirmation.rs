//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::PaymentNextActionConfirmPaymentRequest;
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentNextActionRequestConfirmation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_request: Option<PaymentNextActionConfirmPaymentRequest>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub after: Option<crate::Timestamp>,
}
