//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::PaymentNextActionRedirectLatestVisit;
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentNextActionRedirect {
    pub redirect_url: String,
    pub valid_until: crate::Timestamp,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latest_visit: Option<PaymentNextActionRedirectLatestVisit>,
}
