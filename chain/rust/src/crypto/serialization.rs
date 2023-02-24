// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use super::cbor_encodings::*;
use super::*;
use cbor_event::de::Deserializer;
use cbor_event::se::Serializer;
use cml_core::error::*;
use cml_core::serialization::*;
use cml_crypto::RawBytesEncoding;
use std::io::{BufRead, Seek, SeekFrom, Write};

impl Serialize for BootstrapWitness {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(
            self.encodings
                .as_ref()
                .map(|encs| encs.len_encoding)
                .unwrap_or_default()
                .to_len_sz(4, force_canonical),
        )?;
        serializer.write_bytes_sz(
            &self.public_key.to_raw_bytes(),
            self.encodings
                .as_ref()
                .map(|encs| encs.public_key_encoding.clone())
                .unwrap_or_default()
                .to_str_len_sz(self.public_key.to_raw_bytes().len() as u64, force_canonical),
        )?;
        serializer.write_bytes_sz(
            &self.signature.to_raw_bytes(),
            self.encodings
                .as_ref()
                .map(|encs| encs.signature_encoding.clone())
                .unwrap_or_default()
                .to_str_len_sz(self.signature.to_raw_bytes().len() as u64, force_canonical),
        )?;
        serializer.write_bytes_sz(
            &self.chain_code,
            self.encodings
                .as_ref()
                .map(|encs| encs.chain_code_encoding.clone())
                .unwrap_or_default()
                .to_str_len_sz(self.chain_code.len() as u64, force_canonical),
        )?;
        serializer.write_bytes_sz(
            &self.attributes,
            self.encodings
                .as_ref()
                .map(|encs| encs.attributes_encoding.clone())
                .unwrap_or_default()
                .to_str_len_sz(self.attributes.len() as u64, force_canonical),
        )?;
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for BootstrapWitness {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let len_encoding: LenEncoding = len.into();
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(4)?;
        (|| -> Result<_, DeserializeError> {
            let (public_key, public_key_encoding) = raw
                .bytes_sz()
                .map_err(Into::<DeserializeError>::into)
                .and_then(|(bytes, enc)| {
                    Vkey::from_raw_bytes(&bytes)
                        .map(|bytes| (bytes, StringEncoding::from(enc)))
                        .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
                })
                .map_err(|e: DeserializeError| e.annotate("public_key"))?;
            let (signature, signature_encoding) = raw
                .bytes_sz()
                .map_err(Into::<DeserializeError>::into)
                .and_then(|(bytes, enc)| {
                    Ed25519Signature::from_raw_bytes(&bytes)
                        .map(|bytes| (bytes, StringEncoding::from(enc)))
                        .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
                })
                .map_err(|e: DeserializeError| e.annotate("signature"))?;
            let (chain_code, chain_code_encoding) = raw
                .bytes_sz()
                .map(|(bytes, enc)| (bytes, StringEncoding::from(enc)))
                .map_err(Into::<DeserializeError>::into)
                .map_err(|e: DeserializeError| e.annotate("chain_code"))?;
            let (attributes, attributes_encoding) = raw
                .bytes_sz()
                .map(|(bytes, enc)| (bytes, StringEncoding::from(enc)))
                .map_err(Into::<DeserializeError>::into)
                .map_err(|e: DeserializeError| e.annotate("attributes"))?;
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(BootstrapWitness {
                public_key,
                signature,
                chain_code,
                attributes,
                encodings: Some(BootstrapWitnessEncoding {
                    len_encoding,
                    public_key_encoding,
                    signature_encoding,
                    chain_code_encoding,
                    attributes_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("BootstrapWitness"))
    }
}

impl Serialize for KESSignature {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_bytes_sz(
            &self.inner,
            self.encodings
                .as_ref()
                .map(|encs| encs.inner_encoding.clone())
                .unwrap_or_default()
                .to_str_len_sz(self.inner.len() as u64, force_canonical),
        )
    }
}

impl Deserialize for KESSignature {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let (inner, inner_encoding) = raw
            .bytes_sz()
            .map(|(bytes, enc)| (bytes, StringEncoding::from(enc)))?;
        if inner.len() != 32 {
            return Err(DeserializeError::new(
                "KESSignature",
                DeserializeFailure::RangeCheck {
                    found: inner.len(),
                    min: Some(32),
                    max: Some(32),
                },
            ));
        }
        Ok(Self {
            inner,
            encodings: Some(KESSignatureEncoding { inner_encoding }),
        })
    }
}

impl Serialize for Nonce {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            Nonce::I0 {
                i0_encoding,
                len_encoding,
            } => {
                serializer.write_array_sz(len_encoding.to_len_sz(1, force_canonical))?;
                serializer
                    .write_unsigned_integer_sz(0u64, fit_sz(0u64, *i0_encoding, force_canonical))?;
                len_encoding.end(serializer, force_canonical)?;
                Ok(serializer)
            }
            Nonce::Nonce1 {
                bytes,
                len_encoding,
                index_0_encoding,
                bytes_encoding,
            } => {
                serializer.write_array_sz(len_encoding.to_len_sz(2, force_canonical))?;
                serializer.write_unsigned_integer_sz(
                    1u64,
                    fit_sz(1u64, *index_0_encoding, force_canonical),
                )?;
                serializer.write_bytes_sz(
                    &bytes,
                    bytes_encoding.to_str_len_sz(bytes.len() as u64, force_canonical),
                )?;
                len_encoding.end(serializer, force_canonical)?;
                Ok(serializer)
            }
        }
    }
}

impl Deserialize for Nonce {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array_sz()?;
            let len_encoding: LenEncoding = len.into();
            let mut read_len = CBORReadLen::new(len);
            let initial_position = raw.as_mut_ref().seek(SeekFrom::Current(0)).unwrap();
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let (i0_value, i0_encoding) = raw.unsigned_integer_sz()?;
                if i0_value != 0 {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(i0_value),
                        expected: Key::Uint(0),
                    }
                    .into());
                }
                Ok(Some(i0_encoding))
            })(raw)
            {
                Ok(i0_encoding) => {
                    return Ok(Self::I0 {
                        i0_encoding,
                        len_encoding,
                    })
                }
                Err(_) => raw
                    .as_mut_ref()
                    .seek(SeekFrom::Start(initial_position))
                    .unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let index_0_encoding = (|| -> Result<_, DeserializeError> {
                    let (index_0_value, index_0_encoding) = raw.unsigned_integer_sz()?;
                    if index_0_value != 1 {
                        return Err(DeserializeFailure::FixedValueMismatch {
                            found: Key::Uint(index_0_value),
                            expected: Key::Uint(1),
                        }
                        .into());
                    }
                    Ok(Some(index_0_encoding))
                })()
                .map_err(|e| e.annotate("index_0"))?;
                let (bytes, bytes_encoding) = raw
                    .bytes_sz()
                    .map(|(bytes, enc)| (bytes, StringEncoding::from(enc)))
                    .map_err(Into::<DeserializeError>::into)
                    .map_err(|e: DeserializeError| e.annotate("bytes"))?;
                match len {
                    cbor_event::LenSz::Len(_, _) => (),
                    cbor_event::LenSz::Indefinite => match raw.special()? {
                        cbor_event::Special::Break => (),
                        _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                    },
                }
                Ok(Self::Nonce1 {
                    bytes,
                    len_encoding,
                    index_0_encoding,
                    bytes_encoding,
                })
            })(raw)
            {
                Ok(variant) => return Ok(variant),
                Err(_) => raw
                    .as_mut_ref()
                    .seek(SeekFrom::Start(initial_position))
                    .unwrap(),
            };
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Err(DeserializeError::new(
                "Nonce",
                DeserializeFailure::NoVariantMatched,
            ))
        })()
        .map_err(|e| e.annotate("Nonce"))
    }
}

