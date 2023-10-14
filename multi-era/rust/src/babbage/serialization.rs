// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use super::cbor_encodings::*;
use super::*;
use cbor_event::de::Deserializer;
use cbor_event::se::Serializer;
use cml_chain::address::RewardAccount;
use cml_chain::assets::AssetName;
use cml_chain::PolicyId;
use cml_core::error::*;
use cml_core::serialization::*;
use cml_crypto::Ed25519KeyHash;
use cml_crypto::RawBytesEncoding;
use std::io::{BufRead, Seek, SeekFrom, Write};

impl Serialize for BabbageAuxiliaryData {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            BabbageAuxiliaryData::Shelley(shelley) => {
                shelley.serialize(serializer, force_canonical)
            }
            BabbageAuxiliaryData::ShelleyMA(shelley_m_a) => {
                shelley_m_a.serialize(serializer, force_canonical)
            }
            BabbageAuxiliaryData::Babbage(babbage) => {
                babbage.serialize(serializer, force_canonical)
            }
        }
    }
}

impl Deserialize for BabbageAuxiliaryData {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let initial_position = raw.as_mut_ref().stream_position().unwrap();
            let mut errs = Vec::new();
            let deser_variant: Result<_, DeserializeError> = ShelleyFormatAuxData::deserialize(raw);
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
                ShelleyMaFormatAuxData::deserialize(raw);
            match deser_variant {
                Ok(shelley_m_a) => return Ok(Self::ShelleyMA(shelley_m_a)),
                Err(e) => {
                    errs.push(e.annotate("ShelleyMA"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant: Result<_, DeserializeError> = BabbageFormatAuxData::deserialize(raw);
            match deser_variant {
                Ok(babbage) => return Ok(Self::Babbage(babbage)),
                Err(e) => {
                    errs.push(e.annotate("Babbage"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            Err(DeserializeError::new(
                "BabbageAuxiliaryData",
                DeserializeFailure::NoVariantMatchedWithCauses(errs),
            ))
        })()
        .map_err(|e| e.annotate("BabbageAuxiliaryData"))
    }
}

impl Serialize for BabbageBlock {
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
                .map(|encs| encs.auxiliary_data_set_encoding)
                .unwrap_or_default()
                .to_len_sz(self.auxiliary_data_set.len() as u64, force_canonical),
        )?;
        let mut key_order = self
            .auxiliary_data_set
            .iter()
            .map(|(k, v)| {
                let mut buf = cbor_event::se::Serializer::new_vec();
                let auxiliary_data_set_key_encoding = self
                    .encodings
                    .as_ref()
                    .and_then(|encs| encs.auxiliary_data_set_key_encodings.get(k))
                    .cloned()
                    .unwrap_or_default();
                buf.write_unsigned_integer_sz(
                    *k as u64,
                    fit_sz(*k as u64, auxiliary_data_set_key_encoding, force_canonical),
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
            .map(|encs| encs.auxiliary_data_set_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)?;
        serializer.write_array_sz(
            self.encodings
                .as_ref()
                .map(|encs| encs.invalid_transactions_encoding)
                .unwrap_or_default()
                .to_len_sz(self.invalid_transactions.len() as u64, force_canonical),
        )?;
        for (i, element) in self.invalid_transactions.iter().enumerate() {
            let invalid_transactions_elem_encoding = self
                .encodings
                .as_ref()
                .and_then(|encs| encs.invalid_transactions_elem_encodings.get(i))
                .cloned()
                .unwrap_or_default();
            serializer.write_unsigned_integer_sz(
                *element as u64,
                fit_sz(
                    *element as u64,
                    invalid_transactions_elem_encoding,
                    force_canonical,
                ),
            )?;
        }
        self.encodings
            .as_ref()
            .map(|encs| encs.invalid_transactions_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)?;
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for BabbageBlock {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let len_encoding: LenEncoding = len.into();
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(5)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let header = Header::deserialize(raw).map_err(|e: DeserializeError| e.annotate("header"))?;
            let (transaction_bodies, transaction_bodies_encoding) = (|| -> Result<_, DeserializeError> {
                let mut transaction_bodies_arr = Vec::new();
                let len = raw.array_sz()?;
                let transaction_bodies_encoding = len.into();
                while match len { cbor_event::LenSz::Len(n, _) => (transaction_bodies_arr.len() as u64) < n, cbor_event::LenSz::Indefinite => true, } {
                    if raw.cbor_type()? == cbor_event::Type::Special {
                        assert_eq!(raw.special()?, cbor_event::Special::Break);
                        break;
                    }
                    transaction_bodies_arr.push(BabbageTransactionBody::deserialize(raw)?);
                }
                Ok((transaction_bodies_arr, transaction_bodies_encoding))
            })().map_err(|e| e.annotate("transaction_bodies"))?;
            let (transaction_witness_sets, transaction_witness_sets_encoding) = (|| -> Result<_, DeserializeError> {
                let mut transaction_witness_sets_arr = Vec::new();
                let len = raw.array_sz()?;
                let transaction_witness_sets_encoding = len.into();
                while match len { cbor_event::LenSz::Len(n, _) => (transaction_witness_sets_arr.len() as u64) < n, cbor_event::LenSz::Indefinite => true, } {
                    if raw.cbor_type()? == cbor_event::Type::Special {
                        assert_eq!(raw.special()?, cbor_event::Special::Break);
                        break;
                    }
                    transaction_witness_sets_arr.push(BabbageTransactionWitnessSet::deserialize(raw)?);
                }
                Ok((transaction_witness_sets_arr, transaction_witness_sets_encoding))
            })().map_err(|e| e.annotate("transaction_witness_sets"))?;
            let (auxiliary_data_set, auxiliary_data_set_encoding, auxiliary_data_set_key_encodings) = (|| -> Result<_, DeserializeError> {
                let mut auxiliary_data_set_table = OrderedHashMap::new();
                let auxiliary_data_set_len = raw.map_sz()?;
                let auxiliary_data_set_encoding = auxiliary_data_set_len.into();
                let mut auxiliary_data_set_key_encodings = BTreeMap::new();
                while match auxiliary_data_set_len { cbor_event::LenSz::Len(n, _) => (auxiliary_data_set_table.len() as u64) < n, cbor_event::LenSz::Indefinite => true, } {
                    if raw.cbor_type()? == cbor_event::Type::Special {
                        assert_eq!(raw.special()?, cbor_event::Special::Break);
                        break;
                    }
                    let (auxiliary_data_set_key, auxiliary_data_set_key_encoding) = raw.unsigned_integer_sz().map(|(x, enc)| (x as u16, Some(enc)))?;
                    let auxiliary_data_set_value = BabbageAuxiliaryData::deserialize(raw)?;
                    if auxiliary_data_set_table.insert(auxiliary_data_set_key, auxiliary_data_set_value).is_some() {
                        return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from("some complicated/unsupported type"))).into());
                    }
                    auxiliary_data_set_key_encodings.insert(auxiliary_data_set_key, auxiliary_data_set_key_encoding);
                }
                Ok((auxiliary_data_set_table, auxiliary_data_set_encoding, auxiliary_data_set_key_encodings))
            })().map_err(|e| e.annotate("auxiliary_data_set"))?;
            let (invalid_transactions, invalid_transactions_encoding, invalid_transactions_elem_encodings) = (|| -> Result<_, DeserializeError> {
                let mut invalid_transactions_arr = Vec::new();
                let len = raw.array_sz()?;
                let invalid_transactions_encoding = len.into();
                let mut invalid_transactions_elem_encodings = Vec::new();
                while match len { cbor_event::LenSz::Len(n, _) => (invalid_transactions_arr.len() as u64) < n, cbor_event::LenSz::Indefinite => true, } {
                    if raw.cbor_type()? == cbor_event::Type::Special {
                        assert_eq!(raw.special()?, cbor_event::Special::Break);
                        break;
                    }
                    let (invalid_transactions_elem, invalid_transactions_elem_encoding) = raw.unsigned_integer_sz().map(|(x, enc)| (x as u16, Some(enc)))?;
                    invalid_transactions_arr.push(invalid_transactions_elem);
                    invalid_transactions_elem_encodings.push(invalid_transactions_elem_encoding);
                }
                Ok((invalid_transactions_arr, invalid_transactions_encoding, invalid_transactions_elem_encodings))
            })().map_err(|e| e.annotate("invalid_transactions"))?;
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(BabbageBlock {
                header,
                transaction_bodies,
                transaction_witness_sets,
                auxiliary_data_set,
                invalid_transactions,
                encodings: Some(BabbageBlockEncoding {
                    len_encoding,
                    transaction_bodies_encoding,
                    transaction_witness_sets_encoding,
                    auxiliary_data_set_encoding,
                    auxiliary_data_set_key_encodings,
                    invalid_transactions_encoding,
                    invalid_transactions_elem_encodings,
                }),
            })
        })().map_err(|e| e.annotate("BabbageBlock"))
    }
}

impl Serialize for BabbageCostModels {
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
                        }
            })
            .map(|encs| encs.orig_deser_order.clone())
            .unwrap_or_else(|| vec![0, 1]);
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

