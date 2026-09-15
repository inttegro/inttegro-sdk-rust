//! Generated, typed Inttegro domain or request value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// A typed `OrderCreatedFromResourceType` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OrderCreatedFromResourceType {
    #[serde(rename = "purchase_intent")]
    PurchaseIntent,
}
