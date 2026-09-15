//! Generated, typed Inttegro domain or request value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfirmPaymentRequest {
    pub order_id: String,
    pub payment_id: String,
    pub confirmation_id: String,
    pub token: String,
}
