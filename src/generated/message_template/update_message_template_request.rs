//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{
    MessageTemplateChannel, MessageTemplateEmailContentInput, MessageTemplateSMSContentInput,
    MessageTemplateVariableInput,
};
use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateMessageTemplateRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub about: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel: Option<MessageTemplateChannel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variables: Option<Vec<MessageTemplateVariableInput>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sms: Option<MessageTemplateSMSContentInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<MessageTemplateEmailContentInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<String>>,
    pub id: String,
}
