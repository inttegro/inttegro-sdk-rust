//! Generated, typed Inttegro domain or request value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// A typed `CheckoutOrderStatus` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CheckoutOrderStatus {
    #[serde(rename = "preparing")]
    Preparing,
    #[serde(rename = "requires_payment")]
    RequiresPayment,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "canceled")]
    Canceled,
    #[serde(rename = "expired")]
    Expired,
}