impl Deserialize for BabbageCostModels {
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
                encodings: Some(BabbageCostModelsEncoding {
                    len_encoding,
                    orig_deser_order,
                    plutus_v1_key_encoding,
                    plutus_v1_encoding,
                    plutus_v2_key_encoding,
                    plutus_v2_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("BabbageCostModels"))
    }
}

impl Serialize for BabbageFormatAuxData {
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
                        }
            })
            .map(|encs| encs.orig_deser_order.clone())
            .unwrap_or_else(|| vec![0, 1, 2, 3]);
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

impl Deserialize for BabbageFormatAuxData {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let (tag, tag_encoding) = raw.tag_sz()?;
        if tag != 259 {
            return Err(DeserializeError::new(
                "BabbageFormatAuxData",
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
                encodings: Some(BabbageFormatAuxDataEncoding {
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
                }),
            })
        })()
        .map_err(|e| e.annotate("BabbageFormatAuxData"))
    }
}

impl Serialize for BabbageFormatTxOut {
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

impl Deserialize for BabbageFormatTxOut {
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
                                            BabbageScript::deserialize(inner_de)?,
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
                encodings: Some(BabbageFormatTxOutEncoding {
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
        .map_err(|e| e.annotate("BabbageFormatTxOut"))
    }
}

impl Serialize for BabbageProtocolParamUpdate {
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
                    } + match &self.protocol_version {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.min_pool_cost {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.ada_per_utxo_byte {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.cost_models_for_script_languages {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.execution_costs {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.max_tx_ex_units {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.max_block_ex_units {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.max_value_size {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.collateral_percentage {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.max_collateral_inputs {
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
                        } + match &self.protocol_version {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.min_pool_cost {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.ada_per_utxo_byte {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.cost_models_for_script_languages {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.execution_costs {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.max_tx_ex_units {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.max_block_ex_units {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.max_value_size {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.collateral_percentage {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.max_collateral_inputs {
                            Some(_) => 1,
                            None => 0,
                        }
            })
            .map(|encs| encs.orig_deser_order.clone())
            .unwrap_or_else(|| {
                vec![
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                ]
            });
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
                13 => {
                    if let Some(field) = &self.min_pool_cost {
                        serializer.write_unsigned_integer_sz(
                            16u64,
                            fit_sz(
                                16u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.min_pool_cost_key_encoding)
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
                                    .map(|encs| encs.min_pool_cost_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                    }
                }
                14 => {
                    if let Some(field) = &self.ada_per_utxo_byte {
                        serializer.write_unsigned_integer_sz(
                            17u64,
                            fit_sz(
                                17u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.ada_per_utxo_byte_key_encoding)
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
                                    .map(|encs| encs.ada_per_utxo_byte_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                    }
                }
                15 => {
                    if let Some(field) = &self.cost_models_for_script_languages {
                        serializer.write_unsigned_integer_sz(
                            18u64,
                            fit_sz(
                                18u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.cost_models_for_script_languages_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        field.serialize(serializer, force_canonical)?;
                    }
                }
                16 => {
                    if let Some(field) = &self.execution_costs {
                        serializer.write_unsigned_integer_sz(
                            19u64,
                            fit_sz(
                                19u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.execution_costs_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        field.serialize(serializer, force_canonical)?;
                    }
                }
                17 => {
                    if let Some(field) = &self.max_tx_ex_units {
                        serializer.write_unsigned_integer_sz(
                            20u64,
                            fit_sz(
                                20u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.max_tx_ex_units_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        field.serialize(serializer, force_canonical)?;
                    }
                }
                18 => {
                    if let Some(field) = &self.max_block_ex_units {
                        serializer.write_unsigned_integer_sz(
                            21u64,
                            fit_sz(
                                21u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.max_block_ex_units_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        field.serialize(serializer, force_canonical)?;
                    }
                }
                19 => {
                    if let Some(field) = &self.max_value_size {
                        serializer.write_unsigned_integer_sz(
                            22u64,
                            fit_sz(
                                22u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.max_value_size_key_encoding)
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
                                    .map(|encs| encs.max_value_size_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                    }
                }
                20 => {
                    if let Some(field) = &self.collateral_percentage {
                        serializer.write_unsigned_integer_sz(
                            23u64,
                            fit_sz(
                                23u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.collateral_percentage_key_encoding)
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
                                    .map(|encs| encs.collateral_percentage_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                    }
                }
                21 => {
                    if let Some(field) = &self.max_collateral_inputs {
                        serializer.write_unsigned_integer_sz(
                            24u64,
                            fit_sz(
                                24u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.max_collateral_inputs_key_encoding)
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
                                    .map(|encs| encs.max_collateral_inputs_encoding)
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

impl Deserialize for BabbageProtocolParamUpdate {
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
            let mut protocol_version_key_encoding = None;
            let mut protocol_version = None;
            let mut min_pool_cost_encoding = None;
            let mut min_pool_cost_key_encoding = None;
            let mut min_pool_cost = None;
            let mut ada_per_utxo_byte_encoding = None;
            let mut ada_per_utxo_byte_key_encoding = None;
            let mut ada_per_utxo_byte = None;
            let mut cost_models_for_script_languages_key_encoding = None;
            let mut cost_models_for_script_languages = None;
            let mut execution_costs_key_encoding = None;
            let mut execution_costs = None;
            let mut max_tx_ex_units_key_encoding = None;
            let mut max_tx_ex_units = None;
            let mut max_block_ex_units_key_encoding = None;
            let mut max_block_ex_units = None;
            let mut max_value_size_encoding = None;
            let mut max_value_size_key_encoding = None;
            let mut max_value_size = None;
            let mut collateral_percentage_encoding = None;
            let mut collateral_percentage_key_encoding = None;
            let mut collateral_percentage = None;
            let mut max_collateral_inputs_encoding = None;
            let mut max_collateral_inputs_key_encoding = None;
            let mut max_collateral_inputs = None;
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
                                        .map(|(x, enc)| (x, Some(enc)))
                                        .map_err(Into::<DeserializeError>::into)
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
                                        .map(|(x, enc)| (x, Some(enc)))
                                        .map_err(Into::<DeserializeError>::into)
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
                                        .map(|(x, enc)| (x, Some(enc)))
                                        .map_err(Into::<DeserializeError>::into)
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
                                        .map(|(x, enc)| (x, Some(enc)))
                                        .map_err(Into::<DeserializeError>::into)
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
                                        .map(|(x, enc)| (x, Some(enc)))
                                        .map_err(Into::<DeserializeError>::into)
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
                                        .map(|(x, enc)| (x, Some(enc)))
                                        .map_err(Into::<DeserializeError>::into)
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
                                        .map(|(x, enc)| (x, Some(enc)))
                                        .map_err(Into::<DeserializeError>::into)
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
                                        .map(|(x, enc)| (x, Some(enc)))
                                        .map_err(Into::<DeserializeError>::into)
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
                                        .map(|(x, enc)| (x, Some(enc)))
                                        .map_err(Into::<DeserializeError>::into)
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
                            orig_deser_order.push(12);
                        }
                        (16, key_enc) => {
                            if min_pool_cost.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(16)).into());
                            }
                            let (tmp_min_pool_cost, tmp_min_pool_cost_encoding) =
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    raw.unsigned_integer_sz()
                                        .map(|(x, enc)| (x, Some(enc)))
                                        .map_err(Into::<DeserializeError>::into)
                                })()
                                .map_err(|e| e.annotate("min_pool_cost"))?;
                            min_pool_cost = Some(tmp_min_pool_cost);
                            min_pool_cost_encoding = tmp_min_pool_cost_encoding;
                            min_pool_cost_key_encoding = Some(key_enc);
                            orig_deser_order.push(13);
                        }
                        (17, key_enc) => {
                            if ada_per_utxo_byte.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(17)).into());
                            }
                            let (tmp_ada_per_utxo_byte, tmp_ada_per_utxo_byte_encoding) =
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    raw.unsigned_integer_sz()
                                        .map(|(x, enc)| (x, Some(enc)))
                                        .map_err(Into::<DeserializeError>::into)
                                })()
                                .map_err(|e| e.annotate("ada_per_utxo_byte"))?;
                            ada_per_utxo_byte = Some(tmp_ada_per_utxo_byte);
                            ada_per_utxo_byte_encoding = tmp_ada_per_utxo_byte_encoding;
                            ada_per_utxo_byte_key_encoding = Some(key_enc);
                            orig_deser_order.push(14);
                        }
                        (18, key_enc) => {
                            if cost_models_for_script_languages.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(18)).into());
                            }
                            let tmp_cost_models_for_script_languages =
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    BabbageCostModels::deserialize(raw)
                                })()
                                .map_err(|e| e.annotate("cost_models_for_script_languages"))?;
                            cost_models_for_script_languages =
                                Some(tmp_cost_models_for_script_languages);
                            cost_models_for_script_languages_key_encoding = Some(key_enc);
                            orig_deser_order.push(15);
                        }
                        (19, key_enc) => {
                            if execution_costs.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(19)).into());
                            }
                            let tmp_execution_costs = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                ExUnitPrices::deserialize(raw)
                            })()
                            .map_err(|e| e.annotate("execution_costs"))?;
                            execution_costs = Some(tmp_execution_costs);
                            execution_costs_key_encoding = Some(key_enc);
                            orig_deser_order.push(16);
                        }
                        (20, key_enc) => {
                            if max_tx_ex_units.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(20)).into());
                            }
                            let tmp_max_tx_ex_units = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                ExUnits::deserialize(raw)
                            })()
                            .map_err(|e| e.annotate("max_tx_ex_units"))?;
                            max_tx_ex_units = Some(tmp_max_tx_ex_units);
                            max_tx_ex_units_key_encoding = Some(key_enc);
                            orig_deser_order.push(17);
                        }
                        (21, key_enc) => {
                            if max_block_ex_units.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(21)).into());
                            }
                            let tmp_max_block_ex_units = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                ExUnits::deserialize(raw)
                            })()
                            .map_err(|e| e.annotate("max_block_ex_units"))?;
                            max_block_ex_units = Some(tmp_max_block_ex_units);
                            max_block_ex_units_key_encoding = Some(key_enc);
                            orig_deser_order.push(18);
                        }
                        (22, key_enc) => {
                            if max_value_size.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(22)).into());
                            }
                            let (tmp_max_value_size, tmp_max_value_size_encoding) =
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    raw.unsigned_integer_sz()
                                        .map(|(x, enc)| (x, Some(enc)))
                                        .map_err(Into::<DeserializeError>::into)
                                })()
                                .map_err(|e| e.annotate("max_value_size"))?;
                            max_value_size = Some(tmp_max_value_size);
                            max_value_size_encoding = tmp_max_value_size_encoding;
                            max_value_size_key_encoding = Some(key_enc);
                            orig_deser_order.push(19);
                        }
                        (23, key_enc) => {
                            if collateral_percentage.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(23)).into());
                            }
                            let (tmp_collateral_percentage, tmp_collateral_percentage_encoding) =
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    raw.unsigned_integer_sz()
                                        .map(|(x, enc)| (x, Some(enc)))
                                        .map_err(Into::<DeserializeError>::into)
                                })()
                                .map_err(|e| e.annotate("collateral_percentage"))?;
                            collateral_percentage = Some(tmp_collateral_percentage);
                            collateral_percentage_encoding = tmp_collateral_percentage_encoding;
                            collateral_percentage_key_encoding = Some(key_enc);
                            orig_deser_order.push(20);
                        }
                        (24, key_enc) => {
                            if max_collateral_inputs.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(24)).into());
                            }
                            let (tmp_max_collateral_inputs, tmp_max_collateral_inputs_encoding) =
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    raw.unsigned_integer_sz()
                                        .map(|(x, enc)| (x, Some(enc)))
                                        .map_err(Into::<DeserializeError>::into)
                                })()
                                .map_err(|e| e.annotate("max_collateral_inputs"))?;
                            max_collateral_inputs = Some(tmp_max_collateral_inputs);
                            max_collateral_inputs_encoding = tmp_max_collateral_inputs_encoding;
                            max_collateral_inputs_key_encoding = Some(key_enc);
                            orig_deser_order.push(21);
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
                protocol_version,
                min_pool_cost,
                ada_per_utxo_byte,
                cost_models_for_script_languages,
                execution_costs,
                max_tx_ex_units,
                max_block_ex_units,
                max_value_size,
                collateral_percentage,
                max_collateral_inputs,
                encodings: Some(BabbageProtocolParamUpdateEncoding {
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
                    protocol_version_key_encoding,
                    min_pool_cost_key_encoding,
                    min_pool_cost_encoding,
                    ada_per_utxo_byte_key_encoding,
                    ada_per_utxo_byte_encoding,
                    cost_models_for_script_languages_key_encoding,
                    execution_costs_key_encoding,
                    max_tx_ex_units_key_encoding,
                    max_block_ex_units_key_encoding,
                    max_value_size_key_encoding,
                    max_value_size_encoding,
                    collateral_percentage_key_encoding,
                    collateral_percentage_encoding,
                    max_collateral_inputs_key_encoding,
                    max_collateral_inputs_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("BabbageProtocolParamUpdate"))
    }
}

