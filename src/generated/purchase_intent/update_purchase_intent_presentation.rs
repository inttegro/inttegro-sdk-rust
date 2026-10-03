//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::UpdatePurchaseIntentPresentationBuyPage;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdatePurchaseIntentPresentation {
    pub buy_page: UpdatePurchaseIntentPresentationBuyPage,
}
