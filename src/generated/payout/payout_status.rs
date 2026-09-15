//! Generated, typed Inttegro domain or request value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// A typed `PayoutStatus` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PayoutStatus {
    #[serde(rename = "initialized")]
    Initialized,
    #[serde(rename = "scheduled")]
    Scheduled,
    #[serde(rename = "processing")]
    Processing,
    #[serde(rename = "executing")]
    Executing,
    #[serde(rename = "succeeded")]
    Succeeded,
    #[serde(rename = "invalid")]
    Invalid,
    #[serde(rename = "canceled")]
    Canceled,
}
