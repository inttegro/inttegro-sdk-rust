//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{UploadRequestReviewReasonInput, UploadReviewDecision};
use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReviewUploadRequestAttemptByIDRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public_message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reasons: Option<Vec<UploadRequestReviewReasonInput>>,
    pub attempt_id: String,
    pub decision: UploadReviewDecision,
    pub id: String,
}
