// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use super::cbor_encodings::*;
use super::*;
use cbor_event::de::Deserializer;
use cbor_event::se::Serializer;
use cml_core::error::*;
use cml_core::serialization::*;
use std::io::{BufRead, Seek, SeekFrom, Write};

// PlutusData::Bytes uses this specific encoding:
use crate::utils::{read_bounded_bytes, write_bounded_bytes};

impl Serialize for CostModels {
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
                    match &self.plutus_v1 {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.plutus_v2 {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.plutus_v3 {
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
                        == match &self.plutus_v1 {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.plutus_v2 {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.plutus_v3 {
                            Some(_) => 1,
                            None => 0,
                        }
            })
            .map(|encs| encs.orig_deser_order.clone())
            .unwrap_or_else(|| vec![0, 1, 2]);
        for field_index in deser_order {
            match field_index {
                0 => {
                    if let Some(field) = &self.plutus_v1 {
                        serializer.write_unsigned_integer_sz(
                            0u64,
                            fit_sz(
                                0u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.plutus_v1_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        serializer.write_array_sz(
                            self.encodings
                                .as_ref()
                                .map(|encs| encs.plutus_v1_encoding)
                                .unwrap_or_default()
                                .to_len_sz(field.len() as u64, force_canonical),
                        )?;
                        for element in field.iter() {
                            element.serialize(serializer, force_canonical)?;
                        }
                        self.encodings
                            .as_ref()
                            .map(|encs| encs.plutus_v1_encoding)
                            .unwrap_or_default()
                            .end(serializer, force_canonical)?;
                    }
                }
                1 => {
                    if let Some(field) = &self.plutus_v2 {
                        serializer.write_unsigned_integer_sz(
                            1u64,
                            fit_sz(
                                1u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.plutus_v2_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        serializer.write_array_sz(
                            self.encodings
                                .as_ref()
                                .map(|encs| encs.plutus_v2_encoding)
                                .unwrap_or_default()
                                .to_len_sz(field.len() as u64, force_canonical),
                        )?;
                        for element in field.iter() {
                            element.serialize(serializer, force_canonical)?;
                        }
                        self.encodings
                            .as_ref()
                            .map(|encs| encs.plutus_v2_encoding)
                            .unwrap_or_default()
                            .end(serializer, force_canonical)?;
                    }
                }
                2 => {
                    if let Some(field) = &self.plutus_v3 {
                        serializer.write_unsigned_integer_sz(
                            2u64,
                            fit_sz(
                                2u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.plutus_v3_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        serializer.write_array_sz(
                            self.encodings
                                .as_ref()
                                .map(|encs| encs.plutus_v3_encoding)
                                .unwrap_or_default()
                                .to_len_sz(field.len() as u64, force_canonical),
                        )?;
                        for element in field.iter() {
                            element.serialize(serializer, force_canonical)?;
                        }
                        self.encodings
                            .as_ref()
                            .map(|encs| encs.plutus_v3_encoding)
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

impl Deserialize for CostModels {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.map_sz()?;
        let len_encoding: LenEncoding = len.into();
        let mut read_len = CBORReadLen::new(len);
        (|| -> Result<_, DeserializeError> {
            let mut orig_deser_order = Vec::new();
            let mut plutus_v1_encoding = LenEncoding::default();
            let mut plutus_v1_key_encoding = None;
            let mut plutus_v1 = None;
            let mut plutus_v2_encoding = LenEncoding::default();
            let mut plutus_v2_key_encoding = None;
            let mut plutus_v2 = None;
            let mut plutus_v3_encoding = LenEncoding::default();
            let mut plutus_v3_key_encoding = None;
            let mut plutus_v3 = None;
            let mut read = 0;
            while match len {
                cbor_event::LenSz::Len(n, _) => read < n,
                cbor_event::LenSz::Indefinite => true,
            } {
                match raw.cbor_type()? {
                    cbor_event::Type::UnsignedInteger => match raw.unsigned_integer_sz()? {
                        (0, key_enc) => {
                            if plutus_v1.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(0)).into());
                            }
                            let (tmp_plutus_v1, tmp_plutus_v1_encoding) =
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    let mut plutus_v1_arr = Vec::new();
                                    let len = raw.array_sz()?;
                                    let plutus_v1_encoding = len.into();
                                    while match len {
                                        cbor_event::LenSz::Len(n, _) => {
                                            (plutus_v1_arr.len() as u64) < n
                                        }
                                        cbor_event::LenSz::Indefinite => true,
                                    } {
                                        if raw.cbor_type()? == cbor_event::Type::Special {
                                            assert_eq!(raw.special()?, cbor_event::Special::Break);
                                            break;
                                        }
                                        plutus_v1_arr.push(Int::deserialize(raw)?);
                                    }
                                    Ok((plutus_v1_arr, plutus_v1_encoding))
                                })()
                                .map_err(|e| e.annotate("plutus_v1"))?;
                            plutus_v1 = Some(tmp_plutus_v1);
                            plutus_v1_encoding = tmp_plutus_v1_encoding;
                            plutus_v1_key_encoding = Some(key_enc);
                            orig_deser_order.push(0);
                        }
                        (1, key_enc) => {
                            if plutus_v2.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(1)).into());
                            }
                            let (tmp_plutus_v2, tmp_plutus_v2_encoding) =
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    let mut plutus_v2_arr = Vec::new();
                                    let len = raw.array_sz()?;
                                    let plutus_v2_encoding = len.into();
                                    while match len {
                                        cbor_event::LenSz::Len(n, _) => {
                                            (plutus_v2_arr.len() as u64) < n
                                        }
                                        cbor_event::LenSz::Indefinite => true,
                                    } {
                                        if raw.cbor_type()? == cbor_event::Type::Special {
                                            assert_eq!(raw.special()?, cbor_event::Special::Break);
                                            break;
                                        }
                                        plutus_v2_arr.push(Int::deserialize(raw)?);
                                    }
                                    Ok((plutus_v2_arr, plutus_v2_encoding))
                                })()
                                .map_err(|e| e.annotate("plutus_v2"))?;
                            plutus_v2 = Some(tmp_plutus_v2);
                            plutus_v2_encoding = tmp_plutus_v2_encoding;
                            plutus_v2_key_encoding = Some(key_enc);
                            orig_deser_order.push(1);
                        }
                        (2, key_enc) => {
                            if plutus_v3.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(2)).into());
                            }
                            let (tmp_plutus_v3, tmp_plutus_v3_encoding) =
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    let mut plutus_v3_arr = Vec::new();
                                    let len = raw.array_sz()?;
                                    let plutus_v3_encoding = len.into();
                                    while match len {
                                        cbor_event::LenSz::Len(n, _) => {
                                            (plutus_v3_arr.len() as u64) < n
                                        }
                                        cbor_event::LenSz::Indefinite => true,
                                    } {
                                        if raw.cbor_type()? == cbor_event::Type::Special {
                                            assert_eq!(raw.special()?, cbor_event::Special::Break);
                                            break;
                                        }
                                        plutus_v3_arr.push(Int::deserialize(raw)?);
                                    }
                                    Ok((plutus_v3_arr, plutus_v3_encoding))
                                })()
                                .map_err(|e| e.annotate("plutus_v3"))?;
                            plutus_v3 = Some(tmp_plutus_v3);
                            plutus_v3_encoding = tmp_plutus_v3_encoding;
                            plutus_v3_key_encoding = Some(key_enc);
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
                plutus_v1,
                plutus_v2,
                plutus_v3,
                encodings: Some(CostModelsEncoding {
                    len_encoding,
                    orig_deser_order,
                    plutus_v1_key_encoding,
                    plutus_v1_encoding,
                    plutus_v2_key_encoding,
                    plutus_v2_encoding,
                    plutus_v3_key_encoding,
                    plutus_v3_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("CostModels"))
    }
}

impl Serialize for ExUnitPrices {
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
        self.mem_price.serialize(serializer, force_canonical)?;
        self.step_price.serialize(serializer, force_canonical)?;
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for ExUnitPrices {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let len_encoding: LenEncoding = len.into();
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let mem_price = Rational::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("mem_price"))?;
            let step_price = Rational::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("step_price"))?;
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(ExUnitPrices {
                mem_price,
                step_price,
                encodings: Some(ExUnitPricesEncoding { len_encoding }),
            })
        })()
        .map_err(|e| e.annotate("ExUnitPrices"))
    }
}

impl Serialize for ExUnits {
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
        serializer.write_unsigned_integer_sz(
            self.mem,
            fit_sz(
                self.mem,
                self.encodings
                    .as_ref()
                    .map(|encs| encs.mem_encoding)
                    .unwrap_or_default(),
                force_canonical,
            ),
        )?;
        serializer.write_unsigned_integer_sz(
            self.steps,
            fit_sz(
                self.steps,
                self.encodings
                    .as_ref()
                    .map(|encs| encs.steps_encoding)
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

impl Deserialize for ExUnits {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let len_encoding: LenEncoding = len.into();
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let (mem, mem_encoding) = raw
                .unsigned_integer_sz()
                .map(|(x, enc)| (x, Some(enc)))
                .map_err(Into::<DeserializeError>::into)
                .map_err(|e: DeserializeError| e.annotate("mem"))?;
            let (steps, steps_encoding) = raw
                .unsigned_integer_sz()
                .map(|(x, enc)| (x, Some(enc)))
                .map_err(Into::<DeserializeError>::into)
                .map_err(|e: DeserializeError| e.annotate("steps"))?;
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(ExUnits {
                mem,
                steps,
                encodings: Some(ExUnitsEncoding {
                    len_encoding,
                    mem_encoding,
                    steps_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("ExUnits"))
    }
}

impl Serialize for PlutusData {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            PlutusData::ConstrPlutusData(constr_plutus_data) => {
                constr_plutus_data.serialize(serializer, force_canonical)
            }
            PlutusData::Map(map) => map.serialize(serializer, force_canonical),
            PlutusData::List {
                list,
                list_encoding,
            } => {
                serializer
                    .write_array_sz(list_encoding.to_len_sz(list.len() as u64, force_canonical))?;
                for element in list.iter() {
                    element.serialize(serializer, force_canonical)?;
                }
                list_encoding.end(serializer, force_canonical)
            }
            PlutusData::Integer(big_int) => big_int.serialize(serializer, force_canonical),
            // hand-written
            PlutusData::Bytes {
                bytes,
                bytes_encoding,
            } => write_bounded_bytes(serializer, bytes, bytes_encoding, force_canonical),
        }
    }
}

impl Deserialize for PlutusData {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            // hand-coded based on generated code
            // 1) we use bounded bytes not
            // 2) to give better errors / direct branch on cbor_type()?
            match raw.cbor_type()? {
                cbor_event::Type::Tag => {
                    // could be large BigInteger or ConstrPlutusData so check tag to see which it is
                    let initial_position = raw.as_mut_ref().stream_position().unwrap();
                    let tag = raw.tag()?;
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                    if tag == 2 || tag == 3 {
                        BigInteger::deserialize(raw)
                            .map(Self::Integer)
                            .map_err(|e| e.annotate("Integer"))
                    } else {
                        ConstrPlutusData::deserialize(raw)
                            .map(Self::ConstrPlutusData)
                            .map_err(|e| e.annotate("ConstrPlutusData"))
                    }
                }
                cbor_event::Type::Map => PlutusMap::deserialize(raw)
                    .map(Self::Map)
                    .map_err(|e| e.annotate("Map")),
                cbor_event::Type::Array => {
                    (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                        let mut list_arr = Vec::new();
                        let len = raw.array_sz()?;
                        let list_encoding = len.into();
                        while match len {
                            cbor_event::LenSz::Len(n, _) => (list_arr.len() as u64) < n,
                            cbor_event::LenSz::Indefinite => true,
                        } {
                            if raw.cbor_type()? == cbor_event::Type::Special {
                                assert_eq!(raw.special()?, cbor_event::Special::Break);
                                break;
                            }
                            list_arr.push(PlutusData::deserialize(raw)?);
                        }
                        Ok(Self::List {
                            list: list_arr,
                            list_encoding,
                        })
                    })(raw)
                    .map_err(|e| e.annotate("List"))
                }
                cbor_event::Type::UnsignedInteger | cbor_event::Type::NegativeInteger => {
                    BigInteger::deserialize(raw)
                        .map(Self::Integer)
                        .map_err(|e| e.annotate("Integer"))
                }
                // hand-written 100% since the format is not just arbitrary CBOR bytes
                cbor_event::Type::Bytes => read_bounded_bytes(raw)
                    .map(|(bytes, bytes_encoding)| Self::Bytes {
                        bytes,
                        bytes_encoding,
                    })
                    .map_err(|e| e.annotate("Bytes")),
                _ => Err(DeserializeFailure::NoVariantMatched.into()),
            }
        })()
        .map_err(|e| e.annotate("PlutusData"))
    }
}

