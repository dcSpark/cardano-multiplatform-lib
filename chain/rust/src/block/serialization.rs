// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use super::cbor_encodings::*;
use super::*;
use cbor_event::de::Deserializer;
use cbor_event::se::Serializer;
use cml_core::error::*;
use cml_core::serialization::*;
use cml_crypto::RawBytesEncoding;
use std::io::{BufRead, Seek, Write};

impl Serialize for Block {
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

impl Deserialize for Block {
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
                    transaction_bodies_arr.push(TransactionBody::deserialize(raw)?);
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
                    transaction_witness_sets_arr.push(TransactionWitnessSet::deserialize(raw)?);
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
                    let auxiliary_data_set_value = AuxiliaryData::deserialize(raw)?;
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
            Ok(Block {
                header,
                transaction_bodies,
                transaction_witness_sets,
                auxiliary_data_set,
                invalid_transactions,
                encodings: Some(BlockEncoding {
                    len_encoding,
                    transaction_bodies_encoding,
                    transaction_witness_sets_encoding,
                    auxiliary_data_set_encoding,
                    auxiliary_data_set_key_encodings,
                    invalid_transactions_encoding,
                    invalid_transactions_elem_encodings,
                }),
            })
        })().map_err(|e| e.annotate("Block"))
    }
}

impl Serialize for Header {
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
        self.header_body.serialize(serializer, force_canonical)?;
        self.body_signature.serialize(serializer, force_canonical)?;
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for Header {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let len_encoding: LenEncoding = len.into();
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let header_body = HeaderBody::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("header_body"))?;
            let body_signature = KESSignature::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("body_signature"))?;
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(Header {
                header_body,
                body_signature,
                encodings: Some(HeaderEncoding { len_encoding }),
            })
        })()
        .map_err(|e| e.annotate("Header"))
    }
}

