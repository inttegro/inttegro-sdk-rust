//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{ContentSafetyStatus, MessageTemplateScannedLink};
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageTemplateSafetyResult {
    pub content_hash: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<MessageTemplateScannedLink>>,
    pub normalized_text: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quarantine_notes: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason_codes: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sanitized_html: Option<String>,
    pub scanner: String,
    pub status: ContentSafetyStatus,
}
