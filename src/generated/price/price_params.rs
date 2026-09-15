//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::Currency;
use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PriceParams {
    pub currency: Currency,
    pub value: i64,
}
