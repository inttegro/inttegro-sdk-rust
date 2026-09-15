//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::FileStorageEncoding;
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PublicFileStorage {
    pub encoding: FileStorageEncoding,
    pub stored_size: i64,
}
