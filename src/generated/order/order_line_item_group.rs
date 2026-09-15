//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{Amount, OrderLineItem};
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderLineItemGroup {
    pub line_items: Vec<OrderLineItem>,
    pub total: Amount,
}
