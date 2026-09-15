//! Generated, typed Inttegro domain or request value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// A typed `MobileMoneyNetwork` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MobileMoneyNetwork {
    #[serde(rename = "airtel")]
    Airtel,
    #[serde(rename = "mtn")]
    Mtn,
    #[serde(rename = "telecel")]
    Telecel,
    #[serde(rename = "vodafone")]
    Vodafone,
}
