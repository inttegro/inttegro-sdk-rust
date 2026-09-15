//! Generated, typed Inttegro domain or request value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CatalogProductWithPriceReferenceInput {
    pub price_id: String,
    pub product_id: String,
    pub quantity: i64,
}
