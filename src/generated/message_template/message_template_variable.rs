//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{MessageTemplateVariableItem, MessageTemplateVariableType};
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageTemplateVariable {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub about: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<MessageTemplateVariableItem>>,
    pub name: String,
    pub required: bool,
    #[serde(rename = "type")]
    pub r#type: MessageTemplateVariableType,
}
