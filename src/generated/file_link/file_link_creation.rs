//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::FileLink;
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileLinkCreation {
    pub file_link: FileLink,
    pub url: String,
}
