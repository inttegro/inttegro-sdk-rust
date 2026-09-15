//! Generated, typed Inttegro domain or request value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChimeEmailEvent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bounce_sub_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bounce_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub complaint_sub_type: Option<String>,
    pub id: String,
    pub occurred_at: crate::Timestamp,
    pub provider: String,
    pub provider_message_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipient: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suppress_recipient: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temporary: Option<bool>,
    #[serde(rename = "type")]
    pub r#type: String,
}
