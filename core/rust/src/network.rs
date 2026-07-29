use crate::{error::DeserializeError, serialization::Deserialize};
use cbor_event::de::Deserializer;
use cbor_event::se::Serializer;
use schemars::JsonSchema;

pub static BYRON_MAINNET_NETWORK_MAGIC: u32 = 764824073;
pub static BYRON_TESTNET_NETWORK_MAGIC: u32 = 1097911063;
pub static SANCHO_TESTNET_NETWORK_MAGIC: u32 = 4;
pub static PREPROD_NETWORK_MAGIC: u32 = 1;
pub static PREVIEW_NETWORK_MAGIC: u32 = 2;

impl core::fmt::Display for ProtocolMagic {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
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

impl ::core::ops::Deref for ProtocolMagic {
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
    fn serialize<'se>(
        &self,
        serializer: &'se mut Serializer,
    ) -> cbor_event::Result<&'se mut Serializer> {
        serializer.write_unsigned_integer(self.0 as u64)
    }
}

impl Deserialize for ProtocolMagic {
    fn deserialize(raw: &mut Deserializer) -> Result<Self, DeserializeError> {
        Ok(Self(raw.unsigned_integer()? as u32))
    }
}
