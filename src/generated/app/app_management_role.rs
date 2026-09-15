//! Generated, typed Inttegro domain or request value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// A typed `AppManagementRole` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AppManagementRole {
    #[serde(rename = "parent")]
    Parent,
    #[serde(rename = "child")]
    Child,
}
