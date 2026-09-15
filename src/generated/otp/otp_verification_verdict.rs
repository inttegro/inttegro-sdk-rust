//! Generated, typed Inttegro domain or request value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// A typed `OTPVerificationVerdict` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OTPVerificationVerdict {
    #[serde(rename = "fail")]
    Fail,
    #[serde(rename = "pass")]
    Pass,
}
