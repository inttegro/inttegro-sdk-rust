//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{LineItemType, ShippingDetailsInput};
use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShippingLineItemInput {
    #[serde(rename = "type")]
    pub r#type: LineItemType,
    pub shipping: ShippingDetailsInput,
}