impl Serialize for BabbageScript {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            BabbageScript::Native {
                script,
                len_encoding,
                tag_encoding,
            } => {
                serializer.write_array_sz(len_encoding.to_len_sz(2, force_canonical))?;
                serializer.write_unsigned_integer_sz(
                    0u64,
                    fit_sz(0u64, *tag_encoding, force_canonical),
                )?;
                script.serialize(serializer, force_canonical)?;
                len_encoding.end(serializer, force_canonical)?;
                Ok(serializer)
            }
            BabbageScript::PlutusV1 {
                script,
                len_encoding,
                tag_encoding,
            } => {
                serializer.write_array_sz(len_encoding.to_len_sz(2, force_canonical))?;
                serializer.write_unsigned_integer_sz(
                    1u64,
                    fit_sz(1u64, *tag_encoding, force_canonical),
                )?;
                script.serialize(serializer, force_canonical)?;
                len_encoding.end(serializer, force_canonical)?;
                Ok(serializer)
            }
            BabbageScript::PlutusV2 {
                script,
                len_encoding,
                tag_encoding,
            } => {
                serializer.write_array_sz(len_encoding.to_len_sz(2, force_canonical))?;
                serializer.write_unsigned_integer_sz(
                    2u64,
                    fit_sz(2u64, *tag_encoding, force_canonical),
                )?;
                script.serialize(serializer, force_canonical)?;
                len_encoding.end(serializer, force_canonical)?;
                Ok(serializer)
            }
        }
    }
}

