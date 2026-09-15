//! Generated, typed Inttegro domain or request value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// A typed `PaymentNextActionType` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PaymentNextActionType {
    #[serde(rename = "confirm_payment")]
    ConfirmPayment,
    #[serde(rename = "execute")]
    Execute,
    #[serde(rename = "redirect")]
    Redirect,
    #[serde(rename = "authorize_payment")]
    AuthorizePayment,
    #[serde(rename = "request_confirmation")]
    RequestConfirmation,
}
