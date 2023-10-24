// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use super::cbor_encodings::*;
use super::*;
use crate::address::RewardAccount;
use crate::governance::{GovActionId, Voter, VotingProcedure};
use crate::{assets::AssetName, Script};
use cbor_event;
use cbor_event::de::Deserializer;
use cbor_event::se::Serializer;
use cml_core::error::*;
use cml_core::serialization::*;
use cml_crypto::RawBytesEncoding;
use cml_crypto::ScriptHash;
use std::io::{BufRead, Seek, SeekFrom, Write};

impl Serialize for AlonzoFormatTxOut {
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
                .to_len_sz(
                    2 + match &self.datum_hash {
                        Some(_x) => 1,
                        None => 0,
                    },
                    force_canonical,
                ),
        )?;
        self.address.serialize(serializer, force_canonical)?;
        self.amount.serialize(serializer, force_canonical)?;
        if let Some(field) = &self.datum_hash {
            serializer.write_bytes_sz(
                field.to_raw_bytes(),
                self.encodings
                    .as_ref()
                    .map(|encs| encs.datum_hash_encoding.clone())
                    .unwrap_or_default()
                    .to_str_len_sz(field.to_raw_bytes().len() as u64, force_canonical),
            )?;
        }
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for AlonzoFormatTxOut {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let len_encoding: LenEncoding = len.into();
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        (|| -> Result<_, DeserializeError> {
            let address =
                Address::deserialize(raw).map_err(|e: DeserializeError| e.annotate("address"))?;
            let amount =
                Value::deserialize(raw).map_err(|e: DeserializeError| e.annotate("amount"))?;
            let (datum_hash, datum_hash_encoding) = if raw
                .cbor_type()
                .map(|ty| ty == cbor_event::Type::Bytes)
                .unwrap_or(false)
            {
                (|| -> Result<_, DeserializeError> {
                    read_len.read_elems(1)?;
                    raw.bytes_sz()
                        .map_err(Into::<DeserializeError>::into)
                        .and_then(|(bytes, enc)| {
                            DatumHash::from_raw_bytes(&bytes)
                                .map(|bytes| (bytes, StringEncoding::from(enc)))
                                .map_err(|e| {
                                    DeserializeFailure::InvalidStructure(Box::new(e)).into()
                                })
                        })
                })()
                .map_err(|e| e.annotate("datum_hash"))
                .map(|(datum_hash, datum_hash_encoding)| (Some(datum_hash), datum_hash_encoding))
            } else {
                Ok((None, StringEncoding::default()))
            }?;
            match len {
                cbor_event::LenSz::Len(_, _) => read_len.finish()?,
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => read_len.finish()?,
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(AlonzoFormatTxOut {
                address,
                amount,
                datum_hash,
                encodings: Some(AlonzoFormatTxOutEncoding {
                    len_encoding,
                    datum_hash_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("AlonzoFormatTxOut"))
    }
}

impl Serialize for ConwayFormatTxOut {
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
                    2 + match &self.datum_option {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.script_reference {
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
                        == 2 + match &self.datum_option {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.script_reference {
                            Some(_) => 1,
                            None => 0,
                        }
            })
            .map(|encs| encs.orig_deser_order.clone())
            .unwrap_or_else(|| vec![0, 1, 2, 3]);
        for field_index in deser_order {
            match field_index {
                0 => {
                    serializer.write_unsigned_integer_sz(
                        0u64,
                        fit_sz(
                            0u64,
                            self.encodings
                                .as_ref()
                                .map(|encs| encs.address_key_encoding)
                                .unwrap_or_default(),
                            force_canonical,
                        ),
                    )?;
                    self.address.serialize(serializer, force_canonical)?;
                }
                1 => {
                    serializer.write_unsigned_integer_sz(
                        1u64,
                        fit_sz(
                            1u64,
                            self.encodings
                                .as_ref()
                                .map(|encs| encs.amount_key_encoding)
                                .unwrap_or_default(),
                            force_canonical,
                        ),
                    )?;
                    self.amount.serialize(serializer, force_canonical)?;
                }
                2 => {
                    if let Some(field) = &self.datum_option {
                        serializer.write_unsigned_integer_sz(
                            2u64,
                            fit_sz(
                                2u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.datum_option_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        field.serialize(serializer, force_canonical)?;
                    }
                }
                3 => {
                    if let Some(field) = &self.script_reference {
                        serializer.write_unsigned_integer_sz(
                            3u64,
                            fit_sz(
                                3u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.script_reference_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        serializer.write_tag_sz(
                            24u64,
                            fit_sz(
                                24u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.script_reference_tag_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        let mut script_reference_inner_se = Serializer::new_vec();
                        field.serialize(&mut script_reference_inner_se, force_canonical)?;
                        let script_reference_bytes = script_reference_inner_se.finalize();
                        serializer.write_bytes_sz(
                            &script_reference_bytes,
                            self.encodings
                                .as_ref()
                                .map(|encs| encs.script_reference_bytes_encoding.clone())
                                .unwrap_or_default()
                                .to_str_len_sz(
                                    script_reference_bytes.len() as u64,
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

impl Deserialize for ConwayFormatTxOut {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.map_sz()?;
        let len_encoding: LenEncoding = len.into();
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        (|| -> Result<_, DeserializeError> {
            let mut orig_deser_order = Vec::new();
            let mut address_key_encoding = None;
            let mut address = None;
            let mut amount_key_encoding = None;
            let mut amount = None;
            let mut datum_option_key_encoding = None;
            let mut datum_option = None;
            let mut script_reference_tag_encoding = None;
            let mut script_reference_bytes_encoding = StringEncoding::default();
            let mut script_reference_key_encoding = None;
            let mut script_reference = None;
            let mut read = 0;
            while match len {
                cbor_event::LenSz::Len(n, _) => read < n,
                cbor_event::LenSz::Indefinite => true,
            } {
                match raw.cbor_type()? {
                    cbor_event::Type::UnsignedInteger => match raw.unsigned_integer_sz()? {
                        (0, key_enc) => {
                            if address.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(0)).into());
                            }
                            let tmp_address = Address::deserialize(raw)
                                .map_err(|e: DeserializeError| e.annotate("address"))?;
                            address = Some(tmp_address);
                            address_key_encoding = Some(key_enc);
                            orig_deser_order.push(0);
                        }
                        (1, key_enc) => {
                            if amount.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(1)).into());
                            }
                            let tmp_amount = Value::deserialize(raw)
                                .map_err(|e: DeserializeError| e.annotate("amount"))?;
                            amount = Some(tmp_amount);
                            amount_key_encoding = Some(key_enc);
                            orig_deser_order.push(1);
                        }
                        (2, key_enc) => {
                            if datum_option.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(2)).into());
                            }
                            let tmp_datum_option = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                DatumOption::deserialize(raw)
                            })()
                            .map_err(|e| e.annotate("datum_option"))?;
                            datum_option = Some(tmp_datum_option);
                            datum_option_key_encoding = Some(key_enc);
                            orig_deser_order.push(2);
                        }
                        (3, key_enc) => {
                            if script_reference.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(3)).into());
                            }
                            let (
                                tmp_script_reference,
                                tmp_script_reference_tag_encoding,
                                tmp_script_reference_bytes_encoding,
                            ) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                match raw.tag_sz()? {
                                    (24, tag_enc) => {
                                        let (
                                            script_reference_bytes,
                                            script_reference_bytes_encoding,
                                        ) = raw.bytes_sz()?;
                                        let inner_de = &mut Deserializer::from(
                                            std::io::Cursor::new(script_reference_bytes),
                                        );
                                        Ok((
                                            Script::deserialize(inner_de)?,
                                            Some(tag_enc),
                                            StringEncoding::from(script_reference_bytes_encoding),
                                        ))
                                    }
                                    (tag, _enc) => Err(DeserializeFailure::TagMismatch {
                                        found: tag,
                                        expected: 24,
                                    }
                                    .into()),
                                }
                            })()
                            .map_err(|e| e.annotate("script_reference"))?;
                            script_reference = Some(tmp_script_reference);
                            script_reference_tag_encoding = tmp_script_reference_tag_encoding;
                            script_reference_bytes_encoding = tmp_script_reference_bytes_encoding;
                            script_reference_key_encoding = Some(key_enc);
                            orig_deser_order.push(3);
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
            let address = match address {
                Some(x) => x,
                None => return Err(DeserializeFailure::MandatoryFieldMissing(Key::Uint(0)).into()),
            };
            let amount = match amount {
                Some(x) => x,
                None => return Err(DeserializeFailure::MandatoryFieldMissing(Key::Uint(1)).into()),
            };
            read_len.finish()?;
            Ok(Self {
                address,
                amount,
                datum_option,
                script_reference,
                encodings: Some(ConwayFormatTxOutEncoding {
                    len_encoding,
                    orig_deser_order,
                    address_key_encoding,
                    amount_key_encoding,
                    datum_option_key_encoding,
                    script_reference_key_encoding,
                    script_reference_tag_encoding,
                    script_reference_bytes_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("ConwayFormatTxOut"))
    }
}

impl Serialize for DatumOption {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            DatumOption::Hash {
                datum_hash,
                len_encoding,
                tag_encoding,
                datum_hash_encoding,
            } => {
                serializer.write_array_sz(len_encoding.to_len_sz(2, force_canonical))?;
                serializer.write_unsigned_integer_sz(
                    0u64,
                    fit_sz(0u64, *tag_encoding, force_canonical),
                )?;
                serializer.write_bytes_sz(
                    datum_hash.to_raw_bytes(),
                    datum_hash_encoding
                        .to_str_len_sz(datum_hash.to_raw_bytes().len() as u64, force_canonical),
                )?;
                len_encoding.end(serializer, force_canonical)?;
                Ok(serializer)
            }
            DatumOption::Datum {
                datum,
                len_encoding,
                tag_encoding,
                datum_tag_encoding,
                datum_bytes_encoding,
            } => {
                serializer.write_array_sz(len_encoding.to_len_sz(2, force_canonical))?;
                serializer.write_unsigned_integer_sz(
                    1u64,
                    fit_sz(1u64, *tag_encoding, force_canonical),
                )?;
                serializer
                    .write_tag_sz(24u64, fit_sz(24u64, *datum_tag_encoding, force_canonical))?;
                let mut datum_inner_se = Serializer::new_vec();
                datum.serialize(&mut datum_inner_se, force_canonical)?;
                let datum_bytes = datum_inner_se.finalize();
                serializer.write_bytes_sz(
                    &datum_bytes,
                    datum_bytes_encoding.to_str_len_sz(datum_bytes.len() as u64, force_canonical),
                )?;
                len_encoding.end(serializer, force_canonical)?;
                Ok(serializer)
            }
        }
    }
}

impl Deserialize for DatumOption {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array_sz()?;
            let len_encoding: LenEncoding = len.into();
            let _read_len = CBORReadLen::new(len);
            let initial_position = raw.as_mut_ref().stream_position().unwrap();
            let mut errs = Vec::new();
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
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
                let (datum_hash, datum_hash_encoding) = raw
                    .bytes_sz()
                    .map_err(Into::<DeserializeError>::into)
                    .and_then(|(bytes, enc)| {
                        DatumHash::from_raw_bytes(&bytes)
                            .map(|bytes| (bytes, StringEncoding::from(enc)))
                            .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
                    })
                    .map_err(|e: DeserializeError| e.annotate("datum_hash"))?;
                match len {
                    cbor_event::LenSz::Len(_, _) => (),
                    cbor_event::LenSz::Indefinite => match raw.special()? {
                        cbor_event::Special::Break => (),
                        _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                    },
                }
                Ok(Self::Hash {
                    datum_hash,
                    len_encoding,
                    tag_encoding,
                    datum_hash_encoding,
                })
            })(raw)
            {
                Ok(variant) => return Ok(variant),
                Err(e) => {
                    errs.push(e.annotate("Hash"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
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
                let (datum, datum_tag_encoding, datum_bytes_encoding) =
                    (|| -> Result<_, DeserializeError> {
                        match raw.tag_sz()? {
                            (24, tag_enc) => {
                                let (datum_bytes, datum_bytes_encoding) = raw.bytes_sz()?;
                                let inner_de =
                                    &mut Deserializer::from(std::io::Cursor::new(datum_bytes));
                                Ok((
                                    PlutusData::deserialize(inner_de)?,
                                    Some(tag_enc),
                                    StringEncoding::from(datum_bytes_encoding),
                                ))
                            }
                            (tag, _enc) => Err(DeserializeFailure::TagMismatch {
                                found: tag,
                                expected: 24,
                            }
                            .into()),
                        }
                    })()
                    .map_err(|e| e.annotate("datum"))?;
                match len {
                    cbor_event::LenSz::Len(_, _) => (),
                    cbor_event::LenSz::Indefinite => match raw.special()? {
                        cbor_event::Special::Break => (),
                        _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                    },
                }
                Ok(Self::Datum {
                    datum,
                    len_encoding,
                    tag_encoding,
                    datum_tag_encoding,
                    datum_bytes_encoding,
                })
            })(raw)
            {
                Ok(variant) => return Ok(variant),
                Err(e) => {
                    errs.push(e.annotate("Datum"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Err(DeserializeError::new(
                "DatumOption",
                DeserializeFailure::NoVariantMatchedWithCauses(errs),
            ))
        })()
        .map_err(|e| e.annotate("DatumOption"))
    }
}

impl Serialize for NativeScript {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            NativeScript::ScriptPubkey(script_pubkey) => {
                script_pubkey.serialize(serializer, force_canonical)
            }
            NativeScript::ScriptAll(script_all) => {
                script_all.serialize(serializer, force_canonical)
            }
            NativeScript::ScriptAny(script_any) => {
                script_any.serialize(serializer, force_canonical)
            }
            NativeScript::ScriptNOfK(script_n_of_k) => {
                script_n_of_k.serialize(serializer, force_canonical)
            }
            NativeScript::ScriptInvalidBefore(script_invalid_before) => {
                script_invalid_before.serialize(serializer, force_canonical)
            }
            NativeScript::ScriptInvalidHereafter(script_invalid_hereafter) => {
                script_invalid_hereafter.serialize(serializer, force_canonical)
            }
        }
    }
}

impl Deserialize for NativeScript {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array_sz()?;
            let mut read_len = CBORReadLen::new(len);
            let initial_position = raw.as_mut_ref().stream_position().unwrap();
            let mut errs = Vec::new();
            let deser_variant: Result<_, DeserializeError> =
                ScriptPubkey::deserialize_as_embedded_group(raw, &mut read_len, len);
            match deser_variant {
                Ok(script_pubkey) => return Ok(Self::ScriptPubkey(script_pubkey)),
                Err(e) => {
                    errs.push(e.annotate("ScriptPubkey"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant: Result<_, DeserializeError> =
                ScriptAll::deserialize_as_embedded_group(raw, &mut read_len, len);
            match deser_variant {
                Ok(script_all) => return Ok(Self::ScriptAll(script_all)),
                Err(e) => {
                    errs.push(e.annotate("ScriptAll"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant: Result<_, DeserializeError> =
                ScriptAny::deserialize_as_embedded_group(raw, &mut read_len, len);
            match deser_variant {
                Ok(script_any) => return Ok(Self::ScriptAny(script_any)),
                Err(e) => {
                    errs.push(e.annotate("ScriptAny"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant: Result<_, DeserializeError> =
                ScriptNOfK::deserialize_as_embedded_group(raw, &mut read_len, len);
            match deser_variant {
                Ok(script_n_of_k) => return Ok(Self::ScriptNOfK(script_n_of_k)),
                Err(e) => {
                    errs.push(e.annotate("ScriptNOfK"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant: Result<_, DeserializeError> =
                ScriptInvalidBefore::deserialize_as_embedded_group(raw, &mut read_len, len);
            match deser_variant {
                Ok(script_invalid_before) => {
                    return Ok(Self::ScriptInvalidBefore(script_invalid_before))
                }
                Err(e) => {
                    errs.push(e.annotate("ScriptInvalidBefore"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant: Result<_, DeserializeError> =
                ScriptInvalidHereafter::deserialize_as_embedded_group(raw, &mut read_len, len);
            match deser_variant {
                Ok(script_invalid_hereafter) => {
                    return Ok(Self::ScriptInvalidHereafter(script_invalid_hereafter))
                }
                Err(e) => {
                    errs.push(e.annotate("ScriptInvalidHereafter"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Err(DeserializeError::new(
                "NativeScript",
                DeserializeFailure::NoVariantMatchedWithCauses(errs),
            ))
        })()
        .map_err(|e| e.annotate("NativeScript"))
    }
}

impl Serialize for ScriptAll {
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

impl SerializeEmbeddedGroup for ScriptAll {
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
                .map(|encs| encs.native_scripts_encoding)
                .unwrap_or_default()
                .to_len_sz(self.native_scripts.len() as u64, force_canonical),
        )?;
        for element in self.native_scripts.iter() {
            element.serialize(serializer, force_canonical)?;
        }
        self.encodings
            .as_ref()
            .map(|encs| encs.native_scripts_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)?;
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for ScriptAll {
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

impl DeserializeEmbeddedGroup for ScriptAll {
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
            let (native_scripts, native_scripts_encoding) = (|| -> Result<_, DeserializeError> {
                let mut native_scripts_arr = Vec::new();
                let len = raw.array_sz()?;
                let native_scripts_encoding = len.into();
                while match len {
                    cbor_event::LenSz::Len(n, _) => (native_scripts_arr.len() as u64) < n,
                    cbor_event::LenSz::Indefinite => true,
                } {
                    if raw.cbor_type()? == cbor_event::Type::Special {
                        assert_eq!(raw.special()?, cbor_event::Special::Break);
                        break;
                    }
                    native_scripts_arr.push(NativeScript::deserialize(raw)?);
                }
                Ok((native_scripts_arr, native_scripts_encoding))
            })()
            .map_err(|e| e.annotate("native_scripts"))?;
            Ok(ScriptAll {
                native_scripts,
                encodings: Some(ScriptAllEncoding {
                    len_encoding,
                    tag_encoding,
                    native_scripts_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("ScriptAll"))
    }
}

impl Serialize for ScriptAny {
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

impl SerializeEmbeddedGroup for ScriptAny {
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
                .map(|encs| encs.native_scripts_encoding)
                .unwrap_or_default()
                .to_len_sz(self.native_scripts.len() as u64, force_canonical),
        )?;
        for element in self.native_scripts.iter() {
            element.serialize(serializer, force_canonical)?;
        }
        self.encodings
            .as_ref()
            .map(|encs| encs.native_scripts_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)?;
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for ScriptAny {
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

impl DeserializeEmbeddedGroup for ScriptAny {
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
            let (native_scripts, native_scripts_encoding) = (|| -> Result<_, DeserializeError> {
                let mut native_scripts_arr = Vec::new();
                let len = raw.array_sz()?;
                let native_scripts_encoding = len.into();
                while match len {
                    cbor_event::LenSz::Len(n, _) => (native_scripts_arr.len() as u64) < n,
                    cbor_event::LenSz::Indefinite => true,
                } {
                    if raw.cbor_type()? == cbor_event::Type::Special {
                        assert_eq!(raw.special()?, cbor_event::Special::Break);
                        break;
                    }
                    native_scripts_arr.push(NativeScript::deserialize(raw)?);
                }
                Ok((native_scripts_arr, native_scripts_encoding))
            })()
            .map_err(|e| e.annotate("native_scripts"))?;
            Ok(ScriptAny {
                native_scripts,
                encodings: Some(ScriptAnyEncoding {
                    len_encoding,
                    tag_encoding,
                    native_scripts_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("ScriptAny"))
    }
}

impl Serialize for ScriptInvalidBefore {
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

impl SerializeEmbeddedGroup for ScriptInvalidBefore {
    fn serialize_as_embedded_group<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer_sz(
            4u64,
            fit_sz(
                4u64,
                self.encodings
                    .as_ref()
                    .map(|encs| encs.tag_encoding)
                    .unwrap_or_default(),
                force_canonical,
            ),
        )?;
        serializer.write_unsigned_integer_sz(
            self.before,
            fit_sz(
                self.before,
                self.encodings
                    .as_ref()
                    .map(|encs| encs.before_encoding)
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

impl Deserialize for ScriptInvalidBefore {
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

impl DeserializeEmbeddedGroup for ScriptInvalidBefore {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        _read_len: &mut CBORReadLen,
        len: cbor_event::LenSz,
    ) -> Result<Self, DeserializeError> {
        let len_encoding = len.into();
        (|| -> Result<_, DeserializeError> {
            let tag_encoding = (|| -> Result<_, DeserializeError> {
                let (tag_value, tag_encoding) = raw.unsigned_integer_sz()?;
                if tag_value != 4 {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(tag_value),
                        expected: Key::Uint(4),
                    }
                    .into());
                }
                Ok(Some(tag_encoding))
            })()
            .map_err(|e| e.annotate("tag"))?;
            let (before, before_encoding) = raw
                .unsigned_integer_sz()
                .map(|(x, enc)| (x, Some(enc)))
                .map_err(Into::<DeserializeError>::into)
                .map_err(|e: DeserializeError| e.annotate("before"))?;
            Ok(ScriptInvalidBefore {
                before,
                encodings: Some(ScriptInvalidBeforeEncoding {
                    len_encoding,
                    tag_encoding,
                    before_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("ScriptInvalidBefore"))
    }
}

impl Serialize for ScriptInvalidHereafter {
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

impl SerializeEmbeddedGroup for ScriptInvalidHereafter {
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
        serializer.write_unsigned_integer_sz(
            self.after,
            fit_sz(
                self.after,
                self.encodings
                    .as_ref()
                    .map(|encs| encs.after_encoding)
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

impl Deserialize for ScriptInvalidHereafter {
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

impl DeserializeEmbeddedGroup for ScriptInvalidHereafter {
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
            let (after, after_encoding) = raw
                .unsigned_integer_sz()
                .map(|(x, enc)| (x, Some(enc)))
                .map_err(Into::<DeserializeError>::into)
                .map_err(|e: DeserializeError| e.annotate("after"))?;
            Ok(ScriptInvalidHereafter {
                after,
                encodings: Some(ScriptInvalidHereafterEncoding {
                    len_encoding,
                    tag_encoding,
                    after_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("ScriptInvalidHereafter"))
    }
}

impl Serialize for ScriptNOfK {
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

impl SerializeEmbeddedGroup for ScriptNOfK {
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
                .map(|encs| encs.native_scripts_encoding)
                .unwrap_or_default()
                .to_len_sz(self.native_scripts.len() as u64, force_canonical),
        )?;
        for element in self.native_scripts.iter() {
            element.serialize(serializer, force_canonical)?;
        }
        self.encodings
            .as_ref()
            .map(|encs| encs.native_scripts_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)?;
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for ScriptNOfK {
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

impl DeserializeEmbeddedGroup for ScriptNOfK {
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
                .map(|(x, enc)| (x, Some(enc)))
                .map_err(Into::<DeserializeError>::into)
                .map_err(|e: DeserializeError| e.annotate("n"))?;
            let (native_scripts, native_scripts_encoding) = (|| -> Result<_, DeserializeError> {
                let mut native_scripts_arr = Vec::new();
                let len = raw.array_sz()?;
                let native_scripts_encoding = len.into();
                while match len {
                    cbor_event::LenSz::Len(n, _) => (native_scripts_arr.len() as u64) < n,
                    cbor_event::LenSz::Indefinite => true,
                } {
                    if raw.cbor_type()? == cbor_event::Type::Special {
                        assert_eq!(raw.special()?, cbor_event::Special::Break);
                        break;
                    }
                    native_scripts_arr.push(NativeScript::deserialize(raw)?);
                }
                Ok((native_scripts_arr, native_scripts_encoding))
            })()
            .map_err(|e| e.annotate("native_scripts"))?;
            Ok(ScriptNOfK {
                n,
                native_scripts,
                encodings: Some(ScriptNOfKEncoding {
                    len_encoding,
                    tag_encoding,
                    n_encoding,
                    native_scripts_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("ScriptNOfK"))
    }
}

impl Serialize for ScriptPubkey {
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

impl SerializeEmbeddedGroup for ScriptPubkey {
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

impl Deserialize for ScriptPubkey {
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

impl DeserializeEmbeddedGroup for ScriptPubkey {
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
            Ok(ScriptPubkey {
                ed25519_key_hash,
                encodings: Some(ScriptPubkeyEncoding {
                    len_encoding,
                    tag_encoding,
                    ed25519_key_hash_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("ScriptPubkey"))
    }
}

impl Serialize for Transaction {
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
        self.body.serialize(serializer, force_canonical)?;
        self.witness_set.serialize(serializer, force_canonical)?;
        serializer.write_special(cbor_event::Special::Bool(self.is_valid))?;
        match &self.auxiliary_data {
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

impl Deserialize for Transaction {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let len_encoding: LenEncoding = len.into();
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(4)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let body = TransactionBody::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("body"))?;
            let witness_set = TransactionWitnessSet::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("witness_set"))?;
            let is_valid = raw
                .bool()
                .map_err(Into::into)
                .map_err(|e: DeserializeError| e.annotate("is_valid"))?;
            let auxiliary_data = (|| -> Result<_, DeserializeError> {
                Ok(match raw.cbor_type()? != cbor_event::Type::Special {
                    true => Some(AuxiliaryData::deserialize(raw)?),
                    false => {
                        if raw.special()? != cbor_event::Special::Null {
                            return Err(DeserializeFailure::ExpectedNull.into());
                        }
                        None
                    }
                })
            })()
            .map_err(|e| e.annotate("auxiliary_data"))?;
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(Transaction {
                body,
                witness_set,
                is_valid,
                auxiliary_data,
                encodings: Some(TransactionEncoding { len_encoding }),
            })
        })()
        .map_err(|e| e.annotate("Transaction"))
    }
}

impl Serialize for TransactionBody {
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
                    3 + match &self.ttl {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.certs {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.withdrawals {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.auxiliary_data_hash {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.validity_interval_start {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.mint {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.script_data_hash {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.collateral_inputs {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.required_signers {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.network_id {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.collateral_return {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.total_collateral {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.reference_inputs {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.voting_procedures {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.proposal_procedures {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.current_treasury_value {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.donation {
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
                        == 3 + match &self.ttl {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.certs {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.withdrawals {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.auxiliary_data_hash {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.validity_interval_start {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.mint {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.script_data_hash {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.collateral_inputs {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.required_signers {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.network_id {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.collateral_return {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.total_collateral {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.reference_inputs {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.voting_procedures {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.proposal_procedures {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.current_treasury_value {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.donation {
                            Some(_) => 1,
                            None => 0,
                        }
            })
            .map(|encs| encs.orig_deser_order.clone())
            .unwrap_or_else(|| {
                vec![
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
                ]
            });
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
                    if let Some(field) = &self.ttl {
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
                            *field,
                            fit_sz(
                                *field,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.ttl_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                    }
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
                7 => {
                    if let Some(field) = &self.validity_interval_start {
                        serializer.write_unsigned_integer_sz(
                            8u64,
                            fit_sz(
                                8u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.validity_interval_start_key_encoding)
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
                                    .map(|encs| encs.validity_interval_start_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                    }
                }
                8 => {
                    if let Some(field) = &self.mint {
                        serializer.write_unsigned_integer_sz(
                            9u64,
                            fit_sz(
                                9u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.mint_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        serializer.write_map_sz(
                            self.encodings
                                .as_ref()
                                .map(|encs| encs.mint_encoding)
                                .unwrap_or_default()
                                .to_len_sz(field.len() as u64, force_canonical),
                        )?;
                        let mut key_order = field
                            .iter()
                            .map(|(k, v)| {
                                let mut buf = cbor_event::se::Serializer::new_vec();
                                let mint_key_encoding = self
                                    .encodings
                                    .as_ref()
                                    .and_then(|encs| encs.mint_key_encodings.get(k))
                                    .cloned()
                                    .unwrap_or_default();
                                buf.write_bytes_sz(
                                    k.to_raw_bytes(),
                                    mint_key_encoding.to_str_len_sz(
                                        k.to_raw_bytes().len() as u64,
                                        force_canonical,
                                    ),
                                )?;
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
                            let (mint_value_encoding, mint_value_value_encodings) = self
                                .encodings
                                .as_ref()
                                .and_then(|encs| encs.mint_value_encodings.get(key))
                                .cloned()
                                .unwrap_or_else(|| (LenEncoding::default(), BTreeMap::new()));
                            serializer.write_map_sz(
                                mint_value_encoding.to_len_sz(value.len() as u64, force_canonical),
                            )?;
                            let mut key_order = value
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
                                let mint_value_value_encoding = mint_value_value_encodings
                                    .get(key)
                                    .cloned()
                                    .unwrap_or_default();
                                if *value >= 0 {
                                    serializer.write_unsigned_integer_sz(
                                        *value as u64,
                                        fit_sz(
                                            *value as u64,
                                            mint_value_value_encoding,
                                            force_canonical,
                                        ),
                                    )?;
                                } else {
                                    serializer.write_negative_integer_sz(
                                        *value as i128,
                                        fit_sz(
                                            (*value + 1).unsigned_abs(),
                                            mint_value_value_encoding,
                                            force_canonical,
                                        ),
                                    )?;
                                }
                            }
                            mint_value_encoding.end(serializer, force_canonical)?;
                        }
                        self.encodings
                            .as_ref()
                            .map(|encs| encs.mint_encoding)
                            .unwrap_or_default()
                            .end(serializer, force_canonical)?;
                    }
                }
                9 => {
                    if let Some(field) = &self.script_data_hash {
                        serializer.write_unsigned_integer_sz(
                            11u64,
                            fit_sz(
                                11u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.script_data_hash_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        serializer.write_bytes_sz(
                            field.to_raw_bytes(),
                            self.encodings
                                .as_ref()
                                .map(|encs| encs.script_data_hash_encoding.clone())
                                .unwrap_or_default()
                                .to_str_len_sz(field.to_raw_bytes().len() as u64, force_canonical),
                        )?;
                    }
                }
                10 => {
                    if let Some(field) = &self.collateral_inputs {
                        serializer.write_unsigned_integer_sz(
                            13u64,
                            fit_sz(
                                13u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.collateral_inputs_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        serializer.write_array_sz(
                            self.encodings
                                .as_ref()
                                .map(|encs| encs.collateral_inputs_encoding)
                                .unwrap_or_default()
                                .to_len_sz(field.len() as u64, force_canonical),
                        )?;
                        for element in field.iter() {
                            element.serialize(serializer, force_canonical)?;
                        }
                        self.encodings
                            .as_ref()
                            .map(|encs| encs.collateral_inputs_encoding)
                            .unwrap_or_default()
                            .end(serializer, force_canonical)?;
                    }
                }
                11 => {
                    if let Some(field) = &self.required_signers {
                        serializer.write_unsigned_integer_sz(
                            14u64,
                            fit_sz(
                                14u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.required_signers_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        serializer.write_array_sz(
                            self.encodings
                                .as_ref()
                                .map(|encs| encs.required_signers_encoding)
                                .unwrap_or_default()
                                .to_len_sz(field.len() as u64, force_canonical),
                        )?;
                        for (i, element) in field.iter().enumerate() {
                            let required_signers_elem_encoding = self
                                .encodings
                                .as_ref()
                                .and_then(|encs| encs.required_signers_elem_encodings.get(i))
                                .cloned()
                                .unwrap_or_default();
                            serializer.write_bytes_sz(
                                element.to_raw_bytes(),
                                required_signers_elem_encoding.to_str_len_sz(
                                    element.to_raw_bytes().len() as u64,
                                    force_canonical,
                                ),
                            )?;
                        }
                        self.encodings
                            .as_ref()
                            .map(|encs| encs.required_signers_encoding)
                            .unwrap_or_default()
                            .end(serializer, force_canonical)?;
                    }
                }
                12 => {
                    if let Some(field) = &self.network_id {
                        serializer.write_unsigned_integer_sz(
                            15u64,
                            fit_sz(
                                15u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.network_id_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        field.serialize(serializer, force_canonical)?;
                    }
                }
                13 => {
                    if let Some(field) = &self.collateral_return {
                        serializer.write_unsigned_integer_sz(
                            16u64,
                            fit_sz(
                                16u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.collateral_return_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        field.serialize(serializer, force_canonical)?;
                    }
                }
                14 => {
                    if let Some(field) = &self.total_collateral {
                        serializer.write_unsigned_integer_sz(
                            17u64,
                            fit_sz(
                                17u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.total_collateral_key_encoding)
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
                                    .map(|encs| encs.total_collateral_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                    }
                }
                15 => {
                    if let Some(field) = &self.reference_inputs {
                        serializer.write_unsigned_integer_sz(
                            18u64,
                            fit_sz(
                                18u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.reference_inputs_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        serializer.write_array_sz(
                            self.encodings
                                .as_ref()
                                .map(|encs| encs.reference_inputs_encoding)
                                .unwrap_or_default()
                                .to_len_sz(field.len() as u64, force_canonical),
                        )?;
                        for element in field.iter() {
                            element.serialize(serializer, force_canonical)?;
                        }
                        self.encodings
                            .as_ref()
                            .map(|encs| encs.reference_inputs_encoding)
                            .unwrap_or_default()
                            .end(serializer, force_canonical)?;
                    }
                }
                16 => {
                    if let Some(field) = &self.voting_procedures {
                        serializer.write_unsigned_integer_sz(
                            19u64,
                            fit_sz(
                                19u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.voting_procedures_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        serializer.write_map_sz(
                            self.encodings
                                .as_ref()
                                .map(|encs| encs.voting_procedures_encoding)
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
                            let voting_procedures_value_encoding = self
                                .encodings
                                .as_ref()
                                .and_then(|encs| encs.voting_procedures_value_encodings.get(key))
                                .cloned()
                                .unwrap_or_default();
                            serializer.write_map_sz(
                                voting_procedures_value_encoding
                                    .to_len_sz(value.len() as u64, force_canonical),
                            )?;
                            let mut key_order = value
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
                            for (key_bytes, _key, value) in key_order {
                                serializer.write_raw_bytes(&key_bytes)?;
                                value.serialize(serializer, force_canonical)?;
                            }
                            voting_procedures_value_encoding.end(serializer, force_canonical)?;
                        }
                        self.encodings
                            .as_ref()
                            .map(|encs| encs.voting_procedures_encoding)
                            .unwrap_or_default()
                            .end(serializer, force_canonical)?;
                    }
                }
                17 => {
                    if let Some(field) = &self.proposal_procedures {
                        serializer.write_unsigned_integer_sz(
                            20u64,
                            fit_sz(
                                20u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.proposal_procedures_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        serializer.write_array_sz(
                            self.encodings
                                .as_ref()
                                .map(|encs| encs.proposal_procedures_encoding)
                                .unwrap_or_default()
                                .to_len_sz(field.len() as u64, force_canonical),
                        )?;
                        for element in field.iter() {
                            element.serialize(serializer, force_canonical)?;
                        }
                        self.encodings
                            .as_ref()
                            .map(|encs| encs.proposal_procedures_encoding)
                            .unwrap_or_default()
                            .end(serializer, force_canonical)?;
                    }
                }
                18 => {
                    if let Some(field) = &self.current_treasury_value {
                        serializer.write_unsigned_integer_sz(
                            21u64,
                            fit_sz(
                                21u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.current_treasury_value_key_encoding)
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
                                    .map(|encs| encs.current_treasury_value_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                    }
                }
                19 => {
                    if let Some(field) = &self.donation {
                        serializer.write_unsigned_integer_sz(
                            22u64,
                            fit_sz(
                                22u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.donation_key_encoding)
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
                                    .map(|encs| encs.donation_encoding)
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

impl Deserialize for TransactionBody {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.map_sz()?;
        let len_encoding: LenEncoding = len.into();
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(3)?;
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
            let mut auxiliary_data_hash_encoding = StringEncoding::default();
            let mut auxiliary_data_hash_key_encoding = None;
            let mut auxiliary_data_hash = None;
            let mut validity_interval_start_encoding = None;
            let mut validity_interval_start_key_encoding = None;
            let mut validity_interval_start = None;
            let mut mint_encoding = LenEncoding::default();
            let mut mint_key_encodings = BTreeMap::new();
            let mut mint_value_encodings = BTreeMap::new();
            let mut mint_key_encoding = None;
            let mut mint = None;
            let mut script_data_hash_encoding = StringEncoding::default();
            let mut script_data_hash_key_encoding = None;
            let mut script_data_hash = None;
            let mut collateral_inputs_encoding = LenEncoding::default();
            let mut collateral_inputs_key_encoding = None;
            let mut collateral_inputs = None;
            let mut required_signers_encoding = LenEncoding::default();
            let mut required_signers_elem_encodings = Vec::new();
            let mut required_signers_key_encoding = None;
            let mut required_signers = None;
            let mut network_id_key_encoding = None;
            let mut network_id = None;
            let mut collateral_return_key_encoding = None;
            let mut collateral_return = None;
            let mut total_collateral_encoding = None;
            let mut total_collateral_key_encoding = None;
            let mut total_collateral = None;
            let mut reference_inputs_encoding = LenEncoding::default();
            let mut reference_inputs_key_encoding = None;
            let mut reference_inputs = None;
            let mut voting_procedures_encoding = LenEncoding::default();
            let mut voting_procedures_value_encodings = BTreeMap::new();
            let mut voting_procedures_key_encoding = None;
            let mut voting_procedures = None;
            let mut proposal_procedures_encoding = LenEncoding::default();
            let mut proposal_procedures_key_encoding = None;
            let mut proposal_procedures = None;
            let mut current_treasury_value_encoding = None;
            let mut current_treasury_value_key_encoding = None;
            let mut current_treasury_value = None;
            let mut donation_encoding = None;
            let mut donation_key_encoding = None;
            let mut donation = None;
            let mut read = 0;
            while match len { cbor_event::LenSz::Len(n, _) => read < n, cbor_event::LenSz::Indefinite => true, } {
                match raw.cbor_type()? {
                    cbor_event::Type::UnsignedInteger => match raw.unsigned_integer_sz()? {
                        (0, key_enc) =>  {
                            if inputs.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(0)).into());
                            }
                            let (tmp_inputs, tmp_inputs_encoding) = (|| -> Result<_, DeserializeError> {
                                let mut inputs_arr = Vec::new();
                                let len = raw.array_sz()?;
                                let inputs_encoding = len.into();
                                while match len { cbor_event::LenSz::Len(n, _) => (inputs_arr.len() as u64) < n, cbor_event::LenSz::Indefinite => true, } {
                                    if raw.cbor_type()? == cbor_event::Type::Special {
                                        assert_eq!(raw.special()?, cbor_event::Special::Break);
                                        break;
                                    }
                                    inputs_arr.push(TransactionInput::deserialize(raw)?);
                                }
                                Ok((inputs_arr, inputs_encoding))
                            })().map_err(|e| e.annotate("inputs"))?;
                            inputs = Some(tmp_inputs);
                            inputs_encoding = tmp_inputs_encoding;
                            inputs_key_encoding = Some(key_enc);
                            orig_deser_order.push(0);
                        },
                        (1, key_enc) =>  {
                            if outputs.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(1)).into());
                            }
                            let (tmp_outputs, tmp_outputs_encoding) = (|| -> Result<_, DeserializeError> {
                                let mut outputs_arr = Vec::new();
                                let len = raw.array_sz()?;
                                let outputs_encoding = len.into();
                                while match len { cbor_event::LenSz::Len(n, _) => (outputs_arr.len() as u64) < n, cbor_event::LenSz::Indefinite => true, } {
                                    if raw.cbor_type()? == cbor_event::Type::Special {
                                        assert_eq!(raw.special()?, cbor_event::Special::Break);
                                        break;
                                    }
                                    outputs_arr.push(TransactionOutput::deserialize(raw)?);
                                }
                                Ok((outputs_arr, outputs_encoding))
                            })().map_err(|e| e.annotate("outputs"))?;
                            outputs = Some(tmp_outputs);
                            outputs_encoding = tmp_outputs_encoding;
                            outputs_key_encoding = Some(key_enc);
                            orig_deser_order.push(1);
                        },
                        (2, key_enc) =>  {
                            if fee.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(2)).into());
                            }
                            let (tmp_fee, tmp_fee_encoding) = raw.unsigned_integer_sz().map(|(x, enc)| (x, Some(enc))).map_err(Into::<DeserializeError>::into).map_err(|e: DeserializeError| e.annotate("fee"))?;
                            fee = Some(tmp_fee);
                            fee_encoding = tmp_fee_encoding;
                            fee_key_encoding = Some(key_enc);
                            orig_deser_order.push(2);
                        },
                        (3, key_enc) =>  {
                            if ttl.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(3)).into());
                            }
                            let (tmp_ttl, tmp_ttl_encoding) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                raw.unsigned_integer_sz().map(|(x, enc)| (x, Some(enc))).map_err(Into::<DeserializeError>::into)
                            })().map_err(|e| e.annotate("ttl"))?;
                            ttl = Some(tmp_ttl);
                            ttl_encoding = tmp_ttl_encoding;
                            ttl_key_encoding = Some(key_enc);
                            orig_deser_order.push(3);
                        },
                        (4, key_enc) =>  {
                            if certs.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(4)).into());
                            }
                            let (tmp_certs, tmp_certs_encoding) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                let mut certs_arr = Vec::new();
                                let len = raw.array_sz()?;
                                let certs_encoding = len.into();
                                while match len { cbor_event::LenSz::Len(n, _) => (certs_arr.len() as u64) < n, cbor_event::LenSz::Indefinite => true, } {
                                    if raw.cbor_type()? == cbor_event::Type::Special {
                                        assert_eq!(raw.special()?, cbor_event::Special::Break);
                                        break;
                                    }
                                    certs_arr.push(Certificate::deserialize(raw)?);
                                }
                                Ok((certs_arr, certs_encoding))
                            })().map_err(|e| e.annotate("certs"))?;
                            certs = Some(tmp_certs);
                            certs_encoding = tmp_certs_encoding;
                            certs_key_encoding = Some(key_enc);
                            orig_deser_order.push(4);
                        },
                        (5, key_enc) =>  {
                            if withdrawals.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(5)).into());
                            }
                            let (tmp_withdrawals, tmp_withdrawals_encoding, tmp_withdrawals_value_encodings) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                let mut withdrawals_table = OrderedHashMap::new();
                                let withdrawals_len = raw.map_sz()?;
                                let withdrawals_encoding = withdrawals_len.into();
                                let mut withdrawals_value_encodings = BTreeMap::new();
                                while match withdrawals_len { cbor_event::LenSz::Len(n, _) => (withdrawals_table.len() as u64) < n, cbor_event::LenSz::Indefinite => true, } {
                                    if raw.cbor_type()? == cbor_event::Type::Special {
                                        assert_eq!(raw.special()?, cbor_event::Special::Break);
                                        break;
                                    }
                                    let withdrawals_key = RewardAccount::deserialize(raw)?;
                                    let (withdrawals_value, withdrawals_value_encoding) = raw.unsigned_integer_sz().map(|(x, enc)| (x, Some(enc)))?;
                                    if withdrawals_table.insert(withdrawals_key.clone(), withdrawals_value).is_some() {
                                        return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from("some complicated/unsupported type"))).into());
                                    }
                                    withdrawals_value_encodings.insert(withdrawals_key, withdrawals_value_encoding);
                                }
                                Ok((withdrawals_table, withdrawals_encoding, withdrawals_value_encodings))
                            })().map_err(|e| e.annotate("withdrawals"))?;
                            withdrawals = Some(tmp_withdrawals);
                            withdrawals_encoding = tmp_withdrawals_encoding;
                            withdrawals_value_encodings = tmp_withdrawals_value_encodings;
                            withdrawals_key_encoding = Some(key_enc);
                            orig_deser_order.push(5);
                        },
                        (7, key_enc) =>  {
                            if auxiliary_data_hash.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(7)).into());
                            }
                            let (tmp_auxiliary_data_hash, tmp_auxiliary_data_hash_encoding) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                raw.bytes_sz().map_err(Into::<DeserializeError>::into).and_then(|(bytes, enc)| AuxiliaryDataHash::from_raw_bytes(&bytes).map(|bytes| (bytes, StringEncoding::from(enc))).map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into()))
                            })().map_err(|e| e.annotate("auxiliary_data_hash"))?;
                            auxiliary_data_hash = Some(tmp_auxiliary_data_hash);
                            auxiliary_data_hash_encoding = tmp_auxiliary_data_hash_encoding;
                            auxiliary_data_hash_key_encoding = Some(key_enc);
                            orig_deser_order.push(6);
                        },
                        (8, key_enc) =>  {
                            if validity_interval_start.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(8)).into());
                            }
                            let (tmp_validity_interval_start, tmp_validity_interval_start_encoding) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                raw.unsigned_integer_sz().map(|(x, enc)| (x, Some(enc))).map_err(Into::<DeserializeError>::into)
                            })().map_err(|e| e.annotate("validity_interval_start"))?;
                            validity_interval_start = Some(tmp_validity_interval_start);
                            validity_interval_start_encoding = tmp_validity_interval_start_encoding;
                            validity_interval_start_key_encoding = Some(key_enc);
                            orig_deser_order.push(7);
                        },
                        (9, key_enc) =>  {
                            if mint.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(9)).into());
                            }
                            let (tmp_mint, tmp_mint_encoding, tmp_mint_key_encodings, tmp_mint_value_encodings) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                let mut mint_table = OrderedHashMap::new();
                                let mint_len = raw.map_sz()?;
                                let mint_encoding = mint_len.into();
                                let mut mint_key_encodings = BTreeMap::new();
                                let mut mint_value_encodings = BTreeMap::new();
                                while match mint_len { cbor_event::LenSz::Len(n, _) => (mint_table.len() as u64) < n, cbor_event::LenSz::Indefinite => true, } {
                                    if raw.cbor_type()? == cbor_event::Type::Special {
                                        assert_eq!(raw.special()?, cbor_event::Special::Break);
                                        break;
                                    }
                                    let (mint_key, mint_key_encoding) = raw.bytes_sz().map_err(Into::<DeserializeError>::into).and_then(|(bytes, enc)| ScriptHash::from_raw_bytes(&bytes).map(|bytes| (bytes, StringEncoding::from(enc))).map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into()))?;
                                    let mut mint_value_table = OrderedHashMap::new();
                                    let mint_value_len = raw.map_sz()?;
                                    let mint_value_encoding = mint_value_len.into();
                                    let mut mint_value_value_encodings = BTreeMap::new();
                                    while match mint_value_len { cbor_event::LenSz::Len(n, _) => (mint_value_table.len() as u64) < n, cbor_event::LenSz::Indefinite => true, } {
                                        if raw.cbor_type()? == cbor_event::Type::Special {
                                            assert_eq!(raw.special()?, cbor_event::Special::Break);
                                            break;
                                        }
                                        let mint_value_key = AssetName::deserialize(raw)?;
                                        let (mint_value_value, mint_value_value_encoding) = match raw.cbor_type()? {
                                            cbor_event::Type::UnsignedInteger => {
                                                let (x, enc) = raw.unsigned_integer_sz()?;
                                                (x as i64, Some(enc))
                                            },
                                            _ => {
                                                let (x, enc) = raw.negative_integer_sz()?;
                                                (x as i64, Some(enc))
                                            },
                                        };
                                        if mint_value_table.insert(mint_value_key.clone(), mint_value_value).is_some() {
                                            return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from("some complicated/unsupported type"))).into());
                                        }
                                        mint_value_value_encodings.insert(mint_value_key, mint_value_value_encoding);
                                    }
                                    let (mint_value, mint_value_encoding, mint_value_value_encodings) = (mint_value_table, mint_value_encoding, mint_value_value_encodings);
                                    if mint_table.insert(mint_key, mint_value).is_some() {
                                        return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from("some complicated/unsupported type"))).into());
                                    }
                                    mint_key_encodings.insert(mint_key, mint_key_encoding);
                                    mint_value_encodings.insert(mint_key, (mint_value_encoding, mint_value_value_encodings));
                                }
                                Ok((mint_table, mint_encoding, mint_key_encodings, mint_value_encodings))
                            })().map_err(|e| e.annotate("mint"))?;
                            mint = Some(tmp_mint);
                            mint_encoding = tmp_mint_encoding;
                            mint_key_encodings = tmp_mint_key_encodings;
                            mint_value_encodings = tmp_mint_value_encodings;
                            mint_key_encoding = Some(key_enc);
                            orig_deser_order.push(8);
                        },
                        (11, key_enc) =>  {
                            if script_data_hash.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(11)).into());
                            }
                            let (tmp_script_data_hash, tmp_script_data_hash_encoding) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                raw.bytes_sz().map_err(Into::<DeserializeError>::into).and_then(|(bytes, enc)| ScriptDataHash::from_raw_bytes(&bytes).map(|bytes| (bytes, StringEncoding::from(enc))).map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into()))
                            })().map_err(|e| e.annotate("script_data_hash"))?;
                            script_data_hash = Some(tmp_script_data_hash);
                            script_data_hash_encoding = tmp_script_data_hash_encoding;
                            script_data_hash_key_encoding = Some(key_enc);
                            orig_deser_order.push(9);
                        },
                        (13, key_enc) =>  {
                            if collateral_inputs.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(13)).into());
                            }
                            let (tmp_collateral_inputs, tmp_collateral_inputs_encoding) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                let mut collateral_inputs_arr = Vec::new();
                                let len = raw.array_sz()?;
                                let collateral_inputs_encoding = len.into();
                                while match len { cbor_event::LenSz::Len(n, _) => (collateral_inputs_arr.len() as u64) < n, cbor_event::LenSz::Indefinite => true, } {
                                    if raw.cbor_type()? == cbor_event::Type::Special {
                                        assert_eq!(raw.special()?, cbor_event::Special::Break);
                                        break;
                                    }
                                    collateral_inputs_arr.push(TransactionInput::deserialize(raw)?);
                                }
                                Ok((collateral_inputs_arr, collateral_inputs_encoding))
                            })().map_err(|e| e.annotate("collateral_inputs"))?;
                            collateral_inputs = Some(tmp_collateral_inputs);
                            collateral_inputs_encoding = tmp_collateral_inputs_encoding;
                            collateral_inputs_key_encoding = Some(key_enc);
                            orig_deser_order.push(10);
                        },
                        (14, key_enc) =>  {
                            if required_signers.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(14)).into());
                            }
                            let (tmp_required_signers, tmp_required_signers_encoding, tmp_required_signers_elem_encodings) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                let mut required_signers_arr = Vec::new();
                                let len = raw.array_sz()?;
                                let required_signers_encoding = len.into();
                                let mut required_signers_elem_encodings = Vec::new();
                                while match len { cbor_event::LenSz::Len(n, _) => (required_signers_arr.len() as u64) < n, cbor_event::LenSz::Indefinite => true, } {
                                    if raw.cbor_type()? == cbor_event::Type::Special {
                                        assert_eq!(raw.special()?, cbor_event::Special::Break);
                                        break;
                                    }
                                    let (required_signers_elem, required_signers_elem_encoding) = raw.bytes_sz().map_err(Into::<DeserializeError>::into).and_then(|(bytes, enc)| Ed25519KeyHash::from_raw_bytes(&bytes).map(|bytes| (bytes, StringEncoding::from(enc))).map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into()))?;
                                    required_signers_arr.push(required_signers_elem);
                                    required_signers_elem_encodings.push(required_signers_elem_encoding);
                                }
                                Ok((required_signers_arr, required_signers_encoding, required_signers_elem_encodings))
                            })().map_err(|e| e.annotate("required_signers"))?;
                            required_signers = Some(tmp_required_signers);
                            required_signers_encoding = tmp_required_signers_encoding;
                            required_signers_elem_encodings = tmp_required_signers_elem_encodings;
                            required_signers_key_encoding = Some(key_enc);
                            orig_deser_order.push(11);
                        },
                        (15, key_enc) =>  {
                            if network_id.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(15)).into());
                            }
                            let tmp_network_id = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                NetworkId::deserialize(raw)
                            })().map_err(|e| e.annotate("network_id"))?;
                            network_id = Some(tmp_network_id);
                            network_id_key_encoding = Some(key_enc);
                            orig_deser_order.push(12);
                        },
                        (16, key_enc) =>  {
                            if collateral_return.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(16)).into());
                            }
                            let tmp_collateral_return = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                TransactionOutput::deserialize(raw)
                            })().map_err(|e| e.annotate("collateral_return"))?;
                            collateral_return = Some(tmp_collateral_return);
                            collateral_return_key_encoding = Some(key_enc);
                            orig_deser_order.push(13);
                        },
                        (17, key_enc) =>  {
                            if total_collateral.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(17)).into());
                            }
                            let (tmp_total_collateral, tmp_total_collateral_encoding) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                raw.unsigned_integer_sz().map(|(x, enc)| (x, Some(enc))).map_err(Into::<DeserializeError>::into)
                            })().map_err(|e| e.annotate("total_collateral"))?;
                            total_collateral = Some(tmp_total_collateral);
                            total_collateral_encoding = tmp_total_collateral_encoding;
                            total_collateral_key_encoding = Some(key_enc);
                            orig_deser_order.push(14);
                        },
                        (18, key_enc) =>  {
                            if reference_inputs.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(18)).into());
                            }
                            let (tmp_reference_inputs, tmp_reference_inputs_encoding) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                let mut reference_inputs_arr = Vec::new();
                                let len = raw.array_sz()?;
                                let reference_inputs_encoding = len.into();
                                while match len { cbor_event::LenSz::Len(n, _) => (reference_inputs_arr.len() as u64) < n, cbor_event::LenSz::Indefinite => true, } {
                                    if raw.cbor_type()? == cbor_event::Type::Special {
                                        assert_eq!(raw.special()?, cbor_event::Special::Break);
                                        break;
                                    }
                                    reference_inputs_arr.push(TransactionInput::deserialize(raw)?);
                                }
                                Ok((reference_inputs_arr, reference_inputs_encoding))
                            })().map_err(|e| e.annotate("reference_inputs"))?;
                            reference_inputs = Some(tmp_reference_inputs);
                            reference_inputs_encoding = tmp_reference_inputs_encoding;
                            reference_inputs_key_encoding = Some(key_enc);
                            orig_deser_order.push(15);
                        },
                        (19, key_enc) =>  {
                            if voting_procedures.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(19)).into());
                            }
                            let (tmp_voting_procedures, tmp_voting_procedures_encoding, tmp_voting_procedures_value_encodings) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                let mut voting_procedures_table = OrderedHashMap::new();
                                let voting_procedures_len = raw.map_sz()?;
                                let voting_procedures_encoding = voting_procedures_len.into();
                                let mut voting_procedures_value_encodings = BTreeMap::new();
                                while match voting_procedures_len { cbor_event::LenSz::Len(n, _) => (voting_procedures_table.len() as u64) < n, cbor_event::LenSz::Indefinite => true, } {
                                    if raw.cbor_type()? == cbor_event::Type::Special {
                                        assert_eq!(raw.special()?, cbor_event::Special::Break);
                                        break;
                                    }
                                    let voting_procedures_key = Voter::deserialize(raw)?;
                                    let mut voting_procedures_value_table = OrderedHashMap::new();
                                    let voting_procedures_value_len = raw.map_sz()?;
                                    let voting_procedures_value_encoding = voting_procedures_value_len.into();
                                    while match voting_procedures_value_len { cbor_event::LenSz::Len(n, _) => (voting_procedures_value_table.len() as u64) < n, cbor_event::LenSz::Indefinite => true, } {
                                        if raw.cbor_type()? == cbor_event::Type::Special {
                                            assert_eq!(raw.special()?, cbor_event::Special::Break);
                                            break;
                                        }
                                        let voting_procedures_value_key = GovActionId::deserialize(raw)?;
                                        let voting_procedures_value_value = VotingProcedure::deserialize(raw)?;
                                        if voting_procedures_value_table.insert(voting_procedures_value_key.clone(), voting_procedures_value_value).is_some() {
                                            return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from("some complicated/unsupported type"))).into());
                                        }
                                    }
                                    let (voting_procedures_value, voting_procedures_value_encoding) = (voting_procedures_value_table, voting_procedures_value_encoding);
                                    if voting_procedures_table.insert(voting_procedures_key.clone(), voting_procedures_value).is_some() {
                                        return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from("some complicated/unsupported type"))).into());
                                    }
                                    voting_procedures_value_encodings.insert(voting_procedures_key, voting_procedures_value_encoding);
                                }
                                Ok((voting_procedures_table, voting_procedures_encoding, voting_procedures_value_encodings))
                            })().map_err(|e| e.annotate("voting_procedures"))?;
                            voting_procedures = Some(tmp_voting_procedures);
                            voting_procedures_encoding = tmp_voting_procedures_encoding;
                            voting_procedures_value_encodings = tmp_voting_procedures_value_encodings;
                            voting_procedures_key_encoding = Some(key_enc);
                            orig_deser_order.push(16);
                        },
                        (20, key_enc) =>  {
                            if proposal_procedures.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(20)).into());
                            }
                            let (tmp_proposal_procedures, tmp_proposal_procedures_encoding) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                let mut proposal_procedures_arr = Vec::new();
                                let len = raw.array_sz()?;
                                let proposal_procedures_encoding = len.into();
                                while match len { cbor_event::LenSz::Len(n, _) => (proposal_procedures_arr.len() as u64) < n, cbor_event::LenSz::Indefinite => true, } {
                                    if raw.cbor_type()? == cbor_event::Type::Special {
                                        assert_eq!(raw.special()?, cbor_event::Special::Break);
                                        break;
                                    }
                                    proposal_procedures_arr.push(ProposalProcedure::deserialize(raw)?);
                                }
                                Ok((proposal_procedures_arr, proposal_procedures_encoding))
                            })().map_err(|e| e.annotate("proposal_procedures"))?;
                            proposal_procedures = Some(tmp_proposal_procedures);
                            proposal_procedures_encoding = tmp_proposal_procedures_encoding;
                            proposal_procedures_key_encoding = Some(key_enc);
                            orig_deser_order.push(17);
                        },
                        (21, key_enc) =>  {
                            if current_treasury_value.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(21)).into());
                            }
                            let (tmp_current_treasury_value, tmp_current_treasury_value_encoding) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                raw.unsigned_integer_sz().map(|(x, enc)| (x, Some(enc))).map_err(Into::<DeserializeError>::into)
                            })().map_err(|e| e.annotate("current_treasury_value"))?;
                            current_treasury_value = Some(tmp_current_treasury_value);
                            current_treasury_value_encoding = tmp_current_treasury_value_encoding;
                            current_treasury_value_key_encoding = Some(key_enc);
                            orig_deser_order.push(18);
                        },
                        (22, key_enc) =>  {
                            if donation.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(22)).into());
                            }
                            let (tmp_donation, tmp_donation_encoding) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                raw.unsigned_integer_sz().map(|(x, enc)| (x, Some(enc))).map_err(Into::<DeserializeError>::into)
                            })().map_err(|e| e.annotate("donation"))?;
                            donation = Some(tmp_donation);
                            donation_encoding = tmp_donation_encoding;
                            donation_key_encoding = Some(key_enc);
                            orig_deser_order.push(19);
                        },
                        (unknown_key, _enc) => return Err(DeserializeFailure::UnknownKey(Key::Uint(unknown_key)).into()),
                    },
                    cbor_event::Type::Text => return Err(DeserializeFailure::UnknownKey(Key::Str(raw.text()?)).into()),
                    cbor_event::Type::Special => match len {
                        cbor_event::LenSz::Len(_, _) => return Err(DeserializeFailure::BreakInDefiniteLen.into()),
                        cbor_event::LenSz::Indefinite => match raw.special()? {
                            cbor_event::Special::Break => break,
                            _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                        },
                    },
                    other_type => return Err(DeserializeFailure::UnexpectedKeyType(other_type).into()),
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
            read_len.finish()?;
            Ok(Self {
                inputs,
                outputs,
                fee,
                ttl,
                certs,
                withdrawals,
                auxiliary_data_hash,
                validity_interval_start,
                // Manual edit: convert since wrapped in AssetBundle API
                mint: mint.map(Into::into),
                script_data_hash,
                collateral_inputs,
                required_signers,
                network_id,
                collateral_return,
                total_collateral,
                reference_inputs,
                voting_procedures,
                proposal_procedures,
                current_treasury_value,
                donation,
                encodings: Some(TransactionBodyEncoding {
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
                    auxiliary_data_hash_key_encoding,
                    auxiliary_data_hash_encoding,
                    validity_interval_start_key_encoding,
                    validity_interval_start_encoding,
                    mint_key_encoding,
                    mint_encoding,
                    mint_key_encodings,
                    mint_value_encodings,
                    script_data_hash_key_encoding,
                    script_data_hash_encoding,
                    collateral_inputs_key_encoding,
                    collateral_inputs_encoding,
                    required_signers_key_encoding,
                    required_signers_encoding,
                    required_signers_elem_encodings,
                    network_id_key_encoding,
                    collateral_return_key_encoding,
                    total_collateral_key_encoding,
                    total_collateral_encoding,
                    reference_inputs_key_encoding,
                    reference_inputs_encoding,
                    voting_procedures_key_encoding,
                    voting_procedures_encoding,
                    voting_procedures_value_encodings,
                    proposal_procedures_key_encoding,
                    proposal_procedures_encoding,
                    current_treasury_value_key_encoding,
                    current_treasury_value_encoding,
                    donation_key_encoding,
                    donation_encoding,
                }),
            })
        })().map_err(|e| e.annotate("TransactionBody"))
    }
}

impl Serialize for TransactionInput {
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
            self.transaction_id.to_raw_bytes(),
            self.encodings
                .as_ref()
                .map(|encs| encs.transaction_id_encoding.clone())
                .unwrap_or_default()
                .to_str_len_sz(
                    self.transaction_id.to_raw_bytes().len() as u64,
                    force_canonical,
                ),
        )?;
        serializer.write_unsigned_integer_sz(
            self.index,
            fit_sz(
                self.index,
                self.encodings
                    .as_ref()
                    .map(|encs| encs.index_encoding)
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

impl Deserialize for TransactionInput {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let len_encoding: LenEncoding = len.into();
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let (transaction_id, transaction_id_encoding) = raw
                .bytes_sz()
                .map_err(Into::<DeserializeError>::into)
                .and_then(|(bytes, enc)| {
                    TransactionHash::from_raw_bytes(&bytes)
                        .map(|bytes| (bytes, StringEncoding::from(enc)))
                        .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
                })
                .map_err(|e: DeserializeError| e.annotate("transaction_id"))?;
            let (index, index_encoding) = raw
                .unsigned_integer_sz()
                .map(|(x, enc)| (x, Some(enc)))
                .map_err(Into::<DeserializeError>::into)
                .map_err(|e: DeserializeError| e.annotate("index"))?;
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(TransactionInput {
                transaction_id,
                index,
                encodings: Some(TransactionInputEncoding {
                    len_encoding,
                    transaction_id_encoding,
                    index_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("TransactionInput"))
    }
}

impl Serialize for TransactionOutput {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            TransactionOutput::AlonzoFormatTxOut(alonzo_format_tx_out) => {
                alonzo_format_tx_out.serialize(serializer, force_canonical)
            }
            TransactionOutput::ConwayFormatTxOut(conway_format_tx_out) => {
                conway_format_tx_out.serialize(serializer, force_canonical)
            }
        }
    }
}

impl Deserialize for TransactionOutput {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            match raw.cbor_type()? {
                cbor_event::Type::Array => Ok(TransactionOutput::AlonzoFormatTxOut(
                    AlonzoFormatTxOut::deserialize(raw)?,
                )),
                cbor_event::Type::Map => Ok(TransactionOutput::ConwayFormatTxOut(
                    ConwayFormatTxOut::deserialize(raw)?,
                )),
                _ => Err(DeserializeError::new(
                    "TransactionOutput",
                    DeserializeFailure::NoVariantMatched,
                )),
            }
        })()
        .map_err(|e| e.annotate("TransactionOutput"))
    }
}

impl Serialize for TransactionWitnessSet {
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
                    } + match &self.plutus_v1_scripts {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.plutus_datums {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.redeemers {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.plutus_v2_scripts {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.plutus_v3_scripts {
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
                        } + match &self.plutus_v1_scripts {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.plutus_datums {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.redeemers {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.plutus_v2_scripts {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.plutus_v3_scripts {
                            Some(_) => 1,
                            None => 0,
                        }
            })
            .map(|encs| encs.orig_deser_order.clone())
            .unwrap_or_else(|| vec![0, 1, 2, 3, 4, 5, 6, 7]);
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
                3 => {
                    if let Some(field) = &self.plutus_v1_scripts {
                        serializer.write_unsigned_integer_sz(
                            3u64,
                            fit_sz(
                                3u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.plutus_v1_scripts_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        serializer.write_array_sz(
                            self.encodings
                                .as_ref()
                                .map(|encs| encs.plutus_v1_scripts_encoding)
                                .unwrap_or_default()
                                .to_len_sz(field.len() as u64, force_canonical),
                        )?;
                        for element in field.iter() {
                            element.serialize(serializer, force_canonical)?;
                        }
                        self.encodings
                            .as_ref()
                            .map(|encs| encs.plutus_v1_scripts_encoding)
                            .unwrap_or_default()
                            .end(serializer, force_canonical)?;
                    }
                }
                4 => {
                    if let Some(field) = &self.plutus_datums {
                        serializer.write_unsigned_integer_sz(
                            4u64,
                            fit_sz(
                                4u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.plutus_datums_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        serializer.write_array_sz(
                            self.encodings
                                .as_ref()
                                .map(|encs| encs.plutus_datums_encoding)
                                .unwrap_or_default()
                                .to_len_sz(field.len() as u64, force_canonical),
                        )?;
                        for element in field.iter() {
                            element.serialize(serializer, force_canonical)?;
                        }
                        self.encodings
                            .as_ref()
                            .map(|encs| encs.plutus_datums_encoding)
                            .unwrap_or_default()
                            .end(serializer, force_canonical)?;
                    }
                }
                5 => {
                    if let Some(field) = &self.redeemers {
                        serializer.write_unsigned_integer_sz(
                            5u64,
                            fit_sz(
                                5u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.redeemers_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        serializer.write_array_sz(
                            self.encodings
                                .as_ref()
                                .map(|encs| encs.redeemers_encoding)
                                .unwrap_or_default()
                                .to_len_sz(field.len() as u64, force_canonical),
                        )?;
                        for element in field.iter() {
                            element.serialize(serializer, force_canonical)?;
                        }
                        self.encodings
                            .as_ref()
                            .map(|encs| encs.redeemers_encoding)
                            .unwrap_or_default()
                            .end(serializer, force_canonical)?;
                    }
                }
                6 => {
                    if let Some(field) = &self.plutus_v2_scripts {
                        serializer.write_unsigned_integer_sz(
                            6u64,
                            fit_sz(
                                6u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.plutus_v2_scripts_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        serializer.write_array_sz(
                            self.encodings
                                .as_ref()
                                .map(|encs| encs.plutus_v2_scripts_encoding)
                                .unwrap_or_default()
                                .to_len_sz(field.len() as u64, force_canonical),
                        )?;
                        for element in field.iter() {
                            element.serialize(serializer, force_canonical)?;
                        }
                        self.encodings
                            .as_ref()
                            .map(|encs| encs.plutus_v2_scripts_encoding)
                            .unwrap_or_default()
                            .end(serializer, force_canonical)?;
                    }
                }
                7 => {
                    if let Some(field) = &self.plutus_v3_scripts {
                        serializer.write_unsigned_integer_sz(
                            7u64,
                            fit_sz(
                                7u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.plutus_v3_scripts_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        serializer.write_array_sz(
                            self.encodings
                                .as_ref()
                                .map(|encs| encs.plutus_v3_scripts_encoding)
                                .unwrap_or_default()
                                .to_len_sz(field.len() as u64, force_canonical),
                        )?;
                        for element in field.iter() {
                            element.serialize(serializer, force_canonical)?;
                        }
                        self.encodings
                            .as_ref()
                            .map(|encs| encs.plutus_v3_scripts_encoding)
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

impl Deserialize for TransactionWitnessSet {
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
            let mut plutus_v1_scripts_encoding = LenEncoding::default();
            let mut plutus_v1_scripts_key_encoding = None;
            let mut plutus_v1_scripts = None;
            let mut plutus_datums_encoding = LenEncoding::default();
            let mut plutus_datums_key_encoding = None;
            let mut plutus_datums = None;
            let mut redeemers_encoding = LenEncoding::default();
            let mut redeemers_key_encoding = None;
            let mut redeemers = None;
            let mut plutus_v2_scripts_encoding = LenEncoding::default();
            let mut plutus_v2_scripts_key_encoding = None;
            let mut plutus_v2_scripts = None;
            let mut plutus_v3_scripts_encoding = LenEncoding::default();
            let mut plutus_v3_scripts_key_encoding = None;
            let mut plutus_v3_scripts = None;
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
                                        native_scripts_arr.push(NativeScript::deserialize(raw)?);
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
                        (3, key_enc) => {
                            if plutus_v1_scripts.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(3)).into());
                            }
                            let (tmp_plutus_v1_scripts, tmp_plutus_v1_scripts_encoding) =
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    let mut plutus_v1_scripts_arr = Vec::new();
                                    let len = raw.array_sz()?;
                                    let plutus_v1_scripts_encoding = len.into();
                                    while match len {
                                        cbor_event::LenSz::Len(n, _) => {
                                            (plutus_v1_scripts_arr.len() as u64) < n
                                        }
                                        cbor_event::LenSz::Indefinite => true,
                                    } {
                                        if raw.cbor_type()? == cbor_event::Type::Special {
                                            assert_eq!(raw.special()?, cbor_event::Special::Break);
                                            break;
                                        }
                                        plutus_v1_scripts_arr
                                            .push(PlutusV1Script::deserialize(raw)?);
                                    }
                                    Ok((plutus_v1_scripts_arr, plutus_v1_scripts_encoding))
                                })()
                                .map_err(|e| e.annotate("plutus_v1_scripts"))?;
                            plutus_v1_scripts = Some(tmp_plutus_v1_scripts);
                            plutus_v1_scripts_encoding = tmp_plutus_v1_scripts_encoding;
                            plutus_v1_scripts_key_encoding = Some(key_enc);
                            orig_deser_order.push(3);
                        }
                        (4, key_enc) => {
                            if plutus_datums.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(4)).into());
                            }
                            let (tmp_plutus_datums, tmp_plutus_datums_encoding) =
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    let mut plutus_datums_arr = Vec::new();
                                    let len = raw.array_sz()?;
                                    let plutus_datums_encoding = len.into();
                                    while match len {
                                        cbor_event::LenSz::Len(n, _) => {
                                            (plutus_datums_arr.len() as u64) < n
                                        }
                                        cbor_event::LenSz::Indefinite => true,
                                    } {
                                        if raw.cbor_type()? == cbor_event::Type::Special {
                                            assert_eq!(raw.special()?, cbor_event::Special::Break);
                                            break;
                                        }
                                        plutus_datums_arr.push(PlutusData::deserialize(raw)?);
                                    }
                                    Ok((plutus_datums_arr, plutus_datums_encoding))
                                })()
                                .map_err(|e| e.annotate("plutus_datums"))?;
                            plutus_datums = Some(tmp_plutus_datums);
                            plutus_datums_encoding = tmp_plutus_datums_encoding;
                            plutus_datums_key_encoding = Some(key_enc);
                            orig_deser_order.push(4);
                        }
                        (5, key_enc) => {
                            if redeemers.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(5)).into());
                            }
                            let (tmp_redeemers, tmp_redeemers_encoding) =
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    let mut redeemers_arr = Vec::new();
                                    let len = raw.array_sz()?;
                                    let redeemers_encoding = len.into();
                                    while match len {
                                        cbor_event::LenSz::Len(n, _) => {
                                            (redeemers_arr.len() as u64) < n
                                        }
                                        cbor_event::LenSz::Indefinite => true,
                                    } {
                                        if raw.cbor_type()? == cbor_event::Type::Special {
                                            assert_eq!(raw.special()?, cbor_event::Special::Break);
                                            break;
                                        }
                                        redeemers_arr.push(Redeemer::deserialize(raw)?);
                                    }
                                    Ok((redeemers_arr, redeemers_encoding))
                                })()
                                .map_err(|e| e.annotate("redeemers"))?;
                            redeemers = Some(tmp_redeemers);
                            redeemers_encoding = tmp_redeemers_encoding;
                            redeemers_key_encoding = Some(key_enc);
                            orig_deser_order.push(5);
                        }
                        (6, key_enc) => {
                            if plutus_v2_scripts.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(6)).into());
                            }
                            let (tmp_plutus_v2_scripts, tmp_plutus_v2_scripts_encoding) =
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    let mut plutus_v2_scripts_arr = Vec::new();
                                    let len = raw.array_sz()?;
                                    let plutus_v2_scripts_encoding = len.into();
                                    while match len {
                                        cbor_event::LenSz::Len(n, _) => {
                                            (plutus_v2_scripts_arr.len() as u64) < n
                                        }
                                        cbor_event::LenSz::Indefinite => true,
                                    } {
                                        if raw.cbor_type()? == cbor_event::Type::Special {
                                            assert_eq!(raw.special()?, cbor_event::Special::Break);
                                            break;
                                        }
                                        plutus_v2_scripts_arr
                                            .push(PlutusV2Script::deserialize(raw)?);
                                    }
                                    Ok((plutus_v2_scripts_arr, plutus_v2_scripts_encoding))
                                })()
                                .map_err(|e| e.annotate("plutus_v2_scripts"))?;
                            plutus_v2_scripts = Some(tmp_plutus_v2_scripts);
                            plutus_v2_scripts_encoding = tmp_plutus_v2_scripts_encoding;
                            plutus_v2_scripts_key_encoding = Some(key_enc);
                            orig_deser_order.push(6);
                        }
                        (7, key_enc) => {
                            if plutus_v3_scripts.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(7)).into());
                            }
                            let (tmp_plutus_v3_scripts, tmp_plutus_v3_scripts_encoding) =
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    let mut plutus_v3_scripts_arr = Vec::new();
                                    let len = raw.array_sz()?;
                                    let plutus_v3_scripts_encoding = len.into();
                                    while match len {
                                        cbor_event::LenSz::Len(n, _) => {
                                            (plutus_v3_scripts_arr.len() as u64) < n
                                        }
                                        cbor_event::LenSz::Indefinite => true,
                                    } {
                                        if raw.cbor_type()? == cbor_event::Type::Special {
                                            assert_eq!(raw.special()?, cbor_event::Special::Break);
                                            break;
                                        }
                                        plutus_v3_scripts_arr
                                            .push(PlutusV3Script::deserialize(raw)?);
                                    }
                                    Ok((plutus_v3_scripts_arr, plutus_v3_scripts_encoding))
                                })()
                                .map_err(|e| e.annotate("plutus_v3_scripts"))?;
                            plutus_v3_scripts = Some(tmp_plutus_v3_scripts);
                            plutus_v3_scripts_encoding = tmp_plutus_v3_scripts_encoding;
                            plutus_v3_scripts_key_encoding = Some(key_enc);
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
            read_len.finish()?;
            Ok(Self {
                vkeywitnesses,
                native_scripts,
                bootstrap_witnesses,
                plutus_v1_scripts,
                plutus_datums,
                redeemers,
                plutus_v2_scripts,
                plutus_v3_scripts,
                encodings: Some(TransactionWitnessSetEncoding {
                    len_encoding,
                    orig_deser_order,
                    vkeywitnesses_key_encoding,
                    vkeywitnesses_encoding,
                    native_scripts_key_encoding,
                    native_scripts_encoding,
                    bootstrap_witnesses_key_encoding,
                    bootstrap_witnesses_encoding,
                    plutus_v1_scripts_key_encoding,
                    plutus_v1_scripts_encoding,
                    plutus_datums_key_encoding,
                    plutus_datums_encoding,
                    redeemers_key_encoding,
                    redeemers_encoding,
                    plutus_v2_scripts_key_encoding,
                    plutus_v2_scripts_encoding,
                    plutus_v3_scripts_key_encoding,
                    plutus_v3_scripts_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("TransactionWitnessSet"))
    }
}
