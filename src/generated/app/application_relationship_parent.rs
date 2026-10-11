//! Generated, typed Inttegro domain or request value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// Public identity of the application that directly contains another application.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplicationRelationshipParent {
    pub id: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
}
