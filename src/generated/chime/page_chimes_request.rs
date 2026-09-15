//! Generated, typed Inttegro domain or request value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PageChimesRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipient: Option<String>,
}