impl Deserialize for BabbageScript {
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
                let script = NativeScript::deserialize(raw)
                    .map_err(|e: DeserializeError| e.annotate("script"))?;
                match len {
                    cbor_event::LenSz::Len(_, _) => (),
                    cbor_event::LenSz::Indefinite => match raw.special()? {
                        cbor_event::Special::Break => (),
                        _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                    },
                }
                Ok(Self::Native {
                    script,
                    len_encoding,
                    tag_encoding,
                })
            })(raw)
            {
                Ok(variant) => return Ok(variant),
                Err(e) => {
                    errs.push(e.annotate("Native"));
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
                let script = PlutusV1Script::deserialize(raw)
                    .map_err(|e: DeserializeError| e.annotate("script"))?;
                match len {
                    cbor_event::LenSz::Len(_, _) => (),
                    cbor_event::LenSz::Indefinite => match raw.special()? {
                        cbor_event::Special::Break => (),
                        _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                    },
                }
                Ok(Self::PlutusV1 {
                    script,
                    len_encoding,
                    tag_encoding,
                })
            })(raw)
            {
                Ok(variant) => return Ok(variant),
                Err(e) => {
                    errs.push(e.annotate("PlutusV1"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
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
                let script = PlutusV2Script::deserialize(raw)
                    .map_err(|e: DeserializeError| e.annotate("script"))?;
                match len {
                    cbor_event::LenSz::Len(_, _) => (),
                    cbor_event::LenSz::Indefinite => match raw.special()? {
                        cbor_event::Special::Break => (),
                        _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                    },
                }
                Ok(Self::PlutusV2 {
                    script,
                    len_encoding,
                    tag_encoding,
                })
            })(raw)
            {
                Ok(variant) => return Ok(variant),
                Err(e) => {
                    errs.push(e.annotate("PlutusV2"));
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
                "BabbageScript",
                DeserializeFailure::NoVariantMatchedWithCauses(errs),
            ))
        })()
        .map_err(|e| e.annotate("BabbageScript"))
    }
}

