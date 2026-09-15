//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::Refund;
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RefundPage {
    pub number: i64,
    pub refunds: Vec<Refund>,
    pub size: i64,
}
