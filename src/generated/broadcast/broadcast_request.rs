//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{BroadcastRequestMessageTemplate, BroadcastRequestRequestMeta, ChimeEmailMessageInput};
use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BroadcastRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_meta: Option<BroadcastRequestRequestMeta>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_template: Option<BroadcastRequestMessageTemplate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<ChimeEmailMessageInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sender: Option<String>,
    pub recipients: Vec<serde_json::Value>,
}
