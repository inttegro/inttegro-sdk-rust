//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{CustomDataInput, CustomerAddressInput};
use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateCustomerRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<CustomerAddressInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<CustomerAddressInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    pub name: String,
}
