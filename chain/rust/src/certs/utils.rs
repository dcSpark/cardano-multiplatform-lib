use std::str::FromStr;

use super::{Ipv4, Ipv6, StakeCredential};
use cml_core::DeserializeError;
use cml_crypto::RawBytesEncoding;

impl StakeCredential {
    // we don't implement RawBytesEncoding as from_raw_bytes() would be unable to distinguish
    pub fn to_raw_bytes(&self) -> &[u8] {
        match self {
            Self::PubKey { hash, .. } => hash.to_raw_bytes(),
            Self::Script { hash, .. } => hash.to_raw_bytes(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum IPStringParsingError {
    #[error("Invalid IP Address String, expected period-separated bytes e.g. 0.0.0.0")]
    StringFormat,
    #[error("Deserializing from bytes: {0:?}")]
    DeserializeError(DeserializeError),
}

impl std::fmt::Display for Ipv4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.inner
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join(".")
        )
    }
}

impl FromStr for Ipv4 {
    type Err = IPStringParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split('.')
            .map(FromStr::from_str)
            .collect::<Result<Vec<u8>, _>>()
            .map_err(|_e| IPStringParsingError::StringFormat)
            .and_then(|bytes| Self::new(bytes).map_err(IPStringParsingError::DeserializeError))
    }
}

impl serde::Serialize for Ipv4 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> serde::de::Deserialize<'de> for Ipv4 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let s = <String as serde::de::Deserialize>::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_e| {
            serde::de::Error::invalid_value(serde::de::Unexpected::Str(&s), &"invalid ipv4 address")
        })
    }
}

impl schemars::JsonSchema for Ipv4 {
    fn schema_name() -> String {
        String::from("Ipv4")
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        String::json_schema(gen)
    }

    fn is_referenceable() -> bool {
        String::is_referenceable()
    }
}

impl std::fmt::Display for Ipv6 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.inner
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join(".")
        )
    }
}

impl FromStr for Ipv6 {
    type Err = IPStringParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split('.')
            .map(FromStr::from_str)
            .collect::<Result<Vec<u8>, _>>()
            .map_err(|_e| IPStringParsingError::StringFormat)
            .and_then(|bytes| Self::new(bytes).map_err(IPStringParsingError::DeserializeError))
    }
}

impl serde::Serialize for Ipv6 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> serde::de::Deserialize<'de> for Ipv6 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let s = <String as serde::de::Deserialize>::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_e| {
            serde::de::Error::invalid_value(serde::de::Unexpected::Str(&s), &"invalid ipv6 address")
        })
    }
}

impl schemars::JsonSchema for Ipv6 {
    fn schema_name() -> String {
        String::from("Ipv6")
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        String::json_schema(gen)
    }

    fn is_referenceable() -> bool {
        String::is_referenceable()
    }
}
