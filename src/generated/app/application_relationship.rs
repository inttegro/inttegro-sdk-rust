//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{AppRelationshipKind, AppRelationshipStatus, ApplicationRelationshipPolicy};
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplicationRelationship {
    pub id: String,
    pub kind: AppRelationshipKind,
    pub policy_version: String,
    pub status: AppRelationshipStatus,
    pub actor_app_id: String,
    pub creator_app_id: String,
    pub placement_parent_app_id: String,
    pub subject_app_id: String,
    pub child_app_id: String,
    pub child_standing: String,
    pub relationship_policy: ApplicationRelationshipPolicy,
    pub retained_creator_authority_exists: bool,
    pub created_at: crate::Timestamp,
}
