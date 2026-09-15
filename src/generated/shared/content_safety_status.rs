//! Generated, typed Inttegro domain or request value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// A typed `ContentSafetyStatus` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContentSafetyStatus {
    #[serde(rename = "allowed")]
    Allowed,
    #[serde(rename = "rejected")]
    Rejected,
    #[serde(rename = "quarantined")]
    Quarantined,
}
