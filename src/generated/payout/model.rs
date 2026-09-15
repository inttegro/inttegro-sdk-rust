//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{Amount, CustomData, PayoutError, PayoutStatus};
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Payout {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<Amount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub balance_transactions: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<crate::Timestamp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomData>,
    pub destination_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<PayoutError>,
    pub execute_after: crate::Timestamp,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub executed_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_at: Option<crate::Timestamp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failed_at: Option<crate::Timestamp>,
    pub id: String,
    pub initiated_at: crate::Timestamp,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initiated_by: Option<String>,
    pub max_amount: Amount,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheduled_at: Option<crate::Timestamp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheduled_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sent_at: Option<crate::Timestamp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    pub status: PayoutStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub succeeded_at: Option<crate::Timestamp>,
}
