//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::PayoutAmountLimits;
use serde::{Deserialize, Serialize};

/// Platform limits applied to payouts.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PayoutSettingsLimits {
    pub amount: PayoutAmountLimits,
}
