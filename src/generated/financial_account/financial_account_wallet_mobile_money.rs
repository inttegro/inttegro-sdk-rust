//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::MobileMoneyNetwork;
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialAccountWalletMobileMoney {
    pub account_number: String,
    pub network: MobileMoneyNetwork,
}
