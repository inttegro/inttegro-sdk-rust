//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::FinancialAccountOwnerInput;
use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialAccountBankRequestBankAccountGhanaBankAccount {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub swift_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub holder: Option<FinancialAccountOwnerInput>,
    pub number: String,
}
