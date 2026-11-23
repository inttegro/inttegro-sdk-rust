//! Effective email delivery settings.

use serde::{Deserialize, Serialize};

/// Email sender defaults and authorized From addresses.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChimeEmailSettings {
    pub default_from: String,
    pub allowed_from: Vec<String>,
}
