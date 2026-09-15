//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::FinancialAccountOwnerUpdateInputAddress;
use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialAccountOwnerUpdateInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<FinancialAccountOwnerUpdateInputAddress>,
}
