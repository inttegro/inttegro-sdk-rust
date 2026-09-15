//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::UpdatePurchaseIntentRequestQuantity;
use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdatePurchaseIntentRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<crate::Timestamp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<UpdatePurchaseIntentRequestQuantity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub purchase_intent_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reactivate: Option<bool>,
}
