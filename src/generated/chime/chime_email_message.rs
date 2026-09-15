//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{ChimeEmailMailbox, ChimeEmailSafetyResult, ChimeEmailSchemaMarkup, MessageHeaders};
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChimeEmailMessage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    #[serde(rename = "from")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from_: Option<ChimeEmailMailbox>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<ChimeEmailMailbox>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<MessageHeaders>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub safety: Option<ChimeEmailSafetyResult>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<ChimeEmailSchemaMarkup>,
}
