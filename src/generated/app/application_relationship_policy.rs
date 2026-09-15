//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{AppCredentialOwner, AppManagementRole};
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplicationRelationshipPolicy {
    pub child_standing: String,
    pub management: AppManagementRole,
    pub credentials: AppCredentialOwner,
}
