//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{
    MessageTemplateChannel, MessageTemplateEmailContent, MessageTemplateSMSContent,
    MessageTemplateStatus, MessageTemplateVariable,
};
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageTemplate {
    pub id: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub about: Option<String>,
    pub channel: MessageTemplateChannel,
    pub purpose: String,
    pub locale: String,
    pub status: MessageTemplateStatus,
    pub version: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub published_version: Option<i64>,
    pub draft_version: i64,
    pub has_unpublished_changes: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variables: Option<Vec<MessageTemplateVariable>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sms: Option<MessageTemplateSMSContent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<MessageTemplateEmailContent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<String>>,
    pub created_at: crate::Timestamp,
    pub updated_at: crate::Timestamp,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub published_at: Option<crate::Timestamp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archived_at: Option<crate::Timestamp>,
}
