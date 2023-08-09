use std::io::{BufRead, Seek, Write};

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
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// implement ord by serialized bytes since cbor_event::Value doesn't implement it
impl Ord for ByronAny {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
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
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.0.serialize(serializer)
    }
}

impl Deserialize for ByronAny {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        use cbor_event::Deserialize;
        cbor_event::Value::deserialize(raw)
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
        let cbor = hex::decode(&cbor_hex).map_err(|_e| {
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
    fn schema_name() -> String {
        String::from("any")
    }
    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        String::json_schema(gen)
    }
    fn is_referenceable() -> bool {
        String::is_referenceable()
    }
}
