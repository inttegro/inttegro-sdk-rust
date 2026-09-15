//! Generated, typed Inttegro domain or request value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// A typed `FileStorageEncoding` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FileStorageEncoding {
    #[serde(rename = "identity")]
    Identity,
    #[serde(rename = "br")]
    Brotli,
}
