use wasm_bindgen::prelude::{wasm_bindgen, JsError};

use cml_crypto::RawBytesEncoding;

pub mod emip3;

#[wasm_bindgen]
pub struct Bip32PrivateKey(cml_crypto::Bip32PrivateKey);

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
        cml_crypto::Bip32PrivateKey::from_128_xprv(bytes)
            .map(Self)
            .map_err(Into::into)
    }
    /// see from_128_xprv
    pub fn to_128_xprv(&self) -> Vec<u8> {
        self.0.to_128_xprv()
    }

    pub fn generate_ed25519_bip32() -> Bip32PrivateKey {
        Self(cml_crypto::Bip32PrivateKey::generate_ed25519_bip32())
    }

    pub fn to_raw_key(&self) -> PrivateKey {
        self.0.to_raw_key().into()
    }

    pub fn to_public(&self) -> Bip32PublicKey {
        Bip32PublicKey(self.0.to_public())
    }

    pub fn from_raw_bytes(bytes: &[u8]) -> Result<Bip32PrivateKey, JsError> {
        cml_crypto::Bip32PrivateKey::from_raw_bytes(bytes)
            .map(Self)
            .map_err(Into::into)
    }

    pub fn to_raw_bytes(&self) -> Vec<u8> {
        self.0.to_raw_bytes().to_vec()
    }

    pub fn from_bech32(bech32_str: &str) -> Result<Bip32PrivateKey, JsError> {
        cml_crypto::Bip32PrivateKey::from_bech32(bech32_str)
            .map(Self)
            .map_err(Into::into)
    }

    pub fn to_bech32(&self) -> String {
        self.0.to_bech32()
    }

    pub fn from_bip39_entropy(entropy: &[u8], password: &[u8]) -> Self {
        Self(cml_crypto::Bip32PrivateKey::from_bip39_entropy(
            entropy, password,
        ))
    }

    pub fn chaincode(&self) -> Vec<u8> {
        self.0.chaincode()
    }
}

impl From<cml_crypto::Bip32PrivateKey> for Bip32PrivateKey {
    fn from(inner: cml_crypto::Bip32PrivateKey) -> Self {
        Self(inner)
    }
}

impl From<Bip32PrivateKey> for cml_crypto::Bip32PrivateKey {
    fn from(wrapper: Bip32PrivateKey) -> Self {
        wrapper.0
    }
}

impl AsRef<cml_crypto::Bip32PrivateKey> for Bip32PrivateKey {
    fn as_ref(&self) -> &cml_crypto::Bip32PrivateKey {
        &self.0
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Bip32PublicKey(cml_crypto::Bip32PublicKey);

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

    pub fn from_raw_bytes(bytes: &[u8]) -> Result<Bip32PublicKey, JsError> {
        cml_crypto::Bip32PublicKey::from_raw_bytes(bytes)
            .map(Self)
            .map_err(Into::into)
    }

    pub fn to_raw_bytes(&self) -> Vec<u8> {
        self.0.to_raw_bytes().to_vec()
    }

    pub fn from_bech32(bech32_str: &str) -> Result<Bip32PublicKey, JsError> {
        cml_crypto::Bip32PublicKey::from_bech32(bech32_str)
            .map(Self)
            .map_err(Into::into)
    }

    pub fn to_bech32(&self) -> String {
        self.0.to_bech32()
    }

    pub fn chaincode(&self) -> Vec<u8> {
        self.0.chaincode()
    }
}

impl From<cml_crypto::Bip32PublicKey> for Bip32PublicKey {
    fn from(inner: cml_crypto::Bip32PublicKey) -> Self {
        Self(inner)
    }
}

impl From<Bip32PublicKey> for cml_crypto::Bip32PublicKey {
    fn from(wrapper: Bip32PublicKey) -> Self {
        wrapper.0
    }
}

impl AsRef<cml_crypto::Bip32PublicKey> for Bip32PublicKey {
    fn as_ref(&self) -> &cml_crypto::Bip32PublicKey {
        &self.0
    }
}

#[wasm_bindgen]
pub struct PrivateKey(cml_crypto::PrivateKey);

#[wasm_bindgen]
impl PrivateKey {
    pub fn to_public(&self) -> PublicKey {
        PublicKey(self.0.to_public())
    }

