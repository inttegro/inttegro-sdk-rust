//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::PaymentMethodOwnerAddress;
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentMethodOwner {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<PaymentMethodOwnerAddress>,
    pub name: String,
}
