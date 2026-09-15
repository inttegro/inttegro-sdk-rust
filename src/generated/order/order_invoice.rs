//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::OrderInvoiceFormat;
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderInvoice {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    pub format: OrderInvoiceFormat,
}
