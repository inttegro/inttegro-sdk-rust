//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::CustomDataInput;
use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomerDataInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataInput>,
    pub name: String,
    pub email_address: String,
    pub phone_number: String,
}
