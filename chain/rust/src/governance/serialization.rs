// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use crate::certs::Credential;

use super::cbor_encodings::*;
use super::*;
use cbor_event::de::Deserializer;
use cbor_event::se::Serializer;
use cml_core::error::*;
use cml_core::serialization::*;
use cml_crypto::AnchorDocHash;
use cml_crypto::RawBytesEncoding;
use std::io::{BufRead, Seek, SeekFrom, Write};

impl Serialize for Anchor {
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
        self.anchor_url.serialize(serializer, force_canonical)?;
        serializer.write_bytes_sz(
            self.anchor_doc_hash.to_raw_bytes(),
            self.encodings
                .as_ref()
                .map(|encs| encs.anchor_doc_hash_encoding.clone())
                .unwrap_or_default()
                .to_str_len_sz(
                    self.anchor_doc_hash.to_raw_bytes().len() as u64,
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

impl Deserialize for Anchor {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let len_encoding: LenEncoding = len.into();
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let anchor_url =
                Url::deserialize(raw).map_err(|e: DeserializeError| e.annotate("anchor_url"))?;
            let (anchor_doc_hash, anchor_doc_hash_encoding) = raw
                .bytes_sz()
                .map_err(Into::<DeserializeError>::into)
                .and_then(|(bytes, enc)| {
                    AnchorDocHash::from_raw_bytes(&bytes)
                        .map(|bytes| (bytes, StringEncoding::from(enc)))
                        .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
                })
                .map_err(|e: DeserializeError| e.annotate("anchor_doc_hash"))?;
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(Anchor {
                anchor_url,
                anchor_doc_hash,
                encodings: Some(AnchorEncoding {
                    len_encoding,
                    anchor_doc_hash_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("Anchor"))
    }
}

impl Serialize for Constitution {
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
        self.anchor.serialize(serializer, force_canonical)?;
        match &self.script_hash {
            Some(x) => serializer.write_bytes_sz(
                x.to_raw_bytes(),
                self.encodings
                    .as_ref()
                    .map(|encs| encs.script_hash_encoding.clone())
                    .unwrap_or_default()
                    .to_str_len_sz(x.to_raw_bytes().len() as u64, force_canonical),
            ),
            None => serializer.write_special(cbor_event::Special::Null),
        }?;
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for Constitution {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let len_encoding: LenEncoding = len.into();
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let anchor =
                Anchor::deserialize(raw).map_err(|e: DeserializeError| e.annotate("anchor"))?;
            let (script_hash, script_hash_encoding) = (|| -> Result<_, DeserializeError> {
                Ok(match raw.cbor_type()? != cbor_event::Type::Special {
                    true => Result::<_, DeserializeError>::Ok(
                        raw.bytes_sz()
                            .map_err(Into::<DeserializeError>::into)
                            .and_then(|(bytes, enc)| {
                                ScriptHash::from_raw_bytes(&bytes)
                                    .map(|bytes| (bytes, StringEncoding::from(enc)))
                                    .map_err(|e| {
                                        DeserializeFailure::InvalidStructure(Box::new(e)).into()
                                    })
                            })?,
                    )
                    .map(|(x, script_hash_encoding)| (Some(x), script_hash_encoding))?,
                    false => {
                        if raw.special()? != cbor_event::Special::Null {
                            return Err(DeserializeFailure::ExpectedNull.into());
                        }
                        (None, StringEncoding::default())
                    }
                })
            })()
            .map_err(|e| e.annotate("script_hash"))?;
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(Constitution {
                anchor,
                script_hash,
                encodings: Some(ConstitutionEncoding {
                    len_encoding,
                    script_hash_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("Constitution"))
    }
}

impl Serialize for GovAction {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            GovAction::ParameterChangeAction(parameter_change_action) => {
                parameter_change_action.serialize(serializer, force_canonical)
            }
            GovAction::HardForkInitiationAction(hard_fork_initiation_action) => {
                hard_fork_initiation_action.serialize(serializer, force_canonical)
            }
            GovAction::TreasuryWithdrawalsAction(treasury_withdrawals_action) => {
                treasury_withdrawals_action.serialize(serializer, force_canonical)
            }
            GovAction::NoConfidence(no_confidence) => {
                no_confidence.serialize(serializer, force_canonical)
            }
            GovAction::UpdateCommittee(update_committee) => {
                update_committee.serialize(serializer, force_canonical)
            }
            GovAction::NewConstitution(new_constitution) => {
                new_constitution.serialize(serializer, force_canonical)
            }
            GovAction::InfoAction {
                info_action_encoding,
                len_encoding,
            } => {
                serializer.write_array_sz(len_encoding.to_len_sz(1, force_canonical))?;
                serializer.write_unsigned_integer_sz(
                    6u64,
                    fit_sz(6u64, *info_action_encoding, force_canonical),
                )?;
                len_encoding.end(serializer, force_canonical)?;
                Ok(serializer)
            }
        }
    }
}

impl Deserialize for GovAction {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array_sz()?;
            let len_encoding: LenEncoding = len.into();
            let initial_position = raw.as_mut_ref().stream_position().unwrap();
            let mut errs = Vec::new();
            let deser_variant = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let mut read_len = CBORReadLen::new(len);
                read_len.read_elems(4)?;
                read_len.finish()?;
                let ret =
                    ParameterChangeAction::deserialize_as_embedded_group(raw, &mut read_len, len);
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
                Ok(parameter_change_action) => {
                    return Ok(Self::ParameterChangeAction(parameter_change_action))
                }
                Err(e) => {
                    errs.push(e.annotate("ParameterChangeAction"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let mut read_len = CBORReadLen::new(len);
                read_len.read_elems(3)?;
                read_len.finish()?;
                let ret = HardForkInitiationAction::deserialize_as_embedded_group(
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
                Ok(hard_fork_initiation_action) => {
                    return Ok(Self::HardForkInitiationAction(hard_fork_initiation_action))
                }
                Err(e) => {
                    errs.push(e.annotate("HardForkInitiationAction"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let mut read_len = CBORReadLen::new(len);
                read_len.read_elems(3)?;
                read_len.finish()?;
                let ret = TreasuryWithdrawalsAction::deserialize_as_embedded_group(
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
                Ok(treasury_withdrawals_action) => {
                    return Ok(Self::TreasuryWithdrawalsAction(treasury_withdrawals_action))
                }
                Err(e) => {
                    errs.push(e.annotate("TreasuryWithdrawalsAction"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let mut read_len = CBORReadLen::new(len);
                read_len.read_elems(2)?;
                read_len.finish()?;
                let ret = NoConfidence::deserialize_as_embedded_group(raw, &mut read_len, len);
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
                Ok(no_confidence) => return Ok(Self::NoConfidence(no_confidence)),
                Err(e) => {
                    errs.push(e.annotate("NoConfidence"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let mut read_len = CBORReadLen::new(len);
                read_len.read_elems(5)?;
                read_len.finish()?;
                let ret = UpdateCommittee::deserialize_as_embedded_group(raw, &mut read_len, len);
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
                Ok(update_committee) => return Ok(Self::UpdateCommittee(update_committee)),
                Err(e) => {
                    errs.push(e.annotate("UpdateCommittee"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let mut read_len = CBORReadLen::new(len);
                read_len.read_elems(3)?;
                read_len.finish()?;
                let ret = NewConstitution::deserialize_as_embedded_group(raw, &mut read_len, len);
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
                Ok(new_constitution) => return Ok(Self::NewConstitution(new_constitution)),
                Err(e) => {
                    errs.push(e.annotate("NewConstitution"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let mut read_len = CBORReadLen::new(len);
                read_len.read_elems(1)?;
                read_len.finish()?;
                let (info_action_value, info_action_encoding) = raw.unsigned_integer_sz()?;
                if info_action_value != 6 {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(info_action_value),
                        expected: Key::Uint(6),
                    }
                    .into());
                }
                let ret = Ok(Some(info_action_encoding));
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
                Ok(info_action_encoding) => {
                    return Ok(Self::InfoAction {
                        info_action_encoding,
                        len_encoding,
                    })
                }
                Err(e) => {
                    errs.push(e.annotate("InfoAction"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            Err(DeserializeError::new(
                "GovAction",
                DeserializeFailure::NoVariantMatchedWithCauses(errs),
            ))
        })()
        .map_err(|e| e.annotate("GovAction"))
    }
}

impl Serialize for GovActionId {
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
            self.gov_action_index,
            fit_sz(
                self.gov_action_index,
                self.encodings
                    .as_ref()
                    .map(|encs| encs.gov_action_index_encoding)
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

impl Deserialize for GovActionId {
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
            let (gov_action_index, gov_action_index_encoding) = raw
                .unsigned_integer_sz()
                .map_err(Into::<DeserializeError>::into)
                .map(|(x, enc)| (x, Some(enc)))
                .map_err(|e: DeserializeError| e.annotate("gov_action_index"))?;
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(GovActionId {
                transaction_id,
                gov_action_index,
                encodings: Some(GovActionIdEncoding {
                    len_encoding,
                    transaction_id_encoding,
                    gov_action_index_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("GovActionId"))
    }
}

impl Serialize for HardForkInitiationAction {
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

impl SerializeEmbeddedGroup for HardForkInitiationAction {
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
        match &self.action_id {
            Some(x) => x.serialize(serializer, force_canonical),
            None => serializer.write_special(cbor_event::Special::Null),
        }?;
        self.version.serialize(serializer, force_canonical)?;
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for HardForkInitiationAction {
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

impl DeserializeEmbeddedGroup for HardForkInitiationAction {
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
            let action_id = (|| -> Result<_, DeserializeError> {
                Ok(match raw.cbor_type()? != cbor_event::Type::Special {
                    true => Some(GovActionId::deserialize(raw)?),
                    false => {
                        if raw.special()? != cbor_event::Special::Null {
                            return Err(DeserializeFailure::ExpectedNull.into());
                        }
                        None
                    }
                })
            })()
            .map_err(|e| e.annotate("action_id"))?;
            let version = ProtocolVersion::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("version"))?;
            Ok(HardForkInitiationAction {
                action_id,
                version,
                encodings: Some(HardForkInitiationActionEncoding {
                    len_encoding,
                    tag_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("HardForkInitiationAction"))
    }
}

impl Serialize for NewConstitution {
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

impl SerializeEmbeddedGroup for NewConstitution {
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
        match &self.action_id {
            Some(x) => x.serialize(serializer, force_canonical),
            None => serializer.write_special(cbor_event::Special::Null),
        }?;
        self.constitution.serialize(serializer, force_canonical)?;
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for NewConstitution {
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

impl DeserializeEmbeddedGroup for NewConstitution {
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
            let action_id = (|| -> Result<_, DeserializeError> {
                Ok(match raw.cbor_type()? != cbor_event::Type::Special {
                    true => Some(GovActionId::deserialize(raw)?),
                    false => {
                        if raw.special()? != cbor_event::Special::Null {
                            return Err(DeserializeFailure::ExpectedNull.into());
                        }
                        None
                    }
                })
            })()
            .map_err(|e| e.annotate("action_id"))?;
            let constitution = Constitution::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("constitution"))?;
            Ok(NewConstitution {
                action_id,
                constitution,
                encodings: Some(NewConstitutionEncoding {
                    len_encoding,
                    tag_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("NewConstitution"))
    }
}

impl Serialize for NoConfidence {
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

impl SerializeEmbeddedGroup for NoConfidence {
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
        match &self.action_id {
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

impl Deserialize for NoConfidence {
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

impl DeserializeEmbeddedGroup for NoConfidence {
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
            let action_id = (|| -> Result<_, DeserializeError> {
                Ok(match raw.cbor_type()? != cbor_event::Type::Special {
                    true => Some(GovActionId::deserialize(raw)?),
                    false => {
                        if raw.special()? != cbor_event::Special::Null {
                            return Err(DeserializeFailure::ExpectedNull.into());
                        }
                        None
                    }
                })
            })()
            .map_err(|e| e.annotate("action_id"))?;
            Ok(NoConfidence {
                action_id,
                encodings: Some(NoConfidenceEncoding {
                    len_encoding,
                    tag_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("NoConfidence"))
    }
}

impl Serialize for ParameterChangeAction {
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

impl SerializeEmbeddedGroup for ParameterChangeAction {
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
        match &self.action_id {
            Some(x) => x.serialize(serializer, force_canonical),
            None => serializer.write_special(cbor_event::Special::Null),
        }?;
        self.update.serialize(serializer, force_canonical)?;
        match &self.policy_hash {
            Some(x) => serializer.write_bytes_sz(
                x.to_raw_bytes(),
                self.encodings
                    .as_ref()
                    .map(|encs| encs.policy_hash_encoding.clone())
                    .unwrap_or_default()
                    .to_str_len_sz(x.to_raw_bytes().len() as u64, force_canonical),
            ),
            None => serializer.write_special(cbor_event::Special::Null),
        }?;
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for ParameterChangeAction {
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

impl DeserializeEmbeddedGroup for ParameterChangeAction {
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
            let action_id = (|| -> Result<_, DeserializeError> {
                Ok(match raw.cbor_type()? != cbor_event::Type::Special {
                    true => Some(GovActionId::deserialize(raw)?),
                    false => {
                        if raw.special()? != cbor_event::Special::Null {
                            return Err(DeserializeFailure::ExpectedNull.into());
                        }
                        None
                    }
                })
            })()
            .map_err(|e| e.annotate("action_id"))?;
            let update = ProtocolParamUpdate::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("update"))?;
            let (policy_hash, policy_hash_encoding) = (|| -> Result<_, DeserializeError> {
                Ok(match raw.cbor_type()? != cbor_event::Type::Special {
                    true => Result::<_, DeserializeError>::Ok(
                        raw.bytes_sz()
                            .map_err(Into::<DeserializeError>::into)
                            .and_then(|(bytes, enc)| {
                                ScriptHash::from_raw_bytes(&bytes)
                                    .map(|bytes| (bytes, StringEncoding::from(enc)))
                                    .map_err(|e| {
                                        DeserializeFailure::InvalidStructure(Box::new(e)).into()
                                    })
                            })?,
                    )
                    .map(|(x, policy_hash_encoding)| (Some(x), policy_hash_encoding))?,
                    false => {
                        if raw.special()? != cbor_event::Special::Null {
                            return Err(DeserializeFailure::ExpectedNull.into());
                        }
                        (None, StringEncoding::default())
                    }
                })
            })()
            .map_err(|e| e.annotate("policy_hash"))?;
            Ok(ParameterChangeAction {
                action_id,
                update,
                policy_hash,
                encodings: Some(ParameterChangeActionEncoding {
                    len_encoding,
                    tag_encoding,
                    policy_hash_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("ParameterChangeAction"))
    }
}

impl Serialize for ProposalProcedure {
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
        serializer.write_unsigned_integer_sz(
            self.deposit,
            fit_sz(
                self.deposit,
                self.encodings
                    .as_ref()
                    .map(|encs| encs.deposit_encoding)
                    .unwrap_or_default(),
                force_canonical,
            ),
        )?;
        self.reward_account.serialize(serializer, force_canonical)?;
        self.gov_action.serialize(serializer, force_canonical)?;
        self.anchor.serialize(serializer, force_canonical)?;
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for ProposalProcedure {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let len_encoding: LenEncoding = len.into();
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(4)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let (deposit, deposit_encoding) = raw
                .unsigned_integer_sz()
                .map_err(Into::<DeserializeError>::into)
                .map(|(x, enc)| (x, Some(enc)))
                .map_err(|e: DeserializeError| e.annotate("deposit"))?;
            let reward_account = RewardAccount::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("reward_account"))?;
            let gov_action = GovAction::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("gov_action"))?;
            let anchor =
                Anchor::deserialize(raw).map_err(|e: DeserializeError| e.annotate("anchor"))?;
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(ProposalProcedure {
                deposit,
                reward_account,
                gov_action,
                anchor,
                encodings: Some(ProposalProcedureEncoding {
                    len_encoding,
                    deposit_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("ProposalProcedure"))
    }
}

impl Serialize for TreasuryWithdrawalsAction {
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

impl SerializeEmbeddedGroup for TreasuryWithdrawalsAction {
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
        serializer.write_map_sz(
            self.encodings
                .as_ref()
                .map(|encs| encs.withdrawal_encoding)
                .unwrap_or_default()
                .to_len_sz(self.withdrawal.len() as u64, force_canonical),
        )?;
        let mut key_order = self
            .withdrawal
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
            let withdrawal_value_encoding = self
                .encodings
                .as_ref()
                .and_then(|encs| encs.withdrawal_value_encodings.get(key))
                .cloned()
                .unwrap_or_default();
            serializer.write_unsigned_integer_sz(
                *value,
                fit_sz(*value, withdrawal_value_encoding, force_canonical),
            )?;
        }
        self.encodings
            .as_ref()
            .map(|encs| encs.withdrawal_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)?;
        match &self.policy_hash {
            Some(x) => serializer.write_bytes_sz(
                x.to_raw_bytes(),
                self.encodings
                    .as_ref()
                    .map(|encs| encs.policy_hash_encoding.clone())
                    .unwrap_or_default()
                    .to_str_len_sz(x.to_raw_bytes().len() as u64, force_canonical),
            ),
            None => serializer.write_special(cbor_event::Special::Null),
        }?;
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for TreasuryWithdrawalsAction {
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

impl DeserializeEmbeddedGroup for TreasuryWithdrawalsAction {
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
            let (withdrawal, withdrawal_encoding, withdrawal_value_encodings) =
                (|| -> Result<_, DeserializeError> {
                    let mut withdrawal_table = OrderedHashMap::new();
                    let withdrawal_len = raw.map_sz()?;
                    let withdrawal_encoding = withdrawal_len.into();
                    let mut withdrawal_value_encodings = BTreeMap::new();
                    while match withdrawal_len {
                        cbor_event::LenSz::Len(n, _) => (withdrawal_table.len() as u64) < n,
                        cbor_event::LenSz::Indefinite => true,
                    } {
                        if raw.cbor_type()? == cbor_event::Type::Special {
                            assert_eq!(raw.special()?, cbor_event::Special::Break);
                            break;
                        }
                        let withdrawal_key = RewardAccount::deserialize(raw)?;
                        let (withdrawal_value, withdrawal_value_encoding) =
                            raw.unsigned_integer_sz().map(|(x, enc)| (x, Some(enc)))?;
                        if withdrawal_table
                            .insert(withdrawal_key.clone(), withdrawal_value)
                            .is_some()
                        {
                            return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from(
                                "some complicated/unsupported type",
                            )))
                            .into());
                        }
                        withdrawal_value_encodings
                            .insert(withdrawal_key, withdrawal_value_encoding);
                    }
                    Ok((
                        withdrawal_table,
                        withdrawal_encoding,
                        withdrawal_value_encodings,
                    ))
                })()
                .map_err(|e| e.annotate("withdrawal"))?;
            let (policy_hash, policy_hash_encoding) = (|| -> Result<_, DeserializeError> {
                Ok(match raw.cbor_type()? != cbor_event::Type::Special {
                    true => Result::<_, DeserializeError>::Ok(
                        raw.bytes_sz()
                            .map_err(Into::<DeserializeError>::into)
                            .and_then(|(bytes, enc)| {
                                ScriptHash::from_raw_bytes(&bytes)
                                    .map(|bytes| (bytes, StringEncoding::from(enc)))
                                    .map_err(|e| {
                                        DeserializeFailure::InvalidStructure(Box::new(e)).into()
                                    })
                            })?,
                    )
                    .map(|(x, policy_hash_encoding)| (Some(x), policy_hash_encoding))?,
                    false => {
                        if raw.special()? != cbor_event::Special::Null {
                            return Err(DeserializeFailure::ExpectedNull.into());
                        }
                        (None, StringEncoding::default())
                    }
                })
            })()
            .map_err(|e| e.annotate("policy_hash"))?;
            Ok(TreasuryWithdrawalsAction {
                withdrawal,
                policy_hash,
                encodings: Some(TreasuryWithdrawalsActionEncoding {
                    len_encoding,
                    tag_encoding,
                    withdrawal_encoding,
                    withdrawal_value_encodings,
                    policy_hash_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("TreasuryWithdrawalsAction"))
    }
}

impl Serialize for UpdateCommittee {
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
                .to_len_sz(5, force_canonical),
        )?;
        self.serialize_as_embedded_group(serializer, force_canonical)
    }
}

impl SerializeEmbeddedGroup for UpdateCommittee {
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
        match &self.action_id {
            Some(x) => x.serialize(serializer, force_canonical),
            None => serializer.write_special(cbor_event::Special::Null),
        }?;
        self.cold_credentials
            .serialize(serializer, force_canonical)?;
        serializer.write_map_sz(
            self.encodings
                .as_ref()
                .map(|encs| encs.credentials_encoding)
                .unwrap_or_default()
                .to_len_sz(self.credentials.len() as u64, force_canonical),
        )?;
        let mut key_order = self
            .credentials
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
            let credentials_value_encoding = self
                .encodings
                .as_ref()
                .and_then(|encs| encs.credentials_value_encodings.get(key))
                .cloned()
                .unwrap_or_default();
            serializer.write_unsigned_integer_sz(
                *value,
                fit_sz(*value, credentials_value_encoding, force_canonical),
            )?;
        }
        self.encodings
            .as_ref()
            .map(|encs| encs.credentials_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)?;
        self.unit_interval.serialize(serializer, force_canonical)?;
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for UpdateCommittee {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(5)?;
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

impl DeserializeEmbeddedGroup for UpdateCommittee {
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
            let action_id = (|| -> Result<_, DeserializeError> {
                Ok(match raw.cbor_type()? != cbor_event::Type::Special {
                    true => Some(GovActionId::deserialize(raw)?),
                    false => {
                        if raw.special()? != cbor_event::Special::Null {
                            return Err(DeserializeFailure::ExpectedNull.into());
                        }
                        None
                    }
                })
            })()
            .map_err(|e| e.annotate("action_id"))?;
            let cold_credentials = SetCommitteeColdCredential::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("cold_credentials"))?;
            let (credentials, credentials_encoding, credentials_value_encodings) =
                (|| -> Result<_, DeserializeError> {
                    let mut credentials_table = OrderedHashMap::new();
                    let credentials_len = raw.map_sz()?;
                    let credentials_encoding = credentials_len.into();
                    let mut credentials_value_encodings = BTreeMap::new();
                    while match credentials_len {
                        cbor_event::LenSz::Len(n, _) => (credentials_table.len() as u64) < n,
                        cbor_event::LenSz::Indefinite => true,
                    } {
                        if raw.cbor_type()? == cbor_event::Type::Special {
                            assert_eq!(raw.special()?, cbor_event::Special::Break);
                            break;
                        }
                        let credentials_key = Credential::deserialize(raw)?;
                        let (credentials_value, credentials_value_encoding) =
                            raw.unsigned_integer_sz().map(|(x, enc)| (x, Some(enc)))?;
                        if credentials_table
                            .insert(credentials_key.clone(), credentials_value)
                            .is_some()
                        {
                            return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from(
                                "some complicated/unsupported type",
                            )))
                            .into());
                        }
                        credentials_value_encodings
                            .insert(credentials_key, credentials_value_encoding);
                    }
                    Ok((
                        credentials_table,
                        credentials_encoding,
                        credentials_value_encodings,
                    ))
                })()
                .map_err(|e| e.annotate("credentials"))?;
            let unit_interval = UnitInterval::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("unit_interval"))?;
            Ok(UpdateCommittee {
                action_id,
                cold_credentials,
                credentials,
                unit_interval,
                encodings: Some(UpdateCommitteeEncoding {
                    len_encoding,
                    tag_encoding,
                    credentials_encoding,
                    credentials_value_encodings,
                }),
            })
        })()
        .map_err(|e| e.annotate("UpdateCommittee"))
    }
}

impl Serialize for Voter {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            Voter::ConstitutionalCommitteeHotKeyHash {
                ed25519_key_hash,
                len_encoding,
                index_0_encoding,
                ed25519_key_hash_encoding,
            } => {
                serializer.write_array_sz(len_encoding.to_len_sz(2, force_canonical))?;
                serializer.write_unsigned_integer_sz(
                    0u64,
                    fit_sz(0u64, *index_0_encoding, force_canonical),
                )?;
                serializer.write_bytes_sz(
                    ed25519_key_hash.to_raw_bytes(),
                    ed25519_key_hash_encoding.to_str_len_sz(
                        ed25519_key_hash.to_raw_bytes().len() as u64,
                        force_canonical,
                    ),
                )?;
                len_encoding.end(serializer, force_canonical)?;
                Ok(serializer)
            }
            Voter::ConstitutionalCommitteeHotScriptHash {
                script_hash,
                len_encoding,
                index_0_encoding,
                script_hash_encoding,
            } => {
                serializer.write_array_sz(len_encoding.to_len_sz(2, force_canonical))?;
                serializer.write_unsigned_integer_sz(
                    1u64,
                    fit_sz(1u64, *index_0_encoding, force_canonical),
                )?;
                serializer.write_bytes_sz(
                    script_hash.to_raw_bytes(),
                    script_hash_encoding
                        .to_str_len_sz(script_hash.to_raw_bytes().len() as u64, force_canonical),
                )?;
                len_encoding.end(serializer, force_canonical)?;
                Ok(serializer)
            }
            Voter::DRepKeyHash {
                ed25519_key_hash,
                len_encoding,
                index_0_encoding,
                ed25519_key_hash_encoding,
            } => {
                serializer.write_array_sz(len_encoding.to_len_sz(2, force_canonical))?;
                serializer.write_unsigned_integer_sz(
                    2u64,
                    fit_sz(2u64, *index_0_encoding, force_canonical),
                )?;
                serializer.write_bytes_sz(
                    ed25519_key_hash.to_raw_bytes(),
                    ed25519_key_hash_encoding.to_str_len_sz(
                        ed25519_key_hash.to_raw_bytes().len() as u64,
                        force_canonical,
                    ),
                )?;
                len_encoding.end(serializer, force_canonical)?;
                Ok(serializer)
            }
            Voter::DRepScriptHash {
                script_hash,
                len_encoding,
                index_0_encoding,
                script_hash_encoding,
            } => {
                serializer.write_array_sz(len_encoding.to_len_sz(2, force_canonical))?;
                serializer.write_unsigned_integer_sz(
                    3u64,
                    fit_sz(3u64, *index_0_encoding, force_canonical),
                )?;
                serializer.write_bytes_sz(
                    script_hash.to_raw_bytes(),
                    script_hash_encoding
                        .to_str_len_sz(script_hash.to_raw_bytes().len() as u64, force_canonical),
                )?;
                len_encoding.end(serializer, force_canonical)?;
                Ok(serializer)
            }
            Voter::StakingPoolKeyHash {
                ed25519_key_hash,
                len_encoding,
                index_0_encoding,
                ed25519_key_hash_encoding,
            } => {
                serializer.write_array_sz(len_encoding.to_len_sz(2, force_canonical))?;
                serializer.write_unsigned_integer_sz(
                    4u64,
                    fit_sz(4u64, *index_0_encoding, force_canonical),
                )?;
                serializer.write_bytes_sz(
                    ed25519_key_hash.to_raw_bytes(),
                    ed25519_key_hash_encoding.to_str_len_sz(
                        ed25519_key_hash.to_raw_bytes().len() as u64,
                        force_canonical,
                    ),
                )?;
                len_encoding.end(serializer, force_canonical)?;
                Ok(serializer)
            }
        }
    }
}

impl Deserialize for Voter {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array_sz()?;
            let len_encoding: LenEncoding = len.into();
            let initial_position = raw.as_mut_ref().stream_position().unwrap();
            let mut errs = Vec::new();
            let variant_deser = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let mut read_len = CBORReadLen::new(len);
                read_len.read_elems(2)?;
                read_len.finish()?;
                let index_0_encoding = (|| -> Result<_, DeserializeError> {
                    let (index_0_value, index_0_encoding) = raw.unsigned_integer_sz()?;
                    if index_0_value != 0 {
                        return Err(DeserializeFailure::FixedValueMismatch {
                            found: Key::Uint(index_0_value),
                            expected: Key::Uint(0),
                        }
                        .into());
                    }
                    Ok(Some(index_0_encoding))
                })()
                .map_err(|e| e.annotate("index_0"))?;
                let (ed25519_key_hash, ed25519_key_hash_encoding) = raw
                    .bytes_sz()
                    .map_err(Into::<DeserializeError>::into)
                    .and_then(|(bytes, enc)| {
                        Ed25519KeyHash::from_raw_bytes(&bytes)
                            .map(|bytes| (bytes, StringEncoding::from(enc)))
                            .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
                    })
                    .map_err(|e: DeserializeError| e.annotate("ed25519_key_hash"))?;
                match len {
                    cbor_event::LenSz::Len(_, _) => (),
                    cbor_event::LenSz::Indefinite => match raw.special()? {
                        cbor_event::Special::Break => (),
                        _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                    },
                }
                Ok(Self::ConstitutionalCommitteeHotKeyHash {
                    ed25519_key_hash,
                    len_encoding,
                    index_0_encoding,
                    ed25519_key_hash_encoding,
                })
            })(raw);
            match variant_deser {
                Ok(variant) => return Ok(variant),
                Err(e) => {
                    errs.push(e.annotate("ConstitutionalCommitteeHotKeyHash"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let variant_deser = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let mut read_len = CBORReadLen::new(len);
                read_len.read_elems(2)?;
                read_len.finish()?;
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
                let (script_hash, script_hash_encoding) = raw
                    .bytes_sz()
                    .map_err(Into::<DeserializeError>::into)
                    .and_then(|(bytes, enc)| {
                        ScriptHash::from_raw_bytes(&bytes)
                            .map(|bytes| (bytes, StringEncoding::from(enc)))
                            .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
                    })
                    .map_err(|e: DeserializeError| e.annotate("script_hash"))?;
                match len {
                    cbor_event::LenSz::Len(_, _) => (),
                    cbor_event::LenSz::Indefinite => match raw.special()? {
                        cbor_event::Special::Break => (),
                        _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                    },
                }
                Ok(Self::ConstitutionalCommitteeHotScriptHash {
                    script_hash,
                    len_encoding,
                    index_0_encoding,
                    script_hash_encoding,
                })
            })(raw);
            match variant_deser {
                Ok(variant) => return Ok(variant),
                Err(e) => {
                    errs.push(e.annotate("ConstitutionalCommitteeHotScriptHash"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let variant_deser = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let mut read_len = CBORReadLen::new(len);
                read_len.read_elems(2)?;
                read_len.finish()?;
                let index_0_encoding = (|| -> Result<_, DeserializeError> {
                    let (index_0_value, index_0_encoding) = raw.unsigned_integer_sz()?;
                    if index_0_value != 2 {
                        return Err(DeserializeFailure::FixedValueMismatch {
                            found: Key::Uint(index_0_value),
                            expected: Key::Uint(2),
                        }
                        .into());
                    }
                    Ok(Some(index_0_encoding))
                })()
                .map_err(|e| e.annotate("index_0"))?;
                let (ed25519_key_hash, ed25519_key_hash_encoding) = raw
                    .bytes_sz()
                    .map_err(Into::<DeserializeError>::into)
                    .and_then(|(bytes, enc)| {
                        Ed25519KeyHash::from_raw_bytes(&bytes)
                            .map(|bytes| (bytes, StringEncoding::from(enc)))
                            .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
                    })
                    .map_err(|e: DeserializeError| e.annotate("ed25519_key_hash"))?;
                match len {
                    cbor_event::LenSz::Len(_, _) => (),
                    cbor_event::LenSz::Indefinite => match raw.special()? {
                        cbor_event::Special::Break => (),
                        _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                    },
                }
                Ok(Self::DRepKeyHash {
                    ed25519_key_hash,
                    len_encoding,
                    index_0_encoding,
                    ed25519_key_hash_encoding,
                })
            })(raw);
            match variant_deser {
                Ok(variant) => return Ok(variant),
                Err(e) => {
                    errs.push(e.annotate("DRepKeyHash"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let variant_deser = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let mut read_len = CBORReadLen::new(len);
                read_len.read_elems(2)?;
                read_len.finish()?;
                let index_0_encoding = (|| -> Result<_, DeserializeError> {
                    let (index_0_value, index_0_encoding) = raw.unsigned_integer_sz()?;
                    if index_0_value != 3 {
                        return Err(DeserializeFailure::FixedValueMismatch {
                            found: Key::Uint(index_0_value),
                            expected: Key::Uint(3),
                        }
                        .into());
                    }
                    Ok(Some(index_0_encoding))
                })()
                .map_err(|e| e.annotate("index_0"))?;
                let (script_hash, script_hash_encoding) = raw
                    .bytes_sz()
                    .map_err(Into::<DeserializeError>::into)
                    .and_then(|(bytes, enc)| {
                        ScriptHash::from_raw_bytes(&bytes)
                            .map(|bytes| (bytes, StringEncoding::from(enc)))
                            .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
                    })
                    .map_err(|e: DeserializeError| e.annotate("script_hash"))?;
                match len {
                    cbor_event::LenSz::Len(_, _) => (),
                    cbor_event::LenSz::Indefinite => match raw.special()? {
                        cbor_event::Special::Break => (),
                        _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                    },
                }
                Ok(Self::DRepScriptHash {
                    script_hash,
                    len_encoding,
                    index_0_encoding,
                    script_hash_encoding,
                })
            })(raw);
            match variant_deser {
                Ok(variant) => return Ok(variant),
                Err(e) => {
                    errs.push(e.annotate("DRepScriptHash"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let variant_deser = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let mut read_len = CBORReadLen::new(len);
                read_len.read_elems(2)?;
                read_len.finish()?;
                let index_0_encoding = (|| -> Result<_, DeserializeError> {
                    let (index_0_value, index_0_encoding) = raw.unsigned_integer_sz()?;
                    if index_0_value != 4 {
                        return Err(DeserializeFailure::FixedValueMismatch {
                            found: Key::Uint(index_0_value),
                            expected: Key::Uint(4),
                        }
                        .into());
                    }
                    Ok(Some(index_0_encoding))
                })()
                .map_err(|e| e.annotate("index_0"))?;
                let (ed25519_key_hash, ed25519_key_hash_encoding) = raw
                    .bytes_sz()
                    .map_err(Into::<DeserializeError>::into)
                    .and_then(|(bytes, enc)| {
                        Ed25519KeyHash::from_raw_bytes(&bytes)
                            .map(|bytes| (bytes, StringEncoding::from(enc)))
                            .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
                    })
                    .map_err(|e: DeserializeError| e.annotate("ed25519_key_hash"))?;
                match len {
                    cbor_event::LenSz::Len(_, _) => (),
                    cbor_event::LenSz::Indefinite => match raw.special()? {
                        cbor_event::Special::Break => (),
                        _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                    },
                }
                Ok(Self::StakingPoolKeyHash {
                    ed25519_key_hash,
                    len_encoding,
                    index_0_encoding,
                    ed25519_key_hash_encoding,
                })
            })(raw);
            match variant_deser {
                Ok(variant) => return Ok(variant),
                Err(e) => {
                    errs.push(e.annotate("StakingPoolKeyHash"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            Err(DeserializeError::new(
                "Voter",
                DeserializeFailure::NoVariantMatchedWithCauses(errs),
            ))
        })()
        .map_err(|e| e.annotate("Voter"))
    }
}

impl Serialize for VotingProcedure {
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
        match &self.vote {
            Vote::No => serializer.write_unsigned_integer_sz(
                0u64,
                fit_sz(
                    0u64,
                    self.encodings
                        .as_ref()
                        .map(|encs| encs.vote_encoding)
                        .unwrap_or_default(),
                    force_canonical,
                ),
            ),
            Vote::Yes => serializer.write_unsigned_integer_sz(
                1u64,
                fit_sz(
                    1u64,
                    self.encodings
                        .as_ref()
                        .map(|encs| encs.vote_encoding)
                        .unwrap_or_default(),
                    force_canonical,
                ),
            ),
            Vote::Abstain => serializer.write_unsigned_integer_sz(
                2u64,
                fit_sz(
                    2u64,
                    self.encodings
                        .as_ref()
                        .map(|encs| encs.vote_encoding)
                        .unwrap_or_default(),
                    force_canonical,
                ),
            ),
        }?;
        match &self.anchor {
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

impl Deserialize for VotingProcedure {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let len_encoding: LenEncoding = len.into();
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let (vote, vote_encoding) = (|| -> Result<_, DeserializeError> {
                let initial_position = raw.as_mut_ref().stream_position().unwrap();
                let deser_variant = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                    let (no_value, no_encoding) = raw.unsigned_integer_sz()?;
                    if no_value != 0 {
                        return Err(DeserializeFailure::FixedValueMismatch {
                            found: Key::Uint(no_value),
                            expected: Key::Uint(0),
                        }
                        .into());
                    }
                    Ok(Some(no_encoding))
                })(raw);
                match deser_variant {
                    Ok(vote_encoding) => return Ok((Vote::No, vote_encoding)),
                    Err(_) => raw
                        .as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap(),
                };
                let deser_variant = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                    let (yes_value, yes_encoding) = raw.unsigned_integer_sz()?;
                    if yes_value != 1 {
                        return Err(DeserializeFailure::FixedValueMismatch {
                            found: Key::Uint(yes_value),
                            expected: Key::Uint(1),
                        }
                        .into());
                    }
                    Ok(Some(yes_encoding))
                })(raw);
                match deser_variant {
                    Ok(vote_encoding) => return Ok((Vote::Yes, vote_encoding)),
                    Err(_) => raw
                        .as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap(),
                };
                let deser_variant = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                    let (abstain_value, abstain_encoding) = raw.unsigned_integer_sz()?;
                    if abstain_value != 2 {
                        return Err(DeserializeFailure::FixedValueMismatch {
                            found: Key::Uint(abstain_value),
                            expected: Key::Uint(2),
                        }
                        .into());
                    }
                    Ok(Some(abstain_encoding))
                })(raw);
                match deser_variant {
                    Ok(vote_encoding) => return Ok((Vote::Abstain, vote_encoding)),
                    Err(_) => raw
                        .as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap(),
                };
                Err(DeserializeError::new(
                    "Vote",
                    DeserializeFailure::NoVariantMatched,
                ))
            })()
            .map_err(|e| e.annotate("vote"))?;
            let anchor = (|| -> Result<_, DeserializeError> {
                Ok(match raw.cbor_type()? != cbor_event::Type::Special {
                    true => Some(Anchor::deserialize(raw)?),
                    false => {
                        if raw.special()? != cbor_event::Special::Null {
                            return Err(DeserializeFailure::ExpectedNull.into());
                        }
                        None
                    }
                })
            })()
            .map_err(|e| e.annotate("anchor"))?;
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(VotingProcedure {
                vote,
                anchor,
                encodings: Some(VotingProcedureEncoding {
                    len_encoding,
                    vote_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("VotingProcedure"))
    }
}
