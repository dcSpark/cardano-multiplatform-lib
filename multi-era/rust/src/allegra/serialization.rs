// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use crate::shelley::ShelleyPoolRegistration;

use super::cbor_encodings::*;
use super::*;
use cbor_event;
use cbor_event::de::Deserializer;
use cbor_event::se::Serializer;
use cml_chain::address::RewardAccount;
use cml_chain::auxdata::ShelleyFormatAuxData;
use cml_chain::auxdata::ShelleyMAFormatAuxData;
use cml_core::error::*;
use cml_core::serialization::*;
use cml_crypto::RawBytesEncoding;
use std::io::{BufRead, Seek, SeekFrom, Write};

impl Serialize for AllegraAuxiliaryData {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            AllegraAuxiliaryData::Shelley(shelley) => {
                shelley.serialize(serializer, force_canonical)
            }
            AllegraAuxiliaryData::ShelleyMA(shelley_ma) => {
                shelley_ma.serialize(serializer, force_canonical)
            }
        }
    }
}

impl Deserialize for AllegraAuxiliaryData {
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
            Err(DeserializeError::new(
                "AllegraAuxiliaryData",
                DeserializeFailure::NoVariantMatchedWithCauses(errs),
            ))
        })()
        .map_err(|e| e.annotate("AllegraAuxiliaryData"))
    }
}

impl Serialize for AllegraBlock {
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

impl Deserialize for AllegraBlock {
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
                    transaction_bodies_arr.push(AllegraTransactionBody::deserialize(raw)?);
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
            Ok(AllegraBlock {
                header,
                transaction_bodies,
                transaction_witness_sets,
                auxiliary_data_set,
                encodings: Some(AllegraBlockEncoding {
                    len_encoding,
                    transaction_bodies_encoding,
                    transaction_witness_sets_encoding,
                    auxiliary_data_set_encoding,
                    auxiliary_data_set_key_encodings,
                }),
            })
        })().map_err(|e| e.annotate("AllegraBlock"))
    }
}

impl Serialize for AllegraCertificate {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            AllegraCertificate::StakeRegistration(stake_registration) => {
                stake_registration.serialize(serializer, force_canonical)
            }
            AllegraCertificate::StakeDeregistration(stake_deregistration) => {
                stake_deregistration.serialize(serializer, force_canonical)
            }
            AllegraCertificate::StakeDelegation(stake_delegation) => {
                stake_delegation.serialize(serializer, force_canonical)
            }
            AllegraCertificate::ShelleyPoolRegistration(shelley_pool_registration) => {
                shelley_pool_registration.serialize(serializer, force_canonical)
            }
            AllegraCertificate::PoolRetirement(pool_retirement) => {
                pool_retirement.serialize(serializer, force_canonical)
            }
            AllegraCertificate::GenesisKeyDelegation(genesis_key_delegation) => {
                genesis_key_delegation.serialize(serializer, force_canonical)
            }
            AllegraCertificate::MoveInstantaneousRewardsCert(move_instantaneous_rewards_cert) => {
                move_instantaneous_rewards_cert.serialize(serializer, force_canonical)
            }
        }
    }
}

impl Deserialize for AllegraCertificate {
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
                let ret = MoveInstantaneousRewardsCert::deserialize_as_embedded_group(
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
                Ok(move_instantaneous_rewards_cert) => {
                    return Ok(Self::MoveInstantaneousRewardsCert(
                        move_instantaneous_rewards_cert,
                    ))
                }
                Err(e) => {
                    errs.push(e.annotate("MoveInstantaneousRewardsCert"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            Err(DeserializeError::new(
                "AllegraCertificate",
                DeserializeFailure::NoVariantMatchedWithCauses(errs),
            ))
        })()
        .map_err(|e| e.annotate("AllegraCertificate"))
    }
}

impl Serialize for AllegraTransaction {
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

impl Deserialize for AllegraTransaction {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let len_encoding: LenEncoding = len.into();
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(3)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let body = AllegraTransactionBody::deserialize(raw)
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
            Ok(AllegraTransaction {
                body,
                witness_set,
                auxiliary_data,
                encodings: Some(AllegraTransactionEncoding { len_encoding }),
            })
        })()
        .map_err(|e| e.annotate("AllegraTransaction"))
    }
}

impl Serialize for AllegraTransactionBody {
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
                        }
            })
            .map(|encs| encs.orig_deser_order.clone())
            .unwrap_or_else(|| vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);
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

