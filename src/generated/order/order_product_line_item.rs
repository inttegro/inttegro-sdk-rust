//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::OrderProductLineItemProduct;
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderProductLineItem {
    #[serde(rename = "type")]
    pub r#type: String,
    pub product: OrderProductLineItemProduct,
}
