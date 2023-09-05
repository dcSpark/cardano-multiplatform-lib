use crate::byron::AddrAttributes;
pub use cml_crypto::{
    AnchorDataHash, AuxiliaryDataHash, BlockBodyHash, BlockHeaderHash, DatumHash, Ed25519KeyHash, Ed25519Signature,
    GenesisDelegateHash, GenesisHash, KESVkey, NonceHash, PoolMetadataHash, ScriptDataHash,
    ScriptHash, TransactionHash, VRFKeyHash, VRFVkey,
};

pub type Vkey = cml_crypto::PublicKey;

pub mod hash;
pub mod utils;
// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

pub mod cbor_encodings;
pub mod serialization;

use cbor_encodings::{
    BootstrapWitnessEncoding, KESSignatureEncoding, VRFCertEncoding, VkeywitnessEncoding,
};
use cml_core::error::*;
use cml_core::serialization::{LenEncoding, StringEncoding};
use std::convert::TryFrom;

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
        if inner.len() != 448 {
            return Err(DeserializeError::new(
                "KESSignature",
                DeserializeFailure::RangeCheck {
                    found: inner.len(),
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
    pub fn new(output: Vec<u8>, proof: Vec<u8>) -> Self {
        Self {
            output,
            proof,
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
