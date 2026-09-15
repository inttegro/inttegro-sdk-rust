//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::Amount;
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderFeeLineItemFee {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,
    pub amount: Amount,
    pub label: String,
}
