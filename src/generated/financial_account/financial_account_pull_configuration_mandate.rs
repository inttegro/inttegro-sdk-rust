//! Generated, typed Inttegro domain or request value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialAccountPullConfigurationMandate {
    pub created_at: crate::Timestamp,
    pub id: String,
    pub ip_address: String,
    pub user_agent: String,
}
