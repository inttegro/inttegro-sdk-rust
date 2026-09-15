//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{PaymentMethodType, UpdateOrderRequestPaymentMethodDataMobileMoney};
use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateOrderRequestPaymentMethodData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile_money: Option<UpdateOrderRequestPaymentMethodDataMobileMoney>,
    #[serde(rename = "type")]
    pub r#type: PaymentMethodType,
}
