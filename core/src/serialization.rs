use crate::error::{DeserializeError, DeserializeFailure};
use std::io::{Seek, BufRead, Write};
use cbor_event::{Sz, de::Deserializer, se::Serializer};

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
            },
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
            },
            cbor_event::LenSz::Indefinite => Ok(()),
        }
    }
}

pub trait DeserializeEmbeddedGroup {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        read_len: &mut CBORReadLen,
        len: cbor_event::LenSz,
    ) -> Result<Self, DeserializeError> where Self: Sized;
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
            cbor_event::LenSz::Len(len, sz) => if cbor_event::Sz::canonical(len) == sz {
                Self::Canonical
            } else {
                Self::Definite(sz)
            },
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
        Some(sz) => if !force_canonical && len <= sz_max(sz) {
            sz
        } else {
            Sz::canonical(len)
        },
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
                Self::Definite(sz) => if sz_max(*sz) >= len {
                    cbor_event::LenSz::Len(len, *sz)
                } else {
                    cbor_event::LenSz::Len(len, cbor_event::Sz::canonical(len))
                },
                Self::Indefinite => cbor_event::LenSz::Indefinite,
            }
        }
    }

    pub fn end<'a, W: Write + Sized>(&self, serializer: &'a mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'a mut Serializer<W>> {
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
                Self::Definite(sz) => if sz_max(*sz) >= len {
                    cbor_event::StringLenSz::Len(*sz)
                } else {
                    cbor_event::StringLenSz::Len(cbor_event::Sz::canonical(len))
                },
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
    /// from bytes originally
    fn to_original_cbor_bytes(&self) -> Vec<u8> {
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

impl<T: cbor_event::se::Serialize> Serialize for T {
    fn serialize<'a, W: Write + Sized>(
        &self,
        serializer: &'a mut Serializer<W>,
        _force_canonical: bool,
    ) -> cbor_event::Result<&'a mut Serializer<W>> {
        <T as cbor_event::se::Serialize>::serialize(self, serializer)
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
    fn deserialize<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
    ) -> Result<Self, DeserializeError> where Self: Sized;

    /// from-bytes using the exact CBOR format specified in the CDDL binary spec.
    /// For hashes/addresses/etc this will include the CBOR bytes type/len/etc.
    fn from_cbor_bytes(data: &[u8]) -> Result<Self, DeserializeError> where Self: Sized {
        let mut raw = Deserializer::from(std::io::Cursor::new(data));
        Self::deserialize(&mut raw)
    }
}

impl<T: cbor_event::de::Deserialize> Deserialize for T {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<T, DeserializeError> {
        T::deserialize(raw).map_err(|e| DeserializeError::from(e))
    }
}

// TODO: remove ToBytes / FromBytes after we regenerate the WASM wrappers.
// This is just so the existing generated to/from bytes code works
pub trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
}

impl<T: Serialize> ToBytes for T {
    fn to_bytes(&self) -> Vec<u8> {
        Serialize::to_original_cbor_bytes(self)
    }
}

pub trait FromBytes {
    fn from_bytes(data: Vec<u8>) -> Result<Self, DeserializeError> where Self: Sized;
}

impl<T: Deserialize> FromBytes for T {
    fn from_bytes(data: Vec<u8>) -> Result<Self, DeserializeError> where Self: Sized {
        Deserialize::from_cbor_bytes(data.as_ref())
    }
}