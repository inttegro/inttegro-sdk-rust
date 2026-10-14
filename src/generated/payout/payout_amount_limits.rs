//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::Amount;
use serde::{Deserialize, Serialize};

/// Inclusive payout amount range.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PayoutAmountLimits {
    pub min: Amount,
    pub max: Amount,
}
