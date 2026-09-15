//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::FileReferenceInput;
use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileReferenceReconcileRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub references: Option<Vec<FileReferenceInput>>,
    pub resource_type: String,
    pub resource_id: String,
}