    pub fn generate_ed25519() -> Self {
        Self(cml_crypto::PrivateKey::generate_ed25519())
    }

    pub fn generate_ed25519extended() -> Self {
        Self(cml_crypto::PrivateKey::generate_ed25519extended())
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
        cml_crypto::PrivateKey::from_bech32(bech32_str)
            .map(Self)
            .map_err(Into::into)
    }

    pub fn to_bech32(&self) -> String {
        self.0.to_bech32()
    }

    pub fn to_raw_bytes(&self) -> Vec<u8> {
        self.0.to_raw_bytes().to_vec()
    }

    pub fn from_extended_bytes(bytes: &[u8]) -> Result<PrivateKey, JsError> {
        cml_crypto::PrivateKey::from_extended_bytes(bytes)
            .map(Self)
            .map_err(Into::into)
    }

    pub fn from_normal_bytes(bytes: &[u8]) -> Result<PrivateKey, JsError> {
        cml_crypto::PrivateKey::from_normal_bytes(bytes)
            .map(Self)
            .map_err(Into::into)
    }

    pub fn sign(&self, message: &[u8]) -> Ed25519Signature {
        Ed25519Signature(self.0.sign(message))
    }
}

impl From<cml_crypto::PrivateKey> for PrivateKey {
    fn from(inner: cml_crypto::PrivateKey) -> Self {
        Self(inner)
    }
}

impl From<PrivateKey> for cml_crypto::PrivateKey {
    fn from(wrapper: PrivateKey) -> Self {
        wrapper.0
    }
}

impl AsRef<cml_crypto::PrivateKey> for PrivateKey {
    fn as_ref(&self) -> &cml_crypto::PrivateKey {
        &self.0
    }
}

/// ED25519 key used as public key
#[wasm_bindgen]
#[derive(Clone)]
pub struct PublicKey(cml_crypto::PublicKey);

#[wasm_bindgen]
impl PublicKey {
    /// Get public key from its bech32 representation
    /// Example:
    /// ```javascript
    /// const pkey = PublicKey.from_bech32(&#39;ed25519_pk1dgaagyh470y66p899txcl3r0jaeaxu6yd7z2dxyk55qcycdml8gszkxze2&#39;);
    /// ```
    pub fn from_bech32(bech32_str: &str) -> Result<PublicKey, JsError> {
        cml_crypto::PublicKey::from_bech32(bech32_str)
            .map(Self)
            .map_err(Into::into)
    }

    pub fn to_bech32(&self) -> String {
        self.0.to_bech32()
    }

