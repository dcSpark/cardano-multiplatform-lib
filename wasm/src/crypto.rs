use super::*;

use core_crypto::CryptoError;

// for to/from bytes
use core::serialization::{Serialize, Deserialize};

#[wasm_bindgen]
pub struct Bip32PrivateKey(core_crypto::Bip32PrivateKey);

#[wasm_bindgen]
impl Bip32PrivateKey {
    /// derive this private key with the given index.
    ///
    /// # Security considerations
    ///
    /// * hard derivation index cannot be soft derived with the public key
    ///
    /// # Hard derivation vs Soft derivation
    ///
    /// If you pass an index below 0x80000000 then it is a soft derivation.
    /// The advantage of soft derivation is that it is possible to derive the
    /// public key too. I.e. derivation the private key with a soft derivation
    /// index and then retrieving the associated public key is equivalent to
    /// deriving the public key associated to the parent private key.
    ///
    /// Hard derivation index does not allow public key derivation.
    ///
    /// This is why deriving the private key should not fail while deriving
    /// the public key may fail (if the derivation index is invalid).
    ///
    pub fn derive(&self, index: u32) -> Self {
        Self(self.0.derive(index))
    }

    /// 128-byte xprv a key format in Cardano that some software still uses or requires
    /// the traditional 96-byte xprv is simply encoded as
    /// prv | chaincode
    /// however, because some software may not know how to compute a public key from a private key,
    /// the 128-byte inlines the public key in the following format
    /// prv | pub | chaincode
    /// so be careful if you see the term "xprv" as it could refer to either one
    /// our library does not require the pub (instead we compute the pub key when needed)
    pub fn from_128_xprv(bytes: &[u8]) -> Result<Bip32PrivateKey, JsError> {
        core_crypto::Bip32PrivateKey::from_128_xprv(bytes).map(Self).map_err(Into::into)
    }
    /// see from_128_xprv
    pub fn to_128_xprv(&self) -> Vec<u8> {
        self.0.to_128_xprv()
    }

    pub fn generate_ed25519_bip32() -> Bip32PrivateKey {
        Self(core_crypto::Bip32PrivateKey::generate_ed25519_bip32())
    }

    pub fn to_raw_key(&self) -> PrivateKey {
        self.0.to_raw_key().into()
    }

    pub fn to_public(&self) -> Bip32PublicKey {
        Bip32PublicKey(self.0.to_public())
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Bip32PrivateKey, JsError> {
        core_crypto::Bip32PrivateKey::from_bytes(bytes).map(Self).map_err(Into::into)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.0.as_bytes()
    }

    pub fn from_bech32(bech32_str: &str) -> Result<Bip32PrivateKey, JsError> {
        core_crypto::Bip32PrivateKey::from_bech32(bech32_str).map(Self).map_err(Into::into)
    }

    pub fn to_bech32(&self) -> String {
        self.0.to_bech32()
    }

    pub fn from_bip39_entropy(entropy: &[u8], password: &[u8]) -> Self {
        Self(core_crypto::Bip32PrivateKey::from_bip39_entropy(entropy, password))
    }

    pub fn chaincode(&self) -> Vec<u8> {
        self.0.chaincode()
    }
}

impl From<core_crypto::Bip32PrivateKey> for Bip32PrivateKey {
    fn from(inner: core_crypto::Bip32PrivateKey) -> Self {
        Self(inner)
    }
}

impl From<Bip32PrivateKey> for core_crypto::Bip32PrivateKey {
    fn from(wrapper: Bip32PrivateKey) -> Self {
        wrapper.0
    }
}


#[wasm_bindgen]
pub struct Bip32PublicKey(core_crypto::Bip32PublicKey);

#[wasm_bindgen]
impl Bip32PublicKey {
    /// derive this public key with the given index.
    ///
    /// # Errors
    ///
    /// If the index is not a soft derivation index (< 0x80000000) then
    /// calling this method will fail.
    ///
    /// # Security considerations
    ///
    /// * hard derivation index cannot be soft derived with the public key
    ///
    /// # Hard derivation vs Soft derivation
    ///
    /// If you pass an index below 0x80000000 then it is a soft derivation.
    /// The advantage of soft derivation is that it is possible to derive the
    /// public key too. I.e. derivation the private key with a soft derivation
    /// index and then retrieving the associated public key is equivalent to
    /// deriving the public key associated to the parent private key.
    ///
    /// Hard derivation index does not allow public key derivation.
    ///
    /// This is why deriving the private key should not fail while deriving
    /// the public key may fail (if the derivation index is invalid).
    ///
    pub fn derive(&self, index: u32) -> Result<Bip32PublicKey, JsError> {
        self.0.derive(index).map(Self).map_err(Into::into)
    }

