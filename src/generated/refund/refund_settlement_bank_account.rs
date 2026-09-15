//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::RefundSettlementGhanaBankAccount;
use serde::{Deserialize, Serialize};

/// Safe bank-account details retained in refund history.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum RefundSettlementBankAccount {
    GhanaBankAccount {
        ghana_bank_account: RefundSettlementGhanaBankAccount,
    },
}
