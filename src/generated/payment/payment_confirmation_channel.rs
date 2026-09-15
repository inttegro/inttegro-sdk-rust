//! Generated, typed Inttegro domain or request value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// A typed `PaymentConfirmationChannel` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PaymentConfirmationChannel {
    #[serde(rename = "sms")]
    Sms,
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "push")]
    Push,
}
