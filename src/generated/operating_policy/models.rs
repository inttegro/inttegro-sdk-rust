//! Typed operating-policy values and requests.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperatingPolicyAmountRange {
    pub min: u64,
    pub max: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperatingPolicyLimits {
    pub amount: OperatingPolicyAmountRange,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperatingPolicyBTAgingSpecReference {
    pub id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperatingPolicyBTAgingSpec {
    pub id: String,
    pub t_plus: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct OperatingPolicyPaymentsCapabilitiesInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currencies: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<OperatingPolicyLimits>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct OperatingPolicyPayoutsCapabilitiesInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<OperatingPolicyLimits>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bt_aging_spec: Option<OperatingPolicyBTAgingSpecReference>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperatingPolicyPaymentMethodCapabilities {
    pub enabled: bool,
    pub confirms_use: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct OperatingPolicyPaymentMethodsCapabilities {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile_money: Option<OperatingPolicyPaymentMethodCapabilities>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<OperatingPolicyPaymentMethodCapabilities>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub motito: Option<OperatingPolicyPaymentMethodCapabilities>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card: Option<OperatingPolicyPaymentMethodCapabilities>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct OperatingPolicyCapabilitiesInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payments: Option<OperatingPolicyPaymentsCapabilitiesInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payouts: Option<OperatingPolicyPayoutsCapabilitiesInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_methods: Option<OperatingPolicyPaymentMethodsCapabilities>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct OperatingPolicyPaymentsCapabilities {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currencies: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<OperatingPolicyLimits>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct OperatingPolicyPayoutsCapabilities {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<OperatingPolicyLimits>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bt_aging_spec: Option<OperatingPolicyBTAgingSpec>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct OperatingPolicyCapabilities {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payments: Option<OperatingPolicyPaymentsCapabilities>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payouts: Option<OperatingPolicyPayoutsCapabilities>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_methods: Option<OperatingPolicyPaymentMethodsCapabilities>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperatingPolicySourceType {
    Inline,
    Template,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperatingPolicySource {
    #[serde(rename = "type")]
    pub source_type: OperatingPolicySourceType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApplicationOperatingPolicy {
    pub capabilities: OperatingPolicyCapabilities,
    pub source: OperatingPolicySource,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperatingPolicy {
    pub id: String,
    pub version: u64,
    pub active_version: u64,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub capabilities: OperatingPolicyCapabilities,
    pub children_policy_id: String,
    pub children_policy_version: u64,
    pub created_at: crate::Timestamp,
    pub updated_at: crate::Timestamp,
    pub version_created_at: crate::Timestamp,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperatingPolicySelectionById {
    pub id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InlineOperatingPolicySelection {
    pub capabilities: OperatingPolicyCapabilitiesInput,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OperatingPolicySelection {
    ById(OperatingPolicySelectionById),
    Inline(InlineOperatingPolicySelection),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreateOperatingPolicyRequest {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub capabilities: OperatingPolicyCapabilitiesInput,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LookupOperatingPolicyRequest {
    pub operating_policy_id: String,
}

/// Pagination parameters for listing versions of one operating policy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PageOperatingPoliciesRequest {
    pub operating_policy_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}

/// A page of immutable operating-policy versions, newest first.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperatingPolicyPage {
    pub number: i64,
    pub size: i64,
    pub count: i64,
    pub total: i64,
    pub has_more: bool,
    pub versions: Vec<OperatingPolicy>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateOperatingPolicyRequest {
    pub operating_policy_id: String,
    pub expected_version: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<OperatingPolicyCapabilitiesInput>,
}
