//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::OrderDiscount;
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderDiscountLineItem {
    #[serde(rename = "type")]
    pub r#type: String,
    pub discount: OrderDiscount,
}
