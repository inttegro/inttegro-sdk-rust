//! Generated, typed Inttegro domain or request value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PayoutSettingsMutationScheduleSpec {
    pub abide: String,
    pub id: String,
    pub label: String,
    pub t_plus: String,
}
