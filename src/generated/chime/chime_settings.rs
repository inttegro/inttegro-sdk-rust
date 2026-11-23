//! Effective, non-secret Chime delivery settings.

use crate::{ChimeEmailSettings, ChimeSmsSettings};
use serde::{Deserialize, Serialize};

/// Typed SMS and email settings used by Chimes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChimeSettings {
    pub sms: ChimeSmsSettings,
    pub email: ChimeEmailSettings,
}
