//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::ProductShipmentInputType;
use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProductShipmentInput {
    #[serde(rename = "type")]
    pub r#type: ProductShipmentInputType,
}
