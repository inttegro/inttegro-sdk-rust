//! Generated, typed Inttegro domain or request value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// A typed `OTPStatus` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OTPStatus {
    #[serde(rename = "canceled")]
    Canceled,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "pending_delivery")]
    PendingDelivery,
    #[serde(rename = "pending_verification")]
    PendingVerification,
    #[serde(rename = "verified")]
    Verified,
}
