//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::OrderFeeLineItemFee;
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderFeeLineItem {
    #[serde(rename = "type")]
    pub r#type: String,
    pub fee: OrderFeeLineItemFee,
}
