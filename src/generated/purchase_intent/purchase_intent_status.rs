//! Generated, typed Inttegro domain or request value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// A typed `PurchaseIntentStatus` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PurchaseIntentStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(rename = "used")]
    Used,
}
