//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{
    Amount, BalanceTransaction, PaymentAttempt, PaymentBillingDetails, PaymentCustomer,
    PaymentError, PaymentMethodSnapshot, PaymentNextAction, PaymentStatus, PayoutConfiguration,
};
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Payment {
    pub id: String,
    pub status: PaymentStatus,
    pub statement_descriptor: String,
    pub amount: Amount,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub billing_details: Option<PaymentBillingDetails>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub balance_transaction: Option<BalanceTransaction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<PaymentMethodSnapshot>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customer: Option<PaymentCustomer>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latest_attempt: Option<PaymentAttempt>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_action: Option<PaymentNextAction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latest_error: Option<PaymentError>,
    pub initiated_at: crate::Timestamp,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub executed_at: Option<crate::Timestamp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paid_at: Option<crate::Timestamp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<crate::Timestamp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub due_at: Option<crate::Timestamp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expired_at: Option<crate::Timestamp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failed_at: Option<crate::Timestamp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paid_offline: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_method_types: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payout_configuration: Option<PayoutConfiguration>,
}
