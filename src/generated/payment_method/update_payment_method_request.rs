//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{CustomDataPatch, UpdatePaymentMethodRequestOwner};
use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdatePaymentMethodRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataPatch>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<UpdatePaymentMethodRequestOwner>,
    pub payment_method_id: String,
}
