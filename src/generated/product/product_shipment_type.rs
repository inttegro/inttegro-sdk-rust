//! Generated, typed Inttegro domain or request value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// A typed `ProductShipmentType` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProductShipmentType {
    #[serde(rename = "delivery")]
    Delivery,
    #[serde(rename = "download")]
    Download,
    #[serde(rename = "render")]
    Render,
    #[serde(rename = "service")]
    Service,
    #[serde(rename = "stream")]
    Stream,
}
