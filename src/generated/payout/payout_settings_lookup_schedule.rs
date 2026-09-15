//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::PayoutSettingsLookupScheduleAgingSpec;
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PayoutSettingsLookupSchedule {
    pub aging_spec: PayoutSettingsLookupScheduleAgingSpec,
    pub description: String,
    pub interval: String,
    pub name: String,
    pub schedule_on: String,
    #[serde(rename = "type")]
    pub r#type: String,
}
