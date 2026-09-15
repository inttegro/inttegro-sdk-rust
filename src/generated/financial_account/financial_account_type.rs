//! Generated, typed Inttegro domain or request value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// A typed `FinancialAccountType` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FinancialAccountType {
    #[serde(rename = "wallet")]
    Wallet,
    #[serde(rename = "bank_account")]
    BankAccount,
    #[serde(rename = "dosh_account")]
    DoshAccount,
}
