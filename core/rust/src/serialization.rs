use crate::error::{DeserializeError, DeserializeFailure};
use cbor_event::{de::Deserializer, se::Serializer, Sz};
use std::io::{BufRead, Seek, Write};

pub struct CBORReadLen {
    deser_len: cbor_event::LenSz,
    read: u64,
}

impl CBORReadLen {
    pub fn new(len: cbor_event::LenSz) -> Self {
        Self {
            deser_len: len,
            read: 0,
        }
    }

    pub fn read(&self) -> u64 {
        self.read
    }

    // Marks {n} values as being read, and if we go past the available definite length
    // given by the CBOR, we return an error.
    pub fn read_elems(&mut self, count: usize) -> Result<(), DeserializeFailure> {
        match self.deser_len {
            cbor_event::LenSz::Len(n, _) => {
                self.read += count as u64;
                if self.read > n {
                    Err(DeserializeFailure::DefiniteLenMismatch(n, None))
                } else {
                    Ok(())
                }
            }
            cbor_event::LenSz::Indefinite => Ok(()),
        }
    }

    pub fn finish(&self) -> Result<(), DeserializeFailure> {
        match self.deser_len {
            cbor_event::LenSz::Len(n, _) => {
                if self.read == n {
                    Ok(())
                } else {
                    Err(DeserializeFailure::DefiniteLenMismatch(n, Some(self.read)))
                }
            }
            cbor_event::LenSz::Indefinite => Ok(()),
        }
    }
}

impl From<cbor_event::Len> for CBORReadLen {
    // to facilitate mixing with crates that use preserve-encodings=false to generate
    // we need to create it from cbor_event::Len instead
    fn from(len: cbor_event::Len) -> Self {
        Self::new(len_to_len_sz(len))
    }
}

pub fn len_to_len_sz(len: cbor_event::Len) -> cbor_event::LenSz {
    match len {
        cbor_event::Len::Len(n) => cbor_event::LenSz::Len(n, fit_sz(n, None, true)),
        cbor_event::Len::Indefinite => cbor_event::LenSz::Indefinite,
    }
}

pub trait DeserializeEmbeddedGroup {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        read_len: &mut CBORReadLen,
        len: cbor_event::LenSz,
    ) -> Result<Self, DeserializeError>
    where
        Self: Sized;
}

