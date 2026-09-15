//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{
    CustomData, ProductAttribute, ProductDimensions, ProductMedia, ProductPriceSummary,
    ProductShipment, ProductType,
};
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Product {
    pub id: String,
    #[serde(rename = "type")]
    pub r#type: ProductType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub about: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prices: Option<Vec<ProductPriceSummary>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shipment: Option<ProductShipment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub media: Option<ProductMedia>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<ProductAttribute>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<ProductDimensions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomData>,
    pub active: bool,
    pub created_at: crate::Timestamp,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<crate::Timestamp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archived_at: Option<crate::Timestamp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub published_at: Option<crate::Timestamp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit_dim: Option<String>,
}
