//! Generated, typed Inttegro domain or request value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// Safe Ghana bank-account routing details retained in refund history.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RefundSettlementGhanaBankAccount {
    pub account_number: String,
    pub last4: String,
}
