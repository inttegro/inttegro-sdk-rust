//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{PurchaseIntentVariant, PurchaseIntentVariantAxis};
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurchaseIntentVariantSet {
    pub active: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_product_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub id: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    pub variant_axes: Vec<PurchaseIntentVariantAxis>,
    pub variants: Vec<PurchaseIntentVariant>,
}
