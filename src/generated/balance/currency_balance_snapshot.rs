//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{BalanceValue, CurrencyBalanceSnapshotRefund, CurrencyBalanceSnapshotReserved};
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CurrencyBalanceSnapshot {
    pub available: BalanceValue,
    pub includes_transactions_before: crate::Timestamp,
    pub pending: BalanceValue,
    pub refund: CurrencyBalanceSnapshotRefund,
    pub reserved: CurrencyBalanceSnapshotReserved,
}
