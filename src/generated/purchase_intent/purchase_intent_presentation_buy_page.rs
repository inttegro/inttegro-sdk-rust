//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::PurchaseIntentPresentationBuyPageText;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurchaseIntentPresentationBuyPage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<PurchaseIntentPresentationBuyPageText>,
}
