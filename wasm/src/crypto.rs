use super::*;

// for to/from bytes
use core::serialization::{Serialize, Deserialize};
use core_crypto::RawBytesEncoding;

// unfortunately concat_idents isn't in stable so we just take in two params
// as to not cause name collisions in the final WASM build in the chain wasm bindings
// TODO: embed the type into comments (looks like it's likely possible using another macro)
//       so that we have less generic documentation
macro_rules! impl_chain_crypto {
    ($name:ident, $primitive:ident) => {
        /// On-chain cryptographic primitive
        #[wasm_bindgen]
        #[derive(Debug, Clone)]
        pub struct $name(core::crypto::ChainCrypto<core_crypto::$primitive>);

        #[wasm_bindgen]
        impl $name {
            /// Get the underlying cryptographic primitive represented here
            pub fn primitive(&self) -> wasm_crypto::$primitive {
                self.0.primitive.clone().into()
            }

            /// Make a default-encoded on-chain cryptographic type based on the primitive
            pub fn new(primitive: &wasm_crypto::$primitive) -> Self {
                primitive.clone().into()
            }

            pub fn to_cbor_bytes(&self, force_canonical: bool) -> Vec<u8> {
                self.0.to_cbor_bytes(force_canonical)
            }

            pub fn from_raw_bytes(bytes: &[u8]) -> Result<$name, JsError> {
                core::crypto::ChainCrypto::<core_crypto::$primitive>::from_raw_bytes(bytes).map(Self).map_err(Into::into)
            }

            pub fn to_json(&self) -> Result<String, JsValue> {
                serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
            }
        
            pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
                JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
            }
        
            pub fn from_json(json: &str) -> Result<$name, JsValue> {
                serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
            }
        }

        // chain-crypto (rust) <-> chain-crypto (wasm)
        impl From<core::crypto::ChainCrypto<core_crypto::$primitive>> for $name {
            fn from(inner: core::crypto::ChainCrypto<core_crypto::$primitive>) -> Self {
                Self(inner)
            }
        }

        impl From<$name> for core::crypto::ChainCrypto<core_crypto::$primitive> {
            fn from(wrapper: $name) -> core::crypto::ChainCrypto<core_crypto::$primitive> {
                wrapper.0
            }
        }

        // crypto (wasm) <-> chain-crypto (wasm)
        impl From<wasm_crypto::$primitive> for $name {
            fn from(primitive: wasm_crypto::$primitive) -> Self {
                Self(core_crypto::$primitive::from(primitive).into())
            }
        }

        impl From<$name> for wasm_crypto::$primitive {
            fn from(wrapper: $name) -> wasm_crypto::$primitive {
                wrapper.0.primitive.into()
            }
        }
    };
}

impl_chain_crypto!(Ed25519SignatureOnChain, Ed25519Signature);
impl_chain_crypto!(Ed25519KeyHashOnChain, Ed25519KeyHash);
impl_chain_crypto!(ScriptHashOnChain, ScriptHash);
// TransactionHash is either a hash of the tx CBOR or a hash of a redeem address (genesis)
impl_chain_crypto!(TransactionHashOnChain, TransactionHash);
impl_chain_crypto!(GenesisDelegateHashOnChain, GenesisDelegateHash);
impl_chain_crypto!(GenesisHashOnChain, GenesisHash);
impl_chain_crypto!(AuxiliaryDataHashOnChain, AuxiliaryDataHash);
impl_chain_crypto!(PoolMetadataHashOnChain, PoolMetadataHash);
impl_chain_crypto!(VRFKeyHashOnChain, VRFKeyHash);
impl_chain_crypto!(BlockBodyHashOnChain, BlockBodyHash);
impl_chain_crypto!(BlockHeaderHashOnChain, BlockHeaderHash);
impl_chain_crypto!(DataHashOnChain, DataHash);
impl_chain_crypto!(ScriptDataHashOnChain, ScriptDataHash);
// We might want to make these two vkeys normal classes later but for now it's just arbitrary bytes for us (used in block parsing)
impl_chain_crypto!(VRFVKeyOnChain, VRFVKey);
impl_chain_crypto!(KESVKeyOnChain, KESVKey);

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
