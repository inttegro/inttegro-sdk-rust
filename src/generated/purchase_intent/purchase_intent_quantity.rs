//! Generated, typed Inttegro domain or request value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurchaseIntentQuantity {
    pub min: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<i64>,
}
