//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{PayoutDestinations, PayoutSettingsMutationSchedule};
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PayoutSettingsMutation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destinations: Option<PayoutDestinations>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fx_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<PayoutSettingsMutationSchedule>,
}
