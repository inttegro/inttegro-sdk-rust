//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{UploadRequestReviewReason, UploadReviewDecision, UploadReviewType};
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UploadRequestReview {
    pub created_at: crate::Timestamp,
    pub decision: UploadReviewDecision,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public_message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reasons: Option<Vec<UploadRequestReviewReason>>,
    pub reviewed_at: crate::Timestamp,
    #[serde(rename = "type")]
    pub r#type: UploadReviewType,
}
