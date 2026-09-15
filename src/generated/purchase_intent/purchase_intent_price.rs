//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{Amount, PurchaseIntentOriginalPrice};
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurchaseIntentPrice {
    pub active: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub nominal: Amount,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original: Option<PurchaseIntentOriginalPrice>,
}
