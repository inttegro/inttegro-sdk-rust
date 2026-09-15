//! Generated, typed Inttegro domain or request value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// A typed `MessageTemplateVariableType` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MessageTemplateVariableType {
    #[serde(rename = "string")]
    String,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "url")]
    Url,
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "phone")]
    Phone,
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "datetime")]
    Datetime,
    #[serde(rename = "array")]
    Array,
}
