//! Generated, typed Inttegro domain or request value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// A typed `BalanceTransactionType` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BalanceTransactionType {
    #[serde(rename = "payment")]
    Payment,
    #[serde(rename = "refund")]
    Refund,
}
