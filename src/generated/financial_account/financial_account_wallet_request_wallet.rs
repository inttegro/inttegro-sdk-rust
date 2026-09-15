//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{FinancialAccountWalletRequestWalletMobileMoney, WalletType};
use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialAccountWalletRequestWallet {
    #[serde(rename = "type")]
    pub r#type: WalletType,
    pub mobile_money: FinancialAccountWalletRequestWalletMobileMoney,
}
