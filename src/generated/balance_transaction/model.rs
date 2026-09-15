//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{BalanceTransactionAmount, BalanceTransactionType, PayoutConfiguration};
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BalanceTransaction {
    pub amount: BalanceTransactionAmount,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub available_at: Option<crate::Timestamp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claimed_at: Option<crate::Timestamp>,
    pub created_at: crate::Timestamp,
    pub id: String,
    pub order_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paid_at: Option<crate::Timestamp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payout_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payout_configuration: Option<PayoutConfiguration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refund_id: Option<String>,
    #[serde(rename = "type")]
    pub r#type: BalanceTransactionType,
}
