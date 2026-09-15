//! Generated, typed Inttegro domain or request value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// A typed `ProductType` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProductType {
    #[serde(rename = "physical")]
    Physical,
    #[serde(rename = "digital")]
    Digital,
    #[serde(rename = "service")]
    Service,
    #[serde(rename = "voucher")]
    Voucher,
    #[serde(rename = "custom")]
    Custom,
    #[serde(rename = "cause")]
    Cause,
}
