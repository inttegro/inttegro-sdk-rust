//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{
    PaymentMethodCard, PaymentMethodSnapshotBankAccount, PaymentMethodSnapshotMobileMoney,
    PaymentMethodSnapshotOwner, PaymentMethodType,
};
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentMethodSnapshot {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<PaymentMethodSnapshotBankAccount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card: Option<PaymentMethodCard>,
    pub created_at: crate::Timestamp,
    pub customer_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile_money: Option<PaymentMethodSnapshotMobileMoney>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<PaymentMethodSnapshotOwner>,
    #[serde(rename = "type")]
    pub r#type: PaymentMethodType,
    pub verified: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verified_at: Option<crate::Timestamp>,
}
