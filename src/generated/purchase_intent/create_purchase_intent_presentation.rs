//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::CreatePurchaseIntentPresentationBuyPage;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreatePurchaseIntentPresentation {
    pub buy_page: CreatePurchaseIntentPresentationBuyPage,
}
