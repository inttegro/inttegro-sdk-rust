//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{MessageTemplateChannel, MessageTemplateSMSContentInput, MessageTemplateVariableInput};
use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateSMSMessageTemplateRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub about: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variables: Option<Vec<MessageTemplateVariableInput>>,
    pub channel: MessageTemplateChannel,
    pub name: String,
    pub purpose: String,
    pub sms: MessageTemplateSMSContentInput,
}