impl Serialize for BabbageTransaction {
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

impl Deserialize for BabbageTransaction {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let len_encoding: LenEncoding = len.into();
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(4)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let body = BabbageTransactionBody::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("body"))?;
            let witness_set = BabbageTransactionWitnessSet::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("witness_set"))?;
            let is_valid = raw
                .bool()
                .map_err(Into::into)
                .map_err(|e: DeserializeError| e.annotate("is_valid"))?;
            let auxiliary_data = (|| -> Result<_, DeserializeError> {
                Ok(match raw.cbor_type()? != cbor_event::Type::Special {
                    true => Some(BabbageAuxiliaryData::deserialize(raw)?),
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
            Ok(BabbageTransaction {
                body,
                witness_set,
                is_valid,
                auxiliary_data,
                encodings: Some(BabbageTransactionEncoding { len_encoding }),
            })
        })()
        .map_err(|e| e.annotate("BabbageTransaction"))
    }
}

impl Serialize for BabbageTransactionBody {
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
                    } + match &self.update {
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
                        } + match &self.update {
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
                        }
            })
            .map(|encs| encs.orig_deser_order.clone())
            .unwrap_or_else(|| vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
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
                8 => {
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
                9 => {
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
                10 => {
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
                11 => {
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
                12 => {
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
                13 => {
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
                14 => {
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
                15 => {
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
                16 => {
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

impl Deserialize for BabbageTransactionBody {
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
            let mut update_key_encoding = None;
            let mut update = None;
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
                                    outputs_arr.push(BabbageTransactionOutput::deserialize(raw)?);
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
                                    certs_arr.push(AllegraCertificate::deserialize(raw)?);
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
                        (6, key_enc) =>  {
                            if update.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(6)).into());
                            }
                            let tmp_update = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                BabbageUpdate::deserialize(raw)
                            })().map_err(|e| e.annotate("update"))?;
                            update = Some(tmp_update);
                            update_key_encoding = Some(key_enc);
                            orig_deser_order.push(6);
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
                            orig_deser_order.push(7);
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
                            orig_deser_order.push(8);
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
                                    let (mint_key, mint_key_encoding) = raw.bytes_sz().map_err(Into::<DeserializeError>::into).and_then(|(bytes, enc)| PolicyId::from_raw_bytes(&bytes).map(|bytes| (bytes, StringEncoding::from(enc))).map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into()))?;
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
                            orig_deser_order.push(9);
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
                            orig_deser_order.push(10);
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
                            orig_deser_order.push(11);
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
                            orig_deser_order.push(12);
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
                            orig_deser_order.push(13);
                        },
                        (16, key_enc) =>  {
                            if collateral_return.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(16)).into());
                            }
                            let tmp_collateral_return = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                BabbageTransactionOutput::deserialize(raw)
                            })().map_err(|e| e.annotate("collateral_return"))?;
                            collateral_return = Some(tmp_collateral_return);
                            collateral_return_key_encoding = Some(key_enc);
                            orig_deser_order.push(14);
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
                            orig_deser_order.push(15);
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
                            orig_deser_order.push(16);
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
                update,
                auxiliary_data_hash,
                validity_interval_start,
                mint: mint.map(Into::into),
                script_data_hash,
                collateral_inputs,
                required_signers,
                network_id,
                collateral_return,
                total_collateral,
                reference_inputs,
                encodings: Some(BabbageTransactionBodyEncoding {
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
                }),
            })
        })().map_err(|e| e.annotate("BabbageTransactionBody"))
    }
}

