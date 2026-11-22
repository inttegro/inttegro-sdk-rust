//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::OrderDocumentDeliverySummaryChannels;
use serde::{Deserialize, Serialize};

/// Accepted invoice or receipt deliveries aggregated across channels.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderDocumentDeliverySummary {
    pub channels: OrderDocumentDeliverySummaryChannels,
    pub latest_sent_at: crate::Timestamp,
    pub total_count: i64,
}
