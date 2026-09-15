//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::PaymentMethodType;
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentMethodTypeSetting {
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<PaymentMethodType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub enabled: bool,
    pub confirms_use: bool,
}
