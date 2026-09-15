//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{
    CreatePurchaseIntentRequestPrice, CreatePurchaseIntentRequestProduct,
    CreatePurchaseIntentRequestQuantity, CreatePurchaseIntentRequestUsage,
};
use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreatePurchaseIntentRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product: Option<CreatePurchaseIntentRequestProduct>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub price: Option<CreatePurchaseIntentRequestPrice>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub price_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage: Option<CreatePurchaseIntentRequestUsage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<crate::Timestamp>,
    pub quantity: CreatePurchaseIntentRequestQuantity,
}
