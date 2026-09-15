//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{ChimeEmailMessage, ChimeRecipient, ChimeTransmission, CustomData};
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Chime {
    pub created_at: crate::Timestamp,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<ChimeEmailMessage>,
    pub full_message: String,
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
    pub recipient: ChimeRecipient,
    pub sender_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transmission: Option<ChimeTransmission>,
}
