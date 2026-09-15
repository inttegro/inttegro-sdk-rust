//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::UploadRequest;
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UploadRequestPage {
    pub number: i64,
    pub size: i64,
    pub upload_requests: Vec<UploadRequest>,
}
