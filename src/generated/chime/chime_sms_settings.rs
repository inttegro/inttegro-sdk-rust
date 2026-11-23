//! Effective SMS delivery settings.

use serde::{Deserialize, Serialize};

/// SMS sender defaults and public sender-length limits.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChimeSmsSettings {
    pub default_sender: String,
    pub min_sender_char_len: i64,
    pub max_sender_char_len: i64,
}
