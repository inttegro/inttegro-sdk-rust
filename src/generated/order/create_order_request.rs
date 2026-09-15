//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{CreateOrderExistingCustomerInput, CreateOrderNewCustomerInput};
use serde::{Deserialize, Serialize};

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateOrderRequest {
    CreateOrderNewCustomerInput(CreateOrderNewCustomerInput),
    CreateOrderExistingCustomerInput(CreateOrderExistingCustomerInput),
}
