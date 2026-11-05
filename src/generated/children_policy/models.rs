//! Typed children-policy values.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChildrenPolicyAmountRange {
    pub min: u64,
    pub max: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChildrenPolicyLimits {
    pub amount: ChildrenPolicyAmountRange,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChildrenPolicyBTAgingSpec {
    pub public_id: String,
    pub t_plus: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChildrenPolicyPaymentsCapabilities {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currencies: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<ChildrenPolicyLimits>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChildrenPolicyPayoutsCapabilities {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<Vec<bool>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<ChildrenPolicyLimits>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bt_aging_spec: Option<Vec<ChildrenPolicyBTAgingSpec>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChildrenPolicyPaymentMethodCapabilities {
    pub enabled: Vec<bool>,
    pub confirms_use: Vec<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ChildrenPolicyPaymentMethodsCapabilities {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile_money: Option<ChildrenPolicyPaymentMethodCapabilities>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<ChildrenPolicyPaymentMethodCapabilities>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub motito: Option<ChildrenPolicyPaymentMethodCapabilities>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card: Option<ChildrenPolicyPaymentMethodCapabilities>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ChildrenPolicyCapabilities {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payments: Option<ChildrenPolicyPaymentsCapabilities>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payouts: Option<ChildrenPolicyPayoutsCapabilities>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_methods: Option<ChildrenPolicyPaymentMethodsCapabilities>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChildrenPolicy {
    pub id: String,
    pub owner_application_id: String,
    pub version: u64,
    pub active_version: u64,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub capabilities: ChildrenPolicyCapabilities,
    pub created_at: crate::Timestamp,
    pub updated_at: crate::Timestamp,
    pub version_created_at: crate::Timestamp,
}
