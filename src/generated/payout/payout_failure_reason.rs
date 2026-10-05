//! Generated, typed Inttegro domain or request value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// A stable, caller-safe reason that a payout failed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PayoutFailureReason {
    #[serde(rename = "provider_declined")]
    ProviderDeclined,
    #[serde(rename = "delivery_failed")]
    DeliveryFailed,
    #[serde(rename = "temporarily_unavailable")]
    TemporarilyUnavailable,
    #[serde(rename = "unknown")]
    Unknown,
}
