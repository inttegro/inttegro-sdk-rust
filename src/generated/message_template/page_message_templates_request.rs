//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{MessageTemplateChannel, MessageTemplateStatus};
use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PageMessageTemplatesRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<MessageTemplateStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel: Option<MessageTemplateChannel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
}
