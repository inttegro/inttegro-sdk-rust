//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::PurchaseIntent;
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurchaseIntentPage {
    pub number: i64,
    pub purchase_intents: Vec<PurchaseIntent>,
    pub size: i64,
}
