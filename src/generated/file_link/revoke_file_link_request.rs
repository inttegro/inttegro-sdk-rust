//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::FileActorInput;
use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RevokeFileLinkRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revoked_by: Option<FileActorInput>,
    pub id: String,
}
