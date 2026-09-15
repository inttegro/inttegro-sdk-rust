//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{MessageHeaders, MessageTemplateMailbox, MessageTemplateSafetyResult};
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RenderedEmailMessageTemplate {
    pub subject: String,
    pub text: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    #[serde(rename = "from")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from_: Option<MessageTemplateMailbox>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<MessageTemplateMailbox>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<MessageHeaders>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub safety: Option<MessageTemplateSafetyResult>,
}