impl Serialize for HeaderBody {
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
        self.vrf_result.serialize(serializer, force_canonical)?;
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
            .serialize(serializer, force_canonical)?;
        self.protocol_version
            .serialize(serializer, force_canonical)?;
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for HeaderBody {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let len_encoding: LenEncoding = len.into();
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(10)?;
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
            let vrf_result = VRFCert::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("vrf_result"))?;
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
            let operational_cert = OperationalCert::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("operational_cert"))?;
            let protocol_version = ProtocolVersion::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("protocol_version"))?;
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(HeaderBody {
                block_number,
                slot,
                prev_hash,
                issuer_vkey,
                vrf_vkey,
                vrf_result,
                block_body_size,
                block_body_hash,
                operational_cert,
                protocol_version,
                encodings: Some(HeaderBodyEncoding {
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
        .map_err(|e| e.annotate("HeaderBody"))
    }
}

impl Serialize for OperationalCert {
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

impl SerializeEmbeddedGroup for OperationalCert {
    fn serialize_as_embedded_group<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_bytes_sz(
            self.hot_vkey.to_raw_bytes(),
            self.encodings
                .as_ref()
                .map(|encs| encs.hot_vkey_encoding.clone())
                .unwrap_or_default()
                .to_str_len_sz(self.hot_vkey.to_raw_bytes().len() as u64, force_canonical),
        )?;
        serializer.write_unsigned_integer_sz(
            self.sequence_number,
            fit_sz(
                self.sequence_number,
                self.encodings
                    .as_ref()
                    .map(|encs| encs.sequence_number_encoding)
                    .unwrap_or_default(),
                force_canonical,
            ),
        )?;
        serializer.write_unsigned_integer_sz(
            self.kes_period,
            fit_sz(
                self.kes_period,
                self.encodings
                    .as_ref()
                    .map(|encs| encs.kes_period_encoding)
                    .unwrap_or_default(),
                force_canonical,
            ),
        )?;
        serializer.write_bytes_sz(
            self.sigma.to_raw_bytes(),
            self.encodings
                .as_ref()
                .map(|encs| encs.sigma_encoding.clone())
                .unwrap_or_default()
                .to_str_len_sz(self.sigma.to_raw_bytes().len() as u64, force_canonical),
        )?;
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for OperationalCert {
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

impl DeserializeEmbeddedGroup for OperationalCert {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        _read_len: &mut CBORReadLen,
        len: cbor_event::LenSz,
    ) -> Result<Self, DeserializeError> {
        let len_encoding = len.into();
        (|| -> Result<_, DeserializeError> {
            let (hot_vkey, hot_vkey_encoding) = raw
                .bytes_sz()
                .map_err(Into::<DeserializeError>::into)
                .and_then(|(bytes, enc)| {
                    KESVkey::from_raw_bytes(&bytes)
                        .map(|bytes| (bytes, StringEncoding::from(enc)))
                        .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
                })
                .map_err(|e: DeserializeError| e.annotate("hot_vkey"))?;
            let (sequence_number, sequence_number_encoding) = raw
                .unsigned_integer_sz()
                .map_err(Into::<DeserializeError>::into)
                .map(|(x, enc)| (x, Some(enc)))
                .map_err(|e: DeserializeError| e.annotate("sequence_number"))?;
            let (kes_period, kes_period_encoding) = raw
                .unsigned_integer_sz()
                .map_err(Into::<DeserializeError>::into)
                .map(|(x, enc)| (x, Some(enc)))
                .map_err(|e: DeserializeError| e.annotate("kes_period"))?;
            let (sigma, sigma_encoding) = raw
                .bytes_sz()
                .map_err(Into::<DeserializeError>::into)
                .and_then(|(bytes, enc)| {
                    Ed25519Signature::from_raw_bytes(&bytes)
                        .map(|bytes| (bytes, StringEncoding::from(enc)))
                        .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
                })
                .map_err(|e: DeserializeError| e.annotate("sigma"))?;
            Ok(OperationalCert {
                hot_vkey,
                sequence_number,
                kes_period,
                sigma,
                encodings: Some(OperationalCertEncoding {
                    len_encoding,
                    hot_vkey_encoding,
                    sequence_number_encoding,
                    kes_period_encoding,
                    sigma_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("OperationalCert"))
    }
}

impl Serialize for ProtocolVersion {
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

impl SerializeEmbeddedGroup for ProtocolVersion {
    fn serialize_as_embedded_group<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer_sz(
            self.major,
            fit_sz(
                self.major,
                self.encodings
                    .as_ref()
                    .map(|encs| encs.major_encoding)
                    .unwrap_or_default(),
                force_canonical,
            ),
        )?;
        serializer.write_unsigned_integer_sz(
            self.minor,
            fit_sz(
                self.minor,
                self.encodings
                    .as_ref()
                    .map(|encs| encs.minor_encoding)
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

impl Deserialize for ProtocolVersion {
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

impl DeserializeEmbeddedGroup for ProtocolVersion {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        _read_len: &mut CBORReadLen,
        len: cbor_event::LenSz,
    ) -> Result<Self, DeserializeError> {
        let len_encoding = len.into();
        (|| -> Result<_, DeserializeError> {
            let (major, major_encoding) = raw
                .unsigned_integer_sz()
                .map_err(Into::<DeserializeError>::into)
                .map(|(x, enc)| (x, Some(enc)))
                .map_err(|e: DeserializeError| e.annotate("major"))?;
            let (minor, minor_encoding) = raw
                .unsigned_integer_sz()
                .map_err(Into::<DeserializeError>::into)
                .map(|(x, enc)| (x, Some(enc)))
                .map_err(|e: DeserializeError| e.annotate("minor"))?;
            Ok(ProtocolVersion {
                major,
                minor,
                encodings: Some(ProtocolVersionEncoding {
                    len_encoding,
                    major_encoding,
                    minor_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("ProtocolVersion"))
    }
}
