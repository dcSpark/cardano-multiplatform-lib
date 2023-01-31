use super::{CryptoError, RawBytesEncoding};
/// This is not for concrete implementations for on-chain things (see chain crate for that)
/// but instead for use there and in other CDDL-defined crates that have bytes that represent
/// crypto primitives.
use cbor_event::{self, de::Deserializer, se::Serializer};
use cml_core::{
    error::{DeserializeError, DeserializeFailure},
    serialization::{Deserialize, Serialize, StringEncoding},
};
use derivative::Derivative;

/// A wrapper around any kind of crypto primitive (just needs RawBytesEncoding implemented)
/// as if it were defined as `<name> = bytes` in CDDL with the byte representation
/// corresponding to RawByteEncoding.
#[derive(Debug, Clone, Derivative)]
#[derivative(Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ChainCrypto<T: RawBytesEncoding> {
    pub primitive: T,
    #[derivative(
        PartialEq = "ignore",
        Ord = "ignore",
        PartialOrd = "ignore",
        Hash = "ignore"
    )]
    pub encoding: StringEncoding,
}

impl<T: RawBytesEncoding> From<T> for ChainCrypto<T> {
    fn from(primitive: T) -> Self {
        Self {
            primitive,
            encoding: StringEncoding::default(),
        }
    }
}

impl<T: RawBytesEncoding> Serialize for ChainCrypto<T> {
    fn serialize<'se, W: std::io::Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        let data = self.primitive.to_raw_bytes();
        serializer.write_bytes_sz(
            &data,
            self.encoding
                .to_str_len_sz(data.len() as u64, force_canonical),
        )
    }
}

impl<T: RawBytesEncoding> Deserialize for ChainCrypto<T> {
    fn deserialize<R: std::io::BufRead>(
        raw: &mut Deserializer<R>,
    ) -> Result<Self, DeserializeError> {
        (|| -> Result<Self, DeserializeError> {
            let (bytes, encoding) = raw.bytes_sz()?;
            T::from_raw_bytes(&bytes)
                .map(|primitive| ChainCrypto {
                    primitive,
                    encoding: encoding.into(),
                })
                .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
        })()
        .map_err(|e| e.annotate("ChainCrypto"))
    }
}

impl<T: RawBytesEncoding> RawBytesEncoding for ChainCrypto<T> {
    fn to_raw_bytes(&self) -> &[u8] {
        self.primitive.to_raw_bytes()
    }

    fn from_raw_bytes(bytes: &[u8]) -> Result<Self, CryptoError> {
        T::from_raw_bytes(bytes).map(Into::into)
    }
}

impl<T: RawBytesEncoding> serde::Serialize for ChainCrypto<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.primitive.to_raw_hex())
    }
}

impl<'de, T: RawBytesEncoding> serde::de::Deserialize<'de> for ChainCrypto<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let s = <String as serde::de::Deserialize>::deserialize(deserializer)?;
        T::from_raw_hex(&s).map(Into::into).map_err(|_e| {
            serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(&s),
                &"hex bytes for signature",
            )
        })
    }
}

impl<T: RawBytesEncoding> schemars::JsonSchema for ChainCrypto<T> {
    fn schema_name() -> String {
        String::from("ChainCrypto")
    }
    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        String::json_schema(gen)
    }
    fn is_referenceable() -> bool {
        String::is_referenceable()
    }
}
