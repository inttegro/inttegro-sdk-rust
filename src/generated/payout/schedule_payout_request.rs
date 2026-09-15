//! Generated, typed Inttegro domain or request value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SchedulePayoutRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execute_after: Option<crate::Timestamp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_amount: Option<i64>,
    pub destination_id: String,
    pub reference: String,
}
