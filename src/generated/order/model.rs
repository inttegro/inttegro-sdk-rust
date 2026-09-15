//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{
    CustomData, InvoiceSettings, OrderCheckoutSettings, OrderCreatedFrom, OrderCustomer,
    OrderInvoice, OrderLineItemGroup, OrderStatus, Payment, Refund,
};
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Order {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<crate::Timestamp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checkout_settings: Option<OrderCheckoutSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<crate::Timestamp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_from: Option<OrderCreatedFrom>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomData>,
    pub customer: OrderCustomer,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<crate::Timestamp>,
    pub id: String,
    pub initiated_at: crate::Timestamp,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invoice: Option<OrderInvoice>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receipt_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refunds: Option<Vec<Refund>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<InvoiceSettings>,
    pub status: OrderStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sealed_at: Option<crate::Timestamp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line_item_group: Option<OrderLineItemGroup>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment: Option<Payment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paid_at: Option<crate::Timestamp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_due_at: Option<crate::Timestamp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}
