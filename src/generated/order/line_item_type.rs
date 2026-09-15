//! Generated, typed Inttegro domain or request value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// A typed `LineItemType` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LineItemType {
    #[serde(rename = "product")]
    Product,
    #[serde(rename = "fee")]
    Fee,
    #[serde(rename = "shipping")]
    Shipping,
}
