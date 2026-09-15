//! Generated, typed Inttegro domain or request value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentMethodOwnerAddress {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    pub country: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line_1: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line_2: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}
