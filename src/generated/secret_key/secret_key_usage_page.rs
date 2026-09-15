//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::SecretKeyUsageRow;
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecretKeyUsagePage {
    pub number: i64,
    pub size: i64,
    pub count: i64,
    pub total: i64,
    pub has_more: bool,
    pub rows: Vec<SecretKeyUsageRow>,
}
