//! Generated, typed Inttegro domain or request value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// Immutable order-line snapshot attached to a refund.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum RefundOrderLineItem {
    Product {
        id: String,
        quantity: i64,
        product: RefundOrderLineItemProduct,
    },
    Fee {
        id: String,
        fee: RefundOrderLineItemAdjustment,
    },
    Shipping {
        id: String,
        shipping: RefundOrderLineItemAdjustment,
    },
}

/// Product identity captured for a refunded product line.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RefundOrderLineItemProduct {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub name: String,
}

/// Descriptive fields captured for a refunded fee or shipping line.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RefundOrderLineItemAdjustment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
