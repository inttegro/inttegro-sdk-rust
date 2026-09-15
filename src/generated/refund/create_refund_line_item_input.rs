//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{AmountParams, RefundReason};
use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateRefundLineItemInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<RefundReason>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason_details: Option<String>,
    pub order_line_item_id: String,
    pub refund_amount: AmountParams,
}
