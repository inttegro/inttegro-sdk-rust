//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{ChimeEmailScannedLink, ContentSafetyStatus};
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChimeEmailSafetyResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ContentSafetyStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason_codes: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sanitized_html: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub normalized_text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ChimeEmailScannedLink>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scanner: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_hash: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quarantine_notes: Option<String>,
}
