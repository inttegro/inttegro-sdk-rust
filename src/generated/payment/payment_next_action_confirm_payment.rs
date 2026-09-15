//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{PaymentNextActionConfirmPaymentAttempt, PaymentNextActionConfirmPaymentRequest};
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentNextActionConfirmPayment {
    pub expires_at: crate::Timestamp,
    pub scheme: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request: Option<PaymentNextActionConfirmPaymentRequest>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attempt: Option<PaymentNextActionConfirmPaymentAttempt>,
    pub confirmed: bool,
    pub status: String,
}
