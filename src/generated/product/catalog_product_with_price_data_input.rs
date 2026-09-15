//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::PriceParams;
use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CatalogProductWithPriceDataInput {
    pub price: PriceParams,
    pub product_id: String,
    pub quantity: i64,
}
