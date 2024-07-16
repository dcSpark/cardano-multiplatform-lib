// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use crate::plutus::PlutusV3Script;

use super::cbor_encodings::*;
use super::*;
use cbor_event::de::Deserializer;
use cbor_event::se::Serializer;
use cml_core::error::*;
use cml_core::serialization::*;
use std::io::{BufRead, Seek, SeekFrom, Write};

impl Serialize for AuxiliaryData {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            AuxiliaryData::Shelley(shelley) => shelley.serialize(serializer, force_canonical),
            AuxiliaryData::ShelleyMA(shelley_ma) => {
                shelley_ma.serialize(serializer, force_canonical)
            }
            AuxiliaryData::Conway(conway) => conway.serialize(serializer, force_canonical),
        }
    }
}

impl Deserialize for AuxiliaryData {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let initial_position = raw.as_mut_ref().stream_position().unwrap();
            let mut errs = Vec::new();
            let deser_variant: Result<_, DeserializeError> = Metadata::deserialize(raw);
            match deser_variant {
                Ok(shelley) => return Ok(Self::Shelley(shelley)),
                Err(e) => {
                    errs.push(e.annotate("Shelley"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant: Result<_, DeserializeError> =
                ShelleyMAFormatAuxData::deserialize(raw);
            match deser_variant {
                Ok(shelley_ma) => return Ok(Self::ShelleyMA(shelley_ma)),
                Err(e) => {
                    errs.push(e.annotate("ShelleyMA"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant: Result<_, DeserializeError> = ConwayFormatAuxData::deserialize(raw);
            match deser_variant {
                Ok(conway) => return Ok(Self::Conway(conway)),
                Err(e) => {
                    errs.push(e.annotate("Conway"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            Err(DeserializeError::new(
                "AuxiliaryData",
                DeserializeFailure::NoVariantMatchedWithCauses(errs),
            ))
        })()
        .map_err(|e| e.annotate("AuxiliaryData"))
    }
}

impl Serialize for ConwayFormatAuxData {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_tag_sz(
            259u64,
            fit_sz(
                259u64,
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
                .map(|encs| encs.len_encoding)
                .unwrap_or_default()
                .to_len_sz(
                    match &self.metadata {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.native_scripts {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.plutus_v1_scripts {
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
                        == match &self.metadata {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.native_scripts {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.plutus_v1_scripts {
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
            .unwrap_or_else(|| vec![0, 1, 2, 3, 4]);
        for field_index in deser_order {
            match field_index {
                0 => {
                    if let Some(field) = &self.metadata {
                        serializer.write_unsigned_integer_sz(
                            0u64,
                            fit_sz(
                                0u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.metadata_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        field.serialize(serializer, force_canonical)?;
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
                    if let Some(field) = &self.plutus_v1_scripts {
                        serializer.write_unsigned_integer_sz(
                            2u64,
                            fit_sz(
                                2u64,
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
                3 => {
                    if let Some(field) = &self.plutus_v2_scripts {
                        serializer.write_unsigned_integer_sz(
                            3u64,
                            fit_sz(
                                3u64,
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
                4 => {
                    if let Some(field) = &self.plutus_v3_scripts {
                        serializer.write_unsigned_integer_sz(
                            4u64,
                            fit_sz(
                                4u64,
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

impl Deserialize for ConwayFormatAuxData {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let (tag, tag_encoding) = raw.tag_sz()?;
        if tag != 259 {
            return Err(DeserializeError::new(
                "ConwayFormatAuxData",
                DeserializeFailure::TagMismatch {
                    found: tag,
                    expected: 259,
                },
            ));
        }
        let len = raw.map_sz()?;
        let len_encoding: LenEncoding = len.into();
        let mut read_len = CBORReadLen::new(len);
        (|| -> Result<_, DeserializeError> {
            let mut orig_deser_order = Vec::new();
            let mut metadata_key_encoding = None;
            let mut metadata = None;
            let mut native_scripts_encoding = LenEncoding::default();
            let mut native_scripts_key_encoding = None;
            let mut native_scripts = None;
            let mut plutus_v1_scripts_encoding = LenEncoding::default();
            let mut plutus_v1_scripts_key_encoding = None;
            let mut plutus_v1_scripts = None;
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
                            if metadata.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(0)).into());
                            }
                            let tmp_metadata = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                Metadata::deserialize(raw)
                            })()
                            .map_err(|e| e.annotate("metadata"))?;
                            metadata = Some(tmp_metadata);
                            metadata_key_encoding = Some(key_enc);
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
                            if plutus_v1_scripts.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(2)).into());
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
                            orig_deser_order.push(2);
                        }
                        (3, key_enc) => {
                            if plutus_v2_scripts.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(3)).into());
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
                            orig_deser_order.push(3);
                        }
                        (4, key_enc) => {
                            if plutus_v3_scripts.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(4)).into());
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
                            orig_deser_order.push(4);
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
                metadata,
                native_scripts,
                plutus_v1_scripts,
                plutus_v2_scripts,
                plutus_v3_scripts,
                encodings: Some(ConwayFormatAuxDataEncoding {
                    tag_encoding: Some(tag_encoding),
                    len_encoding,
                    orig_deser_order,
                    metadata_key_encoding,
                    native_scripts_key_encoding,
                    native_scripts_encoding,
                    plutus_v1_scripts_key_encoding,
                    plutus_v1_scripts_encoding,
                    plutus_v2_scripts_key_encoding,
                    plutus_v2_scripts_encoding,
                    plutus_v3_scripts_key_encoding,
                    plutus_v3_scripts_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("ConwayFormatAuxData"))
    }
}

impl Serialize for ShelleyMAFormatAuxData {
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
        self.transaction_metadata
            .serialize(serializer, force_canonical)?;
        serializer.write_array_sz(
            self.encodings
                .as_ref()
                .map(|encs| encs.auxiliary_scripts_encoding)
                .unwrap_or_default()
                .to_len_sz(self.auxiliary_scripts.len() as u64, force_canonical),
        )?;
        for element in self.auxiliary_scripts.iter() {
            element.serialize(serializer, force_canonical)?;
        }
        self.encodings
            .as_ref()
            .map(|encs| encs.auxiliary_scripts_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)?;
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for ShelleyMAFormatAuxData {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let len_encoding: LenEncoding = len.into();
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let transaction_metadata = Metadata::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("transaction_metadata"))?;
            let (auxiliary_scripts, auxiliary_scripts_encoding) =
                (|| -> Result<_, DeserializeError> {
                    let mut auxiliary_scripts_arr = Vec::new();
                    let len = raw.array_sz()?;
                    let auxiliary_scripts_encoding = len.into();
                    while match len {
                        cbor_event::LenSz::Len(n, _) => (auxiliary_scripts_arr.len() as u64) < n,
                        cbor_event::LenSz::Indefinite => true,
                    } {
                        if raw.cbor_type()? == cbor_event::Type::Special {
                            assert_eq!(raw.special()?, cbor_event::Special::Break);
                            break;
                        }
                        auxiliary_scripts_arr.push(NativeScript::deserialize(raw)?);
                    }
                    Ok((auxiliary_scripts_arr, auxiliary_scripts_encoding))
                })()
                .map_err(|e| e.annotate("auxiliary_scripts"))?;
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(ShelleyMAFormatAuxData {
                transaction_metadata,
                auxiliary_scripts,
                encodings: Some(ShelleyMAFormatAuxDataEncoding {
                    len_encoding,
                    auxiliary_scripts_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("ShelleyMAFormatAuxData"))
    }
}
