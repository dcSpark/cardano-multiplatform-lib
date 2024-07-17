// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use super::cbor_encodings::*;
use super::*;
use cbor_event;
use cbor_event::de::Deserializer;
use cbor_event::se::Serializer;
use cml_chain::address::RewardAccount;
use cml_chain::certs::{PoolMetadata, SingleHostAddr};
use cml_core::error::*;
use cml_core::serialization::*;
use cml_crypto::GenesisDelegateHash;
use cml_crypto::RawBytesEncoding;
use cml_crypto::VRFKeyHash;
use std::io::{BufRead, Seek, SeekFrom, Write};

impl Serialize for GenesisKeyDelegation {
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
        self.serialize_as_embedded_group(serializer, force_canonical)
    }
}

impl SerializeEmbeddedGroup for GenesisKeyDelegation {
    fn serialize_as_embedded_group<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer_sz(
            5u64,
            fit_sz(
                5u64,
                self.encodings
                    .as_ref()
                    .map(|encs| encs.tag_encoding)
                    .unwrap_or_default(),
                force_canonical,
            ),
        )?;
        serializer.write_bytes_sz(
            self.genesis_hash.to_raw_bytes(),
            self.encodings
                .as_ref()
                .map(|encs| encs.genesis_hash_encoding.clone())
                .unwrap_or_default()
                .to_str_len_sz(
                    self.genesis_hash.to_raw_bytes().len() as u64,
                    force_canonical,
                ),
        )?;
        serializer.write_bytes_sz(
            self.genesis_delegate_hash.to_raw_bytes(),
            self.encodings
                .as_ref()
                .map(|encs| encs.genesis_delegate_hash_encoding.clone())
                .unwrap_or_default()
                .to_str_len_sz(
                    self.genesis_delegate_hash.to_raw_bytes().len() as u64,
                    force_canonical,
                ),
        )?;
        serializer.write_bytes_sz(
            self.vrf_key_hash.to_raw_bytes(),
            self.encodings
                .as_ref()
                .map(|encs| encs.vrf_key_hash_encoding.clone())
                .unwrap_or_default()
                .to_str_len_sz(
                    self.vrf_key_hash.to_raw_bytes().len() as u64,
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

impl Deserialize for GenesisKeyDelegation {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(4)?;
        read_len.finish()?;
        let ret = Self::deserialize_as_embedded_group(raw, &mut read_len, len);
        match len {
            cbor_event::LenSz::Len(_, _) => (),
            cbor_event::LenSz::Indefinite => match raw.special()? {
                cbor_event::Special::Break => (),
                _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
            },
        }
        ret
    }
}

impl DeserializeEmbeddedGroup for GenesisKeyDelegation {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        _read_len: &mut CBORReadLen,
        len: cbor_event::LenSz,
    ) -> Result<Self, DeserializeError> {
        let len_encoding = len.into();
        (|| -> Result<_, DeserializeError> {
            let tag_encoding = (|| -> Result<_, DeserializeError> {
                let (tag_value, tag_encoding) = raw.unsigned_integer_sz()?;
                if tag_value != 5 {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(tag_value),
                        expected: Key::Uint(5),
                    }
                    .into());
                }
                Ok(Some(tag_encoding))
            })()
            .map_err(|e| e.annotate("tag"))?;
            let (genesis_hash, genesis_hash_encoding) = raw
                .bytes_sz()
                .map_err(Into::<DeserializeError>::into)
                .and_then(|(bytes, enc)| {
                    GenesisHash::from_raw_bytes(&bytes)
                        .map(|bytes| (bytes, StringEncoding::from(enc)))
                        .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
                })
                .map_err(|e: DeserializeError| e.annotate("genesis_hash"))?;
            let (genesis_delegate_hash, genesis_delegate_hash_encoding) = raw
                .bytes_sz()
                .map_err(Into::<DeserializeError>::into)
                .and_then(|(bytes, enc)| {
                    GenesisDelegateHash::from_raw_bytes(&bytes)
                        .map(|bytes| (bytes, StringEncoding::from(enc)))
                        .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
                })
                .map_err(|e: DeserializeError| e.annotate("genesis_delegate_hash"))?;
            let (vrf_key_hash, vrf_key_hash_encoding) = raw
                .bytes_sz()
                .map_err(Into::<DeserializeError>::into)
                .and_then(|(bytes, enc)| {
                    VRFKeyHash::from_raw_bytes(&bytes)
                        .map(|bytes| (bytes, StringEncoding::from(enc)))
                        .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
                })
                .map_err(|e: DeserializeError| e.annotate("vrf_key_hash"))?;
            Ok(GenesisKeyDelegation {
                genesis_hash,
                genesis_delegate_hash,
                vrf_key_hash,
                encodings: Some(GenesisKeyDelegationEncoding {
                    len_encoding,
                    tag_encoding,
                    genesis_hash_encoding,
                    genesis_delegate_hash_encoding,
                    vrf_key_hash_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("GenesisKeyDelegation"))
    }
}

impl Serialize for MultisigAll {
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
        self.serialize_as_embedded_group(serializer, force_canonical)
    }
}

impl SerializeEmbeddedGroup for MultisigAll {
    fn serialize_as_embedded_group<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer_sz(
            1u64,
            fit_sz(
                1u64,
                self.encodings
                    .as_ref()
                    .map(|encs| encs.tag_encoding)
                    .unwrap_or_default(),
                force_canonical,
            ),
        )?;
        serializer.write_array_sz(
            self.encodings
                .as_ref()
                .map(|encs| encs.multisig_scripts_encoding)
                .unwrap_or_default()
                .to_len_sz(self.multisig_scripts.len() as u64, force_canonical),
        )?;
        for element in self.multisig_scripts.iter() {
            element.serialize(serializer, force_canonical)?;
        }
        self.encodings
            .as_ref()
            .map(|encs| encs.multisig_scripts_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)?;
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for MultisigAll {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        let ret = Self::deserialize_as_embedded_group(raw, &mut read_len, len);
        match len {
            cbor_event::LenSz::Len(_, _) => (),
            cbor_event::LenSz::Indefinite => match raw.special()? {
                cbor_event::Special::Break => (),
                _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
            },
        }
        ret
    }
}

impl DeserializeEmbeddedGroup for MultisigAll {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        _read_len: &mut CBORReadLen,
        len: cbor_event::LenSz,
    ) -> Result<Self, DeserializeError> {
        let len_encoding = len.into();
        (|| -> Result<_, DeserializeError> {
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
            let (multisig_scripts, multisig_scripts_encoding) =
                (|| -> Result<_, DeserializeError> {
                    let mut multisig_scripts_arr = Vec::new();
                    let len = raw.array_sz()?;
                    let multisig_scripts_encoding = len.into();
                    while match len {
                        cbor_event::LenSz::Len(n, _) => (multisig_scripts_arr.len() as u64) < n,
                        cbor_event::LenSz::Indefinite => true,
                    } {
                        if raw.cbor_type()? == cbor_event::Type::Special {
                            assert_eq!(raw.special()?, cbor_event::Special::Break);
                            break;
                        }
                        multisig_scripts_arr.push(MultisigScript::deserialize(raw)?);
                    }
                    Ok((multisig_scripts_arr, multisig_scripts_encoding))
                })()
                .map_err(|e| e.annotate("multisig_scripts"))?;
            Ok(MultisigAll {
                multisig_scripts,
                encodings: Some(MultisigAllEncoding {
                    len_encoding,
                    tag_encoding,
                    multisig_scripts_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("MultisigAll"))
    }
}

impl Serialize for MultisigAny {
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
        self.serialize_as_embedded_group(serializer, force_canonical)
    }
}

impl SerializeEmbeddedGroup for MultisigAny {
    fn serialize_as_embedded_group<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer_sz(
            2u64,
            fit_sz(
                2u64,
                self.encodings
                    .as_ref()
                    .map(|encs| encs.tag_encoding)
                    .unwrap_or_default(),
                force_canonical,
            ),
        )?;
        serializer.write_array_sz(
            self.encodings
                .as_ref()
                .map(|encs| encs.multisig_scripts_encoding)
                .unwrap_or_default()
                .to_len_sz(self.multisig_scripts.len() as u64, force_canonical),
        )?;
        for element in self.multisig_scripts.iter() {
            element.serialize(serializer, force_canonical)?;
        }
        self.encodings
            .as_ref()
            .map(|encs| encs.multisig_scripts_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)?;
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for MultisigAny {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        let ret = Self::deserialize_as_embedded_group(raw, &mut read_len, len);
        match len {
            cbor_event::LenSz::Len(_, _) => (),
            cbor_event::LenSz::Indefinite => match raw.special()? {
                cbor_event::Special::Break => (),
                _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
            },
        }
        ret
    }
}

impl DeserializeEmbeddedGroup for MultisigAny {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        _read_len: &mut CBORReadLen,
        len: cbor_event::LenSz,
    ) -> Result<Self, DeserializeError> {
        let len_encoding = len.into();
        (|| -> Result<_, DeserializeError> {
            let tag_encoding = (|| -> Result<_, DeserializeError> {
                let (tag_value, tag_encoding) = raw.unsigned_integer_sz()?;
                if tag_value != 2 {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(tag_value),
                        expected: Key::Uint(2),
                    }
                    .into());
                }
                Ok(Some(tag_encoding))
            })()
            .map_err(|e| e.annotate("tag"))?;
            let (multisig_scripts, multisig_scripts_encoding) =
                (|| -> Result<_, DeserializeError> {
                    let mut multisig_scripts_arr = Vec::new();
                    let len = raw.array_sz()?;
                    let multisig_scripts_encoding = len.into();
                    while match len {
                        cbor_event::LenSz::Len(n, _) => (multisig_scripts_arr.len() as u64) < n,
                        cbor_event::LenSz::Indefinite => true,
                    } {
                        if raw.cbor_type()? == cbor_event::Type::Special {
                            assert_eq!(raw.special()?, cbor_event::Special::Break);
                            break;
                        }
                        multisig_scripts_arr.push(MultisigScript::deserialize(raw)?);
                    }
                    Ok((multisig_scripts_arr, multisig_scripts_encoding))
                })()
                .map_err(|e| e.annotate("multisig_scripts"))?;
            Ok(MultisigAny {
                multisig_scripts,
                encodings: Some(MultisigAnyEncoding {
                    len_encoding,
                    tag_encoding,
                    multisig_scripts_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("MultisigAny"))
    }
}

impl Serialize for MultisigNOfK {
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
                .to_len_sz(3, force_canonical),
        )?;
        self.serialize_as_embedded_group(serializer, force_canonical)
    }
}

impl SerializeEmbeddedGroup for MultisigNOfK {
    fn serialize_as_embedded_group<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer_sz(
            3u64,
            fit_sz(
                3u64,
                self.encodings
                    .as_ref()
                    .map(|encs| encs.tag_encoding)
                    .unwrap_or_default(),
                force_canonical,
            ),
        )?;
        serializer.write_unsigned_integer_sz(
            self.n,
            fit_sz(
                self.n,
                self.encodings
                    .as_ref()
                    .map(|encs| encs.n_encoding)
                    .unwrap_or_default(),
                force_canonical,
            ),
        )?;
        serializer.write_array_sz(
            self.encodings
                .as_ref()
                .map(|encs| encs.multisig_scripts_encoding)
                .unwrap_or_default()
                .to_len_sz(self.multisig_scripts.len() as u64, force_canonical),
        )?;
        for element in self.multisig_scripts.iter() {
            element.serialize(serializer, force_canonical)?;
        }
        self.encodings
            .as_ref()
            .map(|encs| encs.multisig_scripts_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)?;
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for MultisigNOfK {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(3)?;
        read_len.finish()?;
        let ret = Self::deserialize_as_embedded_group(raw, &mut read_len, len);
        match len {
            cbor_event::LenSz::Len(_, _) => (),
            cbor_event::LenSz::Indefinite => match raw.special()? {
                cbor_event::Special::Break => (),
                _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
            },
        }
        ret
    }
}

impl DeserializeEmbeddedGroup for MultisigNOfK {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        _read_len: &mut CBORReadLen,
        len: cbor_event::LenSz,
    ) -> Result<Self, DeserializeError> {
        let len_encoding = len.into();
        (|| -> Result<_, DeserializeError> {
            let tag_encoding = (|| -> Result<_, DeserializeError> {
                let (tag_value, tag_encoding) = raw.unsigned_integer_sz()?;
                if tag_value != 3 {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(tag_value),
                        expected: Key::Uint(3),
                    }
                    .into());
                }
                Ok(Some(tag_encoding))
            })()
            .map_err(|e| e.annotate("tag"))?;
            let (n, n_encoding) = raw
                .unsigned_integer_sz()
                .map_err(Into::<DeserializeError>::into)
                .map(|(x, enc)| (x, Some(enc)))
                .map_err(|e: DeserializeError| e.annotate("n"))?;
            let (multisig_scripts, multisig_scripts_encoding) =
                (|| -> Result<_, DeserializeError> {
                    let mut multisig_scripts_arr = Vec::new();
                    let len = raw.array_sz()?;
                    let multisig_scripts_encoding = len.into();
                    while match len {
                        cbor_event::LenSz::Len(n, _) => (multisig_scripts_arr.len() as u64) < n,
                        cbor_event::LenSz::Indefinite => true,
                    } {
                        if raw.cbor_type()? == cbor_event::Type::Special {
                            assert_eq!(raw.special()?, cbor_event::Special::Break);
                            break;
                        }
                        multisig_scripts_arr.push(MultisigScript::deserialize(raw)?);
                    }
                    Ok((multisig_scripts_arr, multisig_scripts_encoding))
                })()
                .map_err(|e| e.annotate("multisig_scripts"))?;
            Ok(MultisigNOfK {
                n,
                multisig_scripts,
                encodings: Some(MultisigNOfKEncoding {
                    len_encoding,
                    tag_encoding,
                    n_encoding,
                    multisig_scripts_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("MultisigNOfK"))
    }
}

impl Serialize for MultisigPubkey {
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
        self.serialize_as_embedded_group(serializer, force_canonical)
    }
}

impl SerializeEmbeddedGroup for MultisigPubkey {
    fn serialize_as_embedded_group<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer_sz(
            0u64,
            fit_sz(
                0u64,
                self.encodings
                    .as_ref()
                    .map(|encs| encs.tag_encoding)
                    .unwrap_or_default(),
                force_canonical,
            ),
        )?;
        serializer.write_bytes_sz(
            self.ed25519_key_hash.to_raw_bytes(),
            self.encodings
                .as_ref()
                .map(|encs| encs.ed25519_key_hash_encoding.clone())
                .unwrap_or_default()
                .to_str_len_sz(
                    self.ed25519_key_hash.to_raw_bytes().len() as u64,
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

impl Deserialize for MultisigPubkey {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        let ret = Self::deserialize_as_embedded_group(raw, &mut read_len, len);
        match len {
            cbor_event::LenSz::Len(_, _) => (),
            cbor_event::LenSz::Indefinite => match raw.special()? {
                cbor_event::Special::Break => (),
                _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
            },
        }
        ret
    }
}

impl DeserializeEmbeddedGroup for MultisigPubkey {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        _read_len: &mut CBORReadLen,
        len: cbor_event::LenSz,
    ) -> Result<Self, DeserializeError> {
        let len_encoding = len.into();
        (|| -> Result<_, DeserializeError> {
            let tag_encoding = (|| -> Result<_, DeserializeError> {
                let (tag_value, tag_encoding) = raw.unsigned_integer_sz()?;
                if tag_value != 0 {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(tag_value),
                        expected: Key::Uint(0),
                    }
                    .into());
                }
                Ok(Some(tag_encoding))
            })()
            .map_err(|e| e.annotate("tag"))?;
            let (ed25519_key_hash, ed25519_key_hash_encoding) = raw
                .bytes_sz()
                .map_err(Into::<DeserializeError>::into)
                .and_then(|(bytes, enc)| {
                    Ed25519KeyHash::from_raw_bytes(&bytes)
                        .map(|bytes| (bytes, StringEncoding::from(enc)))
                        .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
                })
                .map_err(|e: DeserializeError| e.annotate("ed25519_key_hash"))?;
            Ok(MultisigPubkey {
                ed25519_key_hash,
                encodings: Some(MultisigPubkeyEncoding {
                    len_encoding,
                    tag_encoding,
                    ed25519_key_hash_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("MultisigPubkey"))
    }
}

impl Serialize for MultisigScript {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            MultisigScript::MultisigPubkey(multisig_pubkey) => {
                multisig_pubkey.serialize(serializer, force_canonical)
            }
            MultisigScript::MultisigAll(multisig_all) => {
                multisig_all.serialize(serializer, force_canonical)
            }
            MultisigScript::MultisigAny(multisig_any) => {
                multisig_any.serialize(serializer, force_canonical)
            }
            MultisigScript::MultisigNOfK(multisig_n_of_k) => {
                multisig_n_of_k.serialize(serializer, force_canonical)
            }
        }
    }
}

impl Deserialize for MultisigScript {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array_sz()?;
            let initial_position = raw.as_mut_ref().stream_position().unwrap();
            let mut errs = Vec::new();
            let deser_variant = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let mut read_len = CBORReadLen::new(len);
                read_len.read_elems(2)?;
                read_len.finish()?;
                let ret = MultisigPubkey::deserialize_as_embedded_group(raw, &mut read_len, len);
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
                Ok(multisig_pubkey) => return Ok(Self::MultisigPubkey(multisig_pubkey)),
                Err(e) => {
                    errs.push(e.annotate("MultisigPubkey"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let mut read_len = CBORReadLen::new(len);
                read_len.read_elems(2)?;
                read_len.finish()?;
                let ret = MultisigAll::deserialize_as_embedded_group(raw, &mut read_len, len);
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
                Ok(multisig_all) => return Ok(Self::MultisigAll(multisig_all)),
                Err(e) => {
                    errs.push(e.annotate("MultisigAll"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let mut read_len = CBORReadLen::new(len);
                read_len.read_elems(2)?;
                read_len.finish()?;
                let ret = MultisigAny::deserialize_as_embedded_group(raw, &mut read_len, len);
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
                Ok(multisig_any) => return Ok(Self::MultisigAny(multisig_any)),
                Err(e) => {
                    errs.push(e.annotate("MultisigAny"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let mut read_len = CBORReadLen::new(len);
                read_len.read_elems(3)?;
                read_len.finish()?;
                let ret = MultisigNOfK::deserialize_as_embedded_group(raw, &mut read_len, len);
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
                Ok(multisig_n_of_k) => return Ok(Self::MultisigNOfK(multisig_n_of_k)),
                Err(e) => {
                    errs.push(e.annotate("MultisigNOfK"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            Err(DeserializeError::new(
                "MultisigScript",
                DeserializeFailure::NoVariantMatchedWithCauses(errs),
            ))
        })()
        .map_err(|e| e.annotate("MultisigScript"))
    }
}

impl Serialize for ProtocolVersionStruct {
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
        self.protocol_version
            .serialize_as_embedded_group(serializer, force_canonical)?;
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for ProtocolVersionStruct {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let len_encoding: LenEncoding = len.into();
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let protocol_version =
                ProtocolVersion::deserialize_as_embedded_group(raw, &mut read_len, len)
                    .map_err(|e: DeserializeError| e.annotate("protocol_version"))?;
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(ProtocolVersionStruct {
                protocol_version,
                encodings: Some(ProtocolVersionStructEncoding { len_encoding }),
            })
        })()
        .map_err(|e| e.annotate("ProtocolVersionStruct"))
    }
}

impl Serialize for ShelleyBlock {
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
        self.header.serialize(serializer, force_canonical)?;
        serializer.write_array_sz(
            self.encodings
                .as_ref()
                .map(|encs| encs.transaction_bodies_encoding)
                .unwrap_or_default()
                .to_len_sz(self.transaction_bodies.len() as u64, force_canonical),
        )?;
        for element in self.transaction_bodies.iter() {
            element.serialize(serializer, force_canonical)?;
        }
        self.encodings
            .as_ref()
            .map(|encs| encs.transaction_bodies_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)?;
        serializer.write_array_sz(
            self.encodings
                .as_ref()
                .map(|encs| encs.transaction_witness_sets_encoding)
                .unwrap_or_default()
                .to_len_sz(self.transaction_witness_sets.len() as u64, force_canonical),
        )?;
        for element in self.transaction_witness_sets.iter() {
            element.serialize(serializer, force_canonical)?;
        }
        self.encodings
            .as_ref()
            .map(|encs| encs.transaction_witness_sets_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)?;
        serializer.write_map_sz(
            self.encodings
                .as_ref()
                .map(|encs| encs.transaction_metadata_set_encoding)
                .unwrap_or_default()
                .to_len_sz(self.transaction_metadata_set.len() as u64, force_canonical),
        )?;
        let mut key_order = self
            .transaction_metadata_set
            .iter()
            .map(|(k, v)| {
                let mut buf = cbor_event::se::Serializer::new_vec();
                let transaction_metadata_set_key_encoding = self
                    .encodings
                    .as_ref()
                    .and_then(|encs| encs.transaction_metadata_set_key_encodings.get(k))
                    .cloned()
                    .unwrap_or_default();
                buf.write_unsigned_integer_sz(
                    *k as u64,
                    fit_sz(
                        *k as u64,
                        transaction_metadata_set_key_encoding,
                        force_canonical,
                    ),
                )?;
                Ok((buf.finalize(), k, v))
            })
            .collect::<Result<Vec<(Vec<u8>, &_, &_)>, cbor_event::Error>>()?;
        if force_canonical {
            key_order.sort_by(|(lhs_bytes, _, _), (rhs_bytes, _, _)| {
                match lhs_bytes.len().cmp(&rhs_bytes.len()) {
                    std::cmp::Ordering::Equal => lhs_bytes.cmp(rhs_bytes),
                    diff_ord => diff_ord,
                }
            });
        }
        for (key_bytes, _key, value) in key_order {
            serializer.write_raw_bytes(&key_bytes)?;
            value.serialize(serializer, force_canonical)?;
        }
        self.encodings
            .as_ref()
            .map(|encs| encs.transaction_metadata_set_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)?;
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for ShelleyBlock {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let len_encoding: LenEncoding = len.into();
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(4)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let header = ShelleyHeader::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("header"))?;
            let (transaction_bodies, transaction_bodies_encoding) =
                (|| -> Result<_, DeserializeError> {
                    let mut transaction_bodies_arr = Vec::new();
                    let len = raw.array_sz()?;
                    let transaction_bodies_encoding = len.into();
                    while match len {
                        cbor_event::LenSz::Len(n, _) => (transaction_bodies_arr.len() as u64) < n,
                        cbor_event::LenSz::Indefinite => true,
                    } {
                        if raw.cbor_type()? == cbor_event::Type::Special {
                            assert_eq!(raw.special()?, cbor_event::Special::Break);
                            break;
                        }
                        transaction_bodies_arr.push(ShelleyTransactionBody::deserialize(raw)?);
                    }
                    Ok((transaction_bodies_arr, transaction_bodies_encoding))
                })()
                .map_err(|e| e.annotate("transaction_bodies"))?;
            let (transaction_witness_sets, transaction_witness_sets_encoding) =
                (|| -> Result<_, DeserializeError> {
                    let mut transaction_witness_sets_arr = Vec::new();
                    let len = raw.array_sz()?;
                    let transaction_witness_sets_encoding = len.into();
                    while match len {
                        cbor_event::LenSz::Len(n, _) => {
                            (transaction_witness_sets_arr.len() as u64) < n
                        }
                        cbor_event::LenSz::Indefinite => true,
                    } {
                        if raw.cbor_type()? == cbor_event::Type::Special {
                            assert_eq!(raw.special()?, cbor_event::Special::Break);
                            break;
                        }
                        transaction_witness_sets_arr
                            .push(ShelleyTransactionWitnessSet::deserialize(raw)?);
                    }
                    Ok((
                        transaction_witness_sets_arr,
                        transaction_witness_sets_encoding,
                    ))
                })()
                .map_err(|e| e.annotate("transaction_witness_sets"))?;
            let (
                transaction_metadata_set,
                transaction_metadata_set_encoding,
                transaction_metadata_set_key_encodings,
            ) = (|| -> Result<_, DeserializeError> {
                let mut transaction_metadata_set_table = OrderedHashMap::new();
                let transaction_metadata_set_len = raw.map_sz()?;
                let transaction_metadata_set_encoding = transaction_metadata_set_len.into();
                let mut transaction_metadata_set_key_encodings = BTreeMap::new();
                while match transaction_metadata_set_len {
                    cbor_event::LenSz::Len(n, _) => {
                        (transaction_metadata_set_table.len() as u64) < n
                    }
                    cbor_event::LenSz::Indefinite => true,
                } {
                    if raw.cbor_type()? == cbor_event::Type::Special {
                        assert_eq!(raw.special()?, cbor_event::Special::Break);
                        break;
                    }
                    let (transaction_metadata_set_key, transaction_metadata_set_key_encoding) = raw
                        .unsigned_integer_sz()
                        .map(|(x, enc)| (x as u16, Some(enc)))?;
                    let transaction_metadata_set_value = Metadata::deserialize(raw)?;
                    if transaction_metadata_set_table
                        .insert(transaction_metadata_set_key, transaction_metadata_set_value)
                        .is_some()
                    {
                        return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from(
                            "some complicated/unsupported type",
                        )))
                        .into());
                    }
                    transaction_metadata_set_key_encodings.insert(
                        transaction_metadata_set_key,
                        transaction_metadata_set_key_encoding,
                    );
                }
                Ok((
                    transaction_metadata_set_table,
                    transaction_metadata_set_encoding,
                    transaction_metadata_set_key_encodings,
                ))
            })()
            .map_err(|e| e.annotate("transaction_metadata_set"))?;
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(ShelleyBlock {
                header,
                transaction_bodies,
                transaction_witness_sets,
                transaction_metadata_set,
                encodings: Some(ShelleyBlockEncoding {
                    len_encoding,
                    transaction_bodies_encoding,
                    transaction_witness_sets_encoding,
                    transaction_metadata_set_encoding,
                    transaction_metadata_set_key_encodings,
                }),
            })
        })()
        .map_err(|e| e.annotate("ShelleyBlock"))
    }
}

impl Serialize for ShelleyCertificate {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            ShelleyCertificate::StakeRegistration(stake_registration) => {
                stake_registration.serialize(serializer, force_canonical)
            }
            ShelleyCertificate::StakeDeregistration(stake_deregistration) => {
                stake_deregistration.serialize(serializer, force_canonical)
            }
            ShelleyCertificate::StakeDelegation(stake_delegation) => {
                stake_delegation.serialize(serializer, force_canonical)
            }
            ShelleyCertificate::ShelleyPoolRegistration(shelley_pool_registration) => {
                shelley_pool_registration.serialize(serializer, force_canonical)
            }
            ShelleyCertificate::PoolRetirement(pool_retirement) => {
                pool_retirement.serialize(serializer, force_canonical)
            }
            ShelleyCertificate::GenesisKeyDelegation(genesis_key_delegation) => {
                genesis_key_delegation.serialize(serializer, force_canonical)
            }
            ShelleyCertificate::ShelleyMoveInstantaneousRewardsCert(
                shelley_move_instantaneous_rewards_cert,
            ) => shelley_move_instantaneous_rewards_cert.serialize(serializer, force_canonical),
        }
    }
}

impl Deserialize for ShelleyCertificate {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array_sz()?;
            let initial_position = raw.as_mut_ref().stream_position().unwrap();
            let mut errs = Vec::new();
            let deser_variant = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let mut read_len = CBORReadLen::new(len);
                read_len.read_elems(2)?;
                read_len.finish()?;
                let ret = StakeRegistration::deserialize_as_embedded_group(raw, &mut read_len, len);
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
                Ok(stake_registration) => return Ok(Self::StakeRegistration(stake_registration)),
                Err(e) => {
                    errs.push(e.annotate("StakeRegistration"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let mut read_len = CBORReadLen::new(len);
                read_len.read_elems(2)?;
                read_len.finish()?;
                let ret =
                    StakeDeregistration::deserialize_as_embedded_group(raw, &mut read_len, len);
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
                Ok(stake_deregistration) => {
                    return Ok(Self::StakeDeregistration(stake_deregistration))
                }
                Err(e) => {
                    errs.push(e.annotate("StakeDeregistration"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let mut read_len = CBORReadLen::new(len);
                read_len.read_elems(3)?;
                read_len.finish()?;
                let ret = StakeDelegation::deserialize_as_embedded_group(raw, &mut read_len, len);
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
                Ok(stake_delegation) => return Ok(Self::StakeDelegation(stake_delegation)),
                Err(e) => {
                    errs.push(e.annotate("StakeDelegation"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let mut read_len = CBORReadLen::new(len);
                read_len.read_elems(10)?;
                read_len.finish()?;
                let ret =
                    ShelleyPoolRegistration::deserialize_as_embedded_group(raw, &mut read_len, len);
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
                Ok(shelley_pool_registration) => {
                    return Ok(Self::ShelleyPoolRegistration(shelley_pool_registration))
                }
                Err(e) => {
                    errs.push(e.annotate("ShelleyPoolRegistration"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let mut read_len = CBORReadLen::new(len);
                read_len.read_elems(3)?;
                read_len.finish()?;
                let ret = PoolRetirement::deserialize_as_embedded_group(raw, &mut read_len, len);
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
                Ok(pool_retirement) => return Ok(Self::PoolRetirement(pool_retirement)),
                Err(e) => {
                    errs.push(e.annotate("PoolRetirement"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let mut read_len = CBORReadLen::new(len);
                read_len.read_elems(4)?;
                read_len.finish()?;
                let ret =
                    GenesisKeyDelegation::deserialize_as_embedded_group(raw, &mut read_len, len);
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
                Ok(genesis_key_delegation) => {
                    return Ok(Self::GenesisKeyDelegation(genesis_key_delegation))
                }
                Err(e) => {
                    errs.push(e.annotate("GenesisKeyDelegation"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let mut read_len = CBORReadLen::new(len);
                read_len.read_elems(2)?;
                read_len.finish()?;
                let ret = ShelleyMoveInstantaneousRewardsCert::deserialize_as_embedded_group(
                    raw,
                    &mut read_len,
                    len,
                );
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
                Ok(shelley_move_instantaneous_rewards_cert) => {
                    return Ok(Self::ShelleyMoveInstantaneousRewardsCert(
                        shelley_move_instantaneous_rewards_cert,
                    ))
                }
                Err(e) => {
                    errs.push(e.annotate("ShelleyMoveInstantaneousRewardsCert"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            Err(DeserializeError::new(
                "ShelleyCertificate",
                DeserializeFailure::NoVariantMatchedWithCauses(errs),
            ))
        })()
        .map_err(|e| e.annotate("ShelleyCertificate"))
    }
}

impl Serialize for ShelleyDNSName {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_text_sz(
            &self.inner,
            self.encodings
                .as_ref()
                .map(|encs| encs.inner_encoding.clone())
                .unwrap_or_default()
                .to_str_len_sz(self.inner.len() as u64, force_canonical),
        )
    }
}

impl Deserialize for ShelleyDNSName {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let (inner, inner_encoding) = raw
            .text_sz()
            .map(|(s, enc)| (s, StringEncoding::from(enc)))?;
        if inner.len() > 64 {
            return Err(DeserializeError::new(
                "ShelleyDNSName",
                DeserializeFailure::RangeCheck {
                    found: inner.len() as isize,
                    min: Some(0),
                    max: Some(64),
                },
            ));
        }
        Ok(Self {
            inner,
            encodings: Some(ShelleyDNSNameEncoding { inner_encoding }),
        })
    }
}

impl Serialize for ShelleyHeader {
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
        self.body.serialize(serializer, force_canonical)?;
        self.signature.serialize(serializer, force_canonical)?;
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for ShelleyHeader {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let len_encoding: LenEncoding = len.into();
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let body = ShelleyHeaderBody::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("body"))?;
            let signature = KESSignature::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("signature"))?;
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(ShelleyHeader {
                body,
                signature,
                encodings: Some(ShelleyHeaderEncoding { len_encoding }),
            })
        })()
        .map_err(|e| e.annotate("ShelleyHeader"))
    }
}

impl Serialize for ShelleyHeaderBody {
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
                .to_len_sz(15, force_canonical),
        )?;
        serializer.write_unsigned_integer_sz(
            self.block_number,
            fit_sz(
                self.block_number,
                self.encodings
                    .as_ref()
                    .map(|encs| encs.block_number_encoding)
                    .unwrap_or_default(),
                force_canonical,
            ),
        )?;
        serializer.write_unsigned_integer_sz(
            self.slot,
            fit_sz(
                self.slot,
                self.encodings
                    .as_ref()
                    .map(|encs| encs.slot_encoding)
                    .unwrap_or_default(),
                force_canonical,
            ),
        )?;
        match &self.prev_hash {
            Some(x) => serializer.write_bytes_sz(
                x.to_raw_bytes(),
                self.encodings
                    .as_ref()
                    .map(|encs| encs.prev_hash_encoding.clone())
                    .unwrap_or_default()
                    .to_str_len_sz(x.to_raw_bytes().len() as u64, force_canonical),
            ),
            None => serializer.write_special(cbor_event::Special::Null),
        }?;
        serializer.write_bytes_sz(
            self.issuer_vkey.to_raw_bytes(),
            self.encodings
                .as_ref()
                .map(|encs| encs.issuer_vkey_encoding.clone())
                .unwrap_or_default()
                .to_str_len_sz(
                    self.issuer_vkey.to_raw_bytes().len() as u64,
                    force_canonical,
                ),
        )?;
        serializer.write_bytes_sz(
            self.vrf_vkey.to_raw_bytes(),
            self.encodings
                .as_ref()
                .map(|encs| encs.vrf_vkey_encoding.clone())
                .unwrap_or_default()
                .to_str_len_sz(self.vrf_vkey.to_raw_bytes().len() as u64, force_canonical),
        )?;
        self.nonce_vrf.serialize(serializer, force_canonical)?;
        self.leader_vrf.serialize(serializer, force_canonical)?;
        serializer.write_unsigned_integer_sz(
            self.block_body_size,
            fit_sz(
                self.block_body_size,
                self.encodings
                    .as_ref()
                    .map(|encs| encs.block_body_size_encoding)
                    .unwrap_or_default(),
                force_canonical,
            ),
        )?;
        serializer.write_bytes_sz(
            self.block_body_hash.to_raw_bytes(),
            self.encodings
                .as_ref()
                .map(|encs| encs.block_body_hash_encoding.clone())
                .unwrap_or_default()
                .to_str_len_sz(
                    self.block_body_hash.to_raw_bytes().len() as u64,
                    force_canonical,
                ),
        )?;
        self.operational_cert
            .serialize_as_embedded_group(serializer, force_canonical)?;
        self.protocol_version
            .serialize_as_embedded_group(serializer, force_canonical)?;
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for ShelleyHeaderBody {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let len_encoding: LenEncoding = len.into();
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(15)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let (block_number, block_number_encoding) = raw
                .unsigned_integer_sz()
                .map_err(Into::<DeserializeError>::into)
                .map(|(x, enc)| (x, Some(enc)))
                .map_err(|e: DeserializeError| e.annotate("block_number"))?;
            let (slot, slot_encoding) = raw
                .unsigned_integer_sz()
                .map_err(Into::<DeserializeError>::into)
                .map(|(x, enc)| (x, Some(enc)))
                .map_err(|e: DeserializeError| e.annotate("slot"))?;
            let (prev_hash, prev_hash_encoding) = (|| -> Result<_, DeserializeError> {
                Ok(match raw.cbor_type()? != cbor_event::Type::Special {
                    true => Result::<_, DeserializeError>::Ok(
                        raw.bytes_sz()
                            .map_err(Into::<DeserializeError>::into)
                            .and_then(|(bytes, enc)| {
                                BlockHeaderHash::from_raw_bytes(&bytes)
                                    .map(|bytes| (bytes, StringEncoding::from(enc)))
                                    .map_err(|e| {
                                        DeserializeFailure::InvalidStructure(Box::new(e)).into()
                                    })
                            })?,
                    )
                    .map(|(x, prev_hash_encoding)| (Some(x), prev_hash_encoding))?,
                    false => {
                        if raw.special()? != cbor_event::Special::Null {
                            return Err(DeserializeFailure::ExpectedNull.into());
                        }
                        (None, StringEncoding::default())
                    }
                })
            })()
            .map_err(|e| e.annotate("prev_hash"))?;
            let (issuer_vkey, issuer_vkey_encoding) = raw
                .bytes_sz()
                .map_err(Into::<DeserializeError>::into)
                .and_then(|(bytes, enc)| {
                    Vkey::from_raw_bytes(&bytes)
                        .map(|bytes| (bytes, StringEncoding::from(enc)))
                        .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
                })
                .map_err(|e: DeserializeError| e.annotate("issuer_vkey"))?;
            let (vrf_vkey, vrf_vkey_encoding) = raw
                .bytes_sz()
                .map_err(Into::<DeserializeError>::into)
                .and_then(|(bytes, enc)| {
                    VRFVkey::from_raw_bytes(&bytes)
                        .map(|bytes| (bytes, StringEncoding::from(enc)))
                        .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
                })
                .map_err(|e: DeserializeError| e.annotate("vrf_vkey"))?;
            let nonce_vrf =
                VRFCert::deserialize(raw).map_err(|e: DeserializeError| e.annotate("nonce_vrf"))?;
            let leader_vrf = VRFCert::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("leader_vrf"))?;
            let (block_body_size, block_body_size_encoding) = raw
                .unsigned_integer_sz()
                .map_err(Into::<DeserializeError>::into)
                .map(|(x, enc)| (x, Some(enc)))
                .map_err(|e: DeserializeError| e.annotate("block_body_size"))?;
            let (block_body_hash, block_body_hash_encoding) = raw
                .bytes_sz()
                .map_err(Into::<DeserializeError>::into)
                .and_then(|(bytes, enc)| {
                    BlockBodyHash::from_raw_bytes(&bytes)
                        .map(|bytes| (bytes, StringEncoding::from(enc)))
                        .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
                })
                .map_err(|e: DeserializeError| e.annotate("block_body_hash"))?;
            let operational_cert =
                OperationalCert::deserialize_as_embedded_group(raw, &mut read_len, len)
                    .map_err(|e: DeserializeError| e.annotate("operational_cert"))?;
            let protocol_version =
                ProtocolVersion::deserialize_as_embedded_group(raw, &mut read_len, len)
                    .map_err(|e: DeserializeError| e.annotate("protocol_version"))?;
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(ShelleyHeaderBody {
                block_number,
                slot,
                prev_hash,
                issuer_vkey,
                vrf_vkey,
                nonce_vrf,
                leader_vrf,
                block_body_size,
                block_body_hash,
                operational_cert,
                protocol_version,
                encodings: Some(ShelleyHeaderBodyEncoding {
                    len_encoding,
                    block_number_encoding,
                    slot_encoding,
                    prev_hash_encoding,
                    issuer_vkey_encoding,
                    vrf_vkey_encoding,
                    block_body_size_encoding,
                    block_body_hash_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("ShelleyHeaderBody"))
    }
}

impl Serialize for ShelleyMoveInstantaneousReward {
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
        match &self.pot {
            MIRPot::Reserve => serializer.write_unsigned_integer_sz(
                0u64,
                fit_sz(
                    0u64,
                    self.encodings
                        .as_ref()
                        .map(|encs| encs.pot_encoding)
                        .unwrap_or_default(),
                    force_canonical,
                ),
            ),
            MIRPot::Treasury => serializer.write_unsigned_integer_sz(
                1u64,
                fit_sz(
                    1u64,
                    self.encodings
                        .as_ref()
                        .map(|encs| encs.pot_encoding)
                        .unwrap_or_default(),
                    force_canonical,
                ),
            ),
        }?;
        serializer.write_map_sz(
            self.encodings
                .as_ref()
                .map(|encs| encs.to_stake_credentials_encoding)
                .unwrap_or_default()
                .to_len_sz(self.to_stake_credentials.len() as u64, force_canonical),
        )?;
        let mut key_order = self
            .to_stake_credentials
            .iter()
            .map(|(k, v)| {
                let mut buf = cbor_event::se::Serializer::new_vec();
                k.serialize(&mut buf, force_canonical)?;
                Ok((buf.finalize(), k, v))
            })
            .collect::<Result<Vec<(Vec<u8>, &_, &_)>, cbor_event::Error>>()?;
        if force_canonical {
            key_order.sort_by(|(lhs_bytes, _, _), (rhs_bytes, _, _)| {
                match lhs_bytes.len().cmp(&rhs_bytes.len()) {
                    std::cmp::Ordering::Equal => lhs_bytes.cmp(rhs_bytes),
                    diff_ord => diff_ord,
                }
            });
        }
        for (key_bytes, key, value) in key_order {
            serializer.write_raw_bytes(&key_bytes)?;
            let to_stake_credentials_value_encoding = self
                .encodings
                .as_ref()
                .and_then(|encs| encs.to_stake_credentials_value_encodings.get(key))
                .cloned()
                .unwrap_or_default();
            serializer.write_unsigned_integer_sz(
                *value,
                fit_sz(*value, to_stake_credentials_value_encoding, force_canonical),
            )?;
        }
        self.encodings
            .as_ref()
            .map(|encs| encs.to_stake_credentials_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)?;
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for ShelleyMoveInstantaneousReward {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let len_encoding: LenEncoding = len.into();
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let (pot, pot_encoding) = (|| -> Result<_, DeserializeError> {
                let initial_position = raw.as_mut_ref().stream_position().unwrap();
                let deser_variant = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                    let (reserve_value, reserve_encoding) = raw.unsigned_integer_sz()?;
                    if reserve_value != 0 {
                        return Err(DeserializeFailure::FixedValueMismatch {
                            found: Key::Uint(reserve_value),
                            expected: Key::Uint(0),
                        }
                        .into());
                    }
                    Ok(Some(reserve_encoding))
                })(raw);
                match deser_variant {
                    Ok(pot_encoding) => return Ok((MIRPot::Reserve, pot_encoding)),
                    Err(_) => raw
                        .as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap(),
                };
                let deser_variant = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                    let (treasury_value, treasury_encoding) = raw.unsigned_integer_sz()?;
                    if treasury_value != 1 {
                        return Err(DeserializeFailure::FixedValueMismatch {
                            found: Key::Uint(treasury_value),
                            expected: Key::Uint(1),
                        }
                        .into());
                    }
                    Ok(Some(treasury_encoding))
                })(raw);
                match deser_variant {
                    Ok(pot_encoding) => return Ok((MIRPot::Treasury, pot_encoding)),
                    Err(_) => raw
                        .as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap(),
                };
                Err(DeserializeError::new(
                    "MIRPot",
                    DeserializeFailure::NoVariantMatched,
                ))
            })()
            .map_err(|e| e.annotate("pot"))?;
            let (
                to_stake_credentials,
                to_stake_credentials_encoding,
                to_stake_credentials_value_encodings,
            ) = (|| -> Result<_, DeserializeError> {
                let mut to_stake_credentials_table = OrderedHashMap::new();
                let to_stake_credentials_len = raw.map_sz()?;
                let to_stake_credentials_encoding = to_stake_credentials_len.into();
                let mut to_stake_credentials_value_encodings = BTreeMap::new();
                while match to_stake_credentials_len {
                    cbor_event::LenSz::Len(n, _) => (to_stake_credentials_table.len() as u64) < n,
                    cbor_event::LenSz::Indefinite => true,
                } {
                    if raw.cbor_type()? == cbor_event::Type::Special {
                        assert_eq!(raw.special()?, cbor_event::Special::Break);
                        break;
                    }
                    let to_stake_credentials_key = StakeCredential::deserialize(raw)?;
                    let (to_stake_credentials_value, to_stake_credentials_value_encoding) =
                        raw.unsigned_integer_sz().map(|(x, enc)| (x, Some(enc)))?;
                    if to_stake_credentials_table
                        .insert(to_stake_credentials_key.clone(), to_stake_credentials_value)
                        .is_some()
                    {
                        return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from(
                            "some complicated/unsupported type",
                        )))
                        .into());
                    }
                    to_stake_credentials_value_encodings.insert(
                        to_stake_credentials_key,
                        to_stake_credentials_value_encoding,
                    );
                }
                Ok((
                    to_stake_credentials_table,
                    to_stake_credentials_encoding,
                    to_stake_credentials_value_encodings,
                ))
            })()
            .map_err(|e| e.annotate("to_stake_credentials"))?;
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(ShelleyMoveInstantaneousReward {
                pot,
                to_stake_credentials,
                encodings: Some(ShelleyMoveInstantaneousRewardEncoding {
                    len_encoding,
                    pot_encoding,
                    to_stake_credentials_encoding,
                    to_stake_credentials_value_encodings,
                }),
            })
        })()
        .map_err(|e| e.annotate("ShelleyMoveInstantaneousReward"))
    }
}

impl Serialize for ShelleyMoveInstantaneousRewardsCert {
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
        self.serialize_as_embedded_group(serializer, force_canonical)
    }
}

impl SerializeEmbeddedGroup for ShelleyMoveInstantaneousRewardsCert {
    fn serialize_as_embedded_group<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer_sz(
            6u64,
            fit_sz(
                6u64,
                self.encodings
                    .as_ref()
                    .map(|encs| encs.tag_encoding)
                    .unwrap_or_default(),
                force_canonical,
            ),
        )?;
        self.shelley_move_instantaneous_reward
            .serialize(serializer, force_canonical)?;
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for ShelleyMoveInstantaneousRewardsCert {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        let ret = Self::deserialize_as_embedded_group(raw, &mut read_len, len);
        match len {
            cbor_event::LenSz::Len(_, _) => (),
            cbor_event::LenSz::Indefinite => match raw.special()? {
                cbor_event::Special::Break => (),
                _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
            },
        }
        ret
    }
}

impl DeserializeEmbeddedGroup for ShelleyMoveInstantaneousRewardsCert {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        _read_len: &mut CBORReadLen,
        len: cbor_event::LenSz,
    ) -> Result<Self, DeserializeError> {
        let len_encoding = len.into();
        (|| -> Result<_, DeserializeError> {
            let tag_encoding = (|| -> Result<_, DeserializeError> {
                let (tag_value, tag_encoding) = raw.unsigned_integer_sz()?;
                if tag_value != 6 {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(tag_value),
                        expected: Key::Uint(6),
                    }
                    .into());
                }
                Ok(Some(tag_encoding))
            })()
            .map_err(|e| e.annotate("tag"))?;
            let shelley_move_instantaneous_reward = ShelleyMoveInstantaneousReward::deserialize(
                raw,
            )
            .map_err(|e: DeserializeError| e.annotate("shelley_move_instantaneous_reward"))?;
            Ok(ShelleyMoveInstantaneousRewardsCert {
                shelley_move_instantaneous_reward,
                encodings: Some(ShelleyMoveInstantaneousRewardsCertEncoding {
                    len_encoding,
                    tag_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("ShelleyMoveInstantaneousRewardsCert"))
    }
}

impl Serialize for ShelleyMultiHostName {
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
        self.serialize_as_embedded_group(serializer, force_canonical)
    }
}

impl SerializeEmbeddedGroup for ShelleyMultiHostName {
    fn serialize_as_embedded_group<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer_sz(
            2u64,
            fit_sz(
                2u64,
                self.encodings
                    .as_ref()
                    .map(|encs| encs.tag_encoding)
                    .unwrap_or_default(),
                force_canonical,
            ),
        )?;
        self.shelley_dns_name
            .serialize(serializer, force_canonical)?;
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for ShelleyMultiHostName {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        let ret = Self::deserialize_as_embedded_group(raw, &mut read_len, len);
        match len {
            cbor_event::LenSz::Len(_, _) => (),
            cbor_event::LenSz::Indefinite => match raw.special()? {
                cbor_event::Special::Break => (),
                _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
            },
        }
        ret
    }
}

impl DeserializeEmbeddedGroup for ShelleyMultiHostName {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        _read_len: &mut CBORReadLen,
        len: cbor_event::LenSz,
    ) -> Result<Self, DeserializeError> {
        let len_encoding = len.into();
        (|| -> Result<_, DeserializeError> {
            let tag_encoding = (|| -> Result<_, DeserializeError> {
                let (tag_value, tag_encoding) = raw.unsigned_integer_sz()?;
                if tag_value != 2 {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(tag_value),
                        expected: Key::Uint(2),
                    }
                    .into());
                }
                Ok(Some(tag_encoding))
            })()
            .map_err(|e| e.annotate("tag"))?;
            let shelley_dns_name = ShelleyDNSName::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("shelley_dns_name"))?;
            Ok(ShelleyMultiHostName {
                shelley_dns_name,
                encodings: Some(ShelleyMultiHostNameEncoding {
                    len_encoding,
                    tag_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("ShelleyMultiHostName"))
    }
}

impl Serialize for ShelleyPoolParams {
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
                .to_len_sz(9, force_canonical),
        )?;
        self.serialize_as_embedded_group(serializer, force_canonical)
    }
}

impl SerializeEmbeddedGroup for ShelleyPoolParams {
    fn serialize_as_embedded_group<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_bytes_sz(
            self.operator.to_raw_bytes(),
            self.encodings
                .as_ref()
                .map(|encs| encs.operator_encoding.clone())
                .unwrap_or_default()
                .to_str_len_sz(self.operator.to_raw_bytes().len() as u64, force_canonical),
        )?;
        serializer.write_bytes_sz(
            self.vrf_keyhash.to_raw_bytes(),
            self.encodings
                .as_ref()
                .map(|encs| encs.vrf_keyhash_encoding.clone())
                .unwrap_or_default()
                .to_str_len_sz(
                    self.vrf_keyhash.to_raw_bytes().len() as u64,
                    force_canonical,
                ),
        )?;
        serializer.write_unsigned_integer_sz(
            self.pledge,
            fit_sz(
                self.pledge,
                self.encodings
                    .as_ref()
                    .map(|encs| encs.pledge_encoding)
                    .unwrap_or_default(),
                force_canonical,
            ),
        )?;
        serializer.write_unsigned_integer_sz(
            self.cost,
            fit_sz(
                self.cost,
                self.encodings
                    .as_ref()
                    .map(|encs| encs.cost_encoding)
                    .unwrap_or_default(),
                force_canonical,
            ),
        )?;
        self.margin.serialize(serializer, force_canonical)?;
        self.reward_account.serialize(serializer, force_canonical)?;
        serializer.write_array_sz(
            self.encodings
                .as_ref()
                .map(|encs| encs.pool_owners_encoding)
                .unwrap_or_default()
                .to_len_sz(self.pool_owners.len() as u64, force_canonical),
        )?;
        for (i, element) in self.pool_owners.iter().enumerate() {
            let pool_owners_elem_encoding = self
                .encodings
                .as_ref()
                .and_then(|encs| encs.pool_owners_elem_encodings.get(i))
                .cloned()
                .unwrap_or_default();
            serializer.write_bytes_sz(
                element.to_raw_bytes(),
                pool_owners_elem_encoding
                    .to_str_len_sz(element.to_raw_bytes().len() as u64, force_canonical),
            )?;
        }
        self.encodings
            .as_ref()
            .map(|encs| encs.pool_owners_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)?;
        serializer.write_array_sz(
            self.encodings
                .as_ref()
                .map(|encs| encs.relays_encoding)
                .unwrap_or_default()
                .to_len_sz(self.relays.len() as u64, force_canonical),
        )?;
        for element in self.relays.iter() {
            element.serialize(serializer, force_canonical)?;
        }
        self.encodings
            .as_ref()
            .map(|encs| encs.relays_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)?;
        match &self.pool_metadata {
            Some(x) => x.serialize(serializer, force_canonical),
            None => serializer.write_special(cbor_event::Special::Null),
        }?;
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for ShelleyPoolParams {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(9)?;
        read_len.finish()?;
        let ret = Self::deserialize_as_embedded_group(raw, &mut read_len, len);
        match len {
            cbor_event::LenSz::Len(_, _) => (),
            cbor_event::LenSz::Indefinite => match raw.special()? {
                cbor_event::Special::Break => (),
                _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
            },
        }
        ret
    }
}

impl DeserializeEmbeddedGroup for ShelleyPoolParams {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        _read_len: &mut CBORReadLen,
        len: cbor_event::LenSz,
    ) -> Result<Self, DeserializeError> {
        let len_encoding = len.into();
        (|| -> Result<_, DeserializeError> {
            let (operator, operator_encoding) = raw
                .bytes_sz()
                .map_err(Into::<DeserializeError>::into)
                .and_then(|(bytes, enc)| {
                    Ed25519KeyHash::from_raw_bytes(&bytes)
                        .map(|bytes| (bytes, StringEncoding::from(enc)))
                        .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
                })
                .map_err(|e: DeserializeError| e.annotate("operator"))?;
            let (vrf_keyhash, vrf_keyhash_encoding) = raw
                .bytes_sz()
                .map_err(Into::<DeserializeError>::into)
                .and_then(|(bytes, enc)| {
                    VRFKeyHash::from_raw_bytes(&bytes)
                        .map(|bytes| (bytes, StringEncoding::from(enc)))
                        .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
                })
                .map_err(|e: DeserializeError| e.annotate("vrf_keyhash"))?;
            let (pledge, pledge_encoding) = raw
                .unsigned_integer_sz()
                .map_err(Into::<DeserializeError>::into)
                .map(|(x, enc)| (x, Some(enc)))
                .map_err(|e: DeserializeError| e.annotate("pledge"))?;
            let (cost, cost_encoding) = raw
                .unsigned_integer_sz()
                .map_err(Into::<DeserializeError>::into)
                .map(|(x, enc)| (x, Some(enc)))
                .map_err(|e: DeserializeError| e.annotate("cost"))?;
            let margin = UnitInterval::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("margin"))?;
            let reward_account = RewardAccount::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("reward_account"))?;
            let (pool_owners, pool_owners_encoding, pool_owners_elem_encodings) =
                (|| -> Result<_, DeserializeError> {
                    let mut pool_owners_arr = Vec::new();
                    let len = raw.array_sz()?;
                    let pool_owners_encoding = len.into();
                    let mut pool_owners_elem_encodings = Vec::new();
                    while match len {
                        cbor_event::LenSz::Len(n, _) => (pool_owners_arr.len() as u64) < n,
                        cbor_event::LenSz::Indefinite => true,
                    } {
                        if raw.cbor_type()? == cbor_event::Type::Special {
                            assert_eq!(raw.special()?, cbor_event::Special::Break);
                            break;
                        }
                        let (pool_owners_elem, pool_owners_elem_encoding) = raw
                            .bytes_sz()
                            .map_err(Into::<DeserializeError>::into)
                            .and_then(|(bytes, enc)| {
                                Ed25519KeyHash::from_raw_bytes(&bytes)
                                    .map(|bytes| (bytes, StringEncoding::from(enc)))
                                    .map_err(|e| {
                                        DeserializeFailure::InvalidStructure(Box::new(e)).into()
                                    })
                            })?;
                        pool_owners_arr.push(pool_owners_elem);
                        pool_owners_elem_encodings.push(pool_owners_elem_encoding);
                    }
                    Ok((
                        pool_owners_arr,
                        pool_owners_encoding,
                        pool_owners_elem_encodings,
                    ))
                })()
                .map_err(|e| e.annotate("pool_owners"))?;
            let (relays, relays_encoding) = (|| -> Result<_, DeserializeError> {
                let mut relays_arr = Vec::new();
                let len = raw.array_sz()?;
                let relays_encoding = len.into();
                while match len {
                    cbor_event::LenSz::Len(n, _) => (relays_arr.len() as u64) < n,
                    cbor_event::LenSz::Indefinite => true,
                } {
                    if raw.cbor_type()? == cbor_event::Type::Special {
                        assert_eq!(raw.special()?, cbor_event::Special::Break);
                        break;
                    }
                    relays_arr.push(ShelleyRelay::deserialize(raw)?);
                }
                Ok((relays_arr, relays_encoding))
            })()
            .map_err(|e| e.annotate("relays"))?;
            let pool_metadata = (|| -> Result<_, DeserializeError> {
                Ok(match raw.cbor_type()? != cbor_event::Type::Special {
                    true => Some(PoolMetadata::deserialize(raw)?),
                    false => {
                        if raw.special()? != cbor_event::Special::Null {
                            return Err(DeserializeFailure::ExpectedNull.into());
                        }
                        None
                    }
                })
            })()
            .map_err(|e| e.annotate("pool_metadata"))?;
            Ok(ShelleyPoolParams {
                operator,
                vrf_keyhash,
                pledge,
                cost,
                margin,
                reward_account,
                pool_owners,
                relays,
                pool_metadata,
                encodings: Some(ShelleyPoolParamsEncoding {
                    len_encoding,
                    operator_encoding,
                    vrf_keyhash_encoding,
                    pledge_encoding,
                    cost_encoding,
                    pool_owners_encoding,
                    pool_owners_elem_encodings,
                    relays_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("ShelleyPoolParams"))
    }
}

impl Serialize for ShelleyPoolRegistration {
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
                .to_len_sz(10, force_canonical),
        )?;
        self.serialize_as_embedded_group(serializer, force_canonical)
    }
}

impl SerializeEmbeddedGroup for ShelleyPoolRegistration {
    fn serialize_as_embedded_group<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer_sz(
            3u64,
            fit_sz(
                3u64,
                self.encodings
                    .as_ref()
                    .map(|encs| encs.tag_encoding)
                    .unwrap_or_default(),
                force_canonical,
            ),
        )?;
        self.pool_params
            .serialize_as_embedded_group(serializer, force_canonical)?;
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for ShelleyPoolRegistration {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(10)?;
        read_len.finish()?;
        let ret = Self::deserialize_as_embedded_group(raw, &mut read_len, len);
        match len {
            cbor_event::LenSz::Len(_, _) => (),
            cbor_event::LenSz::Indefinite => match raw.special()? {
                cbor_event::Special::Break => (),
                _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
            },
        }
        ret
    }
}

impl DeserializeEmbeddedGroup for ShelleyPoolRegistration {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        read_len: &mut CBORReadLen,
        len: cbor_event::LenSz,
    ) -> Result<Self, DeserializeError> {
        let len_encoding = len.into();
        (|| -> Result<_, DeserializeError> {
            let tag_encoding = (|| -> Result<_, DeserializeError> {
                let (tag_value, tag_encoding) = raw.unsigned_integer_sz()?;
                if tag_value != 3 {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(tag_value),
                        expected: Key::Uint(3),
                    }
                    .into());
                }
                Ok(Some(tag_encoding))
            })()
            .map_err(|e| e.annotate("tag"))?;
            let pool_params = ShelleyPoolParams::deserialize_as_embedded_group(raw, read_len, len)
                .map_err(|e: DeserializeError| e.annotate("pool_params"))?;
            Ok(ShelleyPoolRegistration {
                pool_params,
                encodings: Some(ShelleyPoolRegistrationEncoding {
                    len_encoding,
                    tag_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("ShelleyPoolRegistration"))
    }
}

impl Serialize for ShelleyProtocolParamUpdate {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_map_sz(
            self.encodings
                .as_ref()
                .map(|encs| encs.len_encoding)
                .unwrap_or_default()
                .to_len_sz(
                    match &self.minfee_a {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.minfee_b {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.max_block_body_size {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.max_transaction_size {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.max_block_header_size {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.key_deposit {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.pool_deposit {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.maximum_epoch {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.n_opt {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.pool_pledge_influence {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.expansion_rate {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.treasury_growth_rate {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.decentralization_constant {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.extra_entropy {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.protocol_version {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.min_utxo_value {
                        Some(_) => 1,
                        None => 0,
                    },
                    force_canonical,
                ),
        )?;
        let deser_order = self
            .encodings
            .as_ref()
            .filter(|encs| {
                !force_canonical
                    && encs.orig_deser_order.len()
                        == match &self.minfee_a {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.minfee_b {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.max_block_body_size {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.max_transaction_size {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.max_block_header_size {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.key_deposit {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.pool_deposit {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.maximum_epoch {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.n_opt {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.pool_pledge_influence {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.expansion_rate {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.treasury_growth_rate {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.decentralization_constant {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.extra_entropy {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.protocol_version {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.min_utxo_value {
                            Some(_) => 1,
                            None => 0,
                        }
            })
            .map(|encs| encs.orig_deser_order.clone())
            .unwrap_or_else(|| vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
        for field_index in deser_order {
            match field_index {
                0 => {
                    if let Some(field) = &self.minfee_a {
                        serializer.write_unsigned_integer_sz(
                            0u64,
                            fit_sz(
                                0u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.minfee_a_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        serializer.write_unsigned_integer_sz(
                            *field,
                            fit_sz(
                                *field,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.minfee_a_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                    }
                }
                1 => {
                    if let Some(field) = &self.minfee_b {
                        serializer.write_unsigned_integer_sz(
                            1u64,
                            fit_sz(
                                1u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.minfee_b_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        serializer.write_unsigned_integer_sz(
                            *field,
                            fit_sz(
                                *field,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.minfee_b_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                    }
                }
                2 => {
                    if let Some(field) = &self.max_block_body_size {
                        serializer.write_unsigned_integer_sz(
                            2u64,
                            fit_sz(
                                2u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.max_block_body_size_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        serializer.write_unsigned_integer_sz(
                            *field,
                            fit_sz(
                                *field,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.max_block_body_size_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                    }
                }
                3 => {
                    if let Some(field) = &self.max_transaction_size {
                        serializer.write_unsigned_integer_sz(
                            3u64,
                            fit_sz(
                                3u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.max_transaction_size_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        serializer.write_unsigned_integer_sz(
                            *field,
                            fit_sz(
                                *field,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.max_transaction_size_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                    }
                }
                4 => {
                    if let Some(field) = &self.max_block_header_size {
                        serializer.write_unsigned_integer_sz(
                            4u64,
                            fit_sz(
                                4u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.max_block_header_size_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        serializer.write_unsigned_integer_sz(
                            *field,
                            fit_sz(
                                *field,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.max_block_header_size_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                    }
                }
                5 => {
                    if let Some(field) = &self.key_deposit {
                        serializer.write_unsigned_integer_sz(
                            5u64,
                            fit_sz(
                                5u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.key_deposit_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        serializer.write_unsigned_integer_sz(
                            *field,
                            fit_sz(
                                *field,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.key_deposit_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                    }
                }
                6 => {
                    if let Some(field) = &self.pool_deposit {
                        serializer.write_unsigned_integer_sz(
                            6u64,
                            fit_sz(
                                6u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.pool_deposit_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        serializer.write_unsigned_integer_sz(
                            *field,
                            fit_sz(
                                *field,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.pool_deposit_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                    }
                }
                7 => {
                    if let Some(field) = &self.maximum_epoch {
                        serializer.write_unsigned_integer_sz(
                            7u64,
                            fit_sz(
                                7u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.maximum_epoch_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        serializer.write_unsigned_integer_sz(
                            *field,
                            fit_sz(
                                *field,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.maximum_epoch_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                    }
                }
                8 => {
                    if let Some(field) = &self.n_opt {
                        serializer.write_unsigned_integer_sz(
                            8u64,
                            fit_sz(
                                8u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.n_opt_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        serializer.write_unsigned_integer_sz(
                            *field,
                            fit_sz(
                                *field,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.n_opt_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                    }
                }
                9 => {
                    if let Some(field) = &self.pool_pledge_influence {
                        serializer.write_unsigned_integer_sz(
                            9u64,
                            fit_sz(
                                9u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.pool_pledge_influence_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        field.serialize(serializer, force_canonical)?;
                    }
                }
                10 => {
                    if let Some(field) = &self.expansion_rate {
                        serializer.write_unsigned_integer_sz(
                            10u64,
                            fit_sz(
                                10u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.expansion_rate_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        field.serialize(serializer, force_canonical)?;
                    }
                }
                11 => {
                    if let Some(field) = &self.treasury_growth_rate {
                        serializer.write_unsigned_integer_sz(
                            11u64,
                            fit_sz(
                                11u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.treasury_growth_rate_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        field.serialize(serializer, force_canonical)?;
                    }
                }
                12 => {
                    if let Some(field) = &self.decentralization_constant {
                        serializer.write_unsigned_integer_sz(
                            12u64,
                            fit_sz(
                                12u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.decentralization_constant_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        field.serialize(serializer, force_canonical)?;
                    }
                }
                13 => {
                    if let Some(field) = &self.extra_entropy {
                        serializer.write_unsigned_integer_sz(
                            13u64,
                            fit_sz(
                                13u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.extra_entropy_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        field.serialize(serializer, force_canonical)?;
                    }
                }
                14 => {
                    if let Some(field) = &self.protocol_version {
                        serializer.write_unsigned_integer_sz(
                            14u64,
                            fit_sz(
                                14u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.protocol_version_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        field.serialize(serializer, force_canonical)?;
                    }
                }
                15 => {
                    if let Some(field) = &self.min_utxo_value {
                        serializer.write_unsigned_integer_sz(
                            15u64,
                            fit_sz(
                                15u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.min_utxo_value_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        serializer.write_unsigned_integer_sz(
                            *field,
                            fit_sz(
                                *field,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.min_utxo_value_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                    }
                }
                _ => unreachable!(),
            };
        }
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for ShelleyProtocolParamUpdate {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.map_sz()?;
        let len_encoding: LenEncoding = len.into();
        let mut read_len = CBORReadLen::new(len);
        (|| -> Result<_, DeserializeError> {
            let mut orig_deser_order = Vec::new();
            let mut minfee_a_encoding = None;
            let mut minfee_a_key_encoding = None;
            let mut minfee_a = None;
            let mut minfee_b_encoding = None;
            let mut minfee_b_key_encoding = None;
            let mut minfee_b = None;
            let mut max_block_body_size_encoding = None;
            let mut max_block_body_size_key_encoding = None;
            let mut max_block_body_size = None;
            let mut max_transaction_size_encoding = None;
            let mut max_transaction_size_key_encoding = None;
            let mut max_transaction_size = None;
            let mut max_block_header_size_encoding = None;
            let mut max_block_header_size_key_encoding = None;
            let mut max_block_header_size = None;
            let mut key_deposit_encoding = None;
            let mut key_deposit_key_encoding = None;
            let mut key_deposit = None;
            let mut pool_deposit_encoding = None;
            let mut pool_deposit_key_encoding = None;
            let mut pool_deposit = None;
            let mut maximum_epoch_encoding = None;
            let mut maximum_epoch_key_encoding = None;
            let mut maximum_epoch = None;
            let mut n_opt_encoding = None;
            let mut n_opt_key_encoding = None;
            let mut n_opt = None;
            let mut pool_pledge_influence_key_encoding = None;
            let mut pool_pledge_influence = None;
            let mut expansion_rate_key_encoding = None;
            let mut expansion_rate = None;
            let mut treasury_growth_rate_key_encoding = None;
            let mut treasury_growth_rate = None;
            let mut decentralization_constant_key_encoding = None;
            let mut decentralization_constant = None;
            let mut extra_entropy_key_encoding = None;
            let mut extra_entropy = None;
            let mut protocol_version_key_encoding = None;
            let mut protocol_version = None;
            let mut min_utxo_value_encoding = None;
            let mut min_utxo_value_key_encoding = None;
            let mut min_utxo_value = None;
            let mut read = 0;
            while match len {
                cbor_event::LenSz::Len(n, _) => read < n,
                cbor_event::LenSz::Indefinite => true,
            } {
                match raw.cbor_type()? {
                    cbor_event::Type::UnsignedInteger => match raw.unsigned_integer_sz()? {
                        (0, key_enc) => {
                            if minfee_a.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(0)).into());
                            }
                            let (tmp_minfee_a, tmp_minfee_a_encoding) =
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    raw.unsigned_integer_sz()
                                        .map_err(Into::<DeserializeError>::into)
                                        .map(|(x, enc)| (x, Some(enc)))
                                })()
                                .map_err(|e| e.annotate("minfee_a"))?;
                            minfee_a = Some(tmp_minfee_a);
                            minfee_a_encoding = tmp_minfee_a_encoding;
                            minfee_a_key_encoding = Some(key_enc);
                            orig_deser_order.push(0);
                        }
                        (1, key_enc) => {
                            if minfee_b.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(1)).into());
                            }
                            let (tmp_minfee_b, tmp_minfee_b_encoding) =
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    raw.unsigned_integer_sz()
                                        .map_err(Into::<DeserializeError>::into)
                                        .map(|(x, enc)| (x, Some(enc)))
                                })()
                                .map_err(|e| e.annotate("minfee_b"))?;
                            minfee_b = Some(tmp_minfee_b);
                            minfee_b_encoding = tmp_minfee_b_encoding;
                            minfee_b_key_encoding = Some(key_enc);
                            orig_deser_order.push(1);
                        }
                        (2, key_enc) => {
                            if max_block_body_size.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(2)).into());
                            }
                            let (tmp_max_block_body_size, tmp_max_block_body_size_encoding) =
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    raw.unsigned_integer_sz()
                                        .map_err(Into::<DeserializeError>::into)
                                        .map(|(x, enc)| (x, Some(enc)))
                                })()
                                .map_err(|e| e.annotate("max_block_body_size"))?;
                            max_block_body_size = Some(tmp_max_block_body_size);
                            max_block_body_size_encoding = tmp_max_block_body_size_encoding;
                            max_block_body_size_key_encoding = Some(key_enc);
                            orig_deser_order.push(2);
                        }
                        (3, key_enc) => {
                            if max_transaction_size.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(3)).into());
                            }
                            let (tmp_max_transaction_size, tmp_max_transaction_size_encoding) =
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    raw.unsigned_integer_sz()
                                        .map_err(Into::<DeserializeError>::into)
                                        .map(|(x, enc)| (x, Some(enc)))
                                })()
                                .map_err(|e| e.annotate("max_transaction_size"))?;
                            max_transaction_size = Some(tmp_max_transaction_size);
                            max_transaction_size_encoding = tmp_max_transaction_size_encoding;
                            max_transaction_size_key_encoding = Some(key_enc);
                            orig_deser_order.push(3);
                        }
                        (4, key_enc) => {
                            if max_block_header_size.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(4)).into());
                            }
                            let (tmp_max_block_header_size, tmp_max_block_header_size_encoding) =
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    raw.unsigned_integer_sz()
                                        .map_err(Into::<DeserializeError>::into)
                                        .map(|(x, enc)| (x, Some(enc)))
                                })()
                                .map_err(|e| e.annotate("max_block_header_size"))?;
                            max_block_header_size = Some(tmp_max_block_header_size);
                            max_block_header_size_encoding = tmp_max_block_header_size_encoding;
                            max_block_header_size_key_encoding = Some(key_enc);
                            orig_deser_order.push(4);
                        }
                        (5, key_enc) => {
                            if key_deposit.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(5)).into());
                            }
                            let (tmp_key_deposit, tmp_key_deposit_encoding) =
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    raw.unsigned_integer_sz()
                                        .map_err(Into::<DeserializeError>::into)
                                        .map(|(x, enc)| (x, Some(enc)))
                                })()
                                .map_err(|e| e.annotate("key_deposit"))?;
                            key_deposit = Some(tmp_key_deposit);
                            key_deposit_encoding = tmp_key_deposit_encoding;
                            key_deposit_key_encoding = Some(key_enc);
                            orig_deser_order.push(5);
                        }
                        (6, key_enc) => {
                            if pool_deposit.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(6)).into());
                            }
                            let (tmp_pool_deposit, tmp_pool_deposit_encoding) =
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    raw.unsigned_integer_sz()
                                        .map_err(Into::<DeserializeError>::into)
                                        .map(|(x, enc)| (x, Some(enc)))
                                })()
                                .map_err(|e| e.annotate("pool_deposit"))?;
                            pool_deposit = Some(tmp_pool_deposit);
                            pool_deposit_encoding = tmp_pool_deposit_encoding;
                            pool_deposit_key_encoding = Some(key_enc);
                            orig_deser_order.push(6);
                        }
                        (7, key_enc) => {
                            if maximum_epoch.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(7)).into());
                            }
                            let (tmp_maximum_epoch, tmp_maximum_epoch_encoding) =
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    raw.unsigned_integer_sz()
                                        .map_err(Into::<DeserializeError>::into)
                                        .map(|(x, enc)| (x, Some(enc)))
                                })()
                                .map_err(|e| e.annotate("maximum_epoch"))?;
                            maximum_epoch = Some(tmp_maximum_epoch);
                            maximum_epoch_encoding = tmp_maximum_epoch_encoding;
                            maximum_epoch_key_encoding = Some(key_enc);
                            orig_deser_order.push(7);
                        }
                        (8, key_enc) => {
                            if n_opt.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(8)).into());
                            }
                            let (tmp_n_opt, tmp_n_opt_encoding) =
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    raw.unsigned_integer_sz()
                                        .map_err(Into::<DeserializeError>::into)
                                        .map(|(x, enc)| (x, Some(enc)))
                                })()
                                .map_err(|e| e.annotate("n_opt"))?;
                            n_opt = Some(tmp_n_opt);
                            n_opt_encoding = tmp_n_opt_encoding;
                            n_opt_key_encoding = Some(key_enc);
                            orig_deser_order.push(8);
                        }
                        (9, key_enc) => {
                            if pool_pledge_influence.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(9)).into());
                            }
                            let tmp_pool_pledge_influence = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                Rational::deserialize(raw)
                            })()
                            .map_err(|e| e.annotate("pool_pledge_influence"))?;
                            pool_pledge_influence = Some(tmp_pool_pledge_influence);
                            pool_pledge_influence_key_encoding = Some(key_enc);
                            orig_deser_order.push(9);
                        }
                        (10, key_enc) => {
                            if expansion_rate.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(10)).into());
                            }
                            let tmp_expansion_rate = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                UnitInterval::deserialize(raw)
                            })()
                            .map_err(|e| e.annotate("expansion_rate"))?;
                            expansion_rate = Some(tmp_expansion_rate);
                            expansion_rate_key_encoding = Some(key_enc);
                            orig_deser_order.push(10);
                        }
                        (11, key_enc) => {
                            if treasury_growth_rate.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(11)).into());
                            }
                            let tmp_treasury_growth_rate = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                UnitInterval::deserialize(raw)
                            })()
                            .map_err(|e| e.annotate("treasury_growth_rate"))?;
                            treasury_growth_rate = Some(tmp_treasury_growth_rate);
                            treasury_growth_rate_key_encoding = Some(key_enc);
                            orig_deser_order.push(11);
                        }
                        (12, key_enc) => {
                            if decentralization_constant.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(12)).into());
                            }
                            let tmp_decentralization_constant =
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    UnitInterval::deserialize(raw)
                                })()
                                .map_err(|e| e.annotate("decentralization_constant"))?;
                            decentralization_constant = Some(tmp_decentralization_constant);
                            decentralization_constant_key_encoding = Some(key_enc);
                            orig_deser_order.push(12);
                        }
                        (13, key_enc) => {
                            if extra_entropy.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(13)).into());
                            }
                            let tmp_extra_entropy = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                Nonce::deserialize(raw)
                            })()
                            .map_err(|e| e.annotate("extra_entropy"))?;
                            extra_entropy = Some(tmp_extra_entropy);
                            extra_entropy_key_encoding = Some(key_enc);
                            orig_deser_order.push(13);
                        }
                        (14, key_enc) => {
                            if protocol_version.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(14)).into());
                            }
                            let tmp_protocol_version = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                ProtocolVersionStruct::deserialize(raw)
                            })()
                            .map_err(|e| e.annotate("protocol_version"))?;
                            protocol_version = Some(tmp_protocol_version);
                            protocol_version_key_encoding = Some(key_enc);
                            orig_deser_order.push(14);
                        }
                        (15, key_enc) => {
                            if min_utxo_value.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(15)).into());
                            }
                            let (tmp_min_utxo_value, tmp_min_utxo_value_encoding) =
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    raw.unsigned_integer_sz()
                                        .map_err(Into::<DeserializeError>::into)
                                        .map(|(x, enc)| (x, Some(enc)))
                                })()
                                .map_err(|e| e.annotate("min_utxo_value"))?;
                            min_utxo_value = Some(tmp_min_utxo_value);
                            min_utxo_value_encoding = tmp_min_utxo_value_encoding;
                            min_utxo_value_key_encoding = Some(key_enc);
                            orig_deser_order.push(15);
                        }
                        (unknown_key, _enc) => {
                            return Err(
                                DeserializeFailure::UnknownKey(Key::Uint(unknown_key)).into()
                            )
                        }
                    },
                    cbor_event::Type::Text => {
                        return Err(DeserializeFailure::UnknownKey(Key::Str(raw.text()?)).into())
                    }
                    cbor_event::Type::Special => match len {
                        cbor_event::LenSz::Len(_, _) => {
                            return Err(DeserializeFailure::BreakInDefiniteLen.into())
                        }
                        cbor_event::LenSz::Indefinite => match raw.special()? {
                            cbor_event::Special::Break => break,
                            _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                        },
                    },
                    other_type => {
                        return Err(DeserializeFailure::UnexpectedKeyType(other_type).into())
                    }
                }
                read += 1;
            }
            read_len.finish()?;
            Ok(Self {
                minfee_a,
                minfee_b,
                max_block_body_size,
                max_transaction_size,
                max_block_header_size,
                key_deposit,
                pool_deposit,
                maximum_epoch,
                n_opt,
                pool_pledge_influence,
                expansion_rate,
                treasury_growth_rate,
                decentralization_constant,
                extra_entropy,
                protocol_version,
                min_utxo_value,
                encodings: Some(ShelleyProtocolParamUpdateEncoding {
                    len_encoding,
                    orig_deser_order,
                    minfee_a_key_encoding,
                    minfee_a_encoding,
                    minfee_b_key_encoding,
                    minfee_b_encoding,
                    max_block_body_size_key_encoding,
                    max_block_body_size_encoding,
                    max_transaction_size_key_encoding,
                    max_transaction_size_encoding,
                    max_block_header_size_key_encoding,
                    max_block_header_size_encoding,
                    key_deposit_key_encoding,
                    key_deposit_encoding,
                    pool_deposit_key_encoding,
                    pool_deposit_encoding,
                    maximum_epoch_key_encoding,
                    maximum_epoch_encoding,
                    n_opt_key_encoding,
                    n_opt_encoding,
                    pool_pledge_influence_key_encoding,
                    expansion_rate_key_encoding,
                    treasury_growth_rate_key_encoding,
                    decentralization_constant_key_encoding,
                    extra_entropy_key_encoding,
                    protocol_version_key_encoding,
                    min_utxo_value_key_encoding,
                    min_utxo_value_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("ShelleyProtocolParamUpdate"))
    }
}

impl Serialize for ShelleyRelay {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            ShelleyRelay::SingleHostAddr(single_host_addr) => {
                single_host_addr.serialize(serializer, force_canonical)
            }
            ShelleyRelay::ShelleySingleHostName(shelley_single_host_name) => {
                shelley_single_host_name.serialize(serializer, force_canonical)
            }
            ShelleyRelay::ShelleyMultiHostName(shelley_multi_host_name) => {
                shelley_multi_host_name.serialize(serializer, force_canonical)
            }
        }
    }
}

impl Deserialize for ShelleyRelay {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array_sz()?;
            let initial_position = raw.as_mut_ref().stream_position().unwrap();
            let mut errs = Vec::new();
            let deser_variant = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let mut read_len = CBORReadLen::new(len);
                read_len.read_elems(4)?;
                read_len.finish()?;
                let ret = SingleHostAddr::deserialize_as_embedded_group(raw, &mut read_len, len);
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
                Ok(single_host_addr) => return Ok(Self::SingleHostAddr(single_host_addr)),
                Err(e) => {
                    errs.push(e.annotate("SingleHostAddr"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let mut read_len = CBORReadLen::new(len);
                read_len.read_elems(3)?;
                read_len.finish()?;
                let ret =
                    ShelleySingleHostName::deserialize_as_embedded_group(raw, &mut read_len, len);
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
                Ok(shelley_single_host_name) => {
                    return Ok(Self::ShelleySingleHostName(shelley_single_host_name))
                }
                Err(e) => {
                    errs.push(e.annotate("ShelleySingleHostName"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let mut read_len = CBORReadLen::new(len);
                read_len.read_elems(2)?;
                read_len.finish()?;
                let ret =
                    ShelleyMultiHostName::deserialize_as_embedded_group(raw, &mut read_len, len);
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
                Ok(shelley_multi_host_name) => {
                    return Ok(Self::ShelleyMultiHostName(shelley_multi_host_name))
                }
                Err(e) => {
                    errs.push(e.annotate("ShelleyMultiHostName"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            Err(DeserializeError::new(
                "ShelleyRelay",
                DeserializeFailure::NoVariantMatchedWithCauses(errs),
            ))
        })()
        .map_err(|e| e.annotate("ShelleyRelay"))
    }
}

impl Serialize for ShelleySingleHostName {
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
                .to_len_sz(3, force_canonical),
        )?;
        self.serialize_as_embedded_group(serializer, force_canonical)
    }
}

impl SerializeEmbeddedGroup for ShelleySingleHostName {
    fn serialize_as_embedded_group<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer_sz(
            1u64,
            fit_sz(
                1u64,
                self.encodings
                    .as_ref()
                    .map(|encs| encs.tag_encoding)
                    .unwrap_or_default(),
                force_canonical,
            ),
        )?;
        match &self.port {
            Some(x) => serializer.write_unsigned_integer_sz(
                *x as u64,
                fit_sz(
                    *x as u64,
                    self.encodings
                        .as_ref()
                        .map(|encs| encs.port_encoding)
                        .unwrap_or_default(),
                    force_canonical,
                ),
            ),
            None => serializer.write_special(cbor_event::Special::Null),
        }?;
        self.shelley_dns_name
            .serialize(serializer, force_canonical)?;
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for ShelleySingleHostName {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(3)?;
        read_len.finish()?;
        let ret = Self::deserialize_as_embedded_group(raw, &mut read_len, len);
        match len {
            cbor_event::LenSz::Len(_, _) => (),
            cbor_event::LenSz::Indefinite => match raw.special()? {
                cbor_event::Special::Break => (),
                _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
            },
        }
        ret
    }
}

impl DeserializeEmbeddedGroup for ShelleySingleHostName {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        _read_len: &mut CBORReadLen,
        len: cbor_event::LenSz,
    ) -> Result<Self, DeserializeError> {
        let len_encoding = len.into();
        (|| -> Result<_, DeserializeError> {
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
            let (port, port_encoding) = (|| -> Result<_, DeserializeError> {
                Ok(match raw.cbor_type()? != cbor_event::Type::Special {
                    true => Result::<_, DeserializeError>::Ok(
                        raw.unsigned_integer_sz()
                            .map(|(x, enc)| (x as u16, Some(enc)))?,
                    )
                    .map(|(x, port_encoding)| (Some(x), port_encoding))?,
                    false => {
                        if raw.special()? != cbor_event::Special::Null {
                            return Err(DeserializeFailure::ExpectedNull.into());
                        }
                        (None, None)
                    }
                })
            })()
            .map_err(|e| e.annotate("port"))?;
            let shelley_dns_name = ShelleyDNSName::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("shelley__dns_name"))?;
            Ok(ShelleySingleHostName {
                port,
                shelley_dns_name,
                encodings: Some(ShelleySingleHostNameEncoding {
                    len_encoding,
                    tag_encoding,
                    port_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("ShelleySingleHostName"))
    }
}

impl Serialize for ShelleyTransaction {
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
                .to_len_sz(3, force_canonical),
        )?;
        self.body.serialize(serializer, force_canonical)?;
        self.witness_set.serialize(serializer, force_canonical)?;
        match &self.metadata {
            Some(x) => x.serialize(serializer, force_canonical),
            None => serializer.write_special(cbor_event::Special::Null),
        }?;
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for ShelleyTransaction {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let len_encoding: LenEncoding = len.into();
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(3)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let body = ShelleyTransactionBody::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("body"))?;
            let witness_set = ShelleyTransactionWitnessSet::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("witness_set"))?;
            let metadata = (|| -> Result<_, DeserializeError> {
                Ok(match raw.cbor_type()? != cbor_event::Type::Special {
                    true => Some(Metadata::deserialize(raw)?),
                    false => {
                        if raw.special()? != cbor_event::Special::Null {
                            return Err(DeserializeFailure::ExpectedNull.into());
                        }
                        None
                    }
                })
            })()
            .map_err(|e| e.annotate("metadata"))?;
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(ShelleyTransaction {
                body,
                witness_set,
                metadata,
                encodings: Some(ShelleyTransactionEncoding { len_encoding }),
            })
        })()
        .map_err(|e| e.annotate("ShelleyTransaction"))
    }
}

impl Serialize for ShelleyTransactionBody {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_map_sz(
            self.encodings
                .as_ref()
                .map(|encs| encs.len_encoding)
                .unwrap_or_default()
                .to_len_sz(
                    4 + match &self.certs {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.withdrawals {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.update {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.auxiliary_data_hash {
                        Some(_) => 1,
                        None => 0,
                    },
                    force_canonical,
                ),
        )?;
        let deser_order = self
            .encodings
            .as_ref()
            .filter(|encs| {
                !force_canonical
                    && encs.orig_deser_order.len()
                        == 4 + match &self.certs {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.withdrawals {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.update {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.auxiliary_data_hash {
                            Some(_) => 1,
                            None => 0,
                        }
            })
            .map(|encs| encs.orig_deser_order.clone())
            .unwrap_or_else(|| vec![0, 1, 2, 3, 4, 5, 6, 7]);
        for field_index in deser_order {
            match field_index {
                0 => {
                    serializer.write_unsigned_integer_sz(
                        0u64,
                        fit_sz(
                            0u64,
                            self.encodings
                                .as_ref()
                                .map(|encs| encs.inputs_key_encoding)
                                .unwrap_or_default(),
                            force_canonical,
                        ),
                    )?;
                    serializer.write_array_sz(
                        self.encodings
                            .as_ref()
                            .map(|encs| encs.inputs_encoding)
                            .unwrap_or_default()
                            .to_len_sz(self.inputs.len() as u64, force_canonical),
                    )?;
                    for element in self.inputs.iter() {
                        element.serialize(serializer, force_canonical)?;
                    }
                    self.encodings
                        .as_ref()
                        .map(|encs| encs.inputs_encoding)
                        .unwrap_or_default()
                        .end(serializer, force_canonical)?;
                }
                1 => {
                    serializer.write_unsigned_integer_sz(
                        1u64,
                        fit_sz(
                            1u64,
                            self.encodings
                                .as_ref()
                                .map(|encs| encs.outputs_key_encoding)
                                .unwrap_or_default(),
                            force_canonical,
                        ),
                    )?;
                    serializer.write_array_sz(
                        self.encodings
                            .as_ref()
                            .map(|encs| encs.outputs_encoding)
                            .unwrap_or_default()
                            .to_len_sz(self.outputs.len() as u64, force_canonical),
                    )?;
                    for element in self.outputs.iter() {
                        element.serialize(serializer, force_canonical)?;
                    }
                    self.encodings
                        .as_ref()
                        .map(|encs| encs.outputs_encoding)
                        .unwrap_or_default()
                        .end(serializer, force_canonical)?;
                }
                2 => {
                    serializer.write_unsigned_integer_sz(
                        2u64,
                        fit_sz(
                            2u64,
                            self.encodings
                                .as_ref()
                                .map(|encs| encs.fee_key_encoding)
                                .unwrap_or_default(),
                            force_canonical,
                        ),
                    )?;
                    serializer.write_unsigned_integer_sz(
                        self.fee,
                        fit_sz(
                            self.fee,
                            self.encodings
                                .as_ref()
                                .map(|encs| encs.fee_encoding)
                                .unwrap_or_default(),
                            force_canonical,
                        ),
                    )?;
                }
                3 => {
                    serializer.write_unsigned_integer_sz(
                        3u64,
                        fit_sz(
                            3u64,
                            self.encodings
                                .as_ref()
                                .map(|encs| encs.ttl_key_encoding)
                                .unwrap_or_default(),
                            force_canonical,
                        ),
                    )?;
                    serializer.write_unsigned_integer_sz(
                        self.ttl,
                        fit_sz(
                            self.ttl,
                            self.encodings
                                .as_ref()
                                .map(|encs| encs.ttl_encoding)
                                .unwrap_or_default(),
                            force_canonical,
                        ),
                    )?;
                }
                4 => {
                    if let Some(field) = &self.certs {
                        serializer.write_unsigned_integer_sz(
                            4u64,
                            fit_sz(
                                4u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.certs_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        serializer.write_array_sz(
                            self.encodings
                                .as_ref()
                                .map(|encs| encs.certs_encoding)
                                .unwrap_or_default()
                                .to_len_sz(field.len() as u64, force_canonical),
                        )?;
                        for element in field.iter() {
                            element.serialize(serializer, force_canonical)?;
                        }
                        self.encodings
                            .as_ref()
                            .map(|encs| encs.certs_encoding)
                            .unwrap_or_default()
                            .end(serializer, force_canonical)?;
                    }
                }
                5 => {
                    if let Some(field) = &self.withdrawals {
                        serializer.write_unsigned_integer_sz(
                            5u64,
                            fit_sz(
                                5u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.withdrawals_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        serializer.write_map_sz(
                            self.encodings
                                .as_ref()
                                .map(|encs| encs.withdrawals_encoding)
                                .unwrap_or_default()
                                .to_len_sz(field.len() as u64, force_canonical),
                        )?;
                        let mut key_order = field
                            .iter()
                            .map(|(k, v)| {
                                let mut buf = cbor_event::se::Serializer::new_vec();
                                k.serialize(&mut buf, force_canonical)?;
                                Ok((buf.finalize(), k, v))
                            })
                            .collect::<Result<Vec<(Vec<u8>, &_, &_)>, cbor_event::Error>>()?;
                        if force_canonical {
                            key_order.sort_by(
                                |(lhs_bytes, _, _), (rhs_bytes, _, _)| match lhs_bytes
                                    .len()
                                    .cmp(&rhs_bytes.len())
                                {
                                    std::cmp::Ordering::Equal => lhs_bytes.cmp(rhs_bytes),
                                    diff_ord => diff_ord,
                                },
                            );
                        }
                        for (key_bytes, key, value) in key_order {
                            serializer.write_raw_bytes(&key_bytes)?;
                            let withdrawals_value_encoding = self
                                .encodings
                                .as_ref()
                                .and_then(|encs| encs.withdrawals_value_encodings.get(key))
                                .cloned()
                                .unwrap_or_default();
                            serializer.write_unsigned_integer_sz(
                                *value,
                                fit_sz(*value, withdrawals_value_encoding, force_canonical),
                            )?;
                        }
                        self.encodings
                            .as_ref()
                            .map(|encs| encs.withdrawals_encoding)
                            .unwrap_or_default()
                            .end(serializer, force_canonical)?;
                    }
                }
                6 => {
                    if let Some(field) = &self.update {
                        serializer.write_unsigned_integer_sz(
                            6u64,
                            fit_sz(
                                6u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.update_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        field.serialize(serializer, force_canonical)?;
                    }
                }
                7 => {
                    if let Some(field) = &self.auxiliary_data_hash {
                        serializer.write_unsigned_integer_sz(
                            7u64,
                            fit_sz(
                                7u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.auxiliary_data_hash_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        serializer.write_bytes_sz(
                            field.to_raw_bytes(),
                            self.encodings
                                .as_ref()
                                .map(|encs| encs.auxiliary_data_hash_encoding.clone())
                                .unwrap_or_default()
                                .to_str_len_sz(field.to_raw_bytes().len() as u64, force_canonical),
                        )?;
                    }
                }
                _ => unreachable!(),
            };
        }
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for ShelleyTransactionBody {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.map_sz()?;
        let len_encoding: LenEncoding = len.into();
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(4)?;
        (|| -> Result<_, DeserializeError> {
            let mut orig_deser_order = Vec::new();
            let mut inputs_encoding = LenEncoding::default();
            let mut inputs_key_encoding = None;
            let mut inputs = None;
            let mut outputs_encoding = LenEncoding::default();
            let mut outputs_key_encoding = None;
            let mut outputs = None;
            let mut fee_encoding = None;
            let mut fee_key_encoding = None;
            let mut fee = None;
            let mut ttl_encoding = None;
            let mut ttl_key_encoding = None;
            let mut ttl = None;
            let mut certs_encoding = LenEncoding::default();
            let mut certs_key_encoding = None;
            let mut certs = None;
            let mut withdrawals_encoding = LenEncoding::default();
            let mut withdrawals_value_encodings = BTreeMap::new();
            let mut withdrawals_key_encoding = None;
            let mut withdrawals = None;
            let mut update_key_encoding = None;
            let mut update = None;
            let mut auxiliary_data_hash_encoding = StringEncoding::default();
            let mut auxiliary_data_hash_key_encoding = None;
            let mut auxiliary_data_hash = None;
            let mut read = 0;
            while match len {
                cbor_event::LenSz::Len(n, _) => read < n,
                cbor_event::LenSz::Indefinite => true,
            } {
                match raw.cbor_type()? {
                    cbor_event::Type::UnsignedInteger => match raw.unsigned_integer_sz()? {
                        (0, key_enc) => {
                            if inputs.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(0)).into());
                            }
                            let (tmp_inputs, tmp_inputs_encoding) =
                                (|| -> Result<_, DeserializeError> {
                                    let mut inputs_arr = Vec::new();
                                    let len = raw.array_sz()?;
                                    let inputs_encoding = len.into();
                                    while match len {
                                        cbor_event::LenSz::Len(n, _) => {
                                            (inputs_arr.len() as u64) < n
                                        }
                                        cbor_event::LenSz::Indefinite => true,
                                    } {
                                        if raw.cbor_type()? == cbor_event::Type::Special {
                                            assert_eq!(raw.special()?, cbor_event::Special::Break);
                                            break;
                                        }
                                        inputs_arr.push(TransactionInput::deserialize(raw)?);
                                    }
                                    Ok((inputs_arr, inputs_encoding))
                                })()
                                .map_err(|e| e.annotate("inputs"))?;
                            inputs = Some(tmp_inputs);
                            inputs_encoding = tmp_inputs_encoding;
                            inputs_key_encoding = Some(key_enc);
                            orig_deser_order.push(0);
                        }
                        (1, key_enc) => {
                            if outputs.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(1)).into());
                            }
                            let (tmp_outputs, tmp_outputs_encoding) =
                                (|| -> Result<_, DeserializeError> {
                                    let mut outputs_arr = Vec::new();
                                    let len = raw.array_sz()?;
                                    let outputs_encoding = len.into();
                                    while match len {
                                        cbor_event::LenSz::Len(n, _) => {
                                            (outputs_arr.len() as u64) < n
                                        }
                                        cbor_event::LenSz::Indefinite => true,
                                    } {
                                        if raw.cbor_type()? == cbor_event::Type::Special {
                                            assert_eq!(raw.special()?, cbor_event::Special::Break);
                                            break;
                                        }
                                        outputs_arr
                                            .push(ShelleyTransactionOutput::deserialize(raw)?);
                                    }
                                    Ok((outputs_arr, outputs_encoding))
                                })()
                                .map_err(|e| e.annotate("outputs"))?;
                            outputs = Some(tmp_outputs);
                            outputs_encoding = tmp_outputs_encoding;
                            outputs_key_encoding = Some(key_enc);
                            orig_deser_order.push(1);
                        }
                        (2, key_enc) => {
                            if fee.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(2)).into());
                            }
                            let (tmp_fee, tmp_fee_encoding) = raw
                                .unsigned_integer_sz()
                                .map_err(Into::<DeserializeError>::into)
                                .map(|(x, enc)| (x, Some(enc)))
                                .map_err(|e: DeserializeError| e.annotate("fee"))?;
                            fee = Some(tmp_fee);
                            fee_encoding = tmp_fee_encoding;
                            fee_key_encoding = Some(key_enc);
                            orig_deser_order.push(2);
                        }
                        (3, key_enc) => {
                            if ttl.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(3)).into());
                            }
                            let (tmp_ttl, tmp_ttl_encoding) = raw
                                .unsigned_integer_sz()
                                .map_err(Into::<DeserializeError>::into)
                                .map(|(x, enc)| (x, Some(enc)))
                                .map_err(|e: DeserializeError| e.annotate("ttl"))?;
                            ttl = Some(tmp_ttl);
                            ttl_encoding = tmp_ttl_encoding;
                            ttl_key_encoding = Some(key_enc);
                            orig_deser_order.push(3);
                        }
                        (4, key_enc) => {
                            if certs.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(4)).into());
                            }
                            let (tmp_certs, tmp_certs_encoding) =
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    let mut certs_arr = Vec::new();
                                    let len = raw.array_sz()?;
                                    let certs_encoding = len.into();
                                    while match len {
                                        cbor_event::LenSz::Len(n, _) => {
                                            (certs_arr.len() as u64) < n
                                        }
                                        cbor_event::LenSz::Indefinite => true,
                                    } {
                                        if raw.cbor_type()? == cbor_event::Type::Special {
                                            assert_eq!(raw.special()?, cbor_event::Special::Break);
                                            break;
                                        }
                                        certs_arr.push(ShelleyCertificate::deserialize(raw)?);
                                    }
                                    Ok((certs_arr, certs_encoding))
                                })()
                                .map_err(|e| e.annotate("certs"))?;
                            certs = Some(tmp_certs);
                            certs_encoding = tmp_certs_encoding;
                            certs_key_encoding = Some(key_enc);
                            orig_deser_order.push(4);
                        }
                        (5, key_enc) => {
                            if withdrawals.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(5)).into());
                            }
                            let (
                                tmp_withdrawals,
                                tmp_withdrawals_encoding,
                                tmp_withdrawals_value_encodings,
                            ) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                let mut withdrawals_table = OrderedHashMap::new();
                                let withdrawals_len = raw.map_sz()?;
                                let withdrawals_encoding = withdrawals_len.into();
                                let mut withdrawals_value_encodings = BTreeMap::new();
                                while match withdrawals_len {
                                    cbor_event::LenSz::Len(n, _) => {
                                        (withdrawals_table.len() as u64) < n
                                    }
                                    cbor_event::LenSz::Indefinite => true,
                                } {
                                    if raw.cbor_type()? == cbor_event::Type::Special {
                                        assert_eq!(raw.special()?, cbor_event::Special::Break);
                                        break;
                                    }
                                    let withdrawals_key = RewardAccount::deserialize(raw)?;
                                    let (withdrawals_value, withdrawals_value_encoding) =
                                        raw.unsigned_integer_sz().map(|(x, enc)| (x, Some(enc)))?;
                                    if withdrawals_table
                                        .insert(withdrawals_key.clone(), withdrawals_value)
                                        .is_some()
                                    {
                                        return Err(DeserializeFailure::DuplicateKey(Key::Str(
                                            String::from("some complicated/unsupported type"),
                                        ))
                                        .into());
                                    }
                                    withdrawals_value_encodings
                                        .insert(withdrawals_key, withdrawals_value_encoding);
                                }
                                Ok((
                                    withdrawals_table,
                                    withdrawals_encoding,
                                    withdrawals_value_encodings,
                                ))
                            })()
                            .map_err(|e| e.annotate("withdrawals"))?;
                            withdrawals = Some(tmp_withdrawals);
                            withdrawals_encoding = tmp_withdrawals_encoding;
                            withdrawals_value_encodings = tmp_withdrawals_value_encodings;
                            withdrawals_key_encoding = Some(key_enc);
                            orig_deser_order.push(5);
                        }
                        (6, key_enc) => {
                            if update.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(6)).into());
                            }
                            let tmp_update = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                ShelleyUpdate::deserialize(raw)
                            })()
                            .map_err(|e| e.annotate("update"))?;
                            update = Some(tmp_update);
                            update_key_encoding = Some(key_enc);
                            orig_deser_order.push(6);
                        }
                        (7, key_enc) => {
                            if auxiliary_data_hash.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(7)).into());
                            }
                            let (tmp_auxiliary_data_hash, tmp_auxiliary_data_hash_encoding) =
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    raw.bytes_sz()
                                        .map_err(Into::<DeserializeError>::into)
                                        .and_then(|(bytes, enc)| {
                                            AuxiliaryDataHash::from_raw_bytes(&bytes)
                                                .map(|bytes| (bytes, StringEncoding::from(enc)))
                                                .map_err(|e| {
                                                    DeserializeFailure::InvalidStructure(Box::new(
                                                        e,
                                                    ))
                                                    .into()
                                                })
                                        })
                                })()
                                .map_err(|e| e.annotate("auxiliary_data_hash"))?;
                            auxiliary_data_hash = Some(tmp_auxiliary_data_hash);
                            auxiliary_data_hash_encoding = tmp_auxiliary_data_hash_encoding;
                            auxiliary_data_hash_key_encoding = Some(key_enc);
                            orig_deser_order.push(7);
                        }
                        (unknown_key, _enc) => {
                            return Err(
                                DeserializeFailure::UnknownKey(Key::Uint(unknown_key)).into()
                            )
                        }
                    },
                    cbor_event::Type::Text => {
                        return Err(DeserializeFailure::UnknownKey(Key::Str(raw.text()?)).into())
                    }
                    cbor_event::Type::Special => match len {
                        cbor_event::LenSz::Len(_, _) => {
                            return Err(DeserializeFailure::BreakInDefiniteLen.into())
                        }
                        cbor_event::LenSz::Indefinite => match raw.special()? {
                            cbor_event::Special::Break => break,
                            _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                        },
                    },
                    other_type => {
                        return Err(DeserializeFailure::UnexpectedKeyType(other_type).into())
                    }
                }
                read += 1;
            }
            let inputs = match inputs {
                Some(x) => x,
                None => return Err(DeserializeFailure::MandatoryFieldMissing(Key::Uint(0)).into()),
            };
            let outputs = match outputs {
                Some(x) => x,
                None => return Err(DeserializeFailure::MandatoryFieldMissing(Key::Uint(1)).into()),
            };
            let fee = match fee {
                Some(x) => x,
                None => return Err(DeserializeFailure::MandatoryFieldMissing(Key::Uint(2)).into()),
            };
            let ttl = match ttl {
                Some(x) => x,
                None => return Err(DeserializeFailure::MandatoryFieldMissing(Key::Uint(3)).into()),
            };
            read_len.finish()?;
            Ok(Self {
                inputs,
                outputs,
                fee,
                ttl,
                certs,
                withdrawals,
                update,
                auxiliary_data_hash,
                encodings: Some(ShelleyTransactionBodyEncoding {
                    len_encoding,
                    orig_deser_order,
                    inputs_key_encoding,
                    inputs_encoding,
                    outputs_key_encoding,
                    outputs_encoding,
                    fee_key_encoding,
                    fee_encoding,
                    ttl_key_encoding,
                    ttl_encoding,
                    certs_key_encoding,
                    certs_encoding,
                    withdrawals_key_encoding,
                    withdrawals_encoding,
                    withdrawals_value_encodings,
                    update_key_encoding,
                    auxiliary_data_hash_key_encoding,
                    auxiliary_data_hash_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("ShelleyTransactionBody"))
    }
}

impl Serialize for ShelleyTransactionOutput {
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
        self.address.serialize(serializer, force_canonical)?;
        serializer.write_unsigned_integer_sz(
            self.amount,
            fit_sz(
                self.amount,
                self.encodings
                    .as_ref()
                    .map(|encs| encs.amount_encoding)
                    .unwrap_or_default(),
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

impl Deserialize for ShelleyTransactionOutput {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let len_encoding: LenEncoding = len.into();
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let address =
                Address::deserialize(raw).map_err(|e: DeserializeError| e.annotate("address"))?;
            let (amount, amount_encoding) = raw
                .unsigned_integer_sz()
                .map_err(Into::<DeserializeError>::into)
                .map(|(x, enc)| (x, Some(enc)))
                .map_err(|e: DeserializeError| e.annotate("amount"))?;
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(ShelleyTransactionOutput {
                address,
                amount,
                encodings: Some(ShelleyTransactionOutputEncoding {
                    len_encoding,
                    amount_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("ShelleyTransactionOutput"))
    }
}

impl Serialize for ShelleyTransactionWitnessSet {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_map_sz(
            self.encodings
                .as_ref()
                .map(|encs| encs.len_encoding)
                .unwrap_or_default()
                .to_len_sz(
                    match &self.vkeywitnesses {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.native_scripts {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.bootstrap_witnesses {
                        Some(_) => 1,
                        None => 0,
                    },
                    force_canonical,
                ),
        )?;
        let deser_order = self
            .encodings
            .as_ref()
            .filter(|encs| {
                !force_canonical
                    && encs.orig_deser_order.len()
                        == match &self.vkeywitnesses {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.native_scripts {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.bootstrap_witnesses {
                            Some(_) => 1,
                            None => 0,
                        }
            })
            .map(|encs| encs.orig_deser_order.clone())
            .unwrap_or_else(|| vec![0, 1, 2]);
        for field_index in deser_order {
            match field_index {
                0 => {
                    if let Some(field) = &self.vkeywitnesses {
                        serializer.write_unsigned_integer_sz(
                            0u64,
                            fit_sz(
                                0u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.vkeywitnesses_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        serializer.write_array_sz(
                            self.encodings
                                .as_ref()
                                .map(|encs| encs.vkeywitnesses_encoding)
                                .unwrap_or_default()
                                .to_len_sz(field.len() as u64, force_canonical),
                        )?;
                        for element in field.iter() {
                            element.serialize(serializer, force_canonical)?;
                        }
                        self.encodings
                            .as_ref()
                            .map(|encs| encs.vkeywitnesses_encoding)
                            .unwrap_or_default()
                            .end(serializer, force_canonical)?;
                    }
                }
                1 => {
                    if let Some(field) = &self.native_scripts {
                        serializer.write_unsigned_integer_sz(
                            1u64,
                            fit_sz(
                                1u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.native_scripts_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        serializer.write_array_sz(
                            self.encodings
                                .as_ref()
                                .map(|encs| encs.native_scripts_encoding)
                                .unwrap_or_default()
                                .to_len_sz(field.len() as u64, force_canonical),
                        )?;
                        for element in field.iter() {
                            element.serialize(serializer, force_canonical)?;
                        }
                        self.encodings
                            .as_ref()
                            .map(|encs| encs.native_scripts_encoding)
                            .unwrap_or_default()
                            .end(serializer, force_canonical)?;
                    }
                }
                2 => {
                    if let Some(field) = &self.bootstrap_witnesses {
                        serializer.write_unsigned_integer_sz(
                            2u64,
                            fit_sz(
                                2u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.bootstrap_witnesses_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        serializer.write_array_sz(
                            self.encodings
                                .as_ref()
                                .map(|encs| encs.bootstrap_witnesses_encoding)
                                .unwrap_or_default()
                                .to_len_sz(field.len() as u64, force_canonical),
                        )?;
                        for element in field.iter() {
                            element.serialize(serializer, force_canonical)?;
                        }
                        self.encodings
                            .as_ref()
                            .map(|encs| encs.bootstrap_witnesses_encoding)
                            .unwrap_or_default()
                            .end(serializer, force_canonical)?;
                    }
                }
                _ => unreachable!(),
            };
        }
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for ShelleyTransactionWitnessSet {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.map_sz()?;
        let len_encoding: LenEncoding = len.into();
        let mut read_len = CBORReadLen::new(len);
        (|| -> Result<_, DeserializeError> {
            let mut orig_deser_order = Vec::new();
            let mut vkeywitnesses_encoding = LenEncoding::default();
            let mut vkeywitnesses_key_encoding = None;
            let mut vkeywitnesses = None;
            let mut native_scripts_encoding = LenEncoding::default();
            let mut native_scripts_key_encoding = None;
            let mut native_scripts = None;
            let mut bootstrap_witnesses_encoding = LenEncoding::default();
            let mut bootstrap_witnesses_key_encoding = None;
            let mut bootstrap_witnesses = None;
            let mut read = 0;
            while match len {
                cbor_event::LenSz::Len(n, _) => read < n,
                cbor_event::LenSz::Indefinite => true,
            } {
                match raw.cbor_type()? {
                    cbor_event::Type::UnsignedInteger => match raw.unsigned_integer_sz()? {
                        (0, key_enc) => {
                            if vkeywitnesses.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(0)).into());
                            }
                            let (tmp_vkeywitnesses, tmp_vkeywitnesses_encoding) =
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    let mut vkeywitnesses_arr = Vec::new();
                                    let len = raw.array_sz()?;
                                    let vkeywitnesses_encoding = len.into();
                                    while match len {
                                        cbor_event::LenSz::Len(n, _) => {
                                            (vkeywitnesses_arr.len() as u64) < n
                                        }
                                        cbor_event::LenSz::Indefinite => true,
                                    } {
                                        if raw.cbor_type()? == cbor_event::Type::Special {
                                            assert_eq!(raw.special()?, cbor_event::Special::Break);
                                            break;
                                        }
                                        vkeywitnesses_arr.push(Vkeywitness::deserialize(raw)?);
                                    }
                                    Ok((vkeywitnesses_arr, vkeywitnesses_encoding))
                                })()
                                .map_err(|e| e.annotate("vkeywitnesses"))?;
                            vkeywitnesses = Some(tmp_vkeywitnesses);
                            vkeywitnesses_encoding = tmp_vkeywitnesses_encoding;
                            vkeywitnesses_key_encoding = Some(key_enc);
                            orig_deser_order.push(0);
                        }
                        (1, key_enc) => {
                            if native_scripts.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(1)).into());
                            }
                            let (tmp_native_scripts, tmp_native_scripts_encoding) =
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    let mut native_scripts_arr = Vec::new();
                                    let len = raw.array_sz()?;
                                    let native_scripts_encoding = len.into();
                                    while match len {
                                        cbor_event::LenSz::Len(n, _) => {
                                            (native_scripts_arr.len() as u64) < n
                                        }
                                        cbor_event::LenSz::Indefinite => true,
                                    } {
                                        if raw.cbor_type()? == cbor_event::Type::Special {
                                            assert_eq!(raw.special()?, cbor_event::Special::Break);
                                            break;
                                        }
                                        native_scripts_arr.push(MultisigScript::deserialize(raw)?);
                                    }
                                    Ok((native_scripts_arr, native_scripts_encoding))
                                })()
                                .map_err(|e| e.annotate("native_scripts"))?;
                            native_scripts = Some(tmp_native_scripts);
                            native_scripts_encoding = tmp_native_scripts_encoding;
                            native_scripts_key_encoding = Some(key_enc);
                            orig_deser_order.push(1);
                        }
                        (2, key_enc) => {
                            if bootstrap_witnesses.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(2)).into());
                            }
                            let (tmp_bootstrap_witnesses, tmp_bootstrap_witnesses_encoding) =
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    let mut bootstrap_witnesses_arr = Vec::new();
                                    let len = raw.array_sz()?;
                                    let bootstrap_witnesses_encoding = len.into();
                                    while match len {
                                        cbor_event::LenSz::Len(n, _) => {
                                            (bootstrap_witnesses_arr.len() as u64) < n
                                        }
                                        cbor_event::LenSz::Indefinite => true,
                                    } {
                                        if raw.cbor_type()? == cbor_event::Type::Special {
                                            assert_eq!(raw.special()?, cbor_event::Special::Break);
                                            break;
                                        }
                                        bootstrap_witnesses_arr
                                            .push(BootstrapWitness::deserialize(raw)?);
                                    }
                                    Ok((bootstrap_witnesses_arr, bootstrap_witnesses_encoding))
                                })()
                                .map_err(|e| e.annotate("bootstrap_witnesses"))?;
                            bootstrap_witnesses = Some(tmp_bootstrap_witnesses);
                            bootstrap_witnesses_encoding = tmp_bootstrap_witnesses_encoding;
                            bootstrap_witnesses_key_encoding = Some(key_enc);
                            orig_deser_order.push(2);
                        }
                        (unknown_key, _enc) => {
                            return Err(
                                DeserializeFailure::UnknownKey(Key::Uint(unknown_key)).into()
                            )
                        }
                    },
                    cbor_event::Type::Text => {
                        return Err(DeserializeFailure::UnknownKey(Key::Str(raw.text()?)).into())
                    }
                    cbor_event::Type::Special => match len {
                        cbor_event::LenSz::Len(_, _) => {
                            return Err(DeserializeFailure::BreakInDefiniteLen.into())
                        }
                        cbor_event::LenSz::Indefinite => match raw.special()? {
                            cbor_event::Special::Break => break,
                            _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                        },
                    },
                    other_type => {
                        return Err(DeserializeFailure::UnexpectedKeyType(other_type).into())
                    }
                }
                read += 1;
            }
            read_len.finish()?;
            Ok(Self {
                vkeywitnesses,
                native_scripts,
                bootstrap_witnesses,
                encodings: Some(ShelleyTransactionWitnessSetEncoding {
                    len_encoding,
                    orig_deser_order,
                    vkeywitnesses_key_encoding,
                    vkeywitnesses_encoding,
                    native_scripts_key_encoding,
                    native_scripts_encoding,
                    bootstrap_witnesses_key_encoding,
                    bootstrap_witnesses_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("ShelleyTransactionWitnessSet"))
    }
}

impl Serialize for ShelleyUpdate {
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
        serializer.write_map_sz(
            self.encodings
                .as_ref()
                .map(|encs| encs.shelley_proposed_protocol_parameter_updates_encoding)
                .unwrap_or_default()
                .to_len_sz(
                    self.shelley_proposed_protocol_parameter_updates.len() as u64,
                    force_canonical,
                ),
        )?;
        let mut key_order = self
            .shelley_proposed_protocol_parameter_updates
            .iter()
            .map(|(k, v)| {
                let mut buf = cbor_event::se::Serializer::new_vec();
                let shelley_proposed_protocol_parameter_updates_key_encoding = self
                    .encodings
                    .as_ref()
                    .and_then(|encs| {
                        encs.shelley_proposed_protocol_parameter_updates_key_encodings
                            .get(k)
                    })
                    .cloned()
                    .unwrap_or_default();
                buf.write_bytes_sz(
                    k.to_raw_bytes(),
                    shelley_proposed_protocol_parameter_updates_key_encoding
                        .to_str_len_sz(k.to_raw_bytes().len() as u64, force_canonical),
                )?;
                Ok((buf.finalize(), k, v))
            })
            .collect::<Result<Vec<(Vec<u8>, &_, &_)>, cbor_event::Error>>()?;
        if force_canonical {
            key_order.sort_by(|(lhs_bytes, _, _), (rhs_bytes, _, _)| {
                match lhs_bytes.len().cmp(&rhs_bytes.len()) {
                    std::cmp::Ordering::Equal => lhs_bytes.cmp(rhs_bytes),
                    diff_ord => diff_ord,
                }
            });
        }
        for (key_bytes, _key, value) in key_order {
            serializer.write_raw_bytes(&key_bytes)?;
            value.serialize(serializer, force_canonical)?;
        }
        self.encodings
            .as_ref()
            .map(|encs| encs.shelley_proposed_protocol_parameter_updates_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)?;
        serializer.write_unsigned_integer_sz(
            self.epoch,
            fit_sz(
                self.epoch,
                self.encodings
                    .as_ref()
                    .map(|encs| encs.epoch_encoding)
                    .unwrap_or_default(),
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

impl Deserialize for ShelleyUpdate {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let len_encoding: LenEncoding = len.into();
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let (
                shelley_proposed_protocol_parameter_updates,
                shelley_proposed_protocol_parameter_updates_encoding,
                shelley_proposed_protocol_parameter_updates_key_encodings,
            ) = (|| -> Result<_, DeserializeError> {
                let mut shelley_proposed_protocol_parameter_updates_table = OrderedHashMap::new();
                let shelley_proposed_protocol_parameter_updates_len = raw.map_sz()?;
                let shelley_proposed_protocol_parameter_updates_encoding =
                    shelley_proposed_protocol_parameter_updates_len.into();
                let mut shelley_proposed_protocol_parameter_updates_key_encodings = BTreeMap::new();
                while match shelley_proposed_protocol_parameter_updates_len {
                    cbor_event::LenSz::Len(n, _) => {
                        (shelley_proposed_protocol_parameter_updates_table.len() as u64) < n
                    }
                    cbor_event::LenSz::Indefinite => true,
                } {
                    if raw.cbor_type()? == cbor_event::Type::Special {
                        assert_eq!(raw.special()?, cbor_event::Special::Break);
                        break;
                    }
                    let (
                        shelley_proposed_protocol_parameter_updates_key,
                        shelley_proposed_protocol_parameter_updates_key_encoding,
                    ) = raw
                        .bytes_sz()
                        .map_err(Into::<DeserializeError>::into)
                        .and_then(|(bytes, enc)| {
                            GenesisHash::from_raw_bytes(&bytes)
                                .map(|bytes| (bytes, StringEncoding::from(enc)))
                                .map_err(|e| {
                                    DeserializeFailure::InvalidStructure(Box::new(e)).into()
                                })
                        })?;
                    let shelley_proposed_protocol_parameter_updates_value =
                        ShelleyProtocolParamUpdate::deserialize(raw)?;
                    if shelley_proposed_protocol_parameter_updates_table
                        .insert(
                            shelley_proposed_protocol_parameter_updates_key,
                            shelley_proposed_protocol_parameter_updates_value,
                        )
                        .is_some()
                    {
                        return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from(
                            "some complicated/unsupported type",
                        )))
                        .into());
                    }
                    shelley_proposed_protocol_parameter_updates_key_encodings.insert(
                        shelley_proposed_protocol_parameter_updates_key,
                        shelley_proposed_protocol_parameter_updates_key_encoding,
                    );
                }
                Ok((
                    shelley_proposed_protocol_parameter_updates_table,
                    shelley_proposed_protocol_parameter_updates_encoding,
                    shelley_proposed_protocol_parameter_updates_key_encodings,
                ))
            })()
            .map_err(|e| e.annotate("shelley_proposed_protocol_parameter_updates"))?;
            let (epoch, epoch_encoding) = raw
                .unsigned_integer_sz()
                .map_err(Into::<DeserializeError>::into)
                .map(|(x, enc)| (x, Some(enc)))
                .map_err(|e: DeserializeError| e.annotate("epoch"))?;
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(ShelleyUpdate {
                shelley_proposed_protocol_parameter_updates,
                epoch,
                encodings: Some(ShelleyUpdateEncoding {
                    len_encoding,
                    shelley_proposed_protocol_parameter_updates_encoding,
                    shelley_proposed_protocol_parameter_updates_key_encodings,
                    epoch_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("ShelleyUpdate"))
    }
}
