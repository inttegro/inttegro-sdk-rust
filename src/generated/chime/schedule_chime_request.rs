//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{
    ChimeEmailMessageInput, MessageTemplateReferenceInput, ScheduleChimeRequestRequestMeta,
};
use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScheduleChimeRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_meta: Option<ScheduleChimeRequestRequestMeta>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<ChimeEmailMessageInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_template: Option<MessageTemplateReferenceInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sender_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
    pub recipients: Vec<serde_json::Value>,
    pub send_after: crate::Timestamp,
}
