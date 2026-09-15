//! Generated, typed Inttegro domain or request value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// A typed `UploadRequestStatus` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UploadRequestStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "uploading")]
    Uploading,
    #[serde(rename = "fulfilled")]
    Fulfilled,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "canceled")]
    Canceled,
    #[serde(rename = "failed")]
    Failed,
}
