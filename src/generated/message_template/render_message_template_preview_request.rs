//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::MessageTemplateReferenceInput;
use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RenderMessageTemplatePreviewRequest {
    pub message_template: MessageTemplateReferenceInput,
}
