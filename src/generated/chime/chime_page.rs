//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::Chime;
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChimePage {
    pub number: i64,
    pub size: i64,
    pub chimes: Vec<Chime>,
}
