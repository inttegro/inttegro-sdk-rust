//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::PaymentMethodVerificationDelivery;
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentMethodVerificationSession {
    pub payment_method_id: String,
    pub status: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token_sent_at: Option<crate::Timestamp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<crate::Timestamp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delivery: Option<PaymentMethodVerificationDelivery>,
}
