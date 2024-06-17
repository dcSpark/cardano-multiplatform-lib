// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use super::cbor_encodings::*;
use super::*;
use cbor_event::de::Deserializer;
use cbor_event::se::Serializer;
use cml_core::error::*;
use cml_core::serialization::*;
use std::collections::BTreeMap;
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
                .map(|encs| encs.inner_encoding)
                .unwrap_or_default()
                .to_len_sz(self.inner.len() as u64, force_canonical),
        )?;
        let mut key_order = self
            .inner
            .iter()
            .map(|(k, v)| {
                let mut buf = cbor_event::se::Serializer::new_vec();
                let inner_key_encoding = self
                    .encodings
                    .as_ref()
                    .and_then(|encs| encs.inner_key_encodings.get(k))
                    .cloned()
                    .unwrap_or_default();
                buf.write_unsigned_integer_sz(*k, fit_sz(*k, inner_key_encoding, force_canonical))?;
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
            let (inner_value_encoding, inner_value_elem_encodings) = self
                .encodings
                .as_ref()
                .and_then(|encs| encs.inner_value_encodings.get(key))
                .cloned()
                .unwrap_or_else(|| (LenEncoding::default(), Vec::new()));
            serializer.write_array_sz(
                inner_value_encoding.to_len_sz(value.len() as u64, force_canonical),
            )?;
            for (i, element) in value.iter().enumerate() {
                let inner_value_elem_encoding = inner_value_elem_encodings
                    .get(i)
                    .cloned()
                    .unwrap_or_default();
                if *element >= 0 {
                    serializer.write_unsigned_integer_sz(
                        *element as u64,
                        fit_sz(*element as u64, inner_value_elem_encoding, force_canonical),
                    )?;
                } else {
                    serializer.write_negative_integer_sz(
                        *element as i128,
                        fit_sz(
                            (*element + 1).unsigned_abs(),
                            inner_value_elem_encoding,
                            force_canonical,
                        ),
                    )?;
                }
            }
            inner_value_encoding.end(serializer, force_canonical)?;
        }
        self.encodings
            .as_ref()
            .map(|encs| encs.inner_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for CostModels {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut inner_table = OrderedHashMap::new();
        let inner_len = raw.map_sz()?;
        let inner_encoding = inner_len.into();
        let mut inner_key_encodings = BTreeMap::new();
        let mut inner_value_encodings = BTreeMap::new();
        while match inner_len {
            cbor_event::LenSz::Len(n, _) => (inner_table.len() as u64) < n,
            cbor_event::LenSz::Indefinite => true,
        } {
            if raw.cbor_type()? == cbor_event::Type::Special {
                assert_eq!(raw.special()?, cbor_event::Special::Break);
                break;
            }
            let (inner_key, inner_key_encoding) =
                raw.unsigned_integer_sz().map(|(x, enc)| (x, Some(enc)))?;
            let mut inner_value_arr = Vec::new();
            let len = raw.array_sz()?;
            let inner_value_encoding = len.into();
            let mut inner_value_elem_encodings = Vec::new();
            while match len {
                cbor_event::LenSz::Len(n, _) => (inner_value_arr.len() as u64) < n,
                cbor_event::LenSz::Indefinite => true,
            } {
                if raw.cbor_type()? == cbor_event::Type::Special {
                    assert_eq!(raw.special()?, cbor_event::Special::Break);
                    break;
                }
                let (inner_value_elem, inner_value_elem_encoding) = match raw.cbor_type()? {
                    cbor_event::Type::UnsignedInteger => {
                        let (x, enc) = raw.unsigned_integer_sz()?;
                        (x as i64, Some(enc))
                    }
                    _ => {
                        let (x, enc) = raw.negative_integer_sz()?;
                        (x as i64, Some(enc))
                    }
                };
                inner_value_arr.push(inner_value_elem);
                inner_value_elem_encodings.push(inner_value_elem_encoding);
            }
            let (inner_value, inner_value_encoding, inner_value_elem_encodings) = (
                inner_value_arr,
                inner_value_encoding,
                inner_value_elem_encodings,
            );
            if inner_table.insert(inner_key, inner_value).is_some() {
                return Err(DeserializeFailure::DuplicateKey(Key::Uint(inner_key)).into());
            }
            inner_key_encodings.insert(inner_key, inner_key_encoding);
            inner_value_encodings.insert(
                inner_key,
                (inner_value_encoding, inner_value_elem_encodings),
            );
        }
        let (inner, inner_encoding, inner_key_encodings, inner_value_encodings) = (
            inner_table,
            inner_encoding,
            inner_key_encodings,
            inner_value_encodings,
        );
        Ok(Self {
            inner,
            encodings: Some(CostModelsEncoding {
                inner_encoding,
                inner_key_encodings,
                inner_value_encodings,
            }),
        })
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
                .map_err(Into::<DeserializeError>::into)
                .map(|(x, enc)| (x, Some(enc)))
                .map_err(|e: DeserializeError| e.annotate("mem"))?;
            let (steps, steps_encoding) = raw
                .unsigned_integer_sz()
                .map_err(Into::<DeserializeError>::into)
                .map(|(x, enc)| (x, Some(enc)))
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

impl Serialize for LegacyRedeemer {
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
            RedeemerTag::Voting => serializer.write_unsigned_integer_sz(
                4u64,
                fit_sz(
                    4u64,
                    self.encodings
                        .as_ref()
                        .map(|encs| encs.tag_encoding)
                        .unwrap_or_default(),
                    force_canonical,
                ),
            ),
            RedeemerTag::Proposing => serializer.write_unsigned_integer_sz(
                5u64,
                fit_sz(
                    5u64,
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

impl Deserialize for LegacyRedeemer {
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
                let deser_variant = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                    let (voting_value, voting_encoding) = raw.unsigned_integer_sz()?;
                    if voting_value != 4 {
                        return Err(DeserializeFailure::FixedValueMismatch {
                            found: Key::Uint(voting_value),
                            expected: Key::Uint(4),
                        }
                        .into());
                    }
                    Ok(Some(voting_encoding))
                })(raw);
                match deser_variant {
                    Ok(tag_encoding) => return Ok((RedeemerTag::Voting, tag_encoding)),
                    Err(_) => raw
                        .as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap(),
                };
                let deser_variant = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                    let (proposing_value, proposing_encoding) = raw.unsigned_integer_sz()?;
                    if proposing_value != 5 {
                        return Err(DeserializeFailure::FixedValueMismatch {
                            found: Key::Uint(proposing_value),
                            expected: Key::Uint(5),
                        }
                        .into());
                    }
                    Ok(Some(proposing_encoding))
                })(raw);
                match deser_variant {
                    Ok(tag_encoding) => return Ok((RedeemerTag::Proposing, tag_encoding)),
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
            Ok(LegacyRedeemer {
                tag,
                index,
                data,
                ex_units,
                encodings: Some(LegacyRedeemerEncoding {
                    len_encoding,
                    tag_encoding,
                    index_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("LegacyRedeemer"))
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

impl Serialize for RedeemerKey {
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
            RedeemerTag::Voting => serializer.write_unsigned_integer_sz(
                4u64,
                fit_sz(
                    4u64,
                    self.encodings
                        .as_ref()
                        .map(|encs| encs.tag_encoding)
                        .unwrap_or_default(),
                    force_canonical,
                ),
            ),
            RedeemerTag::Proposing => serializer.write_unsigned_integer_sz(
                5u64,
                fit_sz(
                    5u64,
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
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for RedeemerKey {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let len_encoding: LenEncoding = len.into();
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
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
                let deser_variant = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                    let (voting_value, voting_encoding) = raw.unsigned_integer_sz()?;
                    if voting_value != 4 {
                        return Err(DeserializeFailure::FixedValueMismatch {
                            found: Key::Uint(voting_value),
                            expected: Key::Uint(4),
                        }
                        .into());
                    }
                    Ok(Some(voting_encoding))
                })(raw);
                match deser_variant {
                    Ok(tag_encoding) => return Ok((RedeemerTag::Voting, tag_encoding)),
                    Err(_) => raw
                        .as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap(),
                };
                let deser_variant = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                    let (proposing_value, proposing_encoding) = raw.unsigned_integer_sz()?;
                    if proposing_value != 5 {
                        return Err(DeserializeFailure::FixedValueMismatch {
                            found: Key::Uint(proposing_value),
                            expected: Key::Uint(5),
                        }
                        .into());
                    }
                    Ok(Some(proposing_encoding))
                })(raw);
                match deser_variant {
                    Ok(tag_encoding) => return Ok((RedeemerTag::Proposing, tag_encoding)),
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
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(RedeemerKey {
                tag,
                index,
                encodings: Some(RedeemerKeyEncoding {
                    len_encoding,
                    tag_encoding,
                    index_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("RedeemerKey"))
    }
}

impl Serialize for RedeemerVal {
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
        self.data.serialize(serializer, force_canonical)?;
        self.ex_units.serialize(serializer, force_canonical)?;
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for RedeemerVal {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let len_encoding: LenEncoding = len.into();
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
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
            Ok(RedeemerVal {
                data,
                ex_units,
                encodings: Some(RedeemerValEncoding { len_encoding }),
            })
        })()
        .map_err(|e| e.annotate("RedeemerVal"))
    }
}

impl Serialize for Redeemers {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            Redeemers::ArrLegacyRedeemer {
                arr_legacy_redeemer,
                arr_legacy_redeemer_encoding,
            } => {
                serializer.write_array_sz(
                    arr_legacy_redeemer_encoding
                        .to_len_sz(arr_legacy_redeemer.len() as u64, force_canonical),
                )?;
                for element in arr_legacy_redeemer.iter() {
                    element.serialize(serializer, force_canonical)?;
                }
                arr_legacy_redeemer_encoding.end(serializer, force_canonical)
            }
            Redeemers::MapRedeemerKeyToRedeemerVal {
                map_redeemer_key_to_redeemer_val,
                map_redeemer_key_to_redeemer_val_encoding,
            } => {
                serializer.write_map_sz(map_redeemer_key_to_redeemer_val_encoding.to_len_sz(
                    map_redeemer_key_to_redeemer_val.len() as u64,
                    force_canonical,
                ))?;
                let mut key_order = map_redeemer_key_to_redeemer_val
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
                map_redeemer_key_to_redeemer_val_encoding.end(serializer, force_canonical)
            }
        }
    }
}

impl Deserialize for Redeemers {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            match raw.cbor_type()? {
                cbor_event::Type::Array => {
                    let mut arr_legacy_redeemer_arr = Vec::new();
                    let len = raw.array_sz()?;
                    let arr_legacy_redeemer_encoding = len.into();
                    while match len {
                        cbor_event::LenSz::Len(n, _) => (arr_legacy_redeemer_arr.len() as u64) < n,
                        cbor_event::LenSz::Indefinite => true,
                    } {
                        if raw.cbor_type()? == cbor_event::Type::Special {
                            assert_eq!(raw.special()?, cbor_event::Special::Break);
                            break;
                        }
                        arr_legacy_redeemer_arr.push(LegacyRedeemer::deserialize(raw)?);
                    }
                    let (arr_legacy_redeemer, arr_legacy_redeemer_encoding) =
                        (arr_legacy_redeemer_arr, arr_legacy_redeemer_encoding);
                    Ok(Self::ArrLegacyRedeemer {
                        arr_legacy_redeemer,
                        arr_legacy_redeemer_encoding,
                    })
                }
                cbor_event::Type::Map => {
                    let mut map_redeemer_key_to_redeemer_val_table = OrderedHashMap::new();
                    let map_redeemer_key_to_redeemer_val_len = raw.map_sz()?;
                    let map_redeemer_key_to_redeemer_val_encoding =
                        map_redeemer_key_to_redeemer_val_len.into();
                    while match map_redeemer_key_to_redeemer_val_len {
                        cbor_event::LenSz::Len(n, _) => {
                            (map_redeemer_key_to_redeemer_val_table.len() as u64) < n
                        }
                        cbor_event::LenSz::Indefinite => true,
                    } {
                        if raw.cbor_type()? == cbor_event::Type::Special {
                            assert_eq!(raw.special()?, cbor_event::Special::Break);
                            break;
                        }
                        let map_redeemer_key_to_redeemer_val_key = RedeemerKey::deserialize(raw)?;
                        let map_redeemer_key_to_redeemer_val_value = RedeemerVal::deserialize(raw)?;
                        if map_redeemer_key_to_redeemer_val_table
                            .insert(
                                map_redeemer_key_to_redeemer_val_key.clone(),
                                map_redeemer_key_to_redeemer_val_value,
                            )
                            .is_some()
                        {
                            return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from(
                                "some complicated/unsupported type",
                            )))
                            .into());
                        }
                    }
                    let (
                        map_redeemer_key_to_redeemer_val,
                        map_redeemer_key_to_redeemer_val_encoding,
                    ) = (
                        map_redeemer_key_to_redeemer_val_table,
                        map_redeemer_key_to_redeemer_val_encoding,
                    );
                    Ok(Self::MapRedeemerKeyToRedeemerVal {
                        map_redeemer_key_to_redeemer_val,
                        map_redeemer_key_to_redeemer_val_encoding,
                    })
                }
                _ => Err(DeserializeError::new(
                    "Redeemers",
                    DeserializeFailure::NoVariantMatched,
                )),
            }
        })()
        .map_err(|e| e.annotate("Redeemers"))
    }
}
