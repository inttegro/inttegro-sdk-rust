//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{MessageTemplateVariableItemInput, MessageTemplateVariableType};
use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageTemplateVariableInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub about: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<MessageTemplateVariableItemInput>>,
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: MessageTemplateVariableType,
}
