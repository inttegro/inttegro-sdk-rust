//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::FinancialAccountOwnerInputAddress;
use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialAccountOwnerInput {
    pub name: String,
    pub address: FinancialAccountOwnerInputAddress,
}
