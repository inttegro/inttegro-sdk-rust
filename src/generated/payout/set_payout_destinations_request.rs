//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::PayoutDestinations;
use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetPayoutDestinationsRequest {
    pub destinations: PayoutDestinations,
}
