//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{OrderDocumentDeliveryAttempt, OrderDocumentDeliveryFailure, OrderDocumentKind};
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderDocumentDelivery {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deliveries: Option<Vec<OrderDocumentDeliveryAttempt>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_kind: Option<OrderDocumentKind>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failed_channels: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<OrderDocumentDeliveryFailure>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sent_channels: Option<Vec<String>>,
}
