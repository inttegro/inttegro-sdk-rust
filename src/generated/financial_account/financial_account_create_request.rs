//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{
    FinancialAccountBankRequest, FinancialAccountDoshRequest, FinancialAccountWalletRequest,
};
use serde::{Deserialize, Serialize};

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FinancialAccountCreateRequest {
    FinancialAccountWalletRequest(FinancialAccountWalletRequest),
    FinancialAccountBankRequest(FinancialAccountBankRequest),
    FinancialAccountDoshRequest(FinancialAccountDoshRequest),
}
