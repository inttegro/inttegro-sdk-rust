//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{
    BillingDetailsInput, CreateOrderExistingCustomerInputCheckoutSettings,
    CreateOrderExistingCustomerInputRequestMeta, CustomData, InvoiceSettingsInput, LineItemInput,
    OrderPayoutSettingsRequest, PaymentMethodDataInput, ShippingInput,
};
use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateOrderExistingCustomerInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_method_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_method_data: Option<PaymentMethodDataInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receipt_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_prefix: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execute_payment: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub finalize: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_meta: Option<CreateOrderExistingCustomerInputRequestMeta>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checkout_settings: Option<CreateOrderExistingCustomerInputCheckoutSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<InvoiceSettingsInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payout_settings: Option<OrderPayoutSettingsRequest>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub billing_details: Option<BillingDetailsInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shipping: Option<ShippingInput>,
    pub customer_id: String,
    pub line_items: Vec<LineItemInput>,
}
