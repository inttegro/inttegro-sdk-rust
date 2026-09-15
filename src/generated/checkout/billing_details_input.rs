//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::AddressInput;
use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BillingDetailsInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<AddressInput>,
    pub name: String,
    pub email_address: String,
    pub phone_number: String,
}
