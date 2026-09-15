//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{Error, Order, OrderDocumentDelivery};
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderDocumentDeliveryResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delivery: Option<OrderDocumentDelivery>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<Error>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<Order>,
}
