//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::MobileMoneyNetwork;
use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialAccountWalletRequestWalletMobileMoney {
    pub account_number: String,
    pub network: MobileMoneyNetwork,
}
