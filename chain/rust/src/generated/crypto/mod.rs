// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

pub mod cbor_encodings;
pub mod serialization;
pub use crate::AddrAttributes;
pub use crate::AnchorDocHash;
pub use crate::AuxiliaryDataHash;
pub use crate::BlockBodyHash;
pub use crate::BlockHeaderHash;
pub use crate::DatumHash;
pub use crate::Ed25519KeyHash;
pub use crate::Ed25519Signature;
pub use crate::GenesisDelegateHash;
pub use crate::GenesisHash;
pub use crate::KESVkey;
pub use crate::NonceHash;
pub use crate::PoolMetadataHash;
pub use crate::ScriptDataHash;
pub use crate::ScriptHash;
pub use crate::TransactionHash;
pub use crate::VRFKeyHash;
pub use crate::VRFVkey;
pub use crate::Vkey;

use cbor_encodings::{
    BootstrapWitnessEncoding, KESSignatureEncoding, VRFCertEncoding, VkeywitnessEncoding,
};
use cml_core::error::*;
use cml_core::serialization::{LenEncoding, StringEncoding};

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct BootstrapWitness {
    pub public_key: Vkey,
    pub signature: Ed25519Signature,
    pub chain_code: Vec<u8>,
    pub attributes: AddrAttributes,
    #[serde(skip)]
    pub encodings: Option<BootstrapWitnessEncoding>,
}

impl BootstrapWitness {
    pub fn new(
        public_key: Vkey,
        signature: Ed25519Signature,
        chain_code: Vec<u8>,
        attributes: AddrAttributes,
    ) -> Result<Self, DeserializeError> {
        if chain_code.len() != 32 {
            return Err(DeserializeFailure::RangeCheck {
                found: chain_code.len() as i128,
                min: Some(32),
                max: Some(32),
            }
            .into());
        }
        Ok(Self {
            public_key,
            signature,
            chain_code,
            attributes,
            encodings: None,
        })
    }
}

#[derive(Clone, Debug)]
pub struct KESSignature {
    pub(crate) inner: Vec<u8>,
    pub encodings: Option<KESSignatureEncoding>,
}

impl KESSignature {
    pub fn get(&self) -> &Vec<u8> {
        &self.inner
    }

    pub fn new(inner: Vec<u8>) -> Result<Self, DeserializeError> {
        if inner.len() != 448 {
            return Err(DeserializeError::new(
                "KESSignature",
                DeserializeFailure::RangeCheck {
                    found: inner.len() as i128,
                    min: Some(448),
                    max: Some(448),
                },
            ));
        }
        Ok(Self {
            inner,
            encodings: None,
        })
    }
}

impl TryFrom<Vec<u8>> for KESSignature {
    type Error = DeserializeError;

    fn try_from(inner: Vec<u8>) -> Result<Self, Self::Error> {
        KESSignature::new(inner)
    }
}

impl From<KESSignature> for Vec<u8> {
    fn from(wrapper: KESSignature) -> Self {
        wrapper.inner
    }
}

impl serde::Serialize for KESSignature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&hex::encode(self.inner.clone()))
    }
}

impl<'de> serde::de::Deserialize<'de> for KESSignature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let s = <String as serde::de::Deserialize>::deserialize(deserializer)?;
        hex::decode(&s)
            .ok()
            .and_then(|bytes| KESSignature::new(bytes).ok())
            .ok_or_else(|| {
                serde::de::Error::invalid_value(
                    serde::de::Unexpected::Str(&s),
                    &"invalid hex bytes",
                )
            })
    }
}

impl schemars::JsonSchema for KESSignature {
    fn schema_name() -> ::std::borrow::Cow<'static, str> {
        ::std::borrow::Cow::Borrowed("KESSignature")
    }

    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        <String as schemars::JsonSchema>::json_schema(generator)
    }

    fn inline_schema() -> bool {
        <String as schemars::JsonSchema>::inline_schema()
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub enum Nonce {
    Identity {
        #[serde(skip)]
        identity_encoding: Option<cbor_event::Sz>,
        #[serde(skip)]
        len_encoding: LenEncoding,
    },
    Hash {
        hash: NonceHash,
        #[serde(skip)]
        len_encoding: LenEncoding,
        #[serde(skip)]
        tag_encoding: Option<cbor_event::Sz>,
        #[serde(skip)]
        hash_encoding: StringEncoding,
    },
}

impl Nonce {
    pub fn new_identity() -> Self {
        Self::Identity {
            identity_encoding: None,
            len_encoding: LenEncoding::default(),
        }
    }

    pub fn new_hash(hash: NonceHash) -> Self {
        Self::Hash {
            hash,
            len_encoding: LenEncoding::default(),
            tag_encoding: None,
            hash_encoding: StringEncoding::default(),
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct VRFCert {
    pub output: Vec<u8>,
    pub proof: Vec<u8>,
    #[serde(skip)]
    pub encodings: Option<VRFCertEncoding>,
}

impl VRFCert {
    pub fn new(output: Vec<u8>, proof: Vec<u8>) -> Result<Self, DeserializeError> {
        if proof.len() != 80 {
            return Err(DeserializeFailure::RangeCheck {
                found: proof.len() as i128,
                min: Some(80),
                max: Some(80),
            }
            .into());
        }
        Ok(Self {
            output,
            proof,
            encodings: None,
        })
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct Vkeywitness {
    pub vkey: Vkey,
    pub ed25519_signature: Ed25519Signature,
    #[serde(skip)]
    pub encodings: Option<VkeywitnessEncoding>,
}

impl Vkeywitness {
    pub fn new(vkey: Vkey, ed25519_signature: Ed25519Signature) -> Self {
        Self {
            vkey,
            ed25519_signature,
            encodings: None,
        }
    }
}
