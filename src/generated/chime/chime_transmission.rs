//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{ChimeEmailEvent, ChimeTransport};
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChimeTransmission {
    pub address: String,
    pub created_at: crate::Timestamp,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delivered_at: Option<crate::Timestamp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_events: Option<Vec<ChimeEmailEvent>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_failure_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_failure_reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failed_at: Option<crate::Timestamp>,
    pub gateway: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gateway_message_id: Option<String>,
    pub id: String,
    pub initialized_at: crate::Timestamp,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_email_event_at: Option<crate::Timestamp>,
    pub mechanism: ChimeTransport,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sent_at: Option<crate::Timestamp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sent_via: Option<ChimeTransport>,
    pub status: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suppressed_at: Option<crate::Timestamp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suppression_reason: Option<String>,
}
