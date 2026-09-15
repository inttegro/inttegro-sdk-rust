//! Generated, typed Inttegro domain or request value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// A typed `PaymentMethodType` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PaymentMethodType {
    #[serde(rename = "mobile_money")]
    MobileMoney,
    #[serde(rename = "bank_account")]
    BankAccount,
    #[serde(rename = "card")]
    Card,
    #[serde(rename = "motito")]
    Motito,
}
