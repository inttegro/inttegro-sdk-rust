//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::OrderAddress;
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentMethodSnapshotOwner {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<OrderAddress>,
}
