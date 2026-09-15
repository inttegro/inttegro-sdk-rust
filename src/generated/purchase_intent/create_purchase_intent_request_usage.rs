//! Generated, typed Inttegro domain or request value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreatePurchaseIntentRequestUsage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub single_use: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub multi_use: Option<bool>,
}
