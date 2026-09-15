//! Generated, typed Inttegro domain or request value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// Public payment error information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentError {
    pub message: String,
}
