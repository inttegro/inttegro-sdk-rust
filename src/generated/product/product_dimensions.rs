//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{ProductDimensionsCustom, ProductDimensionsDigital, ProductDimensionsPhysical};
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProductDimensions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub physical: Option<ProductDimensionsPhysical>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub digital: Option<ProductDimensionsDigital>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom: Option<ProductDimensionsCustom>,
}
