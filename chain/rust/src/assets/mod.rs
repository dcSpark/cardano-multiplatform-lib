// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

pub mod cbor_encodings;
pub mod serialization;
pub mod utils;

pub use utils::*; //{Value, AssetBundle, Mint, MultiAsset, Coin, PositiveCoin, NonZeroInt64};

use cbor_encodings::AssetNameEncoding;
use cml_core::error::*;

use std::convert::TryFrom;

/// Use TryFrom<&str> / TryInto<&str> for utf8 text conversion and RawBytesEncoding for direct bytes access
#[derive(Clone, Debug, derivative::Derivative)]
#[derivative(Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct AssetName {
    pub inner: Vec<u8>,
    #[derivative(
        PartialEq = "ignore",
        Ord = "ignore",
        PartialOrd = "ignore",
        Hash = "ignore"
    )]
    pub encodings: Option<AssetNameEncoding>,
}

impl AssetName {
    pub fn new(inner: Vec<u8>) -> Result<Self, DeserializeError> {
        if inner.len() > 32 {
            return Err(DeserializeError::new(
                "AssetName",
                DeserializeFailure::RangeCheck {
                    found: inner.len() as isize,
                    min: Some(0),
                    max: Some(32),
                },
            ));
        }
        Ok(Self {
            inner,
            encodings: None,
        })
    }
}

impl TryFrom<Vec<u8>> for AssetName {
    type Error = DeserializeError;

    fn try_from(inner: Vec<u8>) -> Result<Self, Self::Error> {
        AssetName::new(inner)
    }
}

impl From<AssetName> for Vec<u8> {
    fn from(wrapper: AssetName) -> Self {
        wrapper.inner
    }
}

impl serde::Serialize for AssetName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&hex::encode(self.inner.clone()))
    }
}

impl<'de> serde::de::Deserialize<'de> for AssetName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let s = <String as serde::de::Deserialize>::deserialize(deserializer)?;
        hex::decode(&s)
            .ok()
            .and_then(|bytes| AssetName::new(bytes).ok())
            .ok_or_else(|| {
                serde::de::Error::invalid_value(
                    serde::de::Unexpected::Str(&s),
                    &"invalid hex bytes",
                )
            })
    }
}

impl schemars::JsonSchema for AssetName {
    fn schema_name() -> String {
        String::from("AssetName")
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        String::json_schema(gen)
    }

    fn is_referenceable() -> bool {
        String::is_referenceable()
    }
}
