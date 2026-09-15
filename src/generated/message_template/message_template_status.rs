//! Generated, typed Inttegro domain or request value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// A typed `MessageTemplateStatus` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MessageTemplateStatus {
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "published")]
    Published,
    #[serde(rename = "archived")]
    Archived,
}