impl Deserialize for AllegraTransactionBody {
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
                                    outputs_arr.push(ShelleyTransactionOutput::deserialize(raw)?);
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
                            let (tmp_fee, tmp_fee_encoding) = raw.unsigned_integer_sz().map_err(Into::<DeserializeError>::into).map(|(x, enc)| (x, Some(enc))).map_err(|e: DeserializeError| e.annotate("fee"))?;
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
                                raw.unsigned_integer_sz().map_err(Into::<DeserializeError>::into).map(|(x, enc)| (x, Some(enc)))
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
                                raw.unsigned_integer_sz().map_err(Into::<DeserializeError>::into).map(|(x, enc)| (x, Some(enc)))
                            })().map_err(|e| e.annotate("validity_interval_start"))?;
                            validity_interval_start = Some(tmp_validity_interval_start);
                            validity_interval_start_encoding = tmp_validity_interval_start_encoding;
                            validity_interval_start_key_encoding = Some(key_enc);
                            orig_deser_order.push(8);
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
                encodings: Some(AllegraTransactionBodyEncoding {
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
                }),
            })
        })().map_err(|e| e.annotate("AllegraTransactionBody"))
    }
}

impl Serialize for AllegraTransactionWitnessSet {
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

impl Deserialize for AllegraTransactionWitnessSet {
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
                encodings: Some(AllegraTransactionWitnessSetEncoding {
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
        .map_err(|e| e.annotate("AllegraTransactionWitnessSet"))
    }
}

impl Serialize for MIRAction {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            MIRAction::ToStakeCredentials {
                to_stake_credentials,
                to_stake_credentials_encoding,
            } => {
                serializer.write_map_sz(
                    to_stake_credentials_encoding
                        .to_len_sz(to_stake_credentials.len() as u64, force_canonical),
                )?;
                let mut key_order = to_stake_credentials
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
                to_stake_credentials_encoding.end(serializer, force_canonical)
            }
            MIRAction::ToOtherPot {
                to_other_pot,
                to_other_pot_encoding,
            } => serializer.write_unsigned_integer_sz(
                *to_other_pot,
                fit_sz(*to_other_pot, *to_other_pot_encoding, force_canonical),
            ),
        }
    }
}

impl Deserialize for MIRAction {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            match raw.cbor_type()? {
                cbor_event::Type::Map => {
                    let mut to_stake_credentials_table = OrderedHashMap::new();
                    let to_stake_credentials_len = raw.map_sz()?;
                    let to_stake_credentials_encoding = to_stake_credentials_len.into();
                    while match to_stake_credentials_len {
                        cbor_event::LenSz::Len(n, _) => {
                            (to_stake_credentials_table.len() as u64) < n
                        }
                        cbor_event::LenSz::Indefinite => true,
                    } {
                        if raw.cbor_type()? == cbor_event::Type::Special {
                            assert_eq!(raw.special()?, cbor_event::Special::Break);
                            break;
                        }
                        let to_stake_credentials_key = StakeCredential::deserialize(raw)?;
                        let to_stake_credentials_value = DeltaCoin::deserialize(raw)?;
                        if to_stake_credentials_table
                            .insert(to_stake_credentials_key.clone(), to_stake_credentials_value)
                            .is_some()
                        {
                            return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from(
                                "some complicated/unsupported type",
                            )))
                            .into());
                        }
                    }
                    let (to_stake_credentials, to_stake_credentials_encoding) =
                        (to_stake_credentials_table, to_stake_credentials_encoding);
                    Ok(Self::ToStakeCredentials {
                        to_stake_credentials,
                        to_stake_credentials_encoding,
                    })
                }
                cbor_event::Type::UnsignedInteger => {
                    let (to_other_pot, to_other_pot_encoding) =
                        raw.unsigned_integer_sz().map(|(x, enc)| (x, Some(enc)))?;
                    Ok(Self::ToOtherPot {
                        to_other_pot,
                        to_other_pot_encoding,
                    })
                }
                _ => Err(DeserializeError::new(
                    "MIRAction",
                    DeserializeFailure::NoVariantMatched,
                )),
            }
        })()
        .map_err(|e| e.annotate("MIRAction"))
    }
}

impl Serialize for MoveInstantaneousReward {
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
        self.action.serialize(serializer, force_canonical)?;
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for MoveInstantaneousReward {
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
            let action =
                MIRAction::deserialize(raw).map_err(|e: DeserializeError| e.annotate("action"))?;
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(MoveInstantaneousReward {
                pot,
                action,
                encodings: Some(MoveInstantaneousRewardEncoding {
                    len_encoding,
                    pot_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("MoveInstantaneousReward"))
    }
}

impl Serialize for MoveInstantaneousRewardsCert {
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

impl SerializeEmbeddedGroup for MoveInstantaneousRewardsCert {
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
        self.move_instantaneous_reward
            .serialize(serializer, force_canonical)?;
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for MoveInstantaneousRewardsCert {
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

impl DeserializeEmbeddedGroup for MoveInstantaneousRewardsCert {
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
            let move_instantaneous_reward = MoveInstantaneousReward::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("move_instantaneous_reward"))?;
            Ok(MoveInstantaneousRewardsCert {
                move_instantaneous_reward,
                encodings: Some(MoveInstantaneousRewardsCertEncoding {
                    len_encoding,
                    tag_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("MoveInstantaneousRewardsCert"))
    }
}
