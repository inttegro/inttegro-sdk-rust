//! Generated, typed Inttegro domain or request value. Do not edit manually.

use crate::CountryBankDirectory;
use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CountrySpecification {
    pub country_code: String,
    pub country_name: String,
    pub currencies: Vec<String>,
    pub payment_methods: Vec<String>,
    pub payout_schedules: Vec<String>,
    pub bt_aging_specs: Vec<String>,
    pub legal_entity_types: Vec<String>,
    pub financial_account_types: Vec<String>,
    pub id_document_types: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub banks: Option<CountryBankDirectory>,
}
