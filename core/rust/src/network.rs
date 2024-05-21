use crate::{error::DeserializeError, serialization::Deserialize};
use cbor_event::de::Deserializer;
use cbor_event::se::Serializer;
use schemars::JsonSchema;
use std::io::{BufRead, Write};

pub static BYRON_MAINNET_NETWORK_MAGIC: u32 = 764824073;
pub static BYRON_TESTNET_NETWORK_MAGIC: u32 = 1097911063;
pub static SANCHO_TESTNET_NETWORK_MAGIC: u32 = 4;
pub static PREPROD_NETWORK_MAGIC: u32 = 1;
pub static PREVIEW_NETWORK_MAGIC: u32 = 2;

impl std::fmt::Display for ProtocolMagic {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    JsonSchema,
)]
pub struct ProtocolMagic(u32);

impl From<ProtocolMagic> for u32 {
    fn from(val: ProtocolMagic) -> Self {
        val.0
    }
}

impl From<u32> for ProtocolMagic {
    fn from(inner: u32) -> Self {
        ProtocolMagic(inner)
    }
}

impl ::std::ops::Deref for ProtocolMagic {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for ProtocolMagic {
    fn default() -> Self {
        Self(764824073)
    }
}

impl cbor_event::se::Serialize for ProtocolMagic {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(self.0 as u64)
    }
}

impl Deserialize for ProtocolMagic {
    fn deserialize<R: BufRead>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        Ok(Self(raw.unsigned_integer()? as u32))
    }
}
