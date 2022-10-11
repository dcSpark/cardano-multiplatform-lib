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
pub struct Signature {
    pub inner: Vec<u8>,
    #[serde(skip)]
    pub encodings: Option<SignatureEncoding>,
}

impl Signature {
    pub fn get(&self) -> &Vec<u8> {
        &self.inner
    }

    pub fn new(inner: Vec<u8>) -> Result<Self, DeserializeError> {
        if inner.len() != 16 {
            return Err(DeserializeError::new("Signature", DeserializeFailure::RangeCheck{ found: inner.len(), min: Some(16), max: Some(16) }));
        }
        Ok(Self {
            inner,
            encodings: None,
        })
    }
}

impl TryFrom<Vec<u8>> for Signature {
    type Error = DeserializeError;

    fn try_from(inner: Vec<u8>) -> Result<Self, Self::Error> {
        Signature::new(inner)
    }
}

impl From<Signature> for Vec<u8> {
    fn from(wrapper: Signature) -> Self {
        wrapper.inner
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

use super::*;