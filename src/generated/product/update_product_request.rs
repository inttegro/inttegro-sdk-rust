//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{
    CustomData, ProductAttributeInput, ProductDimensionsInput, ProductMediaInput,
    ProductShipmentInput, ProductType,
};
use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateProductRequest {
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ProductType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub about: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shipment: Option<ProductShipmentInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<ProductDimensionsInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit_dimension: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub media: Option<ProductMediaInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<ProductAttributeInput>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomData>,
    pub product_id: String,
}