impl Serialize for PlutusV1Script {
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

impl Deserialize for PlutusV1Script {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let (inner, inner_encoding) = raw
            .bytes_sz()
            .map(|(bytes, enc)| (bytes, StringEncoding::from(enc)))?;
        Ok(Self {
            inner,
            encodings: Some(PlutusV1ScriptEncoding { inner_encoding }),
        })
    }
}

impl Serialize for PlutusV2Script {
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

impl Deserialize for PlutusV2Script {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let (inner, inner_encoding) = raw
            .bytes_sz()
            .map(|(bytes, enc)| (bytes, StringEncoding::from(enc)))?;
        Ok(Self {
            inner,
            encodings: Some(PlutusV2ScriptEncoding { inner_encoding }),
        })
    }
}

impl Serialize for PlutusV3Script {
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

impl Deserialize for PlutusV3Script {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let (inner, inner_encoding) = raw
            .bytes_sz()
            .map(|(bytes, enc)| (bytes, StringEncoding::from(enc)))?;
        Ok(Self {
            inner,
            encodings: Some(PlutusV3ScriptEncoding { inner_encoding }),
        })
    }
}

impl Serialize for Redeemer {
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
        match &self.tag {
            RedeemerTag::Spend => serializer.write_unsigned_integer_sz(
                0u64,
                fit_sz(
                    0u64,
                    self.encodings
                        .as_ref()
                        .map(|encs| encs.tag_encoding)
                        .unwrap_or_default(),
                    force_canonical,
                ),
            ),
            RedeemerTag::Mint => serializer.write_unsigned_integer_sz(
                1u64,
                fit_sz(
                    1u64,
                    self.encodings
                        .as_ref()
                        .map(|encs| encs.tag_encoding)
                        .unwrap_or_default(),
                    force_canonical,
                ),
            ),
            RedeemerTag::Cert => serializer.write_unsigned_integer_sz(
                2u64,
                fit_sz(
                    2u64,
                    self.encodings
                        .as_ref()
                        .map(|encs| encs.tag_encoding)
                        .unwrap_or_default(),
                    force_canonical,
                ),
            ),
            RedeemerTag::Reward => serializer.write_unsigned_integer_sz(
                3u64,
                fit_sz(
                    3u64,
                    self.encodings
                        .as_ref()
                        .map(|encs| encs.tag_encoding)
                        .unwrap_or_default(),
                    force_canonical,
                ),
            ),
        }?;
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
        self.data.serialize(serializer, force_canonical)?;
        self.ex_units.serialize(serializer, force_canonical)?;
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for Redeemer {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let len_encoding: LenEncoding = len.into();
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(4)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let (tag, tag_encoding) = (|| -> Result<_, DeserializeError> {
                let initial_position = raw.as_mut_ref().stream_position().unwrap();
                let deser_variant = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                    let (spend_value, spend_encoding) = raw.unsigned_integer_sz()?;
                    if spend_value != 0 {
                        return Err(DeserializeFailure::FixedValueMismatch {
                            found: Key::Uint(spend_value),
                            expected: Key::Uint(0),
                        }
                        .into());
                    }
                    Ok(Some(spend_encoding))
                })(raw);
                match deser_variant {
                    Ok(tag_encoding) => return Ok((RedeemerTag::Spend, tag_encoding)),
                    Err(_) => raw
                        .as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap(),
                };
                let deser_variant = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                    let (mint_value, mint_encoding) = raw.unsigned_integer_sz()?;
                    if mint_value != 1 {
                        return Err(DeserializeFailure::FixedValueMismatch {
                            found: Key::Uint(mint_value),
                            expected: Key::Uint(1),
                        }
                        .into());
                    }
                    Ok(Some(mint_encoding))
                })(raw);
                match deser_variant {
                    Ok(tag_encoding) => return Ok((RedeemerTag::Mint, tag_encoding)),
                    Err(_) => raw
                        .as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap(),
                };
                let deser_variant = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                    let (cert_value, cert_encoding) = raw.unsigned_integer_sz()?;
                    if cert_value != 2 {
                        return Err(DeserializeFailure::FixedValueMismatch {
                            found: Key::Uint(cert_value),
                            expected: Key::Uint(2),
                        }
                        .into());
                    }
                    Ok(Some(cert_encoding))
                })(raw);
                match deser_variant {
                    Ok(tag_encoding) => return Ok((RedeemerTag::Cert, tag_encoding)),
                    Err(_) => raw
                        .as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap(),
                };
                let deser_variant = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                    let (reward_value, reward_encoding) = raw.unsigned_integer_sz()?;
                    if reward_value != 3 {
                        return Err(DeserializeFailure::FixedValueMismatch {
                            found: Key::Uint(reward_value),
                            expected: Key::Uint(3),
                        }
                        .into());
                    }
                    Ok(Some(reward_encoding))
                })(raw);
                match deser_variant {
                    Ok(tag_encoding) => return Ok((RedeemerTag::Reward, tag_encoding)),
                    Err(_) => raw
                        .as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap(),
                };
                Err(DeserializeError::new(
                    "RedeemerTag",
                    DeserializeFailure::NoVariantMatched,
                ))
            })()
            .map_err(|e| e.annotate("tag"))?;
            let (index, index_encoding) = raw
                .unsigned_integer_sz()
                .map_err(Into::<DeserializeError>::into)
                .map(|(x, enc)| (x, Some(enc)))
                .map_err(|e: DeserializeError| e.annotate("index"))?;
            let data =
                PlutusData::deserialize(raw).map_err(|e: DeserializeError| e.annotate("data"))?;
            let ex_units =
                ExUnits::deserialize(raw).map_err(|e: DeserializeError| e.annotate("ex_units"))?;
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(Redeemer {
                tag,
                index,
                data,
                ex_units,
                encodings: Some(RedeemerEncoding {
                    len_encoding,
                    tag_encoding,
                    index_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("Redeemer"))
    }
}