#[inline]
pub fn sz_max(sz: cbor_event::Sz) -> u64 {
    match sz {
        Sz::Inline => 23u64,
        Sz::One => u8::MAX as u64,
        Sz::Two => u16::MAX as u64,
        Sz::Four => u32::MAX as u64,
        Sz::Eight => u64::MAX,
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum LenEncoding {
    Canonical,
    Definite(cbor_event::Sz),
    Indefinite,
}

impl Default for LenEncoding {
    fn default() -> Self {
        Self::Canonical
    }
}

impl From<cbor_event::LenSz> for LenEncoding {
    fn from(len_sz: cbor_event::LenSz) -> Self {
        match len_sz {
            cbor_event::LenSz::Len(len, sz) => {
                if cbor_event::Sz::canonical(len) == sz {
                    Self::Canonical
                } else {
                    Self::Definite(sz)
                }
            }
            cbor_event::LenSz::Indefinite => Self::Indefinite,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum StringEncoding {
    Canonical,
    Indefinite(Vec<(u64, Sz)>),
    Definite(Sz),
}

impl Default for StringEncoding {
    fn default() -> Self {
        Self::Canonical
    }
}

impl From<cbor_event::StringLenSz> for StringEncoding {
    fn from(len_sz: cbor_event::StringLenSz) -> Self {
        match len_sz {
            cbor_event::StringLenSz::Len(sz) => Self::Definite(sz),
            cbor_event::StringLenSz::Indefinite(lens) => Self::Indefinite(lens),
        }
    }
}

#[inline]
pub fn fit_sz(len: u64, sz: Option<cbor_event::Sz>, force_canonical: bool) -> Sz {
    match sz {
        Some(sz) => {
            if !force_canonical && len <= sz_max(sz) {
                sz
            } else {
                Sz::canonical(len)
            }
        }
        None => Sz::canonical(len),
    }
}

impl LenEncoding {
    pub fn to_len_sz(&self, len: u64, force_canonical: bool) -> cbor_event::LenSz {
        if force_canonical {
            cbor_event::LenSz::Len(len, cbor_event::Sz::canonical(len))
        } else {
            match self {
                Self::Canonical => cbor_event::LenSz::Len(len, cbor_event::Sz::canonical(len)),
                Self::Definite(sz) => {
                    if sz_max(*sz) >= len {
                        cbor_event::LenSz::Len(len, *sz)
                    } else {
                        cbor_event::LenSz::Len(len, cbor_event::Sz::canonical(len))
                    }
                }
                Self::Indefinite => cbor_event::LenSz::Indefinite,
            }
        }
    }

    pub fn end<'a, W: Write + Sized>(
        &self,
        serializer: &'a mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'a mut Serializer<W>> {
        if !force_canonical && *self == Self::Indefinite {
            serializer.write_special(cbor_event::Special::Break)?;
        }
        Ok(serializer)
    }
}

impl StringEncoding {
    pub fn to_str_len_sz(&self, len: u64, force_canonical: bool) -> cbor_event::StringLenSz {
        if force_canonical {
            cbor_event::StringLenSz::Len(cbor_event::Sz::canonical(len))
        } else {
            match self {
                Self::Canonical => cbor_event::StringLenSz::Len(cbor_event::Sz::canonical(len)),
                Self::Definite(sz) => {
                    if sz_max(*sz) >= len {
                        cbor_event::StringLenSz::Len(*sz)
                    } else {
                        cbor_event::StringLenSz::Len(cbor_event::Sz::canonical(len))
                    }
                }
                Self::Indefinite(lens) => cbor_event::StringLenSz::Indefinite(lens.clone()),
            }
        }
    }
}

pub trait Serialize {
    fn serialize<'a, W: Write + Sized>(
        &self,
        serializer: &'a mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'a mut Serializer<W>>;

    /// Bytes of a structure using the CBOR bytes as per the CDDL spec
    /// which for foo = bytes will include the CBOR bytes type/len, etc.
    /// This gives the original bytes in the case where this was created
    /// from bytes originally, or will use whatever the specific encoding
    /// details are present in any encoding details struct for the type.
    fn to_cbor_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf, false).unwrap();
        buf.finalize()
    }

    /// Bytes of a structure using the CBOR bytes as per the CDDL spec
    /// which for foo = bytes will include the CBOR bytes type/len, etc.
    /// This gives the canonically encoded CBOR bytes always
    fn to_canonical_cbor_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf, true).unwrap();
        buf.finalize()
    }
}

pub trait SerializeEmbeddedGroup {
    fn serialize_as_embedded_group<'a, W: Write + Sized>(
        &self,
        serializer: &'a mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'a mut Serializer<W>>;
}

pub trait Deserialize {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError>
    where
        Self: Sized;

    /// from-bytes using the exact CBOR format specified in the CDDL binary spec.
    /// For hashes/addresses/etc this will include the CBOR bytes type/len/etc.
    fn from_cbor_bytes(data: &[u8]) -> Result<Self, DeserializeError>
    where
        Self: Sized,
    {
        let mut raw = Deserializer::from(std::io::Cursor::new(data));
        Self::deserialize(&mut raw)
    }
}

// TODO: remove ToBytes / FromBytes after we regenerate the WASM wrappers.
// This is so the existing generated to/from bytes code works
// We are, however, using this in CIP25 as a way to get to bytes without
// caring about the encoding. We could move it to there or make it more explicit
// that this does not preserve encodings OR do canonical - it's just whatever
// CBOR format. All other parts of CML implement our own Serialize trait with
// the assumption that we preserve encodings. This is based off of cbor_event's
pub trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
}

impl<T: cbor_event::se::Serialize> ToBytes for T {
    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf).unwrap();
        buf.finalize()
    }
}

// TODO: remove ToBytes / FromBytes after we regenerate the WASM wrappers.
// This is just so the existing generated to/from bytes code works
pub trait FromBytes {
    fn from_bytes(data: Vec<u8>) -> Result<Self, DeserializeError>
    where
        Self: Sized;
}

impl<T: Deserialize> FromBytes for T {
    fn from_bytes(data: Vec<u8>) -> Result<Self, DeserializeError>
    where
        Self: Sized,
    {
        let mut raw = Deserializer::from(std::io::Cursor::new(data));
        Self::deserialize(&mut raw).map_err(Into::into)
    }
}
pub trait RawBytesEncoding {
    fn to_raw_bytes(&self) -> &[u8];

    fn from_raw_bytes(bytes: &[u8]) -> Result<Self, DeserializeError>
    where
        Self: Sized;

    fn to_raw_hex(&self) -> String {
        hex::encode(self.to_raw_bytes())
    }

    fn from_raw_hex(hex_str: &str) -> Result<Self, DeserializeError>
    where
        Self: Sized,
    {
        let bytes = hex::decode(hex_str).map_err(|e| {
            DeserializeError::from(DeserializeFailure::InvalidStructure(Box::new(e)))
        })?;
        Self::from_raw_bytes(bytes.as_ref())
    }
}
