//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{BankAccountType, FinancialAccountBankRequestBankAccountGhanaBankAccount};
use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialAccountBankRequestBankAccount {
    #[serde(rename = "type")]
    pub r#type: BankAccountType,
    pub ghana_bank_account: FinancialAccountBankRequestBankAccountGhanaBankAccount,
}
