//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{OrderDiscountLineItem, OrderFeeLineItem, OrderProductLineItem, OrderShippingLineItem};
use serde::{Deserialize, Serialize};

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OrderLineItem {
    OrderProductLineItem(OrderProductLineItem),
    OrderFeeLineItem(OrderFeeLineItem),
    OrderShippingLineItem(OrderShippingLineItem),
    OrderDiscountLineItem(OrderDiscountLineItem),
}
