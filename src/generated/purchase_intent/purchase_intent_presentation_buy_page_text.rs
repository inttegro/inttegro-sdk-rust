//! Generated, typed Inttegro domain or request value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// Merchant-authored copy returned for a hosted Buy page.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurchaseIntentPresentationBuyPageText {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checkout_section_title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount_field_label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_action_label: Option<String>,
}
