// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

pub use cml_crypto_wasm::{
    AnchorDocHash, AuxiliaryDataHash, BlockBodyHash, BlockHeaderHash, DatumHash, Ed25519KeyHash,
    Ed25519Signature, GenesisDelegateHash, GenesisHash, KESVkey, NonceHash, PoolMetadataHash,
    ScriptDataHash, ScriptHash, TransactionHash, VRFKeyHash, VRFVkey,
};

use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

use cml_core_wasm::{impl_wasm_cbor_json_api, impl_wasm_conversions};

use crate::byron::AddrAttributes;

pub type Vkey = cml_crypto_wasm::PublicKey;

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct BootstrapWitness(cml_chain::crypto::BootstrapWitness);

impl_wasm_cbor_json_api!(BootstrapWitness);

impl_wasm_conversions!(cml_chain::crypto::BootstrapWitness, BootstrapWitness);

#[wasm_bindgen]
impl BootstrapWitness {
    pub fn public_key(&self) -> Vkey {
        self.0.public_key.clone().into()
    }

    pub fn signature(&self) -> Ed25519Signature {
        self.0.signature.clone().into()
    }

    pub fn chain_code(&self) -> Vec<u8> {
        self.0.chain_code.clone()
    }

    pub fn attributes(&self) -> AddrAttributes {
        self.0.attributes.clone().into()
    }

    pub fn new(
        public_key: &Vkey,
        signature: &Ed25519Signature,
        chain_code: Vec<u8>,
        attributes: &AddrAttributes,
    ) -> Self {
        Self(cml_chain::crypto::BootstrapWitness::new(
            public_key.clone().into(),
            signature.clone().into(),
            chain_code,
            attributes.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct KESSignature(cml_chain::crypto::KESSignature);

impl_wasm_cbor_json_api!(KESSignature);

impl_wasm_conversions!(cml_chain::crypto::KESSignature, KESSignature);

#[wasm_bindgen]
impl KESSignature {
    pub fn get(&self) -> Vec<u8> {
        self.0.get().clone()
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct Nonce(cml_chain::crypto::Nonce);

impl_wasm_cbor_json_api!(Nonce);

impl_wasm_conversions!(cml_chain::crypto::Nonce, Nonce);

#[wasm_bindgen]
impl Nonce {
    pub fn new_identity() -> Self {
        Self(cml_chain::crypto::Nonce::new_identity())
    }

    pub fn new_hash(hash: &NonceHash) -> Self {
        Self(cml_chain::crypto::Nonce::new_hash(hash.clone().into()))
    }

    pub fn kind(&self) -> NonceKind {
        match &self.0 {
            cml_chain::crypto::Nonce::Identity { .. } => NonceKind::Identity,
            cml_chain::crypto::Nonce::Hash { .. } => NonceKind::Hash,
        }
    }

    pub fn as_hash(&self) -> Option<NonceHash> {
        match &self.0 {
            cml_chain::crypto::Nonce::Hash { hash, .. } => Some((*hash).into()),
            _ => None,
        }
    }
}

#[wasm_bindgen]
pub enum NonceKind {
    Identity,
    Hash,
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct VRFCert(cml_chain::crypto::VRFCert);

impl_wasm_cbor_json_api!(VRFCert);

impl_wasm_conversions!(cml_chain::crypto::VRFCert, VRFCert);

#[wasm_bindgen]
impl VRFCert {
    pub fn output(&self) -> Vec<u8> {
        self.0.output.clone()
    }

    pub fn proof(&self) -> Vec<u8> {
        self.0.proof.clone()
    }

    pub fn new(output: Vec<u8>, proof: Vec<u8>) -> Self {
        Self(cml_chain::crypto::VRFCert::new(output, proof))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct Vkeywitness(cml_chain::crypto::Vkeywitness);

impl_wasm_cbor_json_api!(Vkeywitness);

impl_wasm_conversions!(cml_chain::crypto::Vkeywitness, Vkeywitness);

#[wasm_bindgen]
impl Vkeywitness {
    pub fn vkey(&self) -> Vkey {
        self.0.vkey.clone().into()
    }

    pub fn ed25519_signature(&self) -> Ed25519Signature {
        self.0.ed25519_signature.clone().into()
    }

    pub fn new(vkey: &Vkey, ed25519_signature: &Ed25519Signature) -> Self {
        Self(cml_chain::crypto::Vkeywitness::new(
            vkey.clone().into(),
            ed25519_signature.clone().into(),
        ))
    }
}
