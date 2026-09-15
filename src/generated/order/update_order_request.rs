//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{CustomData, InvoiceSettingsInput, LineItemInput, UpdateOrderRequestPaymentMethodData};
use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateOrderRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clear_payment_method: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<InvoiceSettingsInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub finalize: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line_items: Option<Vec<LineItemInput>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receipt_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_method_data: Option<UpdateOrderRequestPaymentMethodData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_method_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_prefix: Option<String>,
    pub order_id: String,
}
