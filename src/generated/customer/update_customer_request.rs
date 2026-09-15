//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{CustomDataPatch, CustomerAddressInput};
use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateCustomerRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<CustomerAddressInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataPatch>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<CustomerAddressInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    pub customer_id: String,
}
