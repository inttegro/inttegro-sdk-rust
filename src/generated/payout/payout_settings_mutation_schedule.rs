//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::PayoutSettingsMutationScheduleSpec;
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PayoutSettingsMutationSchedule {
    pub description: String,
    pub id: String,
    pub interval: String,
    pub name: String,
    pub schedule_on: String,
    pub spec: PayoutSettingsMutationScheduleSpec,
    #[serde(rename = "type")]
    pub r#type: String,
}
