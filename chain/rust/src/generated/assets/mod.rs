// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
pub mod cbor_encodings;
pub mod serialization;
// cddl-codegen extern re-export contract: this crate's hand-written root lib.rs must re-export
// each name below (`pub use <your_module>::<Name>;`) so the generated glue resolves against the
// user-owned definition. See the extern types section of docs/output_format.
pub use crate::Value;

use cbor_encodings::AssetNameEncoding;
use cml_core::error::*;
use cml_core::serialization::decode_canonical_hex;

/// Use TryFrom<&str> / TryInto<&str> for utf8 text conversion and RawBytesEncoding for direct bytes access
#[derive(Clone, Debug, derivative::Derivative)]
#[derivative(Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct AssetName {
    pub(crate) inner: Vec<u8>,
    #[derivative(
        PartialEq = "ignore",
        Ord = "ignore",
        PartialOrd = "ignore",
        Hash = "ignore"
    )]
    pub encodings: Option<AssetNameEncoding>,
}

impl AssetName {
    pub fn get(&self) -> &Vec<u8> {
        &self.inner
    }

    pub fn new(inner: Vec<u8>) -> Result<Self, DeserializeError> {
        if inner.len() > 32 {
            return Err(DeserializeError::new(
                "AssetName",
                DeserializeFailure::RangeCheck {
                    found: inner.len() as i128,
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
        decode_canonical_hex(&s)
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
    fn schema_name() -> alloc::borrow::Cow<'static, str> {
        alloc::borrow::Cow::Borrowed("AssetName")
    }

    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        <String as schemars::JsonSchema>::json_schema(generator)
    }

    fn inline_schema() -> bool {
        <String as schemars::JsonSchema>::inline_schema()
    }
}

pub type Coin = u64;

// cddl-codegen:replace-start
// Hand-defined as AssetBundle<NonZeroInt64> (checked-arithmetic API over the same map shape);
// generated code keeps handling it as a map through AssetBundle's Deref/DerefMut.
pub use crate::assets::utils::Mint;
// cddl-codegen:replaces
// pub type Mint = OrderedHashMap<PolicyId, OrderedHashMap<AssetName, NonZeroInt64>>;
// cddl-codegen:replace-end

// cddl-codegen:replace-start
// Hand-defined as AssetBundle<PositiveCoin> — same rationale as Mint above.
pub use crate::assets::utils::MultiAsset;
// cddl-codegen:replaces
// pub type MultiAsset = OrderedHashMap<PolicyId, OrderedHashMap<AssetName, u64>>;
// cddl-codegen:replace-end

/// Does not enforce != 0: plain i64 alias for API convenience. Could become a bounds-checked newtype later.
pub type NonZeroInt64 = i64;

/// Does not enforce > 0: plain u64 alias for API convenience. Could become a bounds-checked newtype later.
pub type PositiveCoin = u64;
