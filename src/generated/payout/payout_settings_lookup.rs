//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{PayoutDestinations, PayoutSettingsLookupSchedule};
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PayoutSettingsLookup {
    pub destinations: PayoutDestinations,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fx_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<PayoutSettingsLookupSchedule>,
}
