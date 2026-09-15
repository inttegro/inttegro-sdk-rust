//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{RefundSettlementBankAccount, RefundSettlementMobileMoney};
use serde::{Deserialize, Serialize};

/// A safe snapshot of the original payment method used for the order.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum RefundSettlementPaymentMethod {
    MobileMoney {
        id: String,
        mobile_money: RefundSettlementMobileMoney,
    },
    BankAccount {
        id: String,
        bank_account: RefundSettlementBankAccount,
    },
}
