pub type AddrKeyhash = Hash28;

pub type AuxiliaryDataHash = Hash32;

pub type DatumHash = Hash32;

pub type GenesisDelegateHash = Hash28;

pub type Genesishash = Hash28;

pub type Natural = Vec<u8>;

pub type PoolKeyhash = Hash28;

pub type PoolMetadataHash = Hash32;

pub type Scripthash = Hash28;

pub type VrfKeyhash = Hash32;

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct Hash28(pub(crate) core::Hash28);

#[wasm_bindgen]

impl Hash28 {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<Hash28, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<Hash28, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn get(&self) -> Vec<u8> {
        self.0.get().clone().clone()
    }
}

impl From<core::Hash28> for Hash28 {
    fn from(native: core::Hash28) -> Self {
        Self(native)
    }
}

impl From<Hash28> for core::Hash28 {
    fn from(wasm: Hash28) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct Hash32(pub(crate) core::Hash32);

#[wasm_bindgen]

impl Hash32 {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<Hash32, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<Hash32, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn get(&self) -> Vec<u8> {
        self.0.get().clone().clone()
    }
}

impl From<core::Hash32> for Hash32 {
    fn from(native: core::Hash32) -> Self {
        Self(native)
    }
}

impl From<Hash32> for core::Hash32 {
    fn from(wasm: Hash32) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct KesSignature(pub(crate) core::KesSignature);

#[wasm_bindgen]

impl KesSignature {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<KesSignature, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
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

impl From<core::KesSignature> for KesSignature {
    fn from(native: core::KesSignature) -> Self {
        Self(native)
    }
}

impl From<KesSignature> for core::KesSignature {
    fn from(wasm: KesSignature) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct KesVkey(pub(crate) core::KesVkey);

#[wasm_bindgen]

impl KesVkey {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<KesVkey, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<KesVkey, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn get(&self) -> Vec<u8> {
        self.0.get().clone().clone()
    }
}

impl From<core::KesVkey> for KesVkey {
    fn from(native: core::KesVkey) -> Self {
        Self(native)
    }
}

impl From<KesVkey> for core::KesVkey {
    fn from(wasm: KesVkey) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

pub enum NonceKind {
    I0,
    Nonce1,
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct Nonce(pub(crate) core::Nonce);

#[wasm_bindgen]

impl Nonce {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<Nonce, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<Nonce, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn new_i0() -> Self {
        Self(core::Nonce::new_i0())
    }

    pub fn new_nonce1(bytes: Vec<u8>) -> Self {
        Self(core::Nonce::new_nonce1(bytes))
    }

    pub fn kind(&self) -> NonceKind {
        match &self.0 {
            core::Nonce::I0{ .. } => NonceKind::I0,
            core::Nonce::Nonce1(_) => NonceKind::Nonce1,
        }
    }

    pub fn as_nonce1(&self) -> Option<Nonce1> {
        match &self.0 {
            core::Nonce::Nonce1(nonce1) => Some(nonce1.clone().into()),
            _ => None,
        }
    }
}

impl From<core::Nonce> for Nonce {
    fn from(native: core::Nonce) -> Self {
        Self(native)
    }
}

impl From<Nonce> for core::Nonce {
    fn from(wasm: Nonce) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct Signature(pub(crate) core::Signature);

#[wasm_bindgen]

impl Signature {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<Signature, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<Signature, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn get(&self) -> Vec<u8> {
        self.0.get().clone().clone()
    }
}

impl From<core::Signature> for Signature {
    fn from(native: core::Signature) -> Self {
        Self(native)
    }
}

impl From<Signature> for core::Signature {
    fn from(wasm: Signature) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct SignkeyKES(pub(crate) core::SignkeyKES);

#[wasm_bindgen]

impl SignkeyKES {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<SignkeyKES, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<SignkeyKES, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn get(&self) -> Vec<u8> {
        self.0.get().clone().clone()
    }
}

impl From<core::SignkeyKES> for SignkeyKES {
    fn from(native: core::SignkeyKES) -> Self {
        Self(native)
    }
}

impl From<SignkeyKES> for core::SignkeyKES {
    fn from(wasm: SignkeyKES) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct Vkey(pub(crate) core::Vkey);

#[wasm_bindgen]

impl Vkey {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<Vkey, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
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

    pub fn get(&self) -> Vec<u8> {
        self.0.get().clone().clone()
    }
}

impl From<core::Vkey> for Vkey {
    fn from(native: core::Vkey) -> Self {
        Self(native)
    }
}

impl From<Vkey> for core::Vkey {
    fn from(wasm: Vkey) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct VrfCert(pub(crate) core::VrfCert);

#[wasm_bindgen]

impl VrfCert {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<VrfCert, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
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
        Self(core::VrfCert::new(index_0, bytes))
    }
}

impl From<core::VrfCert> for VrfCert {
    fn from(native: core::VrfCert) -> Self {
        Self(native)
    }
}

impl From<VrfCert> for core::VrfCert {
    fn from(wasm: VrfCert) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct VrfVkey(pub(crate) core::VrfVkey);

#[wasm_bindgen]

impl VrfVkey {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<VrfVkey, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<VrfVkey, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn get(&self) -> Vec<u8> {
        self.0.get().clone().clone()
    }
}

impl From<core::VrfVkey> for VrfVkey {
    fn from(native: core::VrfVkey) -> Self {
        Self(native)
    }
}

impl From<VrfVkey> for core::VrfVkey {
    fn from(wasm: VrfVkey) -> Self {
        wasm.0
    }
}

use super::*;