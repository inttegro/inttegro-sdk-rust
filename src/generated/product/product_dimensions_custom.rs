//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::ProductDimensionDetails;
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProductDimensionsCustom {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size_unit: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<ProductDimensionDetails>,
}
