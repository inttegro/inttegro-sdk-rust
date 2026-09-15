//! Generated, typed Inttegro domain or request value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreatePurchaseIntentRequestQuantity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<i64>,
    pub min: i64,
}
