//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::Amount;
use serde::{Deserialize, Serialize};

/// A sparse view of one balance transaction's contribution to a payout.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PayoutBalanceTransaction {
    /// The exact portion allocated to this payout.
    pub allocated_amount: Amount,
    /// The balance transaction's original amount before allocations.
    pub amount: Amount,
    /// Unique balance transaction identifier.
    pub id: String,
}
