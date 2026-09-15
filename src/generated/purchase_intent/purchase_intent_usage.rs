//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::PurchaseIntentUsageOrder;
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurchaseIntentUsage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub multi_use: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<PurchaseIntentUsageOrder>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub single_use: Option<bool>,
}
