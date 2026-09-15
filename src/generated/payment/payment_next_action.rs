//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{
    PaymentNextActionAuthorize, PaymentNextActionConfirmPayment, PaymentNextActionRedirect,
    PaymentNextActionRequestConfirmation, PaymentNextActionType,
};
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentNextAction {
    #[serde(rename = "type")]
    pub r#type: PaymentNextActionType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confirm_payment: Option<PaymentNextActionConfirmPayment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub redirect: Option<PaymentNextActionRedirect>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authorize: Option<PaymentNextActionAuthorize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_confirmation: Option<PaymentNextActionRequestConfirmation>,
}
