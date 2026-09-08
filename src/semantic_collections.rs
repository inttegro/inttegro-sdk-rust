use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

const CUSTOM_DATA_MAX_KEY_BYTES: usize = 256;
const CUSTOM_DATA_MAX_BYTES: usize = 25 * 1024;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SemanticCollectionError {
    #[error("custom-data key must be at most 256 UTF-8 bytes: {0}")]
    CustomDataKeyTooLong(String),
    #[error("custom data must encode to at most 25 KiB")]
    CustomDataTooLarge,
}

/// Merchant-defined string values with validated, controlled mutation.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
#[serde(transparent)]
pub struct CustomData(BTreeMap<String, String>);

impl CustomData {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn try_from_values(
        values: impl IntoIterator<Item = (String, String)>,
    ) -> Result<Self, SemanticCollectionError> {
        let value = Self(values.into_iter().collect());
        value.validate()?;
        Ok(value)
    }

    pub fn insert(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Option<String>, SemanticCollectionError> {
        let key = key.into();
        let previous = self.0.insert(key.clone(), value.into());
        if let Err(error) = self.validate() {
            match previous.as_ref() {
                Some(previous) => {
                    self.0.insert(key, previous.clone());
                }
                None => {
                    self.0.remove(&key);
                }
            }
            return Err(error);
        }
        Ok(previous)
    }

    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.0.remove(key)
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).map(String::as_str)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &String)> {
        self.0.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    fn validate(&self) -> Result<(), SemanticCollectionError> {
        if let Some(key) = self
            .0
            .keys()
            .find(|key| key.as_bytes().len() > CUSTOM_DATA_MAX_KEY_BYTES)
        {
            return Err(SemanticCollectionError::CustomDataKeyTooLong(key.clone()));
        }
        if serde_json::to_vec(&self.0)
            .expect("string maps are always JSON encodable")
            .len()
            > CUSTOM_DATA_MAX_BYTES
        {
            return Err(SemanticCollectionError::CustomDataTooLarge);
        }
        Ok(())
    }
}

impl<'de> Deserialize<'de> for CustomData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Self(BTreeMap::deserialize(deserializer)?);
        value.validate().map_err(D::Error::custom)?;
        Ok(value)
    }
}

/// Merchant-defined JSON values accepted by create operations.
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
#[serde(transparent)]
pub struct CustomDataInput(BTreeMap<String, Value>);

impl CustomDataInput {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(
        &mut self,
        key: impl Into<String>,
        value: impl Serialize,
    ) -> Result<Option<Value>, SemanticCollectionError> {
        let key = key.into();
        let value = serde_json::to_value(value).expect("serializable values produce JSON");
        let previous = self.0.insert(key.clone(), value);
        if let Err(error) = validate_json_custom_data(&self.0) {
            match previous.as_ref() {
                Some(previous) => {
                    self.0.insert(key, previous.clone());
                }
                None => {
                    self.0.remove(&key);
                }
            }
            return Err(error);
        }
        Ok(previous)
    }

    pub fn remove(&mut self, key: &str) -> Option<Value> {
        self.0.remove(key)
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.0.get(key)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &Value)> {
        self.0.iter()
    }
}

impl<'de> Deserialize<'de> for CustomDataInput {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Self(BTreeMap::deserialize(deserializer)?);
        validate_json_custom_data(&value.0).map_err(D::Error::custom)?;
        Ok(value)
    }
}

/// A custom-data update. JSON `null` explicitly removes a key.
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
#[serde(transparent)]
pub struct CustomDataPatch(BTreeMap<String, Value>);

impl CustomDataPatch {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(
        &mut self,
        key: impl Into<String>,
        value: impl Serialize,
    ) -> Result<Option<Value>, SemanticCollectionError> {
        let key = key.into();
        let value = serde_json::to_value(value).expect("serializable values produce JSON");
        self.insert_validated(key, value)
    }

    pub fn unset(
        &mut self,
        key: impl Into<String>,
    ) -> Result<Option<Value>, SemanticCollectionError> {
        self.insert_validated(key.into(), Value::Null)
    }

    pub fn remove_change(&mut self, key: &str) -> Option<Value> {
        self.0.remove(key)
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.0.get(key)
    }

    fn insert_validated(
        &mut self,
        key: String,
        value: Value,
    ) -> Result<Option<Value>, SemanticCollectionError> {
        let previous = self.0.insert(key.clone(), value);
        if let Err(error) = validate_json_custom_data(&self.0) {
            match previous.as_ref() {
                Some(previous) => {
                    self.0.insert(key, previous.clone());
                }
                None => {
                    self.0.remove(&key);
                }
            }
            return Err(error);
        }
        Ok(previous)
    }
}

