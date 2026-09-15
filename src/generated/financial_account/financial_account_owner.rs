//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::FinancialAccountAddress;
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialAccountOwner {
    pub address: FinancialAccountAddress,
    pub name: String,
}
