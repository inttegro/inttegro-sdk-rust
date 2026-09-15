//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{CustomData, FileActorInput, FileLinkAccessRequest, FileLinkDeliveryInput};
use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateFileLinkRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delivery: Option<FileLinkDeliveryInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access: Option<FileLinkAccessRequest>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<FileActorInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<crate::Timestamp>,
    pub file_id: String,
}
