//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{CustomDataInput, PriceParams, ProductType};
use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineProductDetailsInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub about: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,
    pub name: String,
    pub price: PriceParams,
    pub quantity: i64,
    #[serde(rename = "type")]
    pub r#type: ProductType,
}
