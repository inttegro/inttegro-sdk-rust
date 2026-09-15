//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{ReviewUploadRequestAttemptByIDRequest, ReviewUploadRequestAttemptByOrdinalRequest};
use serde::{Deserialize, Serialize};

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReviewUploadRequestAttemptRequest {
    ReviewUploadRequestAttemptByIDRequest(ReviewUploadRequestAttemptByIDRequest),
    ReviewUploadRequestAttemptByOrdinalRequest(ReviewUploadRequestAttemptByOrdinalRequest),
}
