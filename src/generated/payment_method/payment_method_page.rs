//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::PaymentMethod;
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentMethodPage {
    pub number: i64,
    pub payment_methods: Vec<PaymentMethod>,
    pub size: i64,
}
