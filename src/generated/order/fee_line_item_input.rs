//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{FeeDetailsInput, LineItemType};
use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeeLineItemInput {
    #[serde(rename = "type")]
    pub r#type: LineItemType,
    pub fee: FeeDetailsInput,
}
