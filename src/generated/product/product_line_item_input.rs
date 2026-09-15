//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{LineItemType, ProductLineItemInputProduct};
use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProductLineItemInput {
    #[serde(rename = "type")]
    pub r#type: LineItemType,
    pub product: ProductLineItemInputProduct,
}