impl Serialize for BabbageTransactionOutput {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            BabbageTransactionOutput::AlonzoFormatTxOut(alonzo_format_tx_out) => {
                alonzo_format_tx_out.serialize(serializer, force_canonical)
            }
            BabbageTransactionOutput::BabbageFormatTxOut(babbage_format_tx_out) => {
                babbage_format_tx_out.serialize(serializer, force_canonical)
            }
        }
    }
}

impl Deserialize for BabbageTransactionOutput {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let initial_position = raw.as_mut_ref().stream_position().unwrap();
            let mut errs = Vec::new();
            let deser_variant: Result<_, DeserializeError> = AlonzoFormatTxOut::deserialize(raw);
            match deser_variant {
                Ok(alonzo_format_tx_out) => {
                    return Ok(Self::AlonzoFormatTxOut(alonzo_format_tx_out))
                }
                Err(e) => {
                    errs.push(e.annotate("AlonzoFormatTxOut"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant: Result<_, DeserializeError> = BabbageFormatTxOut::deserialize(raw);
            match deser_variant {
                Ok(babbage_format_tx_out) => {
                    return Ok(Self::BabbageFormatTxOut(babbage_format_tx_out))
                }
                Err(e) => {
                    errs.push(e.annotate("BabbageFormatTxOut"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            Err(DeserializeError::new(
                "BabbageTransactionOutput",
                DeserializeFailure::NoVariantMatchedWithCauses(errs),
            ))
        })()
        .map_err(|e| e.annotate("BabbageTransactionOutput"))
    }
}

impl Serialize for BabbageTransactionWitnessSet {
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
                        }
            })
            .map(|encs| encs.orig_deser_order.clone())
            .unwrap_or_else(|| vec![0, 1, 2, 3, 4, 5, 6]);
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

impl Deserialize for BabbageTransactionWitnessSet {
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
                encodings: Some(BabbageTransactionWitnessSetEncoding {
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
                }),
            })
        })()
        .map_err(|e| e.annotate("BabbageTransactionWitnessSet"))
    }
}

