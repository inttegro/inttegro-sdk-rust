//! Generated, typed Inttegro domain or request value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentNextActionRedirectLatestVisit {
    pub user_agent: String,
    pub ip_address: String,
    pub at: crate::Timestamp,
}
