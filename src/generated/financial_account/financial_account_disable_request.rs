//! Generated, typed Inttegro domain or request value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialAccountDisableRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unset_as_payout_destination: Option<bool>,
    pub account_id: String,
}
