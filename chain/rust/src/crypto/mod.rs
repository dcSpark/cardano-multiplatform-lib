pub use cml_crypto::{
    AuxiliaryDataHash, BlockBodyHash, BlockHeaderHash, DatumHash, Ed25519KeyHash, Ed25519Signature,
    GenesisDelegateHash, GenesisHash, KESVkey, PoolMetadataHash, ScriptDataHash, ScriptHash,
    TransactionHash, VRFKeyHash, VRFVkey,
};

pub type Vkey = cml_crypto::PublicKey;

pub mod utils;
// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

pub mod cbor_encodings;
pub mod serialization;

use cbor_encodings::{
    BootstrapWitnessEncoding, KESSignatureEncoding, SignkeyKESEncoding, VRFCertEncoding,
    VkeywitnessEncoding,
};
use cml_core::error::*;
use cml_core::ordered_hash_map::OrderedHashMap;
use cml_core::serialization::{LenEncoding, StringEncoding};
use std::collections::BTreeMap;
use std::convert::TryFrom;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct BootstrapWitness {
    pub public_key: Vkey,
    pub signature: Ed25519Signature,
    pub chain_code: Vec<u8>,
    pub attributes: Vec<u8>,
    #[serde(skip)]
    pub encodings: Option<BootstrapWitnessEncoding>,
}

impl BootstrapWitness {
    pub fn new(
        public_key: Vkey,
        signature: Ed25519Signature,
        chain_code: Vec<u8>,
        attributes: Vec<u8>,
    ) -> Self {
        Self {
            public_key,
            signature,
            chain_code,
            attributes,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct KESSignature {
    pub inner: Vec<u8>,
    #[serde(skip)]
    pub encodings: Option<KESSignatureEncoding>,
}

impl KESSignature {
    pub fn get(&self) -> &Vec<u8> {
        &self.inner
    }

    pub fn new(inner: Vec<u8>) -> Result<Self, DeserializeError> {
        if inner.len() != 32 {
            return Err(DeserializeError::new(
                "KESSignature",
                DeserializeFailure::RangeCheck {
                    found: inner.len(),
                    min: Some(32),
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

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub enum Nonce {
    I0 {
        #[serde(skip)]
        i0_encoding: Option<cbor_event::Sz>,
        #[serde(skip)]
        len_encoding: LenEncoding,
    },
    Nonce1 {
        bytes: Vec<u8>,
        #[serde(skip)]
        len_encoding: LenEncoding,
        #[serde(skip)]
        index_0_encoding: Option<cbor_event::Sz>,
        #[serde(skip)]
        bytes_encoding: StringEncoding,
    },
}

impl Nonce {
    pub fn new_i0() -> Self {
        Self::I0 {
            i0_encoding: None,
            len_encoding: LenEncoding::default(),
        }
    }

    pub fn new_nonce1(bytes: Vec<u8>) -> Self {
        Self::Nonce1 {
            bytes,
            len_encoding: LenEncoding::default(),
            index_0_encoding: None,
            bytes_encoding: StringEncoding::default(),
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct SignkeyKES {
    pub inner: Vec<u8>,
    #[serde(skip)]
    pub encodings: Option<SignkeyKESEncoding>,
}

impl SignkeyKES {
    pub fn get(&self) -> &Vec<u8> {
        &self.inner
    }

    pub fn new(inner: Vec<u8>) -> Result<Self, DeserializeError> {
        if inner.len() != 16 {
            return Err(DeserializeError::new(
                "SignkeyKES",
                DeserializeFailure::RangeCheck {
                    found: inner.len(),
                    min: Some(16),
                    max: Some(16),
                },
            ));
        }
        Ok(Self {
            inner,
            encodings: None,
        })
    }
}

impl TryFrom<Vec<u8>> for SignkeyKES {
    type Error = DeserializeError;

    fn try_from(inner: Vec<u8>) -> Result<Self, Self::Error> {
        SignkeyKES::new(inner)
    }
}

impl From<SignkeyKES> for Vec<u8> {
    fn from(wrapper: SignkeyKES) -> Self {
        wrapper.inner
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct VRFCert {
    pub index_0: Vec<u8>,
    pub bytes: Vec<u8>,
    #[serde(skip)]
    pub encodings: Option<VRFCertEncoding>,
}

impl VRFCert {
    pub fn new(index_0: Vec<u8>, bytes: Vec<u8>) -> Self {
        Self {
            index_0,
            bytes,
            encodings: None,
        }
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
