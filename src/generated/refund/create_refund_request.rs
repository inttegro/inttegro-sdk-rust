//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{CreateRefundLineItemInput, CustomData, RefundReason, RefundRequestMetaInput};
use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateRefundRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason_details: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_meta: Option<RefundRequestMetaInput>,
    pub line_items: Vec<CreateRefundLineItemInput>,
    pub order_id: String,
    pub reason: RefundReason,
}
