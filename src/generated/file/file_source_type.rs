//! Generated, typed Inttegro domain or request value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// A typed `FileSourceType` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FileSourceType {
    #[serde(rename = "direct")]
    Direct,
    #[serde(rename = "upload_request")]
    UploadRequest,
    #[serde(rename = "service")]
    Service,
}
