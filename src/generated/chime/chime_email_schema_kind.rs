//! Generated, typed Inttegro domain or request value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// A typed `ChimeEmailSchemaKind` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChimeEmailSchemaKind {
    #[serde(rename = "gmail_view_action")]
    GmailViewAction,
    #[serde(rename = "schema_org_order")]
    SchemaOrgOrder,
    #[serde(rename = "schema_org_invoice")]
    SchemaOrgInvoice,
}
