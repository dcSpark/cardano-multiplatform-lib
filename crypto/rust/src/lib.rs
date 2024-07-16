//use crate::byron::{AddrAttributes, AddressContent};
use crate::chain_crypto::bech32::Bech32;
pub use cml_core::{
    error::{DeserializeError, DeserializeFailure},
    serialization::{Deserialize, RawBytesEncoding, Serialize, StringEncoding},
};
use cryptoxide::blake2b::Blake2b;
pub use derivative::Derivative;
use impl_mockchain::key;
use rand::rngs::OsRng;
use std::convert::From;

pub mod emip3;

// brought over from old IOHK code
pub mod chain_core;
pub mod chain_crypto;
pub mod impl_mockchain;
pub mod typed_bytes;

// used in chain_core / chain_crypto
#[macro_use]
extern crate cfg_if;

#[derive(Debug, thiserror::Error)]
pub enum CryptoError {
    #[error("Bech32: {0}")]
    Bech32(#[from] chain_crypto::bech32::Error),
    #[error("ByronError: {0}")]
    Hex(#[from] hex::FromHexError),
    #[error("Deserialization: {0}")]
    Deserialization(#[from] DeserializeError),
    #[error("SecretKeyError: {0}")]
    SecretKey(#[from] chain_crypto::SecretKeyError),
    #[error("PublicKeyError: {0}")]
    PublicKey(#[from] chain_crypto::PublicKeyError),
    #[error("SignatureFromStr: {0}")]
    SignatureFromStr(#[from] chain_crypto::SignatureFromStrError),
    #[error("BootStrapCombine: {0}")]
    BootstrapCombine(#[from] ed25519_bip32::PublicKeyError),
    #[error("SignatureError: {0}")]
    SignatureError(#[from] chain_crypto::SignatureError),
}

// otherwise with 2 Froms (bech32::Error -> chain_crypto::bech32::Error -> CryptoError)
// this can be hard to use (type annotations needed) so we provide a direct one.
impl From<bech32::Error> for CryptoError {
    fn from(e: bech32::Error) -> Self {
        chain_crypto::bech32::Error::Bech32Malformed(e).into()
    }
}

pub fn blake2b224(data: &[u8]) -> [u8; 28] {
    let mut out = [0; 28];
    Blake2b::blake2b(&mut out, data, &[]);
    out
}

pub fn blake2b256(data: &[u8]) -> [u8; 32] {
    let mut out = [0; 32];
    Blake2b::blake2b(&mut out, data, &[]);
    out
}

// All key structs were adapted from js-chain-libs:
// https://github.com/Emurgo/js-chain-libs

pub struct Bip32PrivateKey(chain_crypto::SecretKey<chain_crypto::Ed25519Bip32>);

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
    pub fn derive(&self, index: u32) -> Bip32PrivateKey {
        Bip32PrivateKey(chain_crypto::derive::derive_sk_ed25519(&self.0, index))
    }

    /// 128-byte xprv a key format in Cardano that some software still uses or requires
    /// the traditional 96-byte xprv is simply encoded as
    /// prv | chaincode
    /// however, because some software may not know how to compute a public key from a private key,
    /// the 128-byte inlines the public key in the following format
    /// prv | pub | chaincode
    /// so be careful if you see the term "xprv" as it could refer to either one
    /// our library does not require the pub (instead we compute the pub key when needed)
    pub fn from_128_xprv(bytes: &[u8]) -> Result<Bip32PrivateKey, CryptoError> {
        let mut buf = [0; 96];
        buf[0..64].clone_from_slice(&bytes[0..64]);
        buf[64..96].clone_from_slice(&bytes[96..128]);

        Bip32PrivateKey::from_raw_bytes(&buf).map_err(Into::into)
    }
    /// see from_128_xprv
    pub fn to_128_xprv(&self) -> Vec<u8> {
        let raw_key = self.to_raw_key();
        let prv_key = raw_key.to_raw_bytes();
        let raw_pub_key = self.to_public().to_raw_key();
        let pub_key = raw_pub_key.to_raw_bytes();
        let cc = self.chaincode();

        let mut buf = [0; 128];
        buf[0..64].clone_from_slice(prv_key);
        buf[64..96].clone_from_slice(pub_key);
        buf[96..128].clone_from_slice(&cc);
        buf.to_vec()
    }

    pub fn generate_ed25519_bip32() -> Bip32PrivateKey {
        Bip32PrivateKey(chain_crypto::SecretKey::<chain_crypto::Ed25519Bip32>::generate(OsRng))
    }

    pub fn to_raw_key(&self) -> PrivateKey {
        PrivateKey(key::EitherEd25519SecretKey::Extended(
            chain_crypto::derive::to_raw_sk(&self.0),
        ))
    }

    pub fn to_public(&self) -> Bip32PublicKey {
        Bip32PublicKey(self.0.to_public())
    }

    pub fn from_bech32(bech32_str: &str) -> Result<Bip32PrivateKey, CryptoError> {
        chain_crypto::SecretKey::try_from_bech32_str(bech32_str)
            .map(Bip32PrivateKey)
            .map_err(Into::into)
    }

    pub fn to_bech32(&self) -> String {
        self.0.to_bech32_str()
    }

    pub fn from_bip39_entropy(entropy: &[u8], password: &[u8]) -> Bip32PrivateKey {
        Bip32PrivateKey(chain_crypto::derive::from_bip39_entropy(entropy, password))
    }

    pub fn chaincode(&self) -> Vec<u8> {
        const ED25519_PRIVATE_KEY_LENGTH: usize = 64;
        const XPRV_SIZE: usize = 96;
        self.0.as_ref()[ED25519_PRIVATE_KEY_LENGTH..XPRV_SIZE].to_vec()
    }
}

impl RawBytesEncoding for Bip32PrivateKey {
    fn to_raw_bytes(&self) -> &[u8] {
        self.0.as_ref()
    }

    fn from_raw_bytes(bytes: &[u8]) -> Result<Self, DeserializeError> {
        chain_crypto::SecretKey::<chain_crypto::Ed25519Bip32>::from_binary(bytes)
            .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
            .map(Bip32PrivateKey)
    }
}

#[derive(
    Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct Bip32PublicKey(pub chain_crypto::PublicKey<chain_crypto::Ed25519Bip32>);

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
    pub fn derive(&self, index: u32) -> Result<Bip32PublicKey, ed25519_bip32::DerivationError> {
        chain_crypto::derive::derive_pk_ed25519(&self.0, index).map(Bip32PublicKey)
    }

    pub fn to_raw_key(&self) -> PublicKey {
        PublicKey(chain_crypto::derive::to_raw_pk(&self.0))
    }

    pub fn from_bech32(bech32_str: &str) -> Result<Bip32PublicKey, CryptoError> {
        chain_crypto::PublicKey::try_from_bech32_str(bech32_str)
            .map(Bip32PublicKey)
            .map_err(Into::into)
    }

    pub fn to_bech32(&self) -> String {
        self.0.to_bech32_str()
    }

    pub fn chaincode(&self) -> Vec<u8> {
        const ED25519_PUBLIC_KEY_LENGTH: usize = 32;
        const XPUB_SIZE: usize = 64;
        self.0.as_ref()[ED25519_PUBLIC_KEY_LENGTH..XPUB_SIZE].to_vec()
    }
}

impl RawBytesEncoding for Bip32PublicKey {
    fn to_raw_bytes(&self) -> &[u8] {
        self.0.as_ref()
    }

    fn from_raw_bytes(bytes: &[u8]) -> Result<Self, DeserializeError> {
        chain_crypto::PublicKey::<chain_crypto::Ed25519Bip32>::from_binary(bytes)
            .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
            .map(Bip32PublicKey)
    }
}

impl From<chain_crypto::PublicKey<chain_crypto::Ed25519Bip32>> for Bip32PublicKey {
    fn from(key: chain_crypto::PublicKey<chain_crypto::Ed25519Bip32>) -> Self {
        Self(key)
    }
}

pub struct PrivateKey(key::EitherEd25519SecretKey);

impl From<key::EitherEd25519SecretKey> for PrivateKey {
    fn from(secret_key: key::EitherEd25519SecretKey) -> PrivateKey {
        PrivateKey(secret_key)
    }
}

impl PrivateKey {
    pub fn to_public(&self) -> PublicKey {
        self.0.to_public().into()
    }

    pub fn generate_ed25519() -> PrivateKey {
        let keypair = chain_crypto::SecretKey::<chain_crypto::Ed25519>::generate(OsRng);
        PrivateKey(key::EitherEd25519SecretKey::Normal(keypair))
    }

    pub fn generate_ed25519extended() -> PrivateKey {
        let keypair = chain_crypto::SecretKey::<chain_crypto::Ed25519Extended>::generate(OsRng);
        PrivateKey(key::EitherEd25519SecretKey::Extended(keypair))
    }

    /// Get private key from its bech32 representation
    /// ```rust
    /// use cml_crypto::PrivateKey;
    /// let key = PrivateKey::from_bech32("ed25519_sk1ahfetf02qwwg4dkq7mgp4a25lx5vh9920cr5wnxmpzz9906qvm8qwvlts0").unwrap();
    /// ```
    /// For an extended 25519 key
    /// ```rust
    /// use cml_crypto::PrivateKey;
    /// let key = PrivateKey::from_bech32("ed25519e_sk1gqwl4szuwwh6d0yk3nsqcc6xxc3fpvjlevgwvt60df59v8zd8f8prazt8ln3lmz096ux3xvhhvm3ca9wj2yctdh3pnw0szrma07rt5gl748fp").unwrap();
    /// ```
    pub fn from_bech32(bech32_str: &str) -> Result<PrivateKey, CryptoError> {
        chain_crypto::SecretKey::try_from_bech32_str(bech32_str)
            .map(key::EitherEd25519SecretKey::Extended)
            .or_else(|_| {
                chain_crypto::SecretKey::try_from_bech32_str(bech32_str)
                    .map(key::EitherEd25519SecretKey::Normal)
            })
            .map(PrivateKey)
            .map_err(Into::into)
    }

    pub fn to_bech32(&self) -> String {
        match self.0 {
            key::EitherEd25519SecretKey::Normal(ref secret) => secret.to_bech32_str(),
            key::EitherEd25519SecretKey::Extended(ref secret) => secret.to_bech32_str(),
        }
    }

    pub fn from_extended_bytes(bytes: &[u8]) -> Result<PrivateKey, CryptoError> {
        chain_crypto::SecretKey::from_binary(bytes)
            .map(key::EitherEd25519SecretKey::Extended)
            .map(PrivateKey)
            .map_err(Into::into)
    }

    pub fn from_normal_bytes(bytes: &[u8]) -> Result<PrivateKey, CryptoError> {
        chain_crypto::SecretKey::from_binary(bytes)
            .map(key::EitherEd25519SecretKey::Normal)
            .map(PrivateKey)
            .map_err(Into::into)
    }

    pub fn sign(&self, message: &[u8]) -> Ed25519Signature {
        Ed25519Signature::from(self.0.sign(&message.to_vec()))
    }
}

impl RawBytesEncoding for PrivateKey {
    fn to_raw_bytes(&self) -> &[u8] {
        match self.0 {
            key::EitherEd25519SecretKey::Normal(ref secret) => secret.as_ref(),
            key::EitherEd25519SecretKey::Extended(ref secret) => secret.as_ref(),
        }
    }

    fn from_raw_bytes(bytes: &[u8]) -> Result<Self, DeserializeError> {
        Self::from_normal_bytes(bytes)
            .or_else(|_| Self::from_extended_bytes(bytes))
            .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
    }
}

/// ED25519 key used as public key
#[derive(
    Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct PublicKey(pub chain_crypto::PublicKey<chain_crypto::Ed25519>);

impl From<chain_crypto::PublicKey<chain_crypto::Ed25519>> for PublicKey {
    fn from(key: chain_crypto::PublicKey<chain_crypto::Ed25519>) -> PublicKey {
        PublicKey(key)
    }
}

impl PublicKey {
    /// Get public key from its bech32 representation
    /// Example:
    /// ```rust
    /// use cml_crypto::PublicKey;
    /// let key = PublicKey::from_bech32("ed25519_pk1dgaagyh470y66p899txcl3r0jaeaxu6yd7z2dxyk55qcycdml8gszkxze2").unwrap();
    /// ```
    pub fn from_bech32(bech32_str: &str) -> Result<PublicKey, CryptoError> {
        chain_crypto::PublicKey::try_from_bech32_str(bech32_str)
            .map(PublicKey)
            .map_err(Into::into)
    }

    pub fn to_bech32(&self) -> String {
        self.0.to_bech32_str()
    }

    pub fn verify(&self, data: &[u8], signature: &Ed25519Signature) -> bool {
        signature.0.verify_slice(&self.0, data) == chain_crypto::Verification::Success
    }

    pub fn hash(&self) -> Ed25519KeyHash {
        Ed25519KeyHash::from(blake2b224(self.to_raw_bytes()))
    }
}

impl RawBytesEncoding for PublicKey {
    fn to_raw_bytes(&self) -> &[u8] {
        self.0.as_ref()
    }

    fn from_raw_bytes(bytes: &[u8]) -> Result<Self, DeserializeError> {
        chain_crypto::PublicKey::from_binary(bytes)
            .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
            .map(PublicKey)
    }
}

macro_rules! impl_signature {
    ($name:ident, $signee_type:ty, $verifier_type:ty) => {
        #[derive(Debug, Clone, Eq, PartialEq)]
        pub struct $name(chain_crypto::Signature<$signee_type, $verifier_type>);

        impl $name {
            pub fn to_bech32(&self) -> String {
                use crate::chain_crypto::bech32::Bech32;
                self.0.to_bech32_str()
            }

            pub fn to_hex(&self) -> String {
                hex::encode(&self.0.as_ref())
            }

            pub fn from_bech32(bech32_str: &str) -> Result<Self, CryptoError> {
                use crate::chain_crypto::bech32::Bech32;
                chain_crypto::Signature::try_from_bech32_str(&bech32_str)
                    .map(Self)
                    .map_err(Into::into)
            }

            pub fn from_hex(input: &str) -> Result<Self, CryptoError> {
                use crate::chain_core::property::FromStr;
                chain_crypto::Signature::from_str(input)
                    .map(Self)
                    .map_err(Into::into)
            }
        }

        impl RawBytesEncoding for $name {
            fn to_raw_bytes(&self) -> &[u8] {
                self.0.as_ref()
            }

            fn from_raw_bytes(bytes: &[u8]) -> Result<Self, DeserializeError> {
                chain_crypto::Signature::from_binary(bytes.as_ref())
                    .map(Self)
                    .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
            }
        }

        impl serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_str(&self.to_hex())
            }
        }

        impl<'de> serde::de::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::de::Deserializer<'de>,
            {
                let s = <String as serde::de::Deserialize>::deserialize(deserializer)?;
                $name::from_hex(&s).map_err(|_e| {
                    serde::de::Error::invalid_value(
                        serde::de::Unexpected::Str(&s),
                        &"hex bytes for signature",
                    )
                })
            }
        }

        impl schemars::JsonSchema for $name {
            fn schema_name() -> String {
                String::from(stringify!($name))
            }
            fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
                String::json_schema(gen)
            }
            fn is_referenceable() -> bool {
                String::is_referenceable()
            }
        }

        impl Ord for $name {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.0.as_ref().cmp(other.0.as_ref())
            }
        }

        impl PartialOrd for $name {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        // allow since both act on the raw bytes
        #[allow(clippy::derived_hash_with_manual_eq)]
        impl std::hash::Hash for $name {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.0.as_ref().hash(state)
            }
        }

        impl From<chain_crypto::Signature<$signee_type, $verifier_type>> for $name {
            fn from(sig: chain_crypto::Signature<$signee_type, $verifier_type>) -> Self {
                Self(sig)
            }
        }
    };
}

impl_signature!(Ed25519Signature, Vec<u8>, chain_crypto::Ed25519);

#[macro_export]
macro_rules! impl_hash_type {
    ($name:ident, $byte_count:expr) => {
        #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
        pub struct $name([u8; $byte_count]);

        impl $name {
            pub const BYTE_COUNT: usize = $byte_count;

            pub fn to_bech32(&self, prefix: &str) -> Result<String, CryptoError> {
                use bech32::ToBase32;
                bech32::encode(&prefix, self.0.as_ref().to_base32()).map_err(Into::into)
            }

            pub fn from_bech32(bech_str: &str) -> Result<$name, CryptoError> {
                let (_hrp, u5data) = bech32::decode(bech_str)
                    .map_err(chain_crypto::bech32::Error::Bech32Malformed)?;
                let data: Vec<u8> = bech32::FromBase32::from_base32(&u5data)
                    .map_err(chain_crypto::bech32::Error::Bech32Malformed)?;
                Self::from_raw_bytes(&data).map_err(Into::into)
            }

            pub fn to_hex(&self) -> String {
                hex::encode(&self.0.as_ref())
            }

            pub fn from_hex(input: &str) -> Result<Self, DeserializeError> {
                let hex_bytes = hex::decode(input).map_err(|e| {
                    DeserializeError::from(DeserializeFailure::InvalidStructure(Box::new(e)))
                })?;
                Self::from_raw_bytes(&hex_bytes)
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.to_raw_hex())
            }
        }

        impl From<[u8; $byte_count]> for $name {
            fn from(bytes: [u8; $byte_count]) -> Self {
                Self(bytes)
            }
        }

        impl From<$name> for [u8; $byte_count] {
            fn from($name(bytes): $name) -> Self {
                bytes
            }
        }

        impl RawBytesEncoding for $name {
            fn to_raw_bytes(&self) -> &[u8] {
                self.0.as_ref()
            }

            fn from_raw_bytes(bytes: &[u8]) -> Result<Self, DeserializeError> {
                use std::convert::TryInto;
                match bytes.len() {
                    $byte_count => Ok($name(bytes[..$byte_count].try_into().unwrap())),
                    other_len => {
                        let cbor_error = cbor_event::Error::WrongLen(
                            $byte_count,
                            cbor_event::Len::Len(other_len as u64),
                            "hash length",
                        );
                        Err(DeserializeError::new(
                            stringify!($name),
                            DeserializeFailure::CBOR(cbor_error),
                        ))
                    }
                }
            }
        }

        impl serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_str(&self.to_hex())
            }
        }

        impl<'de> serde::de::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::de::Deserializer<'de>,
            {
                let s = <String as serde::de::Deserialize>::deserialize(deserializer)?;
                $name::from_hex(&s).map_err(|_e| {
                    serde::de::Error::invalid_value(
                        serde::de::Unexpected::Str(&s),
                        &"hex bytes for hash",
                    )
                })
            }
        }

        impl schemars::JsonSchema for $name {
            fn schema_name() -> String {
                String::from(stringify!($name))
            }
            fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
                String::json_schema(gen)
            }
            fn is_referenceable() -> bool {
                String::is_referenceable()
            }
        }
    };
}

