//! Generated, typed Inttegro domain or request value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// A typed `ChimeRecipientType` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChimeRecipientType {
    #[serde(rename = "phone")]
    Phone,
    #[serde(rename = "email")]
    Email,
}
