//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::MobileMoneyNetwork;
use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentMethodDataInputMobileMoney {
    pub network: MobileMoneyNetwork,
    pub account_number: String,
}
