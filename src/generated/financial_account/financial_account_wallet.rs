//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{FinancialAccountWalletMobileMoney, WalletType};
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialAccountWallet {
    pub id: String,
    #[serde(rename = "type")]
    pub r#type: WalletType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile_money: Option<FinancialAccountWalletMobileMoney>,
}
