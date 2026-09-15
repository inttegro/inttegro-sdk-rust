//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{MessageHeaders, MessageTemplateMailboxInput};
use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageTemplateEmailContentInput {
    #[serde(rename = "from")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from_: Option<MessageTemplateMailboxInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<MessageTemplateMailboxInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<MessageHeaders>,
    pub subject: String,
    pub html: String,
}
