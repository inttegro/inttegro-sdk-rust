//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::FinancialInstitutionBankBranch;
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialInstitutionBank {
    pub bank_account_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch: Option<FinancialInstitutionBankBranch>,
    pub code_scheme: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort_code_prefix: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub swift_code: Option<String>,
}
