//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{FileUploadReceipt, UploadRequest};
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UploadFulfillment {
    pub upload_request: UploadRequest,
    pub file: FileUploadReceipt,
}
