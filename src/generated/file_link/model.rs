//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{
    CustomData, FileLinkAccess, FileLinkActor, FileLinkDelivery, FileLinkKind, FileLinkStatus,
    FileMetadata,
};
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileLink {
    pub id: String,
    pub kind: FileLinkKind,
    pub file_id: String,
    pub purpose: String,
    pub status: FileLinkStatus,
    pub active: bool,
    pub delivery: FileLinkDelivery,
    pub access: FileLinkAccess,
    pub created_by: FileLinkActor,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revoked_by: Option<FileLinkActor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<FileMetadata>,
    pub created_at: crate::Timestamp,
    pub updated_at: crate::Timestamp,
    pub expires_at: crate::Timestamp,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revoked_at: Option<crate::Timestamp>,
}
