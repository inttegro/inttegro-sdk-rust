//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::PaymentMethodSnapshotGhanaBankAccount;
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentMethodSnapshotBankAccount {
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ghana_bank_account: Option<PaymentMethodSnapshotGhanaBankAccount>,
}
