//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::OrderDocumentFormat;
use serde::{Deserialize, Serialize};

/// Hosted document formats for a payment receipt.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderReceiptFormat {
    pub web: OrderDocumentFormat,
    pub pdf: OrderDocumentFormat,
}

/// Receipt issued for a paid order payment.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderReceipt {
    pub format: OrderReceiptFormat,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delivery_summary: Option<serde_json::Value>,
}
