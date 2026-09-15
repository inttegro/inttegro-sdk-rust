//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{ChimeRecipientEmail, ChimeRecipientPhone, ChimeRecipientType};
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChimeRecipient {
    #[serde(rename = "type")]
    pub r#type: ChimeRecipientType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<ChimeRecipientPhone>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<ChimeRecipientEmail>,
}
