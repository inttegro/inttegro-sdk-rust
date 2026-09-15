//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::BalanceTransaction;
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BalanceTransactionPage {
    pub number: i64,
    pub size: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transactions: Option<Vec<BalanceTransaction>>,
}
