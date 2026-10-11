//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{AppRelationshipKind, AppRelationshipStatus, ApplicationRelationshipParent};
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplicationRelationship {
    pub id: String,
    pub kind: AppRelationshipKind,
    pub parent: ApplicationRelationshipParent,
    pub status: AppRelationshipStatus,
    pub created_at: crate::Timestamp,
}
