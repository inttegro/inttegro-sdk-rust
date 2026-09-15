//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::PaymentMethodTypeSetting;
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentMethodSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile_money: Option<PaymentMethodTypeSetting>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<PaymentMethodTypeSetting>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card: Option<PaymentMethodTypeSetting>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub motito: Option<PaymentMethodTypeSetting>,
}
