//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::PurchaseIntentPresentationBuyPage;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurchaseIntentPresentation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buy_page: Option<PurchaseIntentPresentationBuyPage>,
}
