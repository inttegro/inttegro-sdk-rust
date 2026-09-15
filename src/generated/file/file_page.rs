//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::File;
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FilePage {
    pub number: i64,
    pub size: i64,
    pub files: Vec<File>,
}
