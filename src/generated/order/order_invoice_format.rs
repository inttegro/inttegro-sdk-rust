//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::OrderDocumentFormat;
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderInvoiceFormat {
    pub web: OrderDocumentFormat,
    pub pdf: OrderDocumentFormat,
}
