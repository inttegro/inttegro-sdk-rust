//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::AddressInput;
use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShippingInput {
    pub address: AddressInput,
}
