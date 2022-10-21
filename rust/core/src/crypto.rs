#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema, Derivative)]
#[derivative(Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Hash28 {
    pub inner: Vec<u8>,
    #[derivative(PartialEq="ignore", Ord="ignore", PartialOrd="ignore", Hash="ignore")]
    #[serde(skip)]
    pub encodings: Option<Hash28Encoding>,
}

impl Hash28 {
    pub fn get(&self) -> &Vec<u8> {
        &self.inner
    }

    pub fn new(inner: Vec<u8>) -> Result<Self, DeserializeError> {
        if inner.len() != 28 {
            return Err(DeserializeError::new("Hash28", DeserializeFailure::RangeCheck{ found: inner.len(), min: Some(28), max: Some(28) }));
        }
        Ok(Self {
            inner,
            encodings: None,
        })
    }
}

impl TryFrom<Vec<u8>> for Hash28 {
    type Error = DeserializeError;

    fn try_from(inner: Vec<u8>) -> Result<Self, Self::Error> {
        Hash28::new(inner)
    }
}

impl From<Hash28> for Vec<u8> {
    fn from(wrapper: Hash28) -> Self {
        wrapper.inner
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct Hash32 {
    pub inner: Vec<u8>,
    #[serde(skip)]
    pub encodings: Option<Hash32Encoding>,
}

impl Hash32 {
    pub fn get(&self) -> &Vec<u8> {
        &self.inner
    }

    pub fn new(inner: Vec<u8>) -> Result<Self, DeserializeError> {
        if inner.len() != 32 {
            return Err(DeserializeError::new("Hash32", DeserializeFailure::RangeCheck{ found: inner.len(), min: Some(32), max: Some(32) }));
        }
        Ok(Self {
            inner,
            encodings: None,
        })
    }
}

impl TryFrom<Vec<u8>> for Hash32 {
    type Error = DeserializeError;

    fn try_from(inner: Vec<u8>) -> Result<Self, Self::Error> {
        Hash32::new(inner)
    }
}

impl From<Hash32> for Vec<u8> {
    fn from(wrapper: Hash32) -> Self {
        wrapper.inner
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct KesSignature {
    pub inner: Vec<u8>,
    #[serde(skip)]
    pub encodings: Option<KesSignatureEncoding>,
}

impl KesSignature {
    pub fn get(&self) -> &Vec<u8> {
        &self.inner
    }

    pub fn new(inner: Vec<u8>) -> Result<Self, DeserializeError> {
        if inner.len() != 32 {
            return Err(DeserializeError::new("KesSignature", DeserializeFailure::RangeCheck{ found: inner.len(), min: Some(32), max: Some(32) }));
        }
        Ok(Self {
            inner,
            encodings: None,
        })
    }
}

impl TryFrom<Vec<u8>> for KesSignature {
    type Error = DeserializeError;

    fn try_from(inner: Vec<u8>) -> Result<Self, Self::Error> {
        KesSignature::new(inner)
    }
}

impl From<KesSignature> for Vec<u8> {
    fn from(wrapper: KesSignature) -> Self {
        wrapper.inner
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct KesVkey {
    pub inner: Vec<u8>,
    #[serde(skip)]
    pub encodings: Option<KesVkeyEncoding>,
}

impl KesVkey {
    pub fn get(&self) -> &Vec<u8> {
        &self.inner
    }

    pub fn new(inner: Vec<u8>) -> Result<Self, DeserializeError> {
        if inner.len() != 8 {
            return Err(DeserializeError::new("KesVkey", DeserializeFailure::RangeCheck{ found: inner.len(), min: Some(8), max: Some(8) }));
        }
        Ok(Self {
            inner,
            encodings: None,
        })
    }
}

impl TryFrom<Vec<u8>> for KesVkey {
    type Error = DeserializeError;

    fn try_from(inner: Vec<u8>) -> Result<Self, Self::Error> {
        KesVkey::new(inner)
    }
}

impl From<KesVkey> for Vec<u8> {
    fn from(wrapper: KesVkey) -> Self {
        wrapper.inner
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub enum Nonce {
    I0 {
        #[serde(skip)]
        i0_encoding: Option<cbor_event::Sz>,
        #[serde(skip)]
        outer_len_encoding: LenEncoding,
    }
    ,
    Nonce1(Nonce1),
}

impl Nonce {
    pub fn new_i0() -> Self {
        Self::I0 {
            i0_encoding: None,
            outer_len_encoding: LenEncoding::default(),
        }
    }

    pub fn new_nonce1(bytes: Vec<u8>) -> Self {
        Self::Nonce1(Nonce1::new(bytes))
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
            return Err(DeserializeError::new("SignkeyKES", DeserializeFailure::RangeCheck{ found: inner.len(), min: Some(16), max: Some(16) }));
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
pub struct Vkey {
    pub inner: Vec<u8>,
    #[serde(skip)]
    pub encodings: Option<VkeyEncoding>,
}

impl Vkey {
    pub fn get(&self) -> &Vec<u8> {
        &self.inner
    }

    pub fn new(inner: Vec<u8>) -> Result<Self, DeserializeError> {
        if inner.len() != 8 {
            return Err(DeserializeError::new("Vkey", DeserializeFailure::RangeCheck{ found: inner.len(), min: Some(8), max: Some(8) }));
        }
        Ok(Self {
            inner,
            encodings: None,
        })
    }
}

impl TryFrom<Vec<u8>> for Vkey {
    type Error = DeserializeError;

    fn try_from(inner: Vec<u8>) -> Result<Self, Self::Error> {
        Vkey::new(inner)
    }
}

impl From<Vkey> for Vec<u8> {
    fn from(wrapper: Vkey) -> Self {
        wrapper.inner
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct VrfCert {
    pub index_0: Vec<u8>,
    pub bytes: Vec<u8>,
    #[serde(skip)]
    pub encodings: Option<VrfCertEncoding>,
}

impl VrfCert {
    pub fn new(index_0: Vec<u8>, bytes: Vec<u8>) -> Self {
        Self {
            index_0,
            bytes,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct VrfVkey {
    pub inner: Vec<u8>,
    #[serde(skip)]
    pub encodings: Option<VrfVkeyEncoding>,
}

impl VrfVkey {
    pub fn get(&self) -> &Vec<u8> {
        &self.inner
    }

    pub fn new(inner: Vec<u8>) -> Result<Self, DeserializeError> {
        if inner.len() != 8 {
            return Err(DeserializeError::new("VrfVkey", DeserializeFailure::RangeCheck{ found: inner.len(), min: Some(8), max: Some(8) }));
        }
        Ok(Self {
            inner,
            encodings: None,
        })
    }
}

impl TryFrom<Vec<u8>> for VrfVkey {
    type Error = DeserializeError;

    fn try_from(inner: Vec<u8>) -> Result<Self, Self::Error> {
        VrfVkey::new(inner)
    }
}

impl From<VrfVkey> for Vec<u8> {
    fn from(wrapper: VrfVkey) -> Self {
        wrapper.inner
    }
}


use cryptoxide::blake2b::Blake2b;

use super::*;

pub (crate) fn blake2b224(data: &[u8]) -> [u8; 28] {
    let mut out = [0; 28];
    Blake2b::blake2b(&mut out, data, &[]);
    out
}

pub (crate) fn blake2b256(data: &[u8]) -> [u8; 32] {
    let mut out = [0; 32];
    Blake2b::blake2b(&mut out, data, &[]);
    out
}

// All key structs were taken from js-chain-libs:
// https://github.com/Emurgo/js-chain-libs

#[wasm_bindgen]
pub struct Bip32PrivateKey(crypto::SecretKey<crypto::Ed25519Bip32>);

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
    pub fn derive(&self, index: u32) -> Bip32PrivateKey {
        Bip32PrivateKey(crypto::derive::derive_sk_ed25519(&self.0, index))
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
        let mut buf = [0; 96];
        buf[0..64].clone_from_slice(&bytes[0..64]);
        buf[64..96].clone_from_slice(&bytes[96..128]);

        Bip32PrivateKey::from_bytes(&buf)
    }
    /// see from_128_xprv
    pub fn to_128_xprv(&self) -> Vec<u8> {
        let prv_key = self.to_raw_key().as_bytes();
        let pub_key = self.to_public().to_raw_key().as_bytes();
        let cc = self.chaincode();

        let mut buf = [0; 128];
        buf[0..64].clone_from_slice(&prv_key);
        buf[64..96].clone_from_slice(&pub_key);
        buf[96..128].clone_from_slice(&cc);
        buf.to_vec()
    }

    pub fn generate_ed25519_bip32() -> Result<Bip32PrivateKey, JsError> {
        Ok(OsRng)
            .map(crypto::SecretKey::<crypto::Ed25519Bip32>::generate)
            .map(Bip32PrivateKey)
    }

    pub fn to_raw_key(&self) -> PrivateKey {
        PrivateKey(key::EitherEd25519SecretKey::Extended(
            crypto::derive::to_raw_sk(&self.0),
        ))
    }

    pub fn to_public(&self) -> Bip32PublicKey {
        Bip32PublicKey(self.0.to_public().into())
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Bip32PrivateKey, JsError> {
        crypto::SecretKey::<crypto::Ed25519Bip32>::from_binary(bytes)
            .map_err(|e| JsError::from_str(&format!("{}", e)))
            .map(Bip32PrivateKey)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.0.as_ref().to_vec()
    }

    pub fn from_bech32(bech32_str: &str) -> Result<Bip32PrivateKey, JsError> {
        crypto::SecretKey::try_from_bech32_str(&bech32_str)
            .map(Bip32PrivateKey)
            .map_err(|_| JsError::from_str("Invalid secret key"))
    }

    pub fn to_bech32(&self) -> String {
        self.0.to_bech32_str()
    }

    pub fn from_bip39_entropy(entropy: &[u8], password: &[u8]) -> Bip32PrivateKey {
        Bip32PrivateKey(crypto::derive::from_bip39_entropy(&entropy, &password))
    }

    pub fn chaincode(&self) -> Vec<u8> {
        const ED25519_PRIVATE_KEY_LENGTH: usize = 64;
        const XPRV_SIZE: usize = 96;
        self.0.as_ref()[ED25519_PRIVATE_KEY_LENGTH..XPRV_SIZE].to_vec()
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct Bip32PublicKey(pub(crate) crypto::PublicKey<crypto::Ed25519Bip32>);

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
        crypto::derive::derive_pk_ed25519(&self.0, index)
            .map(Bip32PublicKey)
            .map_err(|e| JsError::from_str(&format! {"{:?}", e}))
    }

    pub fn to_raw_key(&self) -> PublicKey {
        PublicKey(crypto::derive::to_raw_pk(&self.0))
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Bip32PublicKey, JsError> {
        crypto::PublicKey::<crypto::Ed25519Bip32>::from_binary(bytes)
            .map_err(|e| JsError::from_str(&format!("{}", e)))
            .map(Bip32PublicKey)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.0.as_ref().to_vec()
    }

    pub fn from_bech32(bech32_str: &str) -> Result<Bip32PublicKey, JsError> {
        crypto::PublicKey::try_from_bech32_str(&bech32_str)
            .map(Bip32PublicKey)
            .map_err(|e| JsError::from_str(&format!("{}", e)))
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


#[wasm_bindgen]
pub struct PrivateKey(key::EitherEd25519SecretKey);

impl From<key::EitherEd25519SecretKey> for PrivateKey {
    fn from(secret_key: key::EitherEd25519SecretKey) -> PrivateKey {
        PrivateKey(secret_key)
    }
}

#[wasm_bindgen]
impl PrivateKey {
    pub fn to_public(&self) -> PublicKey {
        self.0.to_public().into()
    }

    pub fn generate_ed25519() -> Result<PrivateKey, JsError> {
        Ok(OsRng)
            .map(crypto::SecretKey::<crypto::Ed25519>::generate)
            .map(key::EitherEd25519SecretKey::Normal)
            .map(PrivateKey)
    }

    pub fn generate_ed25519extended() -> Result<PrivateKey, JsError> {
        Ok(OsRng)
            .map(crypto::SecretKey::<crypto::Ed25519Extended>::generate)
            .map(key::EitherEd25519SecretKey::Extended)
            .map(PrivateKey)
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
        crypto::SecretKey::try_from_bech32_str(&bech32_str)
            .map(key::EitherEd25519SecretKey::Extended)
            .or_else(|_| {
                crypto::SecretKey::try_from_bech32_str(&bech32_str)
                    .map(key::EitherEd25519SecretKey::Normal)
            })
            .map(PrivateKey)
            .map_err(|_| JsError::from_str("Invalid secret key"))
    }

    pub fn to_bech32(&self) -> String {
        match self.0 {
            key::EitherEd25519SecretKey::Normal(ref secret) => secret.to_bech32_str(),
            key::EitherEd25519SecretKey::Extended(ref secret) => secret.to_bech32_str(),
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        match self.0 {
            key::EitherEd25519SecretKey::Normal(ref secret) => secret.as_ref().to_vec(),
            key::EitherEd25519SecretKey::Extended(ref secret) => secret.as_ref().to_vec(),
        }
    }

    pub fn from_extended_bytes(bytes: &[u8]) -> Result<PrivateKey, JsError> {
        crypto::SecretKey::from_binary(bytes)
            .map(key::EitherEd25519SecretKey::Extended)
            .map(PrivateKey)
            .map_err(|_| JsError::from_str("Invalid extended secret key"))
    }

    pub fn from_normal_bytes(bytes: &[u8]) -> Result<PrivateKey, JsError> {
        crypto::SecretKey::from_binary(bytes)
            .map(key::EitherEd25519SecretKey::Normal)
            .map(PrivateKey)
            .map_err(|_| JsError::from_str("Invalid normal secret key"))
    }

    pub fn sign(&self, message: &[u8]) -> Ed25519Signature {
        Ed25519Signature(self.0.sign(&message.to_vec()))
    }
}

/// ED25519 key used as public key
#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct PublicKey(pub(crate) crypto::PublicKey<crypto::Ed25519>);

impl From<crypto::PublicKey<crypto::Ed25519>> for PublicKey {
    fn from(key: crypto::PublicKey<crypto::Ed25519>) -> PublicKey {
        PublicKey(key)
    }
}

#[wasm_bindgen]
impl PublicKey {
    /// Get public key from its bech32 representation
    /// Example:
    /// ```javascript
    /// const pkey = PublicKey.from_bech32(&#39;ed25519_pk1dgaagyh470y66p899txcl3r0jaeaxu6yd7z2dxyk55qcycdml8gszkxze2&#39;);
    /// ```
    pub fn from_bech32(bech32_str: &str) -> Result<PublicKey, JsError> {
        crypto::PublicKey::try_from_bech32_str(&bech32_str)
            .map(PublicKey)
            .map_err(|_| JsError::from_str("Malformed public key"))
    }

    pub fn to_bech32(&self) -> String {
        self.0.to_bech32_str()
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.0.as_ref().to_vec()
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<PublicKey, JsError> {
        crypto::PublicKey::from_binary(bytes)
            .map_err(|e| JsError::from_str(&format!("{}", e)))
            .map(PublicKey)
    }

    pub fn verify(&self, data: &[u8], signature: &Ed25519Signature) -> bool {
        signature.0.verify_slice(&self.0, data) == crypto::Verification::Success
    }

    pub fn hash(&self) -> Ed25519KeyHash {
        Ed25519KeyHash::from(blake2b224(self.as_bytes().as_ref()))
    }
}


#[wasm_bindgen]
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct BootstrapWitness {
    vkey: Vkey,
    signature: Ed25519Signature,
    chain_code: Vec<u8>,
    attributes: AddrAttributes,
}

to_from_bytes!(BootstrapWitness);

to_from_json!(BootstrapWitness);

#[wasm_bindgen]
impl BootstrapWitness {
    pub fn vkey(&self) -> Vkey {
        self.vkey.clone()
    }

    pub fn signature(&self) -> Ed25519Signature {
        self.signature.clone()
    }

    pub fn chain_code(&self) -> Vec<u8> {
        self.chain_code.clone()
    }

    pub fn attributes(&self) -> AddrAttributes {
        self.attributes.clone()
    }

    pub fn new(vkey: &Vkey, signature: &Ed25519Signature, chain_code: Vec<u8>, attributes: &AddrAttributes) -> Self {
        Self {
            vkey: vkey.clone(),
            signature: signature.clone(),
            chain_code,
            attributes: attributes.clone(),
        }
    }

    pub fn to_public_key(&self) -> Result<Bip32PublicKey, JsError> {
        crypto::PublicKey::<crypto::ed25519_derive::Ed25519Bip32>::try_from(self.clone())
            .map(Bip32PublicKey)
            .map_err(|_| JsError::from_str("Invalid public key or byte code"))
    }

    pub fn to_address(&self) -> Result<AddressContent, JsError> {
        AddressContent::try_from(self.clone())
            .map_err(|_| JsError::from_str("Invalid public key or byte code"))
    }
}

impl TryFrom<BootstrapWitness> for crypto::PublicKey<crypto::ed25519_derive::Ed25519Bip32> {
    type Error = ed25519_bip32::PublicKeyError;

    fn try_from(wit: BootstrapWitness) -> Result<Self, Self::Error> {
        combine_pk_and_chaincode(wit.vkey().public_key().0, &wit.chain_code())
    }
}

impl TryFrom<BootstrapWitness> for AddressContent {
    type Error = ed25519_bip32::PublicKeyError;

    fn try_from(wit: BootstrapWitness) -> Result<Self, Self::Error> {
        let protocol_magic = wit.attributes.protocol_magic();
        let key = crypto::PublicKey::<crypto::ed25519_derive::Ed25519Bip32>::try_from(wit)?;
        let address_content = AddressContent::new_simple(&Bip32PublicKey(key), protocol_magic);
        Ok(address_content)
    }
}

impl cbor_event::se::Serialize for BootstrapWitness {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(4))?;
        self.vkey.serialize(serializer)?;
        self.signature.serialize(serializer)?;
        serializer.write_bytes(&self.chain_code)?;
        cbor_event::se::serialize_cbor_in_cbor(&self.attributes, serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for BootstrapWitness {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let ret = Self::deserialize_as_embedded_group(raw, len);
            match len {
                cbor_event::Len::Len(_) => /* TODO: check finite len somewhere */(),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => /* it's ok */(),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("BootstrapWitness"))
    }
}

impl DeserializeEmbeddedGroup for BootstrapWitness {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, _: cbor_event::Len) -> Result<Self, DeserializeError> {
        let vkey = (|| -> Result<_, DeserializeError> {
            Ok(Vkey::deserialize(raw)?)
        })().map_err(|e| e.annotate("vkey"))?;
        let signature = (|| -> Result<_, DeserializeError> {
            Ok(Ed25519Signature::deserialize(raw)?)
        })().map_err(|e| e.annotate("signature"))?;
        let chain_code = (|| -> Result<_, DeserializeError> {
            Ok(raw.bytes()?)
        })().map_err(|e| e.annotate("chain_code"))?;
        let attributes = (|| -> Result<_, DeserializeError> {
            let bytes = raw.bytes()?;
            let mut inner_cbor = Deserializer::from(std::io::Cursor::new(bytes));
            Ok(AddrAttributes::deserialize(&mut inner_cbor)?)
        })().map_err(|e| e.annotate("attributes"))?;
        Ok(BootstrapWitness {
            vkey,
            signature,
            chain_code,
            attributes,
        })
    }
}


impl_signature!(Ed25519Signature, Vec<u8>, crypto::Ed25519);
macro_rules! impl_hash_type {
    ($name:ident, $byte_count:expr) => {
        #[wasm_bindgen]
        #[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(pub (crate) [u8; $byte_count]);

        // hash types are the only types in this library to not expect the entire CBOR structure.
        // There is no CBOR binary tag here just the raw hash bytes.
        from_bytes!($name, bytes, {
            use std::convert::TryInto;
            match bytes.len() {
                $byte_count => Ok($name(bytes[..$byte_count].try_into().unwrap())),
                other_len => {
                    let cbor_error = cbor_event::Error::WrongLen($byte_count, cbor_event::Len::Len(other_len as u64), "hash length");
                    Err(DeserializeError::new(stringify!($name), DeserializeFailure::CBOR(cbor_error)))
                },
            }
        });

        #[wasm_bindgen]
        impl $name {
            // hash types are the only types in this library to not give the entire CBOR structure.
            // There is no CBOR binary tag here just the raw hash bytes.
            pub fn to_bytes(&self) -> Vec<u8> {
                self.0.to_vec()
            }

            pub fn to_bech32(&self, prefix: &str) -> Result<String, JsError> {
                bech32::encode(&prefix, self.to_bytes().to_base32())
                    .map_err(|e| JsError::from_str(&format! {"{:?}", e}))
            }
        
            pub fn from_bech32(bech_str: &str) -> Result<$name, JsError> {
                let (_hrp, u5data) = bech32::decode(bech_str).map_err(|e| JsError::from_str(&e.to_string()))?;
                let data: Vec<u8> = bech32::FromBase32::from_base32(&u5data).unwrap();
                Ok(Self::from_bytes(data)?)
            }

            pub fn to_hex(&self) -> String {
                hex::encode(&self.0)
            }

            pub fn from_hex(hex: &str) -> Result<$name, JsError> {
                let bytes = hex::decode(hex).map_err(|e| JsError::from_str(&format!("hex decode failed: {}", e)))?;
                Self::from_bytes(bytes).map_err(|e| JsError::from_str(&format!("{:?}", e)))
            }
        }

        // associated consts are not supported in wasm_bindgen
        impl $name {
            pub const BYTE_COUNT: usize = $byte_count;
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.to_hex())
            }
        }

        // can't expose [T; N] to wasm for new() but it's useful internally so we implement From trait
        impl From<[u8; $byte_count]> for $name {
            fn from(bytes: [u8; $byte_count]) -> Self {
                Self(bytes)
            }
        }

        impl cbor_event::se::Serialize for $name {
            fn serialize<'se, W: std::io::Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
                serializer.write_bytes(self.0)
            }
        }

        impl Deserialize for $name {
            fn deserialize<R: std::io::BufRead>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
                use std::convert::TryInto;
                (|| -> Result<Self, DeserializeError> {
                    let bytes = raw.bytes()?;
                    if bytes.len() != $byte_count {
                        return Err(DeserializeFailure::CBOR(cbor_event::Error::WrongLen($byte_count, cbor_event::Len::Len(bytes.len() as u64), "hash length")).into());
                    }
                    Ok($name(bytes[..$byte_count].try_into().unwrap()))
                })().map_err(|e| e.annotate(stringify!($name)))
            }
        }

        impl serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where S: serde::Serializer {
                serializer.serialize_str(&self.to_hex())
            }
        }
        
        impl <'de> serde::de::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
            D: serde::de::Deserializer<'de> {
                let s = <String as serde::de::Deserialize>::deserialize(deserializer)?;
                $name::from_hex(&s).map_err(|_e| serde::de::Error::invalid_value(serde::de::Unexpected::Str(&s), &"hex bytes for hash"))
            }
        }
        
        impl JsonSchema for $name {
            fn schema_name() -> String { String::from(stringify!($name)) }
            fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema { String::json_schema(gen) }
            fn is_referenceable() -> bool { String::is_referenceable() }
        }
    }
}
pub(crate) use impl_hash_type;

#[wasm_bindgen]
pub struct LegacyDaedalusPrivateKey(pub (crate) chain_crypto::SecretKey<chain_crypto::LegacyDaedalus>);

#[wasm_bindgen]
impl LegacyDaedalusPrivateKey {
    pub fn from_bytes(bytes: &[u8]) -> Result<LegacyDaedalusPrivateKey, CryptoError> {
        chain_crypto::SecretKey::<chain_crypto::LegacyDaedalus>::from_binary(bytes)
            .map(LegacyDaedalusPrivateKey)
            .map_err(|e| e.into())
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.0.as_ref().to_vec()
    }

    pub fn chaincode(&self) -> Vec<u8> {
        const ED25519_PRIVATE_KEY_LENGTH: usize = 64;
        const XPRV_SIZE: usize = 96;
        self.0.as_ref()[ED25519_PRIVATE_KEY_LENGTH..XPRV_SIZE].to_vec()
    }
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
impl_hash_type!(DataHash, 32);
impl_hash_type!(ScriptDataHash, 32);
// We might want to make these two vkeys normal classes later but for now it's just arbitrary bytes for us (used in block parsing)
impl_hash_type!(VRFVKey, 32);
impl_hash_type!(KESVKey, 32);
// same for this signature
//impl_hash_type!(KESSignature, 448);
