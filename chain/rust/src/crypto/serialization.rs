// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use super::cbor_encodings::*;
use super::*;
use crate::byron::AddrAttributes;
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
            self.public_key.to_raw_bytes(),
            self.encodings
                .as_ref()
                .map(|encs| encs.public_key_encoding.clone())
                .unwrap_or_default()
                .to_str_len_sz(self.public_key.to_raw_bytes().len() as u64, force_canonical),
        )?;
        serializer.write_bytes_sz(
            self.signature.to_raw_bytes(),
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
        let mut attributes_inner_se = Serializer::new_vec();
        // Manual edit: This is from Byron, thus uses cbor_event::Serialize
        cbor_event::Serialize::serialize(&self.attributes, &mut attributes_inner_se)?;
        let attributes_bytes = attributes_inner_se.finalize();
        serializer.write_bytes_sz(
            &attributes_bytes,
            self.encodings
                .as_ref()
                .map(|encs| encs.attributes_bytes_encoding.clone())
                .unwrap_or_default()
                .to_str_len_sz(attributes_bytes.len() as u64, force_canonical),
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
        read_len.finish()?;
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
                .map_err(Into::<DeserializeError>::into)
                .map_err(Into::<DeserializeError>::into)
                .and_then(|(bytes, enc)| {
                    if bytes.len() < 32 || bytes.len() > 32 {
                        Err(DeserializeFailure::RangeCheck {
                            found: bytes.len() as isize,
                            min: Some(32),
                            max: Some(32),
                        }
                        .into())
                    } else {
                        Ok((bytes, StringEncoding::from(enc)))
                    }
                })
                .map_err(|e: DeserializeError| e.annotate("chain_code"))?;
            let (attributes, attributes_bytes_encoding) = (|| -> Result<_, DeserializeError> {
                let (attributes_bytes, attributes_bytes_encoding) = raw.bytes_sz()?;
                let inner_de = &mut Deserializer::from(std::io::Cursor::new(attributes_bytes));
                Ok((
                    AddrAttributes::deserialize(inner_de)?,
                    StringEncoding::from(attributes_bytes_encoding),
                ))
            })()
            .map_err(|e| e.annotate("attributes"))?;
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
                    attributes_bytes_encoding,
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
        if inner.len() != 448 {
            return Err(DeserializeError::new(
                "KESSignature",
                DeserializeFailure::RangeCheck {
                    found: inner.len() as isize,
                    min: Some(448),
                    max: Some(448),
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
            Nonce::Identity {
                identity_encoding,
                len_encoding,
            } => {
                serializer.write_array_sz(len_encoding.to_len_sz(1, force_canonical))?;
                serializer.write_unsigned_integer_sz(
                    0u64,
                    fit_sz(0u64, *identity_encoding, force_canonical),
                )?;
                len_encoding.end(serializer, force_canonical)?;
                Ok(serializer)
            }
            Nonce::Hash {
                hash,
                len_encoding,
                tag_encoding,
                hash_encoding,
            } => {
                serializer.write_array_sz(len_encoding.to_len_sz(2, force_canonical))?;
                serializer.write_unsigned_integer_sz(
                    1u64,
                    fit_sz(1u64, *tag_encoding, force_canonical),
                )?;
                serializer.write_bytes_sz(
                    hash.to_raw_bytes(),
                    hash_encoding.to_str_len_sz(hash.to_raw_bytes().len() as u64, force_canonical),
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
            let initial_position = raw.as_mut_ref().stream_position().unwrap();
            let mut errs = Vec::new();
            let deser_variant = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let mut read_len = CBORReadLen::new(len);
                read_len.read_elems(1)?;
                read_len.finish()?;
                let (identity_value, identity_encoding) = raw.unsigned_integer_sz()?;
                if identity_value != 0 {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(identity_value),
                        expected: Key::Uint(0),
                    }
                    .into());
                }
                let ret = Ok(Some(identity_encoding));
                match len {
                    cbor_event::LenSz::Len(_, _) => (),
                    cbor_event::LenSz::Indefinite => match raw.special()? {
                        cbor_event::Special::Break => (),
                        _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                    },
                }
                ret
            })(raw);
            match deser_variant {
                Ok(identity_encoding) => {
                    return Ok(Self::Identity {
                        identity_encoding,
                        len_encoding,
                    })
                }
                Err(e) => {
                    errs.push(e.annotate("Identity"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let variant_deser = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let mut read_len = CBORReadLen::new(len);
                read_len.read_elems(2)?;
                read_len.finish()?;
                let tag_encoding = (|| -> Result<_, DeserializeError> {
                    let (tag_value, tag_encoding) = raw.unsigned_integer_sz()?;
                    if tag_value != 1 {
                        return Err(DeserializeFailure::FixedValueMismatch {
                            found: Key::Uint(tag_value),
                            expected: Key::Uint(1),
                        }
                        .into());
                    }
                    Ok(Some(tag_encoding))
                })()
                .map_err(|e| e.annotate("tag"))?;
                let (hash, hash_encoding) = raw
                    .bytes_sz()
                    .map_err(Into::<DeserializeError>::into)
                    .and_then(|(bytes, enc)| {
                        NonceHash::from_raw_bytes(&bytes)
                            .map(|bytes| (bytes, StringEncoding::from(enc)))
                            .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
                    })
                    .map_err(|e: DeserializeError| e.annotate("hash"))?;
                match len {
                    cbor_event::LenSz::Len(_, _) => (),
                    cbor_event::LenSz::Indefinite => match raw.special()? {
                        cbor_event::Special::Break => (),
                        _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                    },
                }
                Ok(Self::Hash {
                    hash,
                    len_encoding,
                    tag_encoding,
                    hash_encoding,
                })
            })(raw);
            match variant_deser {
                Ok(variant) => return Ok(variant),
                Err(e) => {
                    errs.push(e.annotate("Hash"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            Err(DeserializeError::new(
                "Nonce",
                DeserializeFailure::NoVariantMatchedWithCauses(errs),
            ))
        })()
        .map_err(|e| e.annotate("Nonce"))
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
            &self.output,
            self.encodings
                .as_ref()
                .map(|encs| encs.output_encoding.clone())
                .unwrap_or_default()
                .to_str_len_sz(self.output.len() as u64, force_canonical),
        )?;
        serializer.write_bytes_sz(
            &self.proof,
            self.encodings
                .as_ref()
                .map(|encs| encs.proof_encoding.clone())
                .unwrap_or_default()
                .to_str_len_sz(self.proof.len() as u64, force_canonical),
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
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let (output, output_encoding) = raw
                .bytes_sz()
                .map_err(Into::<DeserializeError>::into)
                .map(|(bytes, enc)| (bytes, StringEncoding::from(enc)))
                .map_err(|e: DeserializeError| e.annotate("output"))?;
            let (proof, proof_encoding) = raw
                .bytes_sz()
                .map_err(Into::<DeserializeError>::into)
                .map_err(Into::<DeserializeError>::into)
                .and_then(|(bytes, enc)| {
                    if bytes.len() < 80 || bytes.len() > 80 {
                        Err(DeserializeFailure::RangeCheck {
                            found: bytes.len() as isize,
                            min: Some(80),
                            max: Some(80),
                        }
                        .into())
                    } else {
                        Ok((bytes, StringEncoding::from(enc)))
                    }
                })
                .map_err(|e: DeserializeError| e.annotate("proof"))?;
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(VRFCert {
                output,
                proof,
                encodings: Some(VRFCertEncoding {
                    len_encoding,
                    output_encoding,
                    proof_encoding,
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
            self.vkey.to_raw_bytes(),
            self.encodings
                .as_ref()
                .map(|encs| encs.vkey_encoding.clone())
                .unwrap_or_default()
                .to_str_len_sz(self.vkey.to_raw_bytes().len() as u64, force_canonical),
        )?;
        serializer.write_bytes_sz(
            self.ed25519_signature.to_raw_bytes(),
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
        read_len.finish()?;
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
