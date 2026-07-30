use alloc::string::String;
use cbor_event::{de::Deserializer, se::Serializer};
use cml_crypto::impl_hash_type;

use cml_core::error::{DeserializeError, DeserializeFailure};
use cml_core::serialization::Deserialize;
use cml_crypto::chain_crypto;
use cml_crypto::{CryptoError, RawBytesEncoding};
use schemars::JsonSchema;

impl_hash_type!(Blake2b224, 28);
impl_hash_type!(Blake2b256, 32);

// possibly replace later if we run into values not representable here
#[derive(Debug, Clone)]
pub struct ByronAny(cbor_event::Value);

// more methods on ByronAny to inspect it aren't offered as we don't encounter anything
// useful on-chain for this. It's either not present or is an empty array

impl PartialEq for ByronAny {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl Eq for ByronAny {}

impl PartialOrd for ByronAny {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// implement ord by serialized bytes since cbor_event::Value doesn't implement it
impl Ord for ByronAny {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let mut lhs_buf = cbor_event::se::Serializer::new_vec();
        cbor_event::se::Serialize::serialize(self, &mut lhs_buf).unwrap();
        let lhs_bytes = lhs_buf.finalize();
        let mut rhs_buf = cbor_event::se::Serializer::new_vec();
        cbor_event::se::Serialize::serialize(other, &mut rhs_buf).unwrap();
        let rhs_bytes = rhs_buf.finalize();
        lhs_bytes.cmp(&rhs_bytes)
    }
}

impl cbor_event::se::Serialize for ByronAny {
    fn serialize<'se>(
        &self,
        serializer: &'se mut Serializer,
    ) -> cbor_event::Result<&'se mut Serializer> {
        self.0.serialize(serializer)
    }
}

impl Deserialize for ByronAny {
    fn deserialize(raw: &mut Deserializer) -> Result<Self, DeserializeError> {
        <cbor_event::Value as cbor_event::Deserialize>::deserialize(raw)
            .map(Self)
            .map_err(Into::into)
    }
}

impl serde::Serialize for ByronAny {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut buf = cbor_event::se::Serializer::new_vec();
        cbor_event::se::Serialize::serialize(self, &mut buf).unwrap();
        let cbor_hex = hex::encode(buf.finalize());
        serializer.serialize_str(&cbor_hex)
    }
}

impl<'de> serde::de::Deserialize<'de> for ByronAny {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let cbor_hex = <String as serde::de::Deserialize>::deserialize(deserializer)?;
        let cbor = cml_core::hex_grammar::decode_canonical_hex(&cbor_hex).map_err(|_e| {
            serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(&cbor_hex),
                &"invalid hex string",
            )
        })?;
        // this probably will never fail as we're cbor's any
        // but we'll keep this for uncovered things e.g. floats
        cml_core::serialization::Deserialize::from_cbor_bytes(&cbor).map_err(|_e| {
            serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(&cbor_hex),
                &"cbor parsing failed. Unsupported?",
            )
        })
    }
}

impl JsonSchema for ByronAny {
    fn schema_name() -> alloc::borrow::Cow<'static, str> {
        alloc::borrow::Cow::Borrowed("any")
    }
    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        String::json_schema(generator)
    }
    fn inline_schema() -> bool {
        String::inline_schema()
    }
}

// The extern contract for byron_block / byron_tx (specs/multiera/lib.cddl) demands
// cml_core's Serialize. Byron's encoding is deterministic, so force_canonical is a no-op
// and we can delegate straight to the cbor_event impls.
impl cml_core::serialization::Serialize for super::block::ByronBlock {
    fn serialize<'a>(
        &self,
        serializer: &'a mut Serializer,
        _force_canonical: bool,
    ) -> cbor_event::Result<&'a mut Serializer> {
        cbor_event::se::Serialize::serialize(self, serializer)
    }
}

impl cml_core::serialization::Serialize for super::transaction::ByronTx {
    fn serialize<'a>(
        &self,
        serializer: &'a mut Serializer,
        _force_canonical: bool,
    ) -> cbor_event::Result<&'a mut Serializer> {
        cbor_event::se::Serialize::serialize(self, serializer)
    }
}
