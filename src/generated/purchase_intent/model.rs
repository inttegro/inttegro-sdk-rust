//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{
    PurchaseIntentMerchant, PurchaseIntentPrice, PurchaseIntentProduct, PurchaseIntentQuantity,
    PurchaseIntentStatus, PurchaseIntentUsage, PurchaseIntentVariantSet,
};
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurchaseIntent {
    pub allow_variants: bool,
    pub created_at: crate::Timestamp,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<crate::Timestamp>,
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inactive_at: Option<crate::Timestamp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merchant: Option<PurchaseIntentMerchant>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub price: Option<PurchaseIntentPrice>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product: Option<PurchaseIntentProduct>,
    pub quantity: PurchaseIntentQuantity,
    pub status: PurchaseIntentStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<crate::Timestamp>,
    pub usage: PurchaseIntentUsage,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variant_set: Option<PurchaseIntentVariantSet>,
}
