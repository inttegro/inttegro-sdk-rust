//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::PayoutConfigurationDestination;
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PayoutConfiguration {
    pub enable_fx: bool,
    pub destination: PayoutConfigurationDestination,
}
