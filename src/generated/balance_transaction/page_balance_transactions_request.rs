//! Generated, typed Inttegro domain or request value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PageBalanceTransactionsRequest {
    pub page_number: i64,
    pub page_size: i64,
}
