//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::RefundSettlementPaymentMethod;
use serde::{Deserialize, Serialize};

/// Immutable settlement evidence captured when a refund is created.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum RefundSettlement {
    Offline,
    PaymentMethod {
        payment_method: RefundSettlementPaymentMethod,
    },
}
