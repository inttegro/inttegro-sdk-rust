//! Generated, typed Inttegro domain or request value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// A typed `RefundReason` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RefundReason {
    #[serde(rename = "requested_by_customer")]
    RequestedByCustomer,
    #[serde(rename = "duplicate")]
    Duplicate,
    #[serde(rename = "fraudulent")]
    Fraudulent,
    #[serde(rename = "order_canceled")]
    OrderCanceled,
    #[serde(rename = "item_returned")]
    ItemReturned,
    #[serde(rename = "item_damaged")]
    ItemDamaged,
    #[serde(rename = "item_not_received")]
    ItemNotReceived,
    #[serde(rename = "item_not_as_described")]
    ItemNotAsDescribed,
    #[serde(rename = "custom")]
    Custom,
}
