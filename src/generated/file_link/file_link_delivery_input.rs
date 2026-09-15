//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::FileLinkDeliveryMode;
use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileLinkDeliveryInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<FileLinkDeliveryMode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disposition: Option<String>,
}