    pub fn to_raw_key(&self) -> PublicKey {
        PublicKey(self.0.to_raw_key())
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Bip32PublicKey, JsError> {
        core_crypto::Bip32PublicKey::from_bytes(bytes).map(Self).map_err(Into::into)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.0.as_bytes()
    }

    pub fn from_bech32(bech32_str: &str) -> Result<Bip32PublicKey, JsError> {
        core_crypto::Bip32PublicKey::from_bech32(bech32_str).map(Self).map_err(Into::into)
    }

    pub fn to_bech32(&self) -> String {
        self.0.to_bech32()
    }

    pub fn chaincode(&self) -> Vec<u8> {
        self.0.chaincode()
    }
}

impl From<core_crypto::Bip32PublicKey> for Bip32PublicKey {
    fn from(inner: core_crypto::Bip32PublicKey) -> Self {
        Self(inner)
    }
}

impl From<Bip32PublicKey> for core_crypto::Bip32PublicKey {
    fn from(wrapper: Bip32PublicKey) -> Self {
        wrapper.0
    }
}

#[wasm_bindgen]
pub struct PrivateKey(core_crypto::PrivateKey);

#[wasm_bindgen]
impl PrivateKey {
    pub fn to_public(&self) -> PublicKey {
        PublicKey(self.0.to_public())
    }

    pub fn generate_ed25519() -> Self {
        Self(core_crypto::PrivateKey::generate_ed25519())
    }

    pub fn generate_ed25519extended() -> Self {
        Self(core_crypto::PrivateKey::generate_ed25519extended())
    }

    /// Get private key from its bech32 representation
    /// ```javascript
    /// PrivateKey.from_bech32(&#39;ed25519_sk1ahfetf02qwwg4dkq7mgp4a25lx5vh9920cr5wnxmpzz9906qvm8qwvlts0&#39;);
    /// ```
    /// For an extended 25519 key
    /// ```javascript
    /// PrivateKey.from_bech32(&#39;ed25519e_sk1gqwl4szuwwh6d0yk3nsqcc6xxc3fpvjlevgwvt60df59v8zd8f8prazt8ln3lmz096ux3xvhhvm3ca9wj2yctdh3pnw0szrma07rt5gl748fp&#39;);
    /// ```
    pub fn from_bech32(bech32_str: &str) -> Result<PrivateKey, JsError> {
        core_crypto::PrivateKey::from_bech32(bech32_str).map(Self).map_err(Into::into)
    }

    pub fn to_bech32(&self) -> String {
        self.0.to_bech32()
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.0.as_bytes()
    }

    pub fn from_extended_bytes(bytes: &[u8]) -> Result<PrivateKey, JsError> {
        core_crypto::PrivateKey::from_extended_bytes(bytes).map(Self).map_err(Into::into)
    }

    pub fn from_normal_bytes(bytes: &[u8]) -> Result<PrivateKey, JsError> {
        core_crypto::PrivateKey::from_normal_bytes(bytes).map(Self).map_err(Into::into)
    }

    pub fn sign(&self, message: &[u8]) -> Ed25519Signature {
        self.0.sign(message).into()
    }
}

impl From<core_crypto::PrivateKey> for PrivateKey {
    fn from(inner: core_crypto::PrivateKey) -> Self {
        Self(inner)
    }
}

impl From<PrivateKey> for core_crypto::PrivateKey {
    fn from(wrapper: PrivateKey) -> Self {
        wrapper.0
    }
}


/// ED25519 key used as public key
#[wasm_bindgen]
pub struct PublicKey(core_crypto::PublicKey);

#[wasm_bindgen]
impl PublicKey {
    /// Get public key from its bech32 representation
    /// Example:
    /// ```javascript
    /// const pkey = PublicKey.from_bech32(&#39;ed25519_pk1dgaagyh470y66p899txcl3r0jaeaxu6yd7z2dxyk55qcycdml8gszkxze2&#39;);
    /// ```
    pub fn from_bech32(bech32_str: &str) -> Result<PublicKey, JsError> {
        core_crypto::PublicKey::from_bech32(bech32_str).map(Self).map_err(Into::into)
    }

