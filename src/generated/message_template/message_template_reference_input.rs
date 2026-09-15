//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::JsonData;
use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageTemplateReferenceInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variables: Option<JsonData>,
    pub template_id: String,
}
