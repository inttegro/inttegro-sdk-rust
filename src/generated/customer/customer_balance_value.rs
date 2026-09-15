//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::Amount;
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomerBalanceValue {
    pub as_of: crate::Timestamp,
    pub available: Amount,
}
