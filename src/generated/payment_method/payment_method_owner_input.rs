//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::PaymentMethodOwnerInputAddress;
use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentMethodOwnerInput {
    pub address: PaymentMethodOwnerInputAddress,
    pub name: String,
}
