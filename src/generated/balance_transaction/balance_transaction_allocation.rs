//! Caller-safe allocations of a payment balance transaction.

use crate::{BalanceTransactionAllocationStatus, BalanceTransactionAllocationUse};
use serde::{Deserialize, Serialize};

/// An allocation discriminated by its `type` field. Type-specific details stay
/// nested under the corresponding refund or payout object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum BalanceTransactionAllocation {
    Refund {
        id: String,
        status: BalanceTransactionAllocationStatus,
        refund: BalanceTransactionAllocationUse,
        created_at: crate::Timestamp,
        updated_at: crate::Timestamp,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        completed_at: Option<crate::Timestamp>,
    },
    Payout {
        id: String,
        status: BalanceTransactionAllocationStatus,
        payout: BalanceTransactionAllocationUse,
        created_at: crate::Timestamp,
        updated_at: crate::Timestamp,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        completed_at: Option<crate::Timestamp>,
    },
}
