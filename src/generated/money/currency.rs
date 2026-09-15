//! Generated, typed Inttegro domain or request value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// A typed `Currency` value used by the Inttegro API.
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Currency {
    #[serde(rename = "ghs")]
    GHS,
    #[serde(rename = "usd")]
    USD,
    #[serde(rename = "gbp")]
    GBP,
    #[serde(rename = "eur")]
    EUR,
    #[serde(rename = "cny")]
    CNY,
}
