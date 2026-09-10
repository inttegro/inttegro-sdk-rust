use serde::{Deserialize, Serialize};

/// The application's latest GHS balance snapshot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BalanceSnapshot {
    pub ghs: crate::CurrencyBalanceSnapshot,
}
