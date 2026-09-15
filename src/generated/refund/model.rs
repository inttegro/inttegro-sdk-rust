//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{Amount, CustomData, RefundLineItem, RefundReason, RefundSettlement, RefundStatus};
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Refund {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<crate::Timestamp>,
    pub created_at: crate::Timestamp,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failed_at: Option<crate::Timestamp>,
    pub id: String,
    pub line_items: Vec<RefundLineItem>,
    pub order_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processing_at: Option<crate::Timestamp>,
    pub reason: RefundReason,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason_details: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    pub settlement: RefundSettlement,
    pub status: RefundStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub succeeded_at: Option<crate::Timestamp>,
    pub total: Amount,
}
