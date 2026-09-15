//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{
    CustomData, PaymentMethodBankAccount, PaymentMethodCard, PaymentMethodMobileMoney,
    PaymentMethodOwner, PaymentMethodSupplied, PaymentMethodType, PaymentMethodVerification,
};
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentMethod {
    pub active: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archived_at: Option<crate::Timestamp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<PaymentMethodBankAccount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card: Option<PaymentMethodCard>,
    pub created_at: crate::Timestamp,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomData>,
    pub customer_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ephemeral: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_on: Option<crate::Timestamp>,
    pub fingerprint: String,
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile_money: Option<PaymentMethodMobileMoney>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<PaymentMethodOwner>,
    #[serde(rename = "type")]
    pub r#type: PaymentMethodType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub supplied: Option<PaymentMethodSupplied>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verification: Option<PaymentMethodVerification>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verified_at: Option<crate::Timestamp>,
}
