//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::DeliveryChannel;
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderDocumentDeliveryAttempt {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel: Option<DeliveryChannel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chime_id: Option<String>,
}
