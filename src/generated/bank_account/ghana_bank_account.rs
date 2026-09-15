//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::FinancialAccountOwner;
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GhanaBankAccount {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    pub holder: FinancialAccountOwner,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub number: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub swift_code: Option<String>,
}
