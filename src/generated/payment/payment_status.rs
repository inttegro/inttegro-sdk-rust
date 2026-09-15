//! Generated, typed Inttegro domain or request value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// A typed `PaymentStatus` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PaymentStatus {
    #[serde(rename = "initiated")]
    Initiated,
    #[serde(rename = "requires_action")]
    RequiresAction,
    #[serde(rename = "overdue")]
    Overdue,
    #[serde(rename = "executed")]
    Executed,
    #[serde(rename = "paid")]
    Paid,
    #[serde(rename = "canceled")]
    Canceled,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "unknown")]
    Unknown,
}