    pub fn to_bech32(&self) -> String {
        self.0.to_bech32()
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.0.as_bytes()
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<PublicKey, JsError> {
        core_crypto::PublicKey::from_bytes(bytes).map(Self).map_err(Into::into)
    }

    pub fn verify(&self, data: &[u8], signature: &Ed25519Signature) -> bool {
        self.0.verify(data, &signature.0)
    }

    pub fn hash(&self) -> Ed25519KeyHash {
        self.0.hash().into()
    }
}

impl From<core_crypto::PublicKey> for PublicKey {
    fn from(inner: core_crypto::PublicKey) -> Self {
        Self(inner)
    }
}

impl From<PublicKey> for core_crypto::PublicKey {
    fn from(wrapper: PublicKey) -> Self {
        wrapper.0
    }
}

macro_rules! impl_signature {
    ($name:ident) => {
        #[wasm_bindgen]
        #[derive(Debug, Clone)]
        pub struct $name(core_crypto::$name);

        #[wasm_bindgen]
        impl $name {
            pub fn to_raw_bytes(&self) -> Vec<u8> {
                self.0.to_raw_bytes().to_vec()
            }

            pub fn to_bech32(&self) -> String {
                self.0.to_bech32()
            }

            pub fn to_hex(&self) -> String {
                self.0.to_hex()
            }

            pub fn from_bech32(bech32_str: &str) -> Result<$name, JsError> {
                core_crypto::$name::from_bech32(bech32_str).map(Self).map_err(Into::into)
            }

            pub fn from_hex(input: &str) -> Result<$name, JsError> {
                core_crypto::$name::from_hex(input).map(Self).map_err(Into::into)
            }

            pub fn from_raw_bytes(bytes: &[u8]) -> Result<$name, JsError> {
                core_crypto::$name::from_raw_bytes(bytes).map(Self).map_err(Into::into)
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


        impl From<core_crypto::$name> for $name {
            fn from(inner: core_crypto::$name) -> Self {
                Self(inner)
            }
        }

        impl From<$name> for core_crypto::$name {
            fn from(wrapper: $name) -> core_crypto::$name {
                wrapper.0
            }
        }
    };
}

impl_signature!(Ed25519Signature);

macro_rules! impl_hash_type {
    ($name:ident) => {
        #[wasm_bindgen]
        #[derive(Debug, Clone)]
        pub struct $name(core_crypto::$name);

        #[wasm_bindgen]
        impl $name {
            pub fn to_raw_bytes(&self) -> Vec<u8> {
                self.0.to_raw_bytes().to_vec()
            }

            pub fn to_bech32(&self, prefix: &str) -> Result<String, JsError> {
                self.0.to_bech32(prefix).map_err(Into::into)
            }

            pub fn to_hex(&self) -> String {
                self.0.to_hex()
            }

            pub fn from_bech32(bech32_str: &str) -> Result<$name, JsError> {
                core_crypto::$name::from_bech32(bech32_str).map(Self).map_err(Into::into)
            }

            pub fn from_hex(input: &str) -> Result<$name, JsError> {
                core_crypto::$name::from_hex(input).map(Self).map_err(Into::into)
            }

            pub fn from_raw_bytes(bytes: &[u8]) -> Result<$name, JsError> {
                core_crypto::$name::from_raw_bytes(bytes).map(Self).map_err(Into::into)
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

        impl From<core_crypto::$name> for $name {
            fn from(inner: core_crypto::$name) -> Self {
                Self(inner)
            }
        }

        impl From<$name> for core_crypto::$name {
            fn from(wrapper: $name) -> core_crypto::$name {
                wrapper.0
            }
        }
    };
}

impl_hash_type!(Ed25519KeyHash);
impl_hash_type!(ScriptHash);
// TransactionHash is either a hash of the tx CBOR or a hash of a redeem address (genesis)
impl_hash_type!(TransactionHash);
impl_hash_type!(GenesisDelegateHash);
impl_hash_type!(GenesisHash);
impl_hash_type!(AuxiliaryDataHash);
impl_hash_type!(PoolMetadataHash);
impl_hash_type!(VRFKeyHash);
impl_hash_type!(BlockBodyHash);
impl_hash_type!(BlockHeaderHash);
impl_hash_type!(DataHash);
impl_hash_type!(ScriptDataHash);
// We might want to make these two vkeys normal classes later but for now it's just arbitrary bytes for us (used in block parsing)
impl_hash_type!(VRFVKey);
impl_hash_type!(KESVKey);

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct KesSignature(pub(crate) core_crypto::KesSignature);

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

impl From<core_crypto::KesSignature> for KesSignature {
    fn from(native: core_crypto::KesSignature) -> Self {
        Self(native)
    }
}

impl From<KesSignature> for core_crypto::KesSignature {
    fn from(wasm: KesSignature) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Vkey(pub(crate) core_crypto::Vkey);

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

impl From<core_crypto::Vkey> for Vkey {
    fn from(native: core_crypto::Vkey) -> Self {
        Self(native)
    }
}

impl From<Vkey> for core_crypto::Vkey {
    fn from(wasm: Vkey) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct VrfCert(pub(crate) core_crypto::VrfCert);

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
        Self(core_crypto::VrfCert::new(index_0, bytes))
    }
}

impl From<core_crypto::VrfCert> for VrfCert {
    fn from(native: core_crypto::VrfCert) -> Self {
        Self(native)
    }
}

impl From<VrfCert> for core_crypto::VrfCert {
    fn from(wasm: VrfCert) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct VrfVkey(pub(crate) core_crypto::VrfVkey);

#[wasm_bindgen]
impl VrfVkey {
    pub fn to_bytes(&self) -> Vec<u8> {
        Serialize::to_bytes(&self.0)
    }

    pub fn from_bytes(data: &[u8]) -> Result<VrfVkey, JsValue> {
        Deserialize::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
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

impl From<core_crypto::VrfVkey> for VrfVkey {
    fn from(native: core_crypto::VrfVkey) -> Self {
        Self(native)
    }
}

impl From<VrfVkey> for core_crypto::VrfVkey {
    fn from(wasm: VrfVkey) -> Self {
        wasm.0
    }
}