impl Serialize for SignkeyKES {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_bytes_sz(
            &self.inner,
            self.encodings
                .as_ref()
                .map(|encs| encs.inner_encoding.clone())
                .unwrap_or_default()
                .to_str_len_sz(self.inner.len() as u64, force_canonical),
        )
    }
}

impl Deserialize for SignkeyKES {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let (inner, inner_encoding) = raw
            .bytes_sz()
            .map(|(bytes, enc)| (bytes, StringEncoding::from(enc)))?;
        if inner.len() != 16 {
            return Err(DeserializeError::new(
                "SignkeyKES",
                DeserializeFailure::RangeCheck {
                    found: inner.len(),
                    min: Some(16),
                    max: Some(16),
                },
            ));
        }
        Ok(Self {
            inner,
            encodings: Some(SignkeyKESEncoding { inner_encoding }),
        })
    }
}

impl Serialize for VRFCert {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(
            self.encodings
                .as_ref()
                .map(|encs| encs.len_encoding)
                .unwrap_or_default()
                .to_len_sz(2, force_canonical),
        )?;
        serializer.write_bytes_sz(
            &self.index_0,
            self.encodings
                .as_ref()
                .map(|encs| encs.index_0_encoding.clone())
                .unwrap_or_default()
                .to_str_len_sz(self.index_0.len() as u64, force_canonical),
        )?;
        serializer.write_bytes_sz(
            &self.bytes,
            self.encodings
                .as_ref()
                .map(|encs| encs.bytes_encoding.clone())
                .unwrap_or_default()
                .to_str_len_sz(self.bytes.len() as u64, force_canonical),
        )?;
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for VRFCert {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let len_encoding: LenEncoding = len.into();
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        (|| -> Result<_, DeserializeError> {
            let (index_0, index_0_encoding) = raw
                .bytes_sz()
                .map(|(bytes, enc)| (bytes, StringEncoding::from(enc)))
                .map_err(Into::<DeserializeError>::into)
                .map_err(|e: DeserializeError| e.annotate("index_0"))?;
            let (bytes, bytes_encoding) = raw
                .bytes_sz()
                .map(|(bytes, enc)| (bytes, StringEncoding::from(enc)))
                .map_err(Into::<DeserializeError>::into)
                .map_err(|e: DeserializeError| e.annotate("bytes"))?;
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(VRFCert {
                index_0,
                bytes,
                encodings: Some(VRFCertEncoding {
                    len_encoding,
                    index_0_encoding,
                    bytes_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("VRFCert"))
    }
}

impl Serialize for Vkeywitness {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(
            self.encodings
                .as_ref()
                .map(|encs| encs.len_encoding)
                .unwrap_or_default()
                .to_len_sz(2, force_canonical),
        )?;
        serializer.write_bytes_sz(
            &self.vkey.to_raw_bytes(),
            self.encodings
                .as_ref()
                .map(|encs| encs.vkey_encoding.clone())
                .unwrap_or_default()
                .to_str_len_sz(self.vkey.to_raw_bytes().len() as u64, force_canonical),
        )?;
        serializer.write_bytes_sz(
            &self.ed25519_signature.to_raw_bytes(),
            self.encodings
                .as_ref()
                .map(|encs| encs.ed25519_signature_encoding.clone())
                .unwrap_or_default()
                .to_str_len_sz(
                    self.ed25519_signature.to_raw_bytes().len() as u64,
                    force_canonical,
                ),
        )?;
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for Vkeywitness {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let len_encoding: LenEncoding = len.into();
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        (|| -> Result<_, DeserializeError> {
            let (vkey, vkey_encoding) = raw
                .bytes_sz()
                .map_err(Into::<DeserializeError>::into)
                .and_then(|(bytes, enc)| {
                    Vkey::from_raw_bytes(&bytes)
                        .map(|bytes| (bytes, StringEncoding::from(enc)))
                        .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
                })
                .map_err(|e: DeserializeError| e.annotate("vkey"))?;
            let (ed25519_signature, ed25519_signature_encoding) = raw
                .bytes_sz()
                .map_err(Into::<DeserializeError>::into)
                .and_then(|(bytes, enc)| {
                    Ed25519Signature::from_raw_bytes(&bytes)
                        .map(|bytes| (bytes, StringEncoding::from(enc)))
                        .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
                })
                .map_err(|e: DeserializeError| e.annotate("ed25519_signature"))?;
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(Vkeywitness {
                vkey,
                ed25519_signature,
                encodings: Some(VkeywitnessEncoding {
                    len_encoding,
                    vkey_encoding,
                    ed25519_signature_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("Vkeywitness"))
    }
}
