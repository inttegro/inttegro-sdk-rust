//! Public delivery lifecycle for a Chime.

use crate::ChimeTransport;
use serde::{Deserialize, Serialize};

/// Public delivery lifecycle for a Chime.
///
/// Serde ignores undeclared response fields.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChimeTransmission {
    pub address: String,
    pub created_at: crate::Timestamp,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delivered_at: Option<crate::Timestamp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failed_at: Option<crate::Timestamp>,
    pub initialized_at: crate::Timestamp,
    pub mechanism: ChimeTransport,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sent_at: Option<crate::Timestamp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sent_via: Option<ChimeTransport>,
    pub status: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suppressed_at: Option<crate::Timestamp>,
}
