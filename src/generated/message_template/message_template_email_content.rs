//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{MessageHeaders, MessageTemplateMailbox};
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageTemplateEmailContent {
    pub subject: String,
    pub html: String,
    #[serde(rename = "from")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from_: Option<MessageTemplateMailbox>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<MessageTemplateMailbox>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<MessageHeaders>,
}
