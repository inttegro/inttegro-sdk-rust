//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::ChimeTransport;
use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChimeSavedCustomerRecipientInput {
    pub customer_id: String,
    pub transport: ChimeTransport,
}
