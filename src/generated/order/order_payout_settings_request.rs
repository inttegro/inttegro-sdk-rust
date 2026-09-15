//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::OrderPayoutSettingsRequestDestination;
use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderPayoutSettingsRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination: Option<OrderPayoutSettingsRequestDestination>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable_fx: Option<bool>,
}
