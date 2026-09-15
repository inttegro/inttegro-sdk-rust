//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{
    CustomDataInput, DoshAccount, FinancialAccountDoshRequestPullConfiguration,
    FinancialAccountDoshRequestPushConfiguration, FinancialAccountOwnerInput, FinancialAccountType,
};
use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialAccountDoshRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pull_configuration: Option<FinancialAccountDoshRequestPullConfiguration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub push_configuration: Option<FinancialAccountDoshRequestPushConfiguration>,
    pub currency: String,
    pub label: String,
    pub owner: FinancialAccountOwnerInput,
    pub reference: String,
    #[serde(rename = "type")]
    pub r#type: FinancialAccountType,
    pub dosh_account: DoshAccount,
}
