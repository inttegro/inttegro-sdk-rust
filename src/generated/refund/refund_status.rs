//! Generated, typed Inttegro domain or request value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// A typed `RefundStatus` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RefundStatus {
    #[serde(rename = "canceled")]
    Canceled,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "processing")]
    Processing,
    #[serde(rename = "succeeded")]
    Succeeded,
}
