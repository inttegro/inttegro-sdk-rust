//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::MessageTemplate;
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageTemplatesPage {
    pub number: i64,
    pub size: i64,
    pub message_templates: Vec<MessageTemplate>,
}
