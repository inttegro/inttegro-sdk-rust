//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::CountryBank;
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CountryBankDirectory {
    pub bank_account_type: String,
    pub code_scheme: String,
    pub items: Vec<CountryBank>,
}
