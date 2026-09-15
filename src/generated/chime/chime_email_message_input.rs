//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{ChimeEmailMailboxInput, MessageHeaders};
use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChimeEmailMessageInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<MessageHeaders>,
    pub subject: String,
    pub text: String,
    #[serde(rename = "from")]
    pub from_: ChimeEmailMailboxInput,
}
