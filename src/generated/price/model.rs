//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::Currency;
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Price {
    pub currency: Currency,
    pub value: i64,
}
