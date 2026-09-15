//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{ChimeInlineRecipientInputVariant1, ChimeInlineRecipientInputVariant2};
use serde::{Deserialize, Serialize};

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChimeInlineRecipientInput {
    ChimeInlineRecipientInputVariant1(ChimeInlineRecipientInputVariant1),
    ChimeInlineRecipientInputVariant2(ChimeInlineRecipientInputVariant2),
}
