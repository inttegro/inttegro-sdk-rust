use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

/// An offset-aware timestamp decoded from the API's RFC 3339 wire format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Timestamp(OffsetDateTime);

impl Timestamp {
    pub const fn new(value: OffsetDateTime) -> Self {
        Self(value)
    }

    pub const fn into_inner(self) -> OffsetDateTime {
        self.0
    }
}

impl Deref for Timestamp {
    type Target = OffsetDateTime;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<OffsetDateTime> for Timestamp {
    fn from(value: OffsetDateTime) -> Self {
        Self(value)
    }
}

impl From<Timestamp> for OffsetDateTime {
    fn from(value: Timestamp) -> Self {
        value.0
    }
}

impl FromStr for Timestamp {
    type Err = time::error::Parse;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        OffsetDateTime::parse(value, &Rfc3339).map(Self)
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = self.0.format(&Rfc3339).map_err(|_| fmt::Error)?;
        formatter.write_str(&value)
    }
}

impl Serialize for Timestamp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Timestamp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        value.parse().map_err(serde::de::Error::custom)
    }
}
