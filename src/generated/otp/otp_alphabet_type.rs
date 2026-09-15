//! Generated, typed Inttegro domain or request value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// A typed `OTPAlphabetType` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OTPAlphabetType {
    #[serde(rename = "numeric")]
    Numeric,
    #[serde(rename = "alpha")]
    Alpha,
    #[serde(rename = "alphanumeric")]
    Alphanumeric,
}
