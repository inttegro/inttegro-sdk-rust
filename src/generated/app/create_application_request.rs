//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::CreateApplicationRequestRelationshipPolicy;
use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateApplicationRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub legal_entity_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placement_parent_application_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relationship_policy: Option<CreateApplicationRequestRelationshipPolicy>,
    pub name: String,
}
