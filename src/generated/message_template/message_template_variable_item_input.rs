//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::MessageTemplateVariableItemType;
use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageTemplateVariableItemInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub about: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: MessageTemplateVariableItemType,
}
