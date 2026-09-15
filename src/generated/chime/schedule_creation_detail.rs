//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::ChimeEmailMessage;
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScheduleCreationDetail {
    pub created_at: crate::Timestamp,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customer_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<ChimeEmailMessage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub executed_at: Option<crate::Timestamp>,
    pub full_message: String,
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipients: Option<Vec<String>>,
    pub send_after: crate::Timestamp,
    pub sender_id: String,
}
