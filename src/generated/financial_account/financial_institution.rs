//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::{FinancialInstitutionBank, FinancialInstitutionMobileMoneyProvider};
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialInstitution {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank: Option<FinancialInstitutionBank>,
    pub country: String,
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile_money_provider: Option<FinancialInstitutionMobileMoneyProvider>,
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: String,
}
