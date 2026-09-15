//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{CreatePurchaseIntentRequestPriceOriginal, PriceParams};
use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreatePurchaseIntentRequestPrice {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nominal: Option<PriceParams>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original: Option<CreatePurchaseIntentRequestPriceOriginal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_id: Option<String>,
}
