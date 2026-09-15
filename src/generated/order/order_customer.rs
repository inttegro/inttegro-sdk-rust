//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::OrderAddress;
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderCustomer {
    pub id: String,
    pub guest: bool,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<OrderAddress>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<OrderAddress>,
}