impl Serialize for BabbageUpdate {
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
                .map(|encs| encs.updates_encoding)
                .unwrap_or_default()
                .to_len_sz(self.updates.len() as u64, force_canonical),
        )?;
        let mut key_order = self
            .updates
            .iter()
            .map(|(k, v)| {
                let mut buf = cbor_event::se::Serializer::new_vec();
                let updates_key_encoding = self
                    .encodings
                    .as_ref()
                    .and_then(|encs| encs.updates_key_encodings.get(k))
                    .cloned()
                    .unwrap_or_default();
                buf.write_bytes_sz(
                    k.to_raw_bytes(),
                    updates_key_encoding
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
            .map(|encs| encs.updates_encoding)
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

impl Deserialize for BabbageUpdate {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let len_encoding: LenEncoding = len.into();
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let (updates, updates_encoding, updates_key_encodings) =
                (|| -> Result<_, DeserializeError> {
                    let mut updates_table = OrderedHashMap::new();
                    let updates_len = raw.map_sz()?;
                    let updates_encoding = updates_len.into();
                    let mut updates_key_encodings = BTreeMap::new();
                    while match updates_len {
                        cbor_event::LenSz::Len(n, _) => (updates_table.len() as u64) < n,
                        cbor_event::LenSz::Indefinite => true,
                    } {
                        if raw.cbor_type()? == cbor_event::Type::Special {
                            assert_eq!(raw.special()?, cbor_event::Special::Break);
                            break;
                        }
                        let (updates_key, updates_key_encoding) = raw
                            .bytes_sz()
                            .map_err(Into::<DeserializeError>::into)
                            .and_then(|(bytes, enc)| {
                                GenesisHash::from_raw_bytes(&bytes)
                                    .map(|bytes| (bytes, StringEncoding::from(enc)))
                                    .map_err(|e| {
                                        DeserializeFailure::InvalidStructure(Box::new(e)).into()
                                    })
                            })?;
                        let updates_value = BabbageProtocolParamUpdate::deserialize(raw)?;
                        if updates_table
                            .insert(updates_key, updates_value)
                            .is_some()
                        {
                            return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from(
                                "some complicated/unsupported type",
                            )))
                            .into());
                        }
                        updates_key_encodings.insert(updates_key, updates_key_encoding);
                    }
                    Ok((updates_table, updates_encoding, updates_key_encodings))
                })()
                .map_err(|e| e.annotate("updates"))?;
            let (epoch, epoch_encoding) = raw
                .unsigned_integer_sz()
                .map(|(x, enc)| (x, Some(enc)))
                .map_err(Into::<DeserializeError>::into)
                .map_err(|e: DeserializeError| e.annotate("epoch"))?;
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(BabbageUpdate {
                updates,
                epoch,
                encodings: Some(BabbageUpdateEncoding {
                    len_encoding,
                    updates_encoding,
                    updates_key_encodings,
                    epoch_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("BabbageUpdate"))
    }
}
