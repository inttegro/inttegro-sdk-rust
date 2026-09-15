//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{
    CustomData, PaymentMethodOwnerInput, PaymentMethodType,
    TokenizeMobileMoneyPaymentMethodRequestMobileMoney,
};
use serde::{Deserialize, Serialize};

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TokenizeMobileMoneyPaymentMethodRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomData>,
    pub customer_id: String,
    #[serde(rename = "type")]
    pub r#type: PaymentMethodType,
    pub mobile_money: TokenizeMobileMoneyPaymentMethodRequestMobileMoney,
    pub owner: PaymentMethodOwnerInput,
}
