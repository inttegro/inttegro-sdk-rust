//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{
    CustomData, DoshAccount, FinancialAccountBank, FinancialAccountOwner,
    FinancialAccountPullConfiguration, FinancialAccountPushConfiguration, FinancialAccountType,
    FinancialAccountVerification, FinancialAccountWallet, FinancialInstitution, ResourceSupply,
};
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialAccount {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archived_at: Option<crate::Timestamp>,
    pub created_at: crate::Timestamp,
    pub currency: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub institution: Option<FinancialInstitution>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pull_configuration: Option<FinancialAccountPullConfiguration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub push_configuration: Option<FinancialAccountPushConfiguration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub supplied: Option<ResourceSupply>,
    #[serde(rename = "type")]
    pub r#type: FinancialAccountType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verification: Option<FinancialAccountVerification>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<FinancialAccountBank>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disconnected_at: Option<crate::Timestamp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dosh_account: Option<DoshAccount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<FinancialAccountOwner>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wallet: Option<FinancialAccountWallet>,
}