impl_hash_type!(Ed25519KeyHash, 28);
impl_hash_type!(ScriptHash, 28);
// TransactionHash is either a hash of the tx CBOR or a hash of a redeem address (genesis)
impl_hash_type!(TransactionHash, 32);
impl_hash_type!(GenesisDelegateHash, 28);
impl_hash_type!(GenesisHash, 28);
impl_hash_type!(AuxiliaryDataHash, 32);
impl_hash_type!(PoolMetadataHash, 32);
impl_hash_type!(VRFKeyHash, 32);
impl_hash_type!(BlockBodyHash, 32);
impl_hash_type!(BlockHeaderHash, 32);
impl_hash_type!(DatumHash, 32);
impl_hash_type!(ScriptDataHash, 32);
// We might want to make these two vkeys normal classes later but for now it's just arbitrary bytes for us (used in block parsing)
impl_hash_type!(VRFVkey, 32);
impl_hash_type!(KESVkey, 32);
// same for this signature (but lots of traits aren't implemented for [u8; 448] so we can't)
//impl_hash_type!(KESSignature, 448);
impl_hash_type!(NonceHash, 32);
impl_hash_type!(AnchorDocHash, 32);

#[derive(Clone)]
pub struct LegacyDaedalusPrivateKey(chain_crypto::SecretKey<chain_crypto::LegacyDaedalus>);

impl LegacyDaedalusPrivateKey {
    pub fn chaincode(&self) -> Vec<u8> {
        const ED25519_PRIVATE_KEY_LENGTH: usize = 64;
        const XPRV_SIZE: usize = 96;
        self.0.as_ref()[ED25519_PRIVATE_KEY_LENGTH..XPRV_SIZE].to_vec()
    }
}

impl RawBytesEncoding for LegacyDaedalusPrivateKey {
    fn to_raw_bytes(&self) -> &[u8] {
        self.0.as_ref()
    }

    fn from_raw_bytes(bytes: &[u8]) -> Result<LegacyDaedalusPrivateKey, DeserializeError> {
        chain_crypto::SecretKey::<chain_crypto::LegacyDaedalus>::from_binary(bytes)
            .map(LegacyDaedalusPrivateKey)
            .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
    }
}

impl AsRef<chain_crypto::SecretKey<chain_crypto::LegacyDaedalus>> for LegacyDaedalusPrivateKey {
    fn as_ref(&self) -> &chain_crypto::SecretKey<chain_crypto::LegacyDaedalus> {
        &self.0
    }
}