    pub fn to_raw_bytes(&self) -> Vec<u8> {
        self.0.to_raw_bytes().to_vec()
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<PublicKey, JsError> {
        cml_crypto::PublicKey::from_raw_bytes(bytes)
            .map(Self)
            .map_err(Into::into)
    }

    pub fn verify(&self, data: &[u8], signature: &Ed25519Signature) -> bool {
        self.0.verify(data, &signature.0)
    }

    pub fn hash(&self) -> Ed25519KeyHash {
        Ed25519KeyHash(self.0.hash())
    }
}

impl From<cml_crypto::PublicKey> for PublicKey {
    fn from(inner: cml_crypto::PublicKey) -> Self {
        Self(inner)
    }
}

impl From<PublicKey> for cml_crypto::PublicKey {
    fn from(wrapper: PublicKey) -> Self {
        wrapper.0
    }
}

impl AsRef<cml_crypto::PublicKey> for PublicKey {
    fn as_ref(&self) -> &cml_crypto::PublicKey {
        &self.0
    }
}

macro_rules! impl_signature {
    ($name:ident) => {
        #[wasm_bindgen]
        #[derive(Debug, Clone)]
        pub struct $name(cml_crypto::$name);

        #[wasm_bindgen]
        impl $name {
            pub fn to_bech32(&self) -> String {
                self.0.to_bech32()
            }

            pub fn from_bech32(bech32_str: &str) -> Result<$name, wasm_bindgen::JsError> {
                cml_crypto::$name::from_bech32(bech32_str)
                    .map(Into::into)
                    .map(Self)
                    .map_err(Into::into)
            }
        }

        cml_core_wasm::impl_raw_bytes_api!(cml_crypto::$name, $name);

        impl From<cml_crypto::$name> for $name {
            fn from(inner: cml_crypto::$name) -> Self {
                Self(inner)
            }
        }

        impl From<$name> for cml_crypto::$name {
            fn from(wrapper: $name) -> cml_crypto::$name {
                wrapper.0
            }
        }

        impl AsRef<cml_crypto::$name> for $name {
            fn as_ref(&self) -> &cml_crypto::$name {
                &self.0
            }
        }
    };
}

impl_signature!(Ed25519Signature);

#[macro_export]
macro_rules! impl_hash_type_ext {
    ($rust_name:ty, $wasm_name:ident) => {
        #[wasm_bindgen::prelude::wasm_bindgen]
        #[derive(Debug, Clone)]
        pub struct $wasm_name($rust_name);

        #[wasm_bindgen::prelude::wasm_bindgen]
        impl $wasm_name {
            pub fn to_bech32(
                &self,
                prefix: &str,
            ) -> Result<String, wasm_bindgen::prelude::JsError> {
                self.0.to_bech32(prefix).map_err(Into::into)
            }

            pub fn from_bech32(
                bech32_str: &str,
            ) -> Result<$wasm_name, wasm_bindgen::prelude::JsError> {
                <$rust_name>::from_bech32(bech32_str)
                    .map(Into::into)
                    .map(Self)
                    .map_err(Into::into)
            }
        }

        impl From<$rust_name> for $wasm_name {
            fn from(inner: $rust_name) -> Self {
                Self(inner)
            }
        }

        impl From<$wasm_name> for $rust_name {
            fn from(wrapper: $wasm_name) -> $rust_name {
                wrapper.0
            }
        }

        impl AsRef<$rust_name> for $wasm_name {
            fn as_ref(&self) -> &$rust_name {
                &self.0
            }
        }

        cml_core_wasm::impl_raw_bytes_api!($rust_name, $wasm_name);
    };
}

macro_rules! impl_hash_type {
    ($name:ident) => {
        impl_hash_type_ext!(cml_crypto::$name, $name);
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
impl_hash_type!(DatumHash);
impl_hash_type!(ScriptDataHash);
// We might want to make these two vkeys normal classes later but for now it's just arbitrary bytes for us (used in block parsing)
impl_hash_type!(VRFVkey);
impl_hash_type!(KESVkey);
impl_hash_type!(NonceHash);
impl_hash_type!(AnchorDocHash);

#[wasm_bindgen]
#[derive(Clone)]
pub struct LegacyDaedalusPrivateKey(cml_crypto::LegacyDaedalusPrivateKey);

#[wasm_bindgen]
impl LegacyDaedalusPrivateKey {
    pub fn chaincode(&self) -> Vec<u8> {
        self.0.chaincode()
    }
}

impl From<cml_crypto::LegacyDaedalusPrivateKey> for LegacyDaedalusPrivateKey {
    fn from(native: cml_crypto::LegacyDaedalusPrivateKey) -> Self {
        Self(native)
    }
}

impl From<LegacyDaedalusPrivateKey> for cml_crypto::LegacyDaedalusPrivateKey {
    fn from(wasm: LegacyDaedalusPrivateKey) -> Self {
        wasm.0
    }
}

impl AsRef<cml_crypto::LegacyDaedalusPrivateKey> for LegacyDaedalusPrivateKey {
    fn as_ref(&self) -> &cml_crypto::LegacyDaedalusPrivateKey {
        &self.0
    }
}
