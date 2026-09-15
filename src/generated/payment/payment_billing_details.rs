//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::PaymentMethodSnapshotOwner;
use serde::{Deserialize, Serialize};

/// Billing information captured for a payment.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentBillingDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<PaymentMethodSnapshotOwner>,
}
