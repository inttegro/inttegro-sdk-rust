//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{Amount, RefundReason};
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RefundLineItem {
    pub id: String,
    pub order_line_item_id: String,
    pub original_amount_paid: Amount,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<RefundReason>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason_details: Option<String>,
    pub refund_amount: Amount,
}
