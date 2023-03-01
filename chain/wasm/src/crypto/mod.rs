// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use cml_core::ordered_hash_map::OrderedHashMap;
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

use cml_crypto_wasm::Ed25519Signature;

pub type Vkey = cml_crypto_wasm::PublicKey;

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct BootstrapWitness(cml_chain::crypto::BootstrapWitness);

#[wasm_bindgen]
impl BootstrapWitness {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<BootstrapWitness, JsValue> {
        cml_chain::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<BootstrapWitness, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn public_key(&self) -> Vkey {
        self.0.public_key.clone().into()
    }

    pub fn signature(&self) -> Ed25519Signature {
        self.0.signature.clone().into()
    }

    pub fn chain_code(&self) -> Vec<u8> {
        self.0.chain_code.clone()
    }

    pub fn attributes(&self) -> Vec<u8> {
        self.0.attributes.clone()
    }

    pub fn new(
        public_key: &Vkey,
        signature: &Ed25519Signature,
        chain_code: Vec<u8>,
        attributes: Vec<u8>,
    ) -> Self {
        Self(cml_chain::crypto::BootstrapWitness::new(
            public_key.clone().into(),
            signature.clone().into(),
            chain_code,
            attributes,
        ))
    }
}

impl From<cml_chain::crypto::BootstrapWitness> for BootstrapWitness {
    fn from(native: cml_chain::crypto::BootstrapWitness) -> Self {
        Self(native)
    }
}

impl From<BootstrapWitness> for cml_chain::crypto::BootstrapWitness {
    fn from(wasm: BootstrapWitness) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::crypto::BootstrapWitness> for BootstrapWitness {
    fn as_ref(&self) -> &cml_chain::crypto::BootstrapWitness {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct KESSignature(cml_chain::crypto::KESSignature);

#[wasm_bindgen]
impl KESSignature {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<KESSignature, JsValue> {
        cml_chain::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<KESSignature, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn get(&self) -> Vec<u8> {
        self.0.get().clone()
    }
}

impl From<cml_chain::crypto::KESSignature> for KESSignature {
    fn from(native: cml_chain::crypto::KESSignature) -> Self {
        Self(native)
    }
}

impl From<KESSignature> for cml_chain::crypto::KESSignature {
    fn from(wasm: KESSignature) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::crypto::KESSignature> for KESSignature {
    fn as_ref(&self) -> &cml_chain::crypto::KESSignature {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct Nonce(cml_chain::crypto::Nonce);

#[wasm_bindgen]
impl Nonce {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<Nonce, JsValue> {
        cml_chain::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<Nonce, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn new_i0() -> Self {
        Self(cml_chain::crypto::Nonce::new_i0())
    }

    pub fn new_nonce1(bytes: Vec<u8>) -> Self {
        Self(cml_chain::crypto::Nonce::new_nonce1(bytes))
    }

    pub fn kind(&self) -> NonceKind {
        match &self.0 {
            cml_chain::crypto::Nonce::I0 { .. } => NonceKind::I0,
            cml_chain::crypto::Nonce::Nonce1 { .. } => NonceKind::Nonce1,
        }
    }

    pub fn as_nonce1(&self) -> Option<Vec<u8>> {
        match &self.0 {
            cml_chain::crypto::Nonce::Nonce1 { bytes, .. } => Some(bytes.clone()),
            _ => None,
        }
    }
}

impl From<cml_chain::crypto::Nonce> for Nonce {
    fn from(native: cml_chain::crypto::Nonce) -> Self {
        Self(native)
    }
}

impl From<Nonce> for cml_chain::crypto::Nonce {
    fn from(wasm: Nonce) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::crypto::Nonce> for Nonce {
    fn as_ref(&self) -> &cml_chain::crypto::Nonce {
        &self.0
    }
}

#[wasm_bindgen]
pub enum NonceKind {
    I0,
    Nonce1,
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct SignkeyKES(cml_chain::crypto::SignkeyKES);

#[wasm_bindgen]
impl SignkeyKES {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<SignkeyKES, JsValue> {
        cml_chain::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<SignkeyKES, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn get(&self) -> Vec<u8> {
        self.0.get().clone()
    }
}

impl From<cml_chain::crypto::SignkeyKES> for SignkeyKES {
    fn from(native: cml_chain::crypto::SignkeyKES) -> Self {
        Self(native)
    }
}

impl From<SignkeyKES> for cml_chain::crypto::SignkeyKES {
    fn from(wasm: SignkeyKES) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::crypto::SignkeyKES> for SignkeyKES {
    fn as_ref(&self) -> &cml_chain::crypto::SignkeyKES {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct VRFCert(cml_chain::crypto::VRFCert);

#[wasm_bindgen]
impl VRFCert {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<VRFCert, JsValue> {
        cml_chain::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<VRFCert, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn index_0(&self) -> Vec<u8> {
        self.0.index_0.clone()
    }

    pub fn bytes(&self) -> Vec<u8> {
        self.0.bytes.clone()
    }

    pub fn new(index_0: Vec<u8>, bytes: Vec<u8>) -> Self {
        Self(cml_chain::crypto::VRFCert::new(index_0, bytes))
    }
}

impl From<cml_chain::crypto::VRFCert> for VRFCert {
    fn from(native: cml_chain::crypto::VRFCert) -> Self {
        Self(native)
    }
}

impl From<VRFCert> for cml_chain::crypto::VRFCert {
    fn from(wasm: VRFCert) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::crypto::VRFCert> for VRFCert {
    fn as_ref(&self) -> &cml_chain::crypto::VRFCert {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct Vkeywitness(cml_chain::crypto::Vkeywitness);

#[wasm_bindgen]
impl Vkeywitness {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<Vkeywitness, JsValue> {
        cml_chain::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<Vkeywitness, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

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

impl From<cml_chain::crypto::Vkeywitness> for Vkeywitness {
    fn from(native: cml_chain::crypto::Vkeywitness) -> Self {
        Self(native)
    }
}

impl From<Vkeywitness> for cml_chain::crypto::Vkeywitness {
    fn from(wasm: Vkeywitness) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::crypto::Vkeywitness> for Vkeywitness {
    fn as_ref(&self) -> &cml_chain::crypto::Vkeywitness {
        &self.0
    }
}
