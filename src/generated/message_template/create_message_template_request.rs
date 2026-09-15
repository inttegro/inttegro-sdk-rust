//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{CreateEmailMessageTemplateRequest, CreateSMSMessageTemplateRequest};
use serde::{Deserialize, Serialize};

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateMessageTemplateRequest {
    CreateSMSMessageTemplateRequest(CreateSMSMessageTemplateRequest),
    CreateEmailMessageTemplateRequest(CreateEmailMessageTemplateRequest),
}
