//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{MessageTemplate, RenderedMessageTemplate};
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageTemplatePreview {
    pub message_template: MessageTemplate,
    pub rendered: RenderedMessageTemplate,
}
