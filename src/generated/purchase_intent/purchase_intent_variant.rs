//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{PurchaseIntentPrice, PurchaseIntentProduct, VariantValues};
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurchaseIntentVariant {
    pub active: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub price: Option<PurchaseIntentPrice>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product: Option<PurchaseIntentProduct>,
    pub product_id: String,
    pub variant_values: VariantValues,
}
