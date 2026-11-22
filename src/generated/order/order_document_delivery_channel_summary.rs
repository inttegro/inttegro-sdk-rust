//! Generated, typed Inttegro domain or request value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// Accepted deliveries aggregated for one channel.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderDocumentDeliveryChannelSummary {
    pub count: i64,
    pub latest_sent_at: crate::Timestamp,
}
