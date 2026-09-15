//! Generated, typed Inttegro domain or request value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PageProductsRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    pub page_number: i64,
}
