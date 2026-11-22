//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::OrderDocumentDeliveryChannelSummary;
use serde::{Deserialize, Serialize};

/// Accepted delivery totals grouped by channel.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderDocumentDeliverySummaryChannels {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<OrderDocumentDeliveryChannelSummary>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sms: Option<OrderDocumentDeliveryChannelSummary>,
}
