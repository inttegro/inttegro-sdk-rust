//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::MobileMoneyNetwork;
use serde::{Deserialize, Serialize};

/// Safe mobile-money details retained in refund history.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RefundSettlementMobileMoney {
    pub account_number: String,
    pub last4: String,
    pub network: MobileMoneyNetwork,
}
