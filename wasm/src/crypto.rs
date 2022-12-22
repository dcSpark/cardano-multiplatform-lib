use super::*;

// for to/from bytes
use core::serialization::{Serialize, Deserialize};
use core_crypto::RawBytesEncoding;

use wasm_crypto::impl_chain_crypto;

impl_chain_crypto!(Ed25519SignatureOnChain, Ed25519Signature, wasm_crypto);
impl_chain_crypto!(Ed25519KeyHashOnChain, Ed25519KeyHash, wasm_crypto);
impl_chain_crypto!(ScriptHashOnChain, ScriptHash, wasm_crypto);
// TransactionHash is either a hash of the tx CBOR or a hash of a redeem address (genesis)
impl_chain_crypto!(TransactionHashOnChain, TransactionHash, wasm_crypto);
impl_chain_crypto!(GenesisDelegateHashOnChain, GenesisDelegateHash, wasm_crypto);
impl_chain_crypto!(GenesisHashOnChain, GenesisHash, wasm_crypto);
impl_chain_crypto!(AuxiliaryDataHashOnChain, AuxiliaryDataHash, wasm_crypto);
impl_chain_crypto!(PoolMetadataHashOnChain, PoolMetadataHash, wasm_crypto);
impl_chain_crypto!(VRFKeyHashOnChain, VRFKeyHash, wasm_crypto);
impl_chain_crypto!(BlockBodyHashOnChain, BlockBodyHash, wasm_crypto);
impl_chain_crypto!(BlockHeaderHashOnChain, BlockHeaderHash, wasm_crypto);
impl_chain_crypto!(DataHashOnChain, DataHash, wasm_crypto);
impl_chain_crypto!(ScriptDataHashOnChain, ScriptDataHash, wasm_crypto);
// We might want to make these two vkeys normal classes later but for now it's just arbitrary bytes for us (used in block parsing)
impl_chain_crypto!(VRFVKeyOnChain, VRFVKey, wasm_crypto);
impl_chain_crypto!(KESVKeyOnChain, KESVKey, wasm_crypto);

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct KesSignature(pub(crate) core::crypto::KesSignature);

#[wasm_bindgen]
impl KesSignature {
    pub fn to_bytes(&self) -> Vec<u8> {
        Serialize::to_bytes(&self.0)
    }

    pub fn from_bytes(data: &[u8]) -> Result<KesSignature, JsValue> {
        Deserialize::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<KesSignature, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn get(&self) -> Vec<u8> {
        self.0.get().clone().clone()
    }
}

impl From<core::crypto::KesSignature> for KesSignature {
    fn from(native: core::crypto::KesSignature) -> Self {
        Self(native)
    }
}

impl From<KesSignature> for core::crypto::KesSignature {
    fn from(wasm: KesSignature) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Vkey(pub(crate) core::crypto::Vkey);

#[wasm_bindgen]
impl Vkey {
    pub fn to_bytes(&self) -> Vec<u8> {
        Serialize::to_bytes(&self.0)
    }

    pub fn from_bytes(data: &[u8]) -> Result<Vkey, JsValue> {
        Deserialize::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<Vkey, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }
}

impl From<core::crypto::Vkey> for Vkey {
    fn from(native: core::crypto::Vkey) -> Self {
        Self(native)
    }
}

impl From<Vkey> for core::crypto::Vkey {
    fn from(wasm: Vkey) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct VrfCert(pub(crate) core::crypto::VrfCert);

#[wasm_bindgen]
impl VrfCert {
    pub fn to_bytes(&self) -> Vec<u8> {
        Serialize::to_bytes(&self.0)
    }

    pub fn from_bytes(data: &[u8]) -> Result<VrfCert, JsValue> {
        Deserialize::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<VrfCert, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn index_0(&self) -> Vec<u8> {
        self.0.index_0.clone()
    }

    pub fn bytes(&self) -> Vec<u8> {
        self.0.bytes.clone()
    }

    pub fn new(index_0: Vec<u8>, bytes: Vec<u8>) -> Self {
        Self(core::crypto::VrfCert::new(index_0, bytes))
    }
}

impl From<core::crypto::VrfCert> for VrfCert {
    fn from(native: core::crypto::VrfCert) -> Self {
        Self(native)
    }
}

impl From<VrfCert> for core::crypto::VrfCert {
    fn from(wasm: VrfCert) -> Self {
        wasm.0
    }
}
