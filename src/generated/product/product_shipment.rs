//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{
    ProductDelivery, ProductDownload, ProductRender, ProductService, ProductShipmentType,
    ProductStream,
};
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProductShipment {
    #[serde(rename = "type")]
    pub r#type: ProductShipmentType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delivery: Option<ProductDelivery>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub download: Option<ProductDownload>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub render: Option<ProductRender>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<ProductService>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stream: Option<ProductStream>,
}
