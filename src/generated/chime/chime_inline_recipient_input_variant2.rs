//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{ChimeInlineRecipientInputVariant2Email, ChimeRecipientType};
use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChimeInlineRecipientInputVariant2 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub email: ChimeInlineRecipientInputVariant2Email,
    #[serde(rename = "type")]
    pub r#type: ChimeRecipientType,
}
