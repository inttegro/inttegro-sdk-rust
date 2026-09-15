//! Generated, typed Inttegro domain or request value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// A typed `FileDelivery` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FileDelivery {
    #[serde(rename = "stream")]
    Stream,
    #[serde(rename = "redirect")]
    Redirect,
}
