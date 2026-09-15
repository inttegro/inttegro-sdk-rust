//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{
    CustomDataInput, FinancialAccountBankRequestBankAccount,
    FinancialAccountBankRequestPullConfiguration, FinancialAccountBankRequestPushConfiguration,
    FinancialAccountOwnerInput, FinancialAccountType,
};
use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialAccountBankRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<FinancialAccountOwnerInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pull_configuration: Option<FinancialAccountBankRequestPullConfiguration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub push_configuration: Option<FinancialAccountBankRequestPushConfiguration>,
    pub currency: String,
    pub label: String,
    pub reference: String,
    #[serde(rename = "type")]
    pub r#type: FinancialAccountType,
    pub bank_account: FinancialAccountBankRequestBankAccount,
}
