//! Generated, typed Inttegro domain or request value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// A typed `UploadReviewType` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UploadReviewType {
    #[serde(rename = "automatic")]
    Automatic,
    #[serde(rename = "manual")]
    Manual,
}
