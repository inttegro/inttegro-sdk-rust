//! Generated, typed Inttegro domain value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// Whether a payout contribution remains reserved, completed, or released.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PayoutBalanceTransactionStatus {
    #[serde(rename = "reserved")]
    Reserved,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "released")]
    Released,
}
