//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::FinancialAccount;
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialAccountPage {
    pub accounts: Vec<FinancialAccount>,
    pub number: i64,
    pub size: i64,
}
