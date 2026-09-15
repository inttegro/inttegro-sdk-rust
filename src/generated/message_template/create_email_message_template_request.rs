//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{
    MessageTemplateChannel, MessageTemplateEmailContentInput, MessageTemplateVariableInput,
};
use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateEmailMessageTemplateRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub about: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variables: Option<Vec<MessageTemplateVariableInput>>,
    pub channel: MessageTemplateChannel,
    pub email: MessageTemplateEmailContentInput,
    pub name: String,
    pub purpose: String,
}
