//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::RefundRequestMetaInput;
use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CancelRefundRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_meta: Option<RefundRequestMetaInput>,
    pub refund_id: String,
}
