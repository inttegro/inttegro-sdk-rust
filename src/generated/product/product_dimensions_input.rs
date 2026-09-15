//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{
    ProductDimensionsInputCustom, ProductDimensionsInputDigital, ProductDimensionsInputPhysical,
};
use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProductDimensionsInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub physical: Option<ProductDimensionsInputPhysical>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub digital: Option<ProductDimensionsInputDigital>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom: Option<ProductDimensionsInputCustom>,
}
