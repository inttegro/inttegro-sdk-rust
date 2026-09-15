//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{
    CustomDataInput, FinancialAccountOwnerInput, FinancialAccountType,
    FinancialAccountWalletRequestPullConfiguration, FinancialAccountWalletRequestPushConfiguration,
    FinancialAccountWalletRequestWallet,
};
use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialAccountWalletRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pull_configuration: Option<FinancialAccountWalletRequestPullConfiguration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub push_configuration: Option<FinancialAccountWalletRequestPushConfiguration>,
    pub currency: String,
    pub label: String,
    pub owner: FinancialAccountOwnerInput,
    pub reference: String,
    #[serde(rename = "type")]
    pub r#type: FinancialAccountType,
    pub wallet: FinancialAccountWalletRequestWallet,
}
