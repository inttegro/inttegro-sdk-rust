//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{PaymentAttemptError, PaymentAttemptStatus};
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentAttempt {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_method_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_method_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<PaymentAttemptError>,
    pub status: PaymentAttemptStatus,
    pub initiated_at: crate::Timestamp,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub succeeded_at: Option<crate::Timestamp>,
}