impl<'de> Deserialize<'de> for CustomDataPatch {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Self(BTreeMap::deserialize(deserializer)?);
        validate_json_custom_data(&value.0).map_err(D::Error::custom)?;
        Ok(value)
    }
}

fn validate_json_custom_data(
    values: &BTreeMap<String, Value>,
) -> Result<(), SemanticCollectionError> {
    if let Some(key) = values
        .keys()
        .find(|key| key.as_bytes().len() > CUSTOM_DATA_MAX_KEY_BYTES)
    {
        return Err(SemanticCollectionError::CustomDataKeyTooLong(key.clone()));
    }
    if serde_json::to_vec(values)
        .expect("JSON values are always JSON encodable")
        .len()
        > CUSTOM_DATA_MAX_BYTES
    {
        return Err(SemanticCollectionError::CustomDataTooLarge);
    }
    Ok(())
}

macro_rules! string_collection {
    ($name:ident, $description:literal) => {
        #[doc = $description]
        #[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(BTreeMap<String, String>);

        impl $name {
            pub fn new() -> Self {
                Self::default()
            }

            pub fn insert(
                &mut self,
                key: impl Into<String>,
                value: impl Into<String>,
            ) -> Option<String> {
                self.0.insert(key.into(), value.into())
            }

            pub fn remove(&mut self, key: &str) -> Option<String> {
                self.0.remove(key)
            }

            pub fn get(&self, key: &str) -> Option<&str> {
                self.0.get(key).map(String::as_str)
            }

            pub fn iter(&self) -> impl Iterator<Item = (&String, &String)> {
                self.0.iter()
            }
        }
    };
}

string_collection!(
    FileMetadata,
    "Metadata attached to a file or upload request."
);
string_collection!(MessageHeaders, "Message headers keyed by header name.");
string_collection!(
    PayoutDestinations,
    "Payout destinations keyed by currency or configured route."
);
string_collection!(
    ProductDimensionDetails,
    "Custom product-dimension details keyed by attribute name."
);
string_collection!(
    VariantValues,
    "Selected product variant values keyed by attribute name."
);

/// An intentionally open JSON object whose schema belongs to an external system.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct JsonData(BTreeMap<String, Value>);

impl JsonData {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, key: impl Into<String>, value: impl Serialize) -> Option<Value> {
        self.0.insert(
            key.into(),
            serde_json::to_value(value).expect("serializable values produce JSON"),
        )
    }

    pub fn remove(&mut self, key: &str) -> Option<Value> {
        self.0.remove(key)
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.0.get(key)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &Value)> {
        self.0.iter()
    }
}

/// A Dosh financial account. The API v1 representation is intentionally empty.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct DoshAccount {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FinancialAccountVerificationRequest {
    pub id: Option<String>,
    pub mechanism: Option<String>,
    #[serde(rename = "type")]
    pub kind: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FinancialAccountVerification {
    pub initiated_at: String,
    pub completed_at: Option<String>,
    pub request: FinancialAccountVerificationRequest,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrderPayoutDestination {
    pub financial_account_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrderPayoutSettings {
    pub destination: Option<OrderPayoutDestination>,
    pub enable_fx: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Address {
    pub name: String,
    pub phone_number: String,
    pub line1: String,
    pub line2: Option<String>,
    pub town: String,
    pub region: Option<String>,
    pub district: Option<String>,
    pub country: String,
    pub post_code: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Shipping {
    pub address: Address,
}

/// Account balances keyed by currency.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BalanceSnapshot(BTreeMap<String, crate::CurrencyBalanceSnapshot>);

impl BalanceSnapshot {
    pub fn get(&self, currency: &str) -> Option<&crate::CurrencyBalanceSnapshot> {
        self.0.get(currency)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &crate::CurrencyBalanceSnapshot)> {
        self.0.iter()
    }
}

/// Customer balances keyed by currency.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CustomerBalance(BTreeMap<String, crate::CustomerBalanceValue>);

impl CustomerBalance {
    pub fn get(&self, currency: &str) -> Option<&crate::CustomerBalanceValue> {
        self.0.get(currency)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &crate::CustomerBalanceValue)> {
        self.0.iter()
    }
}

/// Country capabilities keyed by lowercase country code.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CountrySpecifications(BTreeMap<String, crate::CountrySpecification>);

impl CountrySpecifications {
    pub fn get(&self, country_code: &str) -> Option<&crate::CountrySpecification> {
        self.0.get(country_code)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &crate::CountrySpecification)> {
        self.0.iter()
    }
}
