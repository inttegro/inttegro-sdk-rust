//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::PaymentConfirmationChannel;
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentNextActionConfirmPaymentRequest {
    pub id: String,
    pub recipient: String,
    pub sent_via: PaymentConfirmationChannel,
    pub token_size: i64,
    pub sender_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
