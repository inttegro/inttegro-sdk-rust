//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::FinancialAccountPullConfigurationMandate;
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialAccountPullConfiguration {
    pub enabled_at: crate::Timestamp,
    pub mandate: FinancialAccountPullConfigurationMandate,
}
