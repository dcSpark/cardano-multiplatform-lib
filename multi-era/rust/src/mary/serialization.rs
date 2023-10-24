// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use super::cbor_encodings::*;
use super::*;
use cbor_event;
use cbor_event::de::Deserializer;
use cbor_event::se::Serializer;
use cml_chain::address::RewardAccount;
use cml_chain::assets::AssetName;
use cml_chain::PolicyId;
use cml_core::error::*;
use cml_core::serialization::*;
use cml_crypto::RawBytesEncoding;
use std::io::{BufRead, Seek, Write};

impl Serialize for MaryBlock {
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
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for MaryBlock {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let len_encoding: LenEncoding = len.into();
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(4)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let header = ShelleyHeader::deserialize(raw).map_err(|e: DeserializeError| e.annotate("header"))?;
            let (transaction_bodies, transaction_bodies_encoding) = (|| -> Result<_, DeserializeError> {
                let mut transaction_bodies_arr = Vec::new();
                let len = raw.array_sz()?;
                let transaction_bodies_encoding = len.into();
                while match len { cbor_event::LenSz::Len(n, _) => (transaction_bodies_arr.len() as u64) < n, cbor_event::LenSz::Indefinite => true, } {
                    if raw.cbor_type()? == cbor_event::Type::Special {
                        assert_eq!(raw.special()?, cbor_event::Special::Break);
                        break;
                    }
                    transaction_bodies_arr.push(MaryTransactionBody::deserialize(raw)?);
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
                    transaction_witness_sets_arr.push(AllegraTransactionWitnessSet::deserialize(raw)?);
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
                    let auxiliary_data_set_value = AllegraAuxiliaryData::deserialize(raw)?;
                    if auxiliary_data_set_table.insert(auxiliary_data_set_key, auxiliary_data_set_value).is_some() {
                        return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from("some complicated/unsupported type"))).into());
                    }
                    auxiliary_data_set_key_encodings.insert(auxiliary_data_set_key, auxiliary_data_set_key_encoding);
                }
                Ok((auxiliary_data_set_table, auxiliary_data_set_encoding, auxiliary_data_set_key_encodings))
            })().map_err(|e| e.annotate("auxiliary_data_set"))?;
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(MaryBlock {
                header,
                transaction_bodies,
                transaction_witness_sets,
                auxiliary_data_set,
                encodings: Some(MaryBlockEncoding {
                    len_encoding,
                    transaction_bodies_encoding,
                    transaction_witness_sets_encoding,
                    auxiliary_data_set_encoding,
                    auxiliary_data_set_key_encodings,
                }),
            })
        })().map_err(|e| e.annotate("MaryBlock"))
    }
}

impl Serialize for MaryTransaction {
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

impl Deserialize for MaryTransaction {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let len_encoding: LenEncoding = len.into();
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(3)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let body = MaryTransactionBody::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("body"))?;
            let witness_set = AllegraTransactionWitnessSet::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("witness_set"))?;
            let auxiliary_data = (|| -> Result<_, DeserializeError> {
                Ok(match raw.cbor_type()? != cbor_event::Type::Special {
                    true => Some(AllegraAuxiliaryData::deserialize(raw)?),
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
            Ok(MaryTransaction {
                body,
                witness_set,
                auxiliary_data,
                encodings: Some(MaryTransactionEncoding { len_encoding }),
            })
        })()
        .map_err(|e| e.annotate("MaryTransaction"))
    }
}

impl Serialize for MaryTransactionBody {
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
                        }
            })
            .map(|encs| encs.orig_deser_order.clone())
            .unwrap_or_else(|| vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
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

impl Deserialize for MaryTransactionBody {
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
                                    outputs_arr.push(MaryTransactionOutput::deserialize(raw)?);
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
                                ShelleyUpdate::deserialize(raw)
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
                encodings: Some(MaryTransactionBodyEncoding {
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
                }),
            })
        })().map_err(|e| e.annotate("MaryTransactionBody"))
    }
}

impl Serialize for MaryTransactionOutput {
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
        self.amount.serialize(serializer, force_canonical)?;
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for MaryTransactionOutput {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let len_encoding: LenEncoding = len.into();
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let address =
                Address::deserialize(raw).map_err(|e: DeserializeError| e.annotate("address"))?;
            let amount =
                Value::deserialize(raw).map_err(|e: DeserializeError| e.annotate("amount"))?;
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(MaryTransactionOutput {
                address,
                amount,
                encodings: Some(MaryTransactionOutputEncoding { len_encoding }),
            })
        })()
        .map_err(|e| e.annotate("MaryTransactionOutput"))
    }
}
