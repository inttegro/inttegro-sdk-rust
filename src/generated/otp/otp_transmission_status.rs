//! Generated, typed Inttegro domain or request value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// A typed `OTPTransmissionStatus` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OTPTransmissionStatus {
    #[serde(rename = "delivered")]
    Delivered,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "submitted")]
    Submitted,
}
