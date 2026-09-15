//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::Customer;
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomerPage {
    pub customers: Vec<Customer>,
    pub number: i64,
    pub size: i64,
}
