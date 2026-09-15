//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{MessageTemplateChannel, RenderedEmailMessageTemplate, RenderedSMSMessageTemplate};
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RenderedMessageTemplate {
    pub channel: MessageTemplateChannel,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sms: Option<RenderedSMSMessageTemplate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<RenderedEmailMessageTemplate>,
}
