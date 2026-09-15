//! Generated, typed Inttegro domain or request value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompleteOrderRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paid_out_of_band: Option<bool>,
    pub order_id: String,
}
