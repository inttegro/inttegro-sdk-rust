//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{
    CustomData, FileActor, FileDeliveryDetails, FileLatestError, FileMedia, FileMetadata,
    FileScanStatus, FileSource, FileStatus, PublicFileStorage,
};
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct File {
    pub id: String,
    pub purpose: String,
    pub status: FileStatus,
    pub scan_status: FileScanStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    pub content_type: String,
    pub size: i64,
    pub checksum_sha256: String,
    pub created_by: FileActor,
    pub source: FileSource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub media: Option<FileMedia>,
    pub storage: PublicFileStorage,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delivery: Option<FileDeliveryDetails>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latest_error: Option<FileLatestError>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<FileMetadata>,
    pub created_at: crate::Timestamp,
    pub updated_at: crate::Timestamp,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub available_at: Option<crate::Timestamp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<crate::Timestamp>,
}
