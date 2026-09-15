//! Generated, typed Inttegro domain or request value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialAccountAddress {
    pub city: String,
    pub country: String,
    pub line_1: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line_2: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post_code: Option<String>,
    pub region: String,
}
