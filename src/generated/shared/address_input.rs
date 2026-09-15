//! Generated, typed Inttegro domain or request value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddressInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub district: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post_code: Option<String>,
    pub name: String,
    pub phone_number: String,
    pub line1: String,
    pub town: String,
    pub country: String,
}
