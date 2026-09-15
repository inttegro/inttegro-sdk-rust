//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{SecretKey, SecretKeyUsagePage};
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecretKeyUsage {
    pub key: SecretKey,
    pub usage: SecretKeyUsagePage,
}
