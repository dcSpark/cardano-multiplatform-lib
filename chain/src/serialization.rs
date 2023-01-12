use super::*;
use std::io::{Seek, SeekFrom};

pub use cardano_multiplatform_lib_core::{
    error::Key,
    serialization::{
        fit_sz,
        CBORReadLen,
        Deserialize,
        DeserializeEmbeddedGroup,
        Serialize,
        SerializeEmbeddedGroup,
    }
};

impl Serialize for AlonzoAuxData {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_tag_sz(259u64, fit_sz(259u64, self.encodings.as_ref().map(|encs| encs.tag_encoding).unwrap_or_default(), force_canonical))?;
        serializer.write_map_sz(self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().to_len_sz(match &self.key_0 { Some(_) => 1, None => 0 } + match &self.key_1 { Some(_) => 1, None => 0 } + match &self.key_2 { Some(_) => 1, None => 0 } + match &self.key_3 { Some(_) => 1, None => 0 }, force_canonical))?;
        let deser_order = self.encodings.as_ref().filter(|encs| !force_canonical && encs.orig_deser_order.len() == match &self.key_0 { Some(_) => 1, None => 0 } + match &self.key_1 { Some(_) => 1, None => 0 } + match &self.key_2 { Some(_) => 1, None => 0 } + match &self.key_3 { Some(_) => 1, None => 0 }).map(|encs| encs.orig_deser_order.clone()).unwrap_or_else(|| vec![0,1,2,3]);
        for field_index in deser_order {
            match field_index {
                0 => if let Some(field) = &self.key_0 {
                    serializer.write_unsigned_integer_sz(0u64, fit_sz(0u64, self.encodings.as_ref().map(|encs| encs.key_0_key_encoding.clone()).unwrap_or_default(), force_canonical))?;
                    serializer.write_map_sz(self.encodings.as_ref().map(|encs| encs.key_0_encoding.clone()).unwrap_or_default().to_len_sz(field.len() as u64, force_canonical))?;
                    let mut key_order = field.iter().map(|(k, v)| {
                        let mut buf = cbor_event::se::Serializer::new_vec();
                        let key_0_key_encoding = self.encodings.as_ref().and_then(|encs| encs.key_0_key_encodings.get(k)).map(|e| e.clone()).unwrap_or_else(|| None);
                        buf.write_unsigned_integer_sz(*k, fit_sz(*k, key_0_key_encoding, force_canonical))?;
                        Ok((buf.finalize(), k, v))
                    }).collect::<Result<Vec<(Vec<u8>, &_, &_)>, cbor_event::Error>>()?;
                    if force_canonical {
                        key_order.sort_by(|(lhs_bytes, _, _), (rhs_bytes, _, _)| {
                            match lhs_bytes.len().cmp(&rhs_bytes.len()) {
                                std::cmp::Ordering::Equal => lhs_bytes.cmp(&rhs_bytes),
                                diff_ord => diff_ord,
                            }
                        });
                    }
                    for (key_bytes, key, value) in key_order {
                        serializer.write_raw_bytes(&key_bytes)?;
                        value.serialize(serializer, force_canonical)?;
                    }
                    self.encodings.as_ref().map(|encs| encs.key_0_encoding.clone()).unwrap_or_default().end(serializer, force_canonical)?;
                }
                1 => if let Some(field) = &self.key_1 {
                    serializer.write_unsigned_integer_sz(1u64, fit_sz(1u64, self.encodings.as_ref().map(|encs| encs.key_1_key_encoding.clone()).unwrap_or_default(), force_canonical))?;
                    serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.key_1_encoding.clone()).unwrap_or_default().to_len_sz(field.len() as u64, force_canonical))?;
                    for element in field.iter() {
                        element.serialize(serializer, force_canonical)?;
                    }
                    self.encodings.as_ref().map(|encs| encs.key_1_encoding.clone()).unwrap_or_default().end(serializer, force_canonical)?;
                }
                2 => if let Some(field) = &self.key_2 {
                    serializer.write_unsigned_integer_sz(2u64, fit_sz(2u64, self.encodings.as_ref().map(|encs| encs.key_2_key_encoding.clone()).unwrap_or_default(), force_canonical))?;
                    serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.key_2_encoding.clone()).unwrap_or_default().to_len_sz(field.len() as u64, force_canonical))?;
                    for (i, element) in field.iter().enumerate() {
                        let key_2_elem_encoding = self.encodings.as_ref().and_then(|encs| encs.key_2_elem_encodings.get(i)).map(|e| e.clone()).unwrap_or_else(|| StringEncoding::default());
                        serializer.write_bytes_sz(&element, key_2_elem_encoding.to_str_len_sz(element.len() as u64, force_canonical))?;
                    }
                    self.encodings.as_ref().map(|encs| encs.key_2_encoding.clone()).unwrap_or_default().end(serializer, force_canonical)?;
                }
                3 => if let Some(field) = &self.key_3 {
                    serializer.write_unsigned_integer_sz(3u64, fit_sz(3u64, self.encodings.as_ref().map(|encs| encs.key_3_key_encoding.clone()).unwrap_or_default(), force_canonical))?;
                    serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.key_3_encoding.clone()).unwrap_or_default().to_len_sz(field.len() as u64, force_canonical))?;
                    for (i, element) in field.iter().enumerate() {
                        let key_3_elem_encoding = self.encodings.as_ref().and_then(|encs| encs.key_3_elem_encodings.get(i)).map(|e| e.clone()).unwrap_or_else(|| StringEncoding::default());
                        serializer.write_bytes_sz(&element, key_3_elem_encoding.to_str_len_sz(element.len() as u64, force_canonical))?;
                    }
                    self.encodings.as_ref().map(|encs| encs.key_3_encoding.clone()).unwrap_or_default().end(serializer, force_canonical)?;
                }
                _ => unreachable!()
            };
        }
        self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().end(serializer, force_canonical)
    }
}

impl Deserialize for AlonzoAuxData {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let (tag, tag_encoding) = raw.tag_sz()?;
            if tag != 259 {
                return Err(DeserializeError::new("AlonzoAuxData", DeserializeFailure::TagMismatch{ found: tag, expected: 259 }));
            }
            let len = raw.map_sz()?;
            let len_encoding: LenEncoding = len.into();
            let mut read_len = CBORReadLen::new(len);
            let mut orig_deser_order = Vec::new();
            let mut key_0_encoding = LenEncoding::default();
            let mut key_0_key_encodings = BTreeMap::new();
            let mut key_0_key_encoding = None;
            let mut key_0 = None;
            let mut key_1_encoding = LenEncoding::default();
            let mut key_1_key_encoding = None;
            let mut key_1 = None;
            let mut key_2_encoding = LenEncoding::default();
            let mut key_2_elem_encodings = Vec::new();
            let mut key_2_key_encoding = None;
            let mut key_2 = None;
            let mut key_3_encoding = LenEncoding::default();
            let mut key_3_elem_encodings = Vec::new();
            let mut key_3_key_encoding = None;
            let mut key_3 = None;
            let mut read = 0;
            while match len { cbor_event::LenSz::Len(n, _) => read < n as usize, cbor_event::LenSz::Indefinite => true, } {
                match raw.cbor_type()? {
                    CBORType::UnsignedInteger => match raw.unsigned_integer_sz()? {
                        (0, key_enc) =>  {
                            if key_0.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(0)).into());
                            }
                            let (tmp_key_0, tmp_key_0_encoding, tmp_key_0_key_encodings) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                let mut key_0_table = OrderedHashMap::new();
                                let key_0_len = raw.map_sz()?;
                                let key_0_encoding = key_0_len.into();
                                let mut key_0_key_encodings = BTreeMap::new();
                                while match key_0_len { cbor_event::LenSz::Len(n, _) => key_0_table.len() < n as usize, cbor_event::LenSz::Indefinite => true, } {
                                    if raw.cbor_type()? == CBORType::Special {
                                        assert_eq!(raw.special()?, CBORSpecial::Break);
                                        break;
                                    }
                                    let (key_0_key, key_0_key_encoding) = raw.unsigned_integer_sz().map(|(x, enc)| (x, Some(enc)))?;
                                    let key_0_value = TransactionMetadatum::deserialize(raw)?;
                                    if key_0_table.insert(key_0_key.clone(), key_0_value).is_some() {
                                        return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from("some complicated/unsupported type"))).into());
                                    }
                                    key_0_key_encodings.insert(key_0_key.clone(), key_0_key_encoding);
                                }
                                Ok((key_0_table, key_0_encoding, key_0_key_encodings))
                            })().map_err(|e| e.annotate("key_0"))?;
                            key_0 = Some(tmp_key_0);
                            key_0_encoding = tmp_key_0_encoding;
                            key_0_key_encodings = tmp_key_0_key_encodings;
                            key_0_key_encoding = Some(key_enc);
                            orig_deser_order.push(0);
                        },
                        (1, key_enc) =>  {
                            if key_1.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(1)).into());
                            }
                            let (tmp_key_1, tmp_key_1_encoding) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                let mut key_1_arr = Vec::new();
                                let len = raw.array_sz()?;
                                let key_1_encoding = len.into();
                                while match len { cbor_event::LenSz::Len(n, _) => key_1_arr.len() < n as usize, cbor_event::LenSz::Indefinite => true, } {
                                    if raw.cbor_type()? == CBORType::Special {
                                        assert_eq!(raw.special()?, CBORSpecial::Break);
                                        break;
                                    }
                                    key_1_arr.push(NativeScript::deserialize(raw)?);
                                }
                                Ok((key_1_arr, key_1_encoding))
                            })().map_err(|e| e.annotate("key_1"))?;
                            key_1 = Some(tmp_key_1);
                            key_1_encoding = tmp_key_1_encoding;
                            key_1_key_encoding = Some(key_enc);
                            orig_deser_order.push(1);
                        },
                        (2, key_enc) =>  {
                            if key_2.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(2)).into());
                            }
                            let (tmp_key_2, tmp_key_2_encoding, tmp_key_2_elem_encodings) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                let mut key_2_arr = Vec::new();
                                let len = raw.array_sz()?;
                                let key_2_encoding = len.into();
                                let mut key_2_elem_encodings = Vec::new();
                                while match len { cbor_event::LenSz::Len(n, _) => key_2_arr.len() < n as usize, cbor_event::LenSz::Indefinite => true, } {
                                    if raw.cbor_type()? == CBORType::Special {
                                        assert_eq!(raw.special()?, CBORSpecial::Break);
                                        break;
                                    }
                                    let (key_2_elem, key_2_elem_encoding) = raw.bytes_sz().map(|(bytes, enc)| (bytes, StringEncoding::from(enc)))?;
                                    key_2_arr.push(key_2_elem);
                                    key_2_elem_encodings.push(key_2_elem_encoding);
                                }
                                Ok((key_2_arr, key_2_encoding, key_2_elem_encodings))
                            })().map_err(|e| e.annotate("key_2"))?;
                            key_2 = Some(tmp_key_2);
                            key_2_encoding = tmp_key_2_encoding;
                            key_2_elem_encodings = tmp_key_2_elem_encodings;
                            key_2_key_encoding = Some(key_enc);
                            orig_deser_order.push(2);
                        },
                        (3, key_enc) =>  {
                            if key_3.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(3)).into());
                            }
                            let (tmp_key_3, tmp_key_3_encoding, tmp_key_3_elem_encodings) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                let mut key_3_arr = Vec::new();
                                let len = raw.array_sz()?;
                                let key_3_encoding = len.into();
                                let mut key_3_elem_encodings = Vec::new();
                                while match len { cbor_event::LenSz::Len(n, _) => key_3_arr.len() < n as usize, cbor_event::LenSz::Indefinite => true, } {
                                    if raw.cbor_type()? == CBORType::Special {
                                        assert_eq!(raw.special()?, CBORSpecial::Break);
                                        break;
                                    }
                                    let (key_3_elem, key_3_elem_encoding) = raw.bytes_sz().map(|(bytes, enc)| (bytes, StringEncoding::from(enc)))?;
                                    key_3_arr.push(key_3_elem);
                                    key_3_elem_encodings.push(key_3_elem_encoding);
                                }
                                Ok((key_3_arr, key_3_encoding, key_3_elem_encodings))
                            })().map_err(|e| e.annotate("key_3"))?;
                            key_3 = Some(tmp_key_3);
                            key_3_encoding = tmp_key_3_encoding;
                            key_3_elem_encodings = tmp_key_3_elem_encodings;
                            key_3_key_encoding = Some(key_enc);
                            orig_deser_order.push(3);
                        },
                        (unknown_key, _enc) => return Err(DeserializeFailure::UnknownKey(Key::Uint(unknown_key)).into()),
                    },
                    CBORType::Text => {
                        let (text_key, key_enc) = raw.text_sz()?;
                        match text_key.as_str() {
                            unknown_key => return Err(DeserializeFailure::UnknownKey(Key::Str(unknown_key.to_owned())).into()),
                        }
                    },
                    CBORType::Special => match len {
                        cbor_event::LenSz::Len(_, _) => return Err(DeserializeFailure::BreakInDefiniteLen.into()),
                        cbor_event::LenSz::Indefinite => match raw.special()? {
                            CBORSpecial::Break => break,
                            _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                        },
                    },
                    other_type => return Err(DeserializeFailure::UnexpectedKeyType(other_type).into()),
                }
                read += 1;
            }
            read_len.finish()?;
            Ok(Self {
                key_0,
                key_1,
                key_2,
                key_3,
                encodings: Some(AlonzoAuxDataEncoding {
                    tag_encoding: Some(tag_encoding),
                    len_encoding,
                    orig_deser_order,
                    key_0_key_encoding,
                    key_0_encoding,
                    key_0_key_encodings,
                    key_1_key_encoding,
                    key_1_encoding,
                    key_2_key_encoding,
                    key_2_encoding,
                    key_2_elem_encodings,
                    key_3_key_encoding,
                    key_3_encoding,
                    key_3_elem_encodings,
                }),
            })
        })().map_err(|e| e.annotate("AlonzoAuxData"))
    }
}

impl Serialize for AlonzoTxOut {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().to_len_sz(3, force_canonical))?;
        self.address.serialize(serializer, force_canonical)?;
        self.amount.serialize(serializer, force_canonical)?;
        self.datum_hash.serialize(serializer, force_canonical)?;
        self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().end(serializer, force_canonical)
    }
}

impl Deserialize for AlonzoTxOut {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array_sz()?;
            let len_encoding: LenEncoding = len.into();
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(3)?;
            let address = (|| -> Result<_, DeserializeError> {
                Ok(Address::deserialize(raw)?)
            })().map_err(|e| e.annotate("address"))?;
            let amount = (|| -> Result<_, DeserializeError> {
                Ok(Value::deserialize(raw)?)
            })().map_err(|e| e.annotate("amount"))?;
            let datum_hash = (|| -> Result<_, DeserializeError> {
                Ok(DataHash::deserialize(raw)?)
            })().map_err(|e| e.annotate("datum_hash"))?;
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(AlonzoTxOut {
                address,
                amount,
                datum_hash,
                encodings: Some(AlonzoTxOutEncoding {
                    len_encoding,
                }),
            })
        })().map_err(|e| e.annotate("AlonzoTxOut"))
    }
}

impl Serialize for AssetName {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_bytes_sz(&self.inner, self.encodings.as_ref().map(|encs| encs.inner_encoding.clone()).unwrap_or_default().to_str_len_sz(self.inner.len() as u64, force_canonical))
    }
}

impl Deserialize for AssetName {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let (inner, inner_encoding) = raw.bytes_sz().map(|(bytes, enc)| (bytes, StringEncoding::from(enc)))?;
        if inner.len() > 32 {
            return Err(DeserializeError::new("AssetName", DeserializeFailure::RangeCheck{ found: inner.len(), min: Some(0), max: Some(32) }));
        }
        Ok(Self {
            inner,
            encodings: Some(AssetNameEncoding {
                inner_encoding,
            }),
        })
    }
}

impl Serialize for AuxiliaryData {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            AuxiliaryData::ShelleyAuxData{ shelley_aux_data, shelley_aux_data_encoding, shelley_aux_data_key_encodings } => {
                serializer.write_map_sz(shelley_aux_data_encoding.to_len_sz(shelley_aux_data.len() as u64, force_canonical))?;
                let mut key_order = shelley_aux_data.iter().map(|(k, v)| {
                    let mut buf = cbor_event::se::Serializer::new_vec();
                    let shelley_aux_data_key_encoding = shelley_aux_data_key_encodings.get(k).map(|e| e.clone()).unwrap_or_else(|| None);
                    buf.write_unsigned_integer_sz(*k, fit_sz(*k, shelley_aux_data_key_encoding, force_canonical))?;
                    Ok((buf.finalize(), k, v))
                }).collect::<Result<Vec<(Vec<u8>, &_, &_)>, cbor_event::Error>>()?;
                if force_canonical {
                    key_order.sort_by(|(lhs_bytes, _, _), (rhs_bytes, _, _)| {
                        match lhs_bytes.len().cmp(&rhs_bytes.len()) {
                            std::cmp::Ordering::Equal => lhs_bytes.cmp(&rhs_bytes),
                            diff_ord => diff_ord,
                        }
                    });
                }
                for (key_bytes, key, value) in key_order {
                    serializer.write_raw_bytes(&key_bytes)?;
                    value.serialize(serializer, force_canonical)?;
                }
                shelley_aux_data_encoding.end(serializer, force_canonical)
            },
            AuxiliaryData::ShelleyMaAuxData(shelley_ma_aux_data) => {
                shelley_ma_aux_data.serialize(serializer, force_canonical)
            },
            AuxiliaryData::AlonzoAuxData(alonzo_aux_data) => {
                alonzo_aux_data.serialize(serializer, force_canonical)
            },
        }
    }
}

impl Deserialize for AuxiliaryData {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let initial_position = raw.as_mut_ref().seek(SeekFrom::Current(0)).unwrap();
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let mut shelley_aux_data_table = OrderedHashMap::new();
                let shelley_aux_data_len = raw.map_sz()?;
                let shelley_aux_data_encoding = shelley_aux_data_len.into();
                let mut shelley_aux_data_key_encodings = BTreeMap::new();
                while match shelley_aux_data_len { cbor_event::LenSz::Len(n, _) => shelley_aux_data_table.len() < n as usize, cbor_event::LenSz::Indefinite => true, } {
                    if raw.cbor_type()? == CBORType::Special {
                        assert_eq!(raw.special()?, CBORSpecial::Break);
                        break;
                    }
                    let (shelley_aux_data_key, shelley_aux_data_key_encoding) = raw.unsigned_integer_sz().map(|(x, enc)| (x, Some(enc)))?;
                    let shelley_aux_data_value = TransactionMetadatum::deserialize(raw)?;
                    if shelley_aux_data_table.insert(shelley_aux_data_key.clone(), shelley_aux_data_value).is_some() {
                        return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from("some complicated/unsupported type"))).into());
                    }
                    shelley_aux_data_key_encodings.insert(shelley_aux_data_key.clone(), shelley_aux_data_key_encoding);
                }
                Ok((shelley_aux_data_table, shelley_aux_data_encoding, shelley_aux_data_key_encodings))
            })(raw)
            {
                Ok((shelley_aux_data, shelley_aux_data_encoding, shelley_aux_data_key_encodings)) => return Ok(Self::ShelleyAuxData {
                    shelley_aux_data,
                    shelley_aux_data_encoding,
                    shelley_aux_data_key_encodings,
                }),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(ShelleyMaAuxData::deserialize(raw)?)
            })(raw)
            {
                Ok(shelley_ma_aux_data) => return Ok(Self::ShelleyMaAuxData(shelley_ma_aux_data)),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(AlonzoAuxData::deserialize(raw)?)
            })(raw)
            {
                Ok(alonzo_aux_data) => return Ok(Self::AlonzoAuxData(alonzo_aux_data)),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            Err(DeserializeError::new("AuxiliaryData", DeserializeFailure::NoVariantMatched.into()))
        })().map_err(|e| e.annotate("AuxiliaryData"))
    }
}

impl Serialize for BabbageTxOut {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_map_sz(self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().to_len_sz(2 + match &self.key_2 { Some(_) => 1, None => 0 } + match &self.key_3 { Some(_) => 1, None => 0 }, force_canonical))?;
        let deser_order = self.encodings.as_ref().filter(|encs| !force_canonical && encs.orig_deser_order.len() == 2 + match &self.key_2 { Some(_) => 1, None => 0 } + match &self.key_3 { Some(_) => 1, None => 0 }).map(|encs| encs.orig_deser_order.clone()).unwrap_or_else(|| vec![0,1,2,3]);
        for field_index in deser_order {
            match field_index {
                0 => {
                    serializer.write_unsigned_integer_sz(0u64, fit_sz(0u64, self.encodings.as_ref().map(|encs| encs.key_0_key_encoding.clone()).unwrap_or_default(), force_canonical))?;
                    self.key_0.serialize(serializer, force_canonical)?;
                }
                1 => {
                    serializer.write_unsigned_integer_sz(1u64, fit_sz(1u64, self.encodings.as_ref().map(|encs| encs.key_1_key_encoding.clone()).unwrap_or_default(), force_canonical))?;
                    self.key_1.serialize(serializer, force_canonical)?;
                }
                2 => if let Some(field) = &self.key_2 {
                    serializer.write_unsigned_integer_sz(2u64, fit_sz(2u64, self.encodings.as_ref().map(|encs| encs.key_2_key_encoding.clone()).unwrap_or_default(), force_canonical))?;
                    field.serialize(serializer, force_canonical)?;
                }
                3 => if let Some(field) = &self.key_3 {
                    serializer.write_unsigned_integer_sz(3u64, fit_sz(3u64, self.encodings.as_ref().map(|encs| encs.key_3_key_encoding.clone()).unwrap_or_default(), force_canonical))?;
                    serializer.write_tag_sz(24u64, fit_sz(24u64, self.encodings.as_ref().map(|encs| encs.key_3_tag_encoding.clone()).unwrap_or_default(), force_canonical))?;
                    let mut key_3_inner_se = Serializer::new_vec();
                    key_3_inner_se.write_bytes_sz(&field, self.encodings.as_ref().map(|encs| encs.key_3_encoding.clone()).unwrap_or_default().to_str_len_sz(field.len() as u64, force_canonical))?;
                    let key_3_bytes = key_3_inner_se.finalize();
                    serializer.write_bytes_sz(&key_3_bytes, self.encodings.as_ref().map(|encs| encs.key_3_bytes_encoding.clone()).unwrap_or_default().to_str_len_sz(key_3_bytes.len() as u64, force_canonical))?;
                }
                _ => unreachable!()
            };
        }
        self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().end(serializer, force_canonical)
    }
}

impl Deserialize for BabbageTxOut {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.map_sz()?;
            let len_encoding: LenEncoding = len.into();
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(2)?;
            let mut orig_deser_order = Vec::new();
            let mut key_0_key_encoding = None;
            let mut key_0 = None;
            let mut key_1_key_encoding = None;
            let mut key_1 = None;
            let mut key_2_key_encoding = None;
            let mut key_2 = None;
            let mut key_3_tag_encoding = None;
            let mut key_3_bytes_encoding = StringEncoding::default();
            let mut key_3_encoding = StringEncoding::default();
            let mut key_3_key_encoding = None;
            let mut key_3 = None;
            let mut read = 0;
            while match len { cbor_event::LenSz::Len(n, _) => read < n as usize, cbor_event::LenSz::Indefinite => true, } {
                match raw.cbor_type()? {
                    CBORType::UnsignedInteger => match raw.unsigned_integer_sz()? {
                        (0, key_enc) =>  {
                            if key_0.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(0)).into());
                            }
                            let tmp_key_0 = (|| -> Result<_, DeserializeError> {
                                Ok(Address::deserialize(raw)?)
                            })().map_err(|e| e.annotate("key_0"))?;
                            key_0 = Some(tmp_key_0);
                            key_0_key_encoding = Some(key_enc);
                            orig_deser_order.push(0);
                        },
                        (1, key_enc) =>  {
                            if key_1.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(1)).into());
                            }
                            let tmp_key_1 = (|| -> Result<_, DeserializeError> {
                                Ok(Value::deserialize(raw)?)
                            })().map_err(|e| e.annotate("key_1"))?;
                            key_1 = Some(tmp_key_1);
                            key_1_key_encoding = Some(key_enc);
                            orig_deser_order.push(1);
                        },
                        (2, key_enc) =>  {
                            if key_2.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(2)).into());
                            }
                            let tmp_key_2 = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                Ok(DatumOption::deserialize(raw)?)
                            })().map_err(|e| e.annotate("key_2"))?;
                            key_2 = Some(tmp_key_2);
                            key_2_key_encoding = Some(key_enc);
                            orig_deser_order.push(2);
                        },
                        (3, key_enc) =>  {
                            if key_3.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(3)).into());
                            }
                            let (tmp_key_3, tmp_key_3_tag_encoding, tmp_key_3_bytes_encoding, tmp_key_3_encoding) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                Ok(match raw.tag_sz()? {
                                    (24, tag_enc) => {
                                        let (key_3_bytes, key_3_bytes_encoding) = raw.bytes_sz()?;
                                        let mut inner_de = &mut Deserializer::from(std::io::Cursor::new(key_3_bytes));
                                        inner_de.bytes_sz().map(|(bytes, enc)| (bytes, Some(tag_enc), StringEncoding::from(key_3_bytes_encoding), StringEncoding::from(enc)))?
                                    },
                                    (tag, _enc) => return Err(DeserializeFailure::TagMismatch{ found: tag, expected: 24 }.into()),
                                })
                            })().map_err(|e| e.annotate("key_3"))?;
                            key_3 = Some(tmp_key_3);
                            key_3_tag_encoding = tmp_key_3_tag_encoding;
                            key_3_bytes_encoding = tmp_key_3_bytes_encoding;
                            key_3_encoding = tmp_key_3_encoding;
                            key_3_key_encoding = Some(key_enc);
                            orig_deser_order.push(3);
                        },
                        (unknown_key, _enc) => return Err(DeserializeFailure::UnknownKey(Key::Uint(unknown_key)).into()),
                    },
                    CBORType::Text => {
                        let (text_key, key_enc) = raw.text_sz()?;
                        match text_key.as_str() {
                            unknown_key => return Err(DeserializeFailure::UnknownKey(Key::Str(unknown_key.to_owned())).into()),
                        }
                    },
                    CBORType::Special => match len {
                        cbor_event::LenSz::Len(_, _) => return Err(DeserializeFailure::BreakInDefiniteLen.into()),
                        cbor_event::LenSz::Indefinite => match raw.special()? {
                            CBORSpecial::Break => break,
                            _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                        },
                    },
                    other_type => return Err(DeserializeFailure::UnexpectedKeyType(other_type).into()),
                }
                read += 1;
            }
            let key_0 = match key_0 {
                Some(x) => x,
                None => return Err(DeserializeFailure::MandatoryFieldMissing(Key::Uint(0)).into()),
            };
            let key_1 = match key_1 {
                Some(x) => x,
                None => return Err(DeserializeFailure::MandatoryFieldMissing(Key::Uint(1)).into()),
            };
            read_len.finish()?;
            Ok(Self {
                key_0,
                key_1,
                key_2,
                key_3,
                encodings: Some(BabbageTxOutEncoding {
                    len_encoding,
                    orig_deser_order,
                    key_0_key_encoding,
                    key_1_key_encoding,
                    key_2_key_encoding,
                    key_3_key_encoding,
                    key_3_tag_encoding,
                    key_3_bytes_encoding,
                    key_3_encoding,
                }),
            })
        })().map_err(|e| e.annotate("BabbageTxOut"))
    }
}

impl Serialize for BigInt {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().to_len_sz(2, force_canonical))?;
        serializer.write_unsigned_integer_sz(0u64, fit_sz(0u64, self.encodings.as_ref().map(|encs| encs.index_0_encoding.clone()).unwrap_or_default(), force_canonical))?;
        serializer.write_unsigned_integer_sz(1u64, fit_sz(1u64, self.encodings.as_ref().map(|encs| encs.index_1_encoding.clone()).unwrap_or_default(), force_canonical))?;
        self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().end(serializer, force_canonical)
    }
}

impl Deserialize for BigInt {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array_sz()?;
            let len_encoding: LenEncoding = len.into();
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(2)?;
            let index_0_encoding = (|| -> Result<_, DeserializeError> {
                let (index_0_value, index_0_encoding) = raw.unsigned_integer_sz()?;
                if index_0_value != 0 {
                    return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(index_0_value), expected: Key::Uint(0) }.into());
                }
                Ok(Some(index_0_encoding))
            })().map_err(|e| e.annotate("index_0"))?;
            let index_1_encoding = (|| -> Result<_, DeserializeError> {
                let (index_1_value, index_1_encoding) = raw.unsigned_integer_sz()?;
                if index_1_value != 1 {
                    return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(index_1_value), expected: Key::Uint(1) }.into());
                }
                Ok(Some(index_1_encoding))
            })().map_err(|e| e.annotate("index_1"))?;
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(BigInt {
                encodings: Some(BigIntEncoding {
                    len_encoding,
                    index_0_encoding,
                    index_1_encoding,
                }),
            })
        })().map_err(|e| e.annotate("BigInt"))
    }
}

impl Serialize for Block {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().to_len_sz(5, force_canonical))?;
        self.header.serialize(serializer, force_canonical)?;
        serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.transaction_bodies_encoding.clone()).unwrap_or_default().to_len_sz(self.transaction_bodies.len() as u64, force_canonical))?;
        for element in self.transaction_bodies.iter() {
            element.serialize(serializer, force_canonical)?;
        }
        self.encodings.as_ref().map(|encs| encs.transaction_bodies_encoding.clone()).unwrap_or_default().end(serializer, force_canonical)?;
        serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.transaction_witness_sets_encoding.clone()).unwrap_or_default().to_len_sz(self.transaction_witness_sets.len() as u64, force_canonical))?;
        for element in self.transaction_witness_sets.iter() {
            element.serialize(serializer, force_canonical)?;
        }
        self.encodings.as_ref().map(|encs| encs.transaction_witness_sets_encoding.clone()).unwrap_or_default().end(serializer, force_canonical)?;
        serializer.write_map_sz(self.encodings.as_ref().map(|encs| encs.auxiliary_data_set_encoding.clone()).unwrap_or_default().to_len_sz(self.auxiliary_data_set.len() as u64, force_canonical))?;
        let mut key_order = self.auxiliary_data_set.iter().map(|(k, v)| {
            let mut buf = cbor_event::se::Serializer::new_vec();
            let auxiliary_data_set_key_encoding = self.encodings.as_ref().and_then(|encs| encs.auxiliary_data_set_key_encodings.get(k)).map(|e| e.clone()).unwrap_or_else(|| None);
            buf.write_unsigned_integer_sz(*k as u64, fit_sz(*k as u64, auxiliary_data_set_key_encoding, force_canonical))?;
            Ok((buf.finalize(), k, v))
        }).collect::<Result<Vec<(Vec<u8>, &_, &_)>, cbor_event::Error>>()?;
        if force_canonical {
            key_order.sort_by(|(lhs_bytes, _, _), (rhs_bytes, _, _)| {
                match lhs_bytes.len().cmp(&rhs_bytes.len()) {
                    std::cmp::Ordering::Equal => lhs_bytes.cmp(&rhs_bytes),
                    diff_ord => diff_ord,
                }
            });
        }
        for (key_bytes, key, value) in key_order {
            serializer.write_raw_bytes(&key_bytes)?;
            value.serialize(serializer, force_canonical)?;
        }
        self.encodings.as_ref().map(|encs| encs.auxiliary_data_set_encoding.clone()).unwrap_or_default().end(serializer, force_canonical)?;
        serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.invalid_transactions_encoding.clone()).unwrap_or_default().to_len_sz(self.invalid_transactions.len() as u64, force_canonical))?;
        for (i, element) in self.invalid_transactions.iter().enumerate() {
            let invalid_transactions_elem_encoding = self.encodings.as_ref().and_then(|encs| encs.invalid_transactions_elem_encodings.get(i)).map(|e| e.clone()).unwrap_or_else(|| None);
            serializer.write_unsigned_integer_sz(*element as u64, fit_sz(*element as u64, invalid_transactions_elem_encoding, force_canonical))?;
        }
        self.encodings.as_ref().map(|encs| encs.invalid_transactions_encoding.clone()).unwrap_or_default().end(serializer, force_canonical)?;
        self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().end(serializer, force_canonical)
    }
}

impl Deserialize for Block {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array_sz()?;
            let len_encoding: LenEncoding = len.into();
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(5)?;
            let header = (|| -> Result<_, DeserializeError> {
                Ok(Header::deserialize(raw)?)
            })().map_err(|e| e.annotate("header"))?;
            let (transaction_bodies, transaction_bodies_encoding) = (|| -> Result<_, DeserializeError> {
                let mut transaction_bodies_arr = Vec::new();
                let len = raw.array_sz()?;
                let transaction_bodies_encoding = len.into();
                while match len { cbor_event::LenSz::Len(n, _) => transaction_bodies_arr.len() < n as usize, cbor_event::LenSz::Indefinite => true, } {
                    if raw.cbor_type()? == CBORType::Special {
                        assert_eq!(raw.special()?, CBORSpecial::Break);
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
                while match len { cbor_event::LenSz::Len(n, _) => transaction_witness_sets_arr.len() < n as usize, cbor_event::LenSz::Indefinite => true, } {
                    if raw.cbor_type()? == CBORType::Special {
                        assert_eq!(raw.special()?, CBORSpecial::Break);
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
                while match auxiliary_data_set_len { cbor_event::LenSz::Len(n, _) => auxiliary_data_set_table.len() < n as usize, cbor_event::LenSz::Indefinite => true, } {
                    if raw.cbor_type()? == CBORType::Special {
                        assert_eq!(raw.special()?, CBORSpecial::Break);
                        break;
                    }
                    let (auxiliary_data_set_key, auxiliary_data_set_key_encoding) = raw.unsigned_integer_sz().map(|(x, enc)| (x as u16, Some(enc)))?;
                    let auxiliary_data_set_value = AuxiliaryData::deserialize(raw)?;
                    if auxiliary_data_set_table.insert(auxiliary_data_set_key.clone(), auxiliary_data_set_value).is_some() {
                        return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from("some complicated/unsupported type"))).into());
                    }
                    auxiliary_data_set_key_encodings.insert(auxiliary_data_set_key.clone(), auxiliary_data_set_key_encoding);
                }
                Ok((auxiliary_data_set_table, auxiliary_data_set_encoding, auxiliary_data_set_key_encodings))
            })().map_err(|e| e.annotate("auxiliary_data_set"))?;
            let (invalid_transactions, invalid_transactions_encoding, invalid_transactions_elem_encodings) = (|| -> Result<_, DeserializeError> {
                let mut invalid_transactions_arr = Vec::new();
                let len = raw.array_sz()?;
                let invalid_transactions_encoding = len.into();
                let mut invalid_transactions_elem_encodings = Vec::new();
                while match len { cbor_event::LenSz::Len(n, _) => invalid_transactions_arr.len() < n as usize, cbor_event::LenSz::Indefinite => true, } {
                    if raw.cbor_type()? == CBORType::Special {
                        assert_eq!(raw.special()?, CBORSpecial::Break);
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
                    CBORSpecial::Break => (),
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

impl Serialize for Certificate {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            Certificate::StakeRegistration(stake_registration) => stake_registration.serialize(serializer, force_canonical),
            Certificate::StakeDeregistration(stake_deregistration) => stake_deregistration.serialize(serializer, force_canonical),
            Certificate::StakeDelegation(stake_delegation) => stake_delegation.serialize(serializer, force_canonical),
            Certificate::PoolRegistration(pool_registration) => pool_registration.serialize(serializer, force_canonical),
            Certificate::PoolRetirement(pool_retirement) => pool_retirement.serialize(serializer, force_canonical),
            Certificate::GenesisKeyDelegation(genesis_key_delegation) => genesis_key_delegation.serialize(serializer, force_canonical),
            Certificate::MoveInstantaneousRewardsCert(move_instantaneous_rewards_cert) => move_instantaneous_rewards_cert.serialize(serializer, force_canonical),
        }
    }
}

impl Deserialize for Certificate {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array_sz()?;
            let outer_len_encoding: LenEncoding = len.into();
            let mut read_len = CBORReadLen::new(len);
            let initial_position = raw.as_mut_ref().seek(SeekFrom::Current(0)).unwrap();
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(StakeRegistration::deserialize_as_embedded_group(raw, &mut read_len, len)?)
            })(raw)
            {
                Ok(stake_registration) => return Ok(Self::StakeRegistration(stake_registration)),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(StakeDeregistration::deserialize_as_embedded_group(raw, &mut read_len, len)?)
            })(raw)
            {
                Ok(stake_deregistration) => return Ok(Self::StakeDeregistration(stake_deregistration)),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(StakeDelegation::deserialize_as_embedded_group(raw, &mut read_len, len)?)
            })(raw)
            {
                Ok(stake_delegation) => return Ok(Self::StakeDelegation(stake_delegation)),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(PoolRegistration::deserialize_as_embedded_group(raw, &mut read_len, len)?)
            })(raw)
            {
                Ok(pool_registration) => return Ok(Self::PoolRegistration(pool_registration)),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(PoolRetirement::deserialize_as_embedded_group(raw, &mut read_len, len)?)
            })(raw)
            {
                Ok(pool_retirement) => return Ok(Self::PoolRetirement(pool_retirement)),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(GenesisKeyDelegation::deserialize_as_embedded_group(raw, &mut read_len, len)?)
            })(raw)
            {
                Ok(genesis_key_delegation) => return Ok(Self::GenesisKeyDelegation(genesis_key_delegation)),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(MoveInstantaneousRewardsCert::deserialize_as_embedded_group(raw, &mut read_len, len)?)
            })(raw)
            {
                Ok(move_instantaneous_rewards_cert) => return Ok(Self::MoveInstantaneousRewardsCert(move_instantaneous_rewards_cert)),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Err(DeserializeError::new("Certificate", DeserializeFailure::NoVariantMatched.into()))
        })().map_err(|e| e.annotate("Certificate"))
    }
}

impl Serialize for ConstrPlutusData {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_tag_sz(102u64, fit_sz(102u64, self.encodings.as_ref().map(|encs| encs.tag_encoding).unwrap_or_default(), force_canonical))?;
        serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().to_len_sz(2, force_canonical))?;
        serializer.write_unsigned_integer_sz(self.index_0, fit_sz(self.index_0, self.encodings.as_ref().map(|encs| encs.index_0_encoding.clone()).unwrap_or_default(), force_canonical))?;
        serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.plutus_datas_encoding.clone()).unwrap_or_default().to_len_sz(self.plutus_datas.len() as u64, force_canonical))?;
        for element in self.plutus_datas.iter() {
            element.serialize(serializer, force_canonical)?;
        }
        self.encodings.as_ref().map(|encs| encs.plutus_datas_encoding.clone()).unwrap_or_default().end(serializer, force_canonical)?;
        self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().end(serializer, force_canonical)
    }
}

impl Deserialize for ConstrPlutusData {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let (tag, tag_encoding) = raw.tag_sz()?;
            if tag != 102 {
                return Err(DeserializeError::new("ConstrPlutusData", DeserializeFailure::TagMismatch{ found: tag, expected: 102 }));
            }
            let len = raw.array_sz()?;
            let len_encoding: LenEncoding = len.into();
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(2)?;
            let (index_0, index_0_encoding) = (|| -> Result<_, DeserializeError> {
                Ok(raw.unsigned_integer_sz().map(|(x, enc)| (x, Some(enc)))?)
            })().map_err(|e| e.annotate("index_0"))?;
            let (plutus_datas, plutus_datas_encoding) = (|| -> Result<_, DeserializeError> {
                let mut plutus_datas_arr = Vec::new();
                let len = raw.array_sz()?;
                let plutus_datas_encoding = len.into();
                while match len { cbor_event::LenSz::Len(n, _) => plutus_datas_arr.len() < n as usize, cbor_event::LenSz::Indefinite => true, } {
                    if raw.cbor_type()? == CBORType::Special {
                        assert_eq!(raw.special()?, CBORSpecial::Break);
                        break;
                    }
                    plutus_datas_arr.push(PlutusData::deserialize(raw)?);
                }
                Ok((plutus_datas_arr, plutus_datas_encoding))
            })().map_err(|e| e.annotate("plutus_datas"))?;
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(ConstrPlutusData {
                index_0,
                plutus_datas,
                encodings: Some(ConstrPlutusDataEncoding {
                    len_encoding,
                    tag_encoding: Some(tag_encoding),
                    index_0_encoding,
                    plutus_datas_encoding,
                }),
            })
        })().map_err(|e| e.annotate("ConstrPlutusData"))
    }
}

impl Serialize for Costmdls {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_map_sz(self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().to_len_sz(match &self.key_0 { Some(_) => 1, None => 0 } + match &self.key_1 { Some(_) => 1, None => 0 }, force_canonical))?;
        let deser_order = self.encodings.as_ref().filter(|encs| !force_canonical && encs.orig_deser_order.len() == match &self.key_0 { Some(_) => 1, None => 0 } + match &self.key_1 { Some(_) => 1, None => 0 }).map(|encs| encs.orig_deser_order.clone()).unwrap_or_else(|| vec![0,1]);
        for field_index in deser_order {
            match field_index {
                0 => if let Some(field) = &self.key_0 {
                    serializer.write_unsigned_integer_sz(0u64, fit_sz(0u64, self.encodings.as_ref().map(|encs| encs.key_0_key_encoding.clone()).unwrap_or_default(), force_canonical))?;
                    serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.key_0_encoding.clone()).unwrap_or_default().to_len_sz(field.len() as u64, force_canonical))?;
                    for element in field.iter() {
                        element.serialize(serializer, force_canonical)?;
                    }
                    self.encodings.as_ref().map(|encs| encs.key_0_encoding.clone()).unwrap_or_default().end(serializer, force_canonical)?;
                }
                1 => if let Some(field) = &self.key_1 {
                    serializer.write_unsigned_integer_sz(1u64, fit_sz(1u64, self.encodings.as_ref().map(|encs| encs.key_1_key_encoding.clone()).unwrap_or_default(), force_canonical))?;
                    serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.key_1_encoding.clone()).unwrap_or_default().to_len_sz(field.len() as u64, force_canonical))?;
                    for element in field.iter() {
                        element.serialize(serializer, force_canonical)?;
                    }
                    self.encodings.as_ref().map(|encs| encs.key_1_encoding.clone()).unwrap_or_default().end(serializer, force_canonical)?;
                }
                _ => unreachable!()
            };
        }
        self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().end(serializer, force_canonical)
    }
}

impl Deserialize for Costmdls {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.map_sz()?;
            let len_encoding: LenEncoding = len.into();
            let mut read_len = CBORReadLen::new(len);
            let mut orig_deser_order = Vec::new();
            let mut key_0_encoding = LenEncoding::default();
            let mut key_0_key_encoding = None;
            let mut key_0 = None;
            let mut key_1_encoding = LenEncoding::default();
            let mut key_1_key_encoding = None;
            let mut key_1 = None;
            let mut read = 0;
            while match len { cbor_event::LenSz::Len(n, _) => read < n as usize, cbor_event::LenSz::Indefinite => true, } {
                match raw.cbor_type()? {
                    CBORType::UnsignedInteger => match raw.unsigned_integer_sz()? {
                        (0, key_enc) =>  {
                            if key_0.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(0)).into());
                            }
                            let (tmp_key_0, tmp_key_0_encoding) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                let mut key_0_arr = Vec::new();
                                let len = raw.array_sz()?;
                                let key_0_encoding = len.into();
                                while match len { cbor_event::LenSz::Len(n, _) => key_0_arr.len() < n as usize, cbor_event::LenSz::Indefinite => true, } {
                                    if raw.cbor_type()? == CBORType::Special {
                                        assert_eq!(raw.special()?, CBORSpecial::Break);
                                        break;
                                    }
                                    key_0_arr.push(Int::deserialize(raw)?);
                                }
                                Ok((key_0_arr, key_0_encoding))
                            })().map_err(|e| e.annotate("key_0"))?;
                            key_0 = Some(tmp_key_0);
                            key_0_encoding = tmp_key_0_encoding;
                            key_0_key_encoding = Some(key_enc);
                            orig_deser_order.push(0);
                        },
                        (1, key_enc) =>  {
                            if key_1.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(1)).into());
                            }
                            let (tmp_key_1, tmp_key_1_encoding) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                let mut key_1_arr = Vec::new();
                                let len = raw.array_sz()?;
                                let key_1_encoding = len.into();
                                while match len { cbor_event::LenSz::Len(n, _) => key_1_arr.len() < n as usize, cbor_event::LenSz::Indefinite => true, } {
                                    if raw.cbor_type()? == CBORType::Special {
                                        assert_eq!(raw.special()?, CBORSpecial::Break);
                                        break;
                                    }
                                    key_1_arr.push(Int::deserialize(raw)?);
                                }
                                Ok((key_1_arr, key_1_encoding))
                            })().map_err(|e| e.annotate("key_1"))?;
                            key_1 = Some(tmp_key_1);
                            key_1_encoding = tmp_key_1_encoding;
                            key_1_key_encoding = Some(key_enc);
                            orig_deser_order.push(1);
                        },
                        (unknown_key, _enc) => return Err(DeserializeFailure::UnknownKey(Key::Uint(unknown_key)).into()),
                    },
                    CBORType::Text => {
                        let (text_key, key_enc) = raw.text_sz()?;
                        match text_key.as_str() {
                            unknown_key => return Err(DeserializeFailure::UnknownKey(Key::Str(unknown_key.to_owned())).into()),
                        }
                    },
                    CBORType::Special => match len {
                        cbor_event::LenSz::Len(_, _) => return Err(DeserializeFailure::BreakInDefiniteLen.into()),
                        cbor_event::LenSz::Indefinite => match raw.special()? {
                            CBORSpecial::Break => break,
                            _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                        },
                    },
                    other_type => return Err(DeserializeFailure::UnexpectedKeyType(other_type).into()),
                }
                read += 1;
            }
            read_len.finish()?;
            Ok(Self {
                key_0,
                key_1,
                encodings: Some(CostmdlsEncoding {
                    len_encoding,
                    orig_deser_order,
                    key_0_key_encoding,
                    key_0_encoding,
                    key_1_key_encoding,
                    key_1_encoding,
                }),
            })
        })().map_err(|e| e.annotate("Costmdls"))
    }
}

impl Serialize for DatumOption {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            DatumOption::DatumOption0(datum_option0) => datum_option0.serialize(serializer, force_canonical),
            DatumOption::DatumOption1(datum_option1) => datum_option1.serialize(serializer, force_canonical),
        }
    }
}

impl Deserialize for DatumOption {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array_sz()?;
            let outer_len_encoding: LenEncoding = len.into();
            let mut read_len = CBORReadLen::new(len);
            let initial_position = raw.as_mut_ref().seek(SeekFrom::Current(0)).unwrap();
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(DatumOption0::deserialize_as_embedded_group(raw, &mut read_len, len)?)
            })(raw)
            {
                Ok(datum_option0) => return Ok(Self::DatumOption0(datum_option0)),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(DatumOption1::deserialize_as_embedded_group(raw, &mut read_len, len)?)
            })(raw)
            {
                Ok(datum_option1) => return Ok(Self::DatumOption1(datum_option1)),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Err(DeserializeError::new("DatumOption", DeserializeFailure::NoVariantMatched.into()))
        })().map_err(|e| e.annotate("DatumOption"))
    }
}

impl Serialize for DatumOption0 {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().to_len_sz(2, force_canonical))?;
        self.serialize_as_embedded_group(serializer, force_canonical)
    }
}

impl SerializeEmbeddedGroup for DatumOption0 {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer_sz(0u64, fit_sz(0u64, self.encodings.as_ref().map(|encs| encs.index_0_encoding.clone()).unwrap_or_default(), force_canonical))?;
        self.hash32.serialize(serializer, force_canonical)?;
        self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().end(serializer, force_canonical)
    }
}

impl Deserialize for DatumOption0 {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array_sz()?;
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(2)?;
            let ret = Self::deserialize_as_embedded_group(raw, &mut read_len, len);
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("DatumOption0"))
    }
}

impl DeserializeEmbeddedGroup for DatumOption0 {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, read_len: &mut CBORReadLen, len: cbor_event::LenSz) -> Result<Self, DeserializeError> {
        let len_encoding = len.into();
        let index_0_encoding = (|| -> Result<_, DeserializeError> {
            let (index_0_value, index_0_encoding) = raw.unsigned_integer_sz()?;
            if index_0_value != 0 {
                return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(index_0_value), expected: Key::Uint(0) }.into());
            }
            Ok(Some(index_0_encoding))
        })().map_err(|e| e.annotate("index_0"))?;
        let hash32 = (|| -> Result<_, DeserializeError> {
            Ok(DataHash::deserialize(raw)?)
        })().map_err(|e| e.annotate("hash32"))?;
        Ok(DatumOption0 {
            hash32,
            encodings: Some(DatumOption0Encoding {
                len_encoding,
                index_0_encoding,
            }),
        })
    }
}

impl Serialize for DatumOption1 {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().to_len_sz(2, force_canonical))?;
        self.serialize_as_embedded_group(serializer, force_canonical)
    }
}

impl SerializeEmbeddedGroup for DatumOption1 {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer_sz(1u64, fit_sz(1u64, self.encodings.as_ref().map(|encs| encs.index_0_encoding.clone()).unwrap_or_default(), force_canonical))?;
        serializer.write_tag_sz(24u64, fit_sz(24u64, self.encodings.as_ref().map(|encs| encs.data_tag_encoding.clone()).unwrap_or_default(), force_canonical))?;
        let mut data_inner_se = Serializer::new_vec();
        data_inner_se.write_bytes_sz(&self.data, self.encodings.as_ref().map(|encs| encs.data_encoding.clone()).unwrap_or_default().to_str_len_sz(self.data.len() as u64, force_canonical))?;
        let data_bytes = data_inner_se.finalize();
        serializer.write_bytes_sz(&data_bytes, self.encodings.as_ref().map(|encs| encs.data_bytes_encoding.clone()).unwrap_or_default().to_str_len_sz(data_bytes.len() as u64, force_canonical))?;
        self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().end(serializer, force_canonical)
    }
}

impl Deserialize for DatumOption1 {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array_sz()?;
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(2)?;
            let ret = Self::deserialize_as_embedded_group(raw, &mut read_len, len);
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("DatumOption1"))
    }
}

impl DeserializeEmbeddedGroup for DatumOption1 {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, read_len: &mut CBORReadLen, len: cbor_event::LenSz) -> Result<Self, DeserializeError> {
        let len_encoding = len.into();
        let index_0_encoding = (|| -> Result<_, DeserializeError> {
            let (index_0_value, index_0_encoding) = raw.unsigned_integer_sz()?;
            if index_0_value != 1 {
                return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(index_0_value), expected: Key::Uint(1) }.into());
            }
            Ok(Some(index_0_encoding))
        })().map_err(|e| e.annotate("index_0"))?;
        let (data, data_tag_encoding, data_bytes_encoding, data_encoding) = (|| -> Result<_, DeserializeError> {
            Ok(match raw.tag_sz()? {
                (24, tag_enc) => {
                    let (data_bytes, data_bytes_encoding) = raw.bytes_sz()?;
                    let mut inner_de = &mut Deserializer::from(std::io::Cursor::new(data_bytes));
                    inner_de.bytes_sz().map(|(bytes, enc)| (bytes, Some(tag_enc), StringEncoding::from(data_bytes_encoding), StringEncoding::from(enc)))?
                },
                (tag, _enc) => return Err(DeserializeFailure::TagMismatch{ found: tag, expected: 24 }.into()),
            })
        })().map_err(|e| e.annotate("data"))?;
        Ok(DatumOption1 {
            data,
            encodings: Some(DatumOption1Encoding {
                len_encoding,
                index_0_encoding,
                data_tag_encoding,
                data_bytes_encoding,
                data_encoding,
            }),
        })
    }
}

impl Serialize for DnsName {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_text_sz(&self.inner, self.encodings.as_ref().map(|encs| encs.inner_encoding.clone()).unwrap_or_default().to_str_len_sz(self.inner.len() as u64, force_canonical))
    }
}

impl Deserialize for DnsName {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let (inner, inner_encoding) = raw.text_sz().map(|(s, enc)| (s, StringEncoding::from(enc)))?;
        if inner.len() > 64 {
            return Err(DeserializeError::new("DnsName", DeserializeFailure::RangeCheck{ found: inner.len(), min: Some(0), max: Some(64) }));
        }
        Ok(Self {
            inner,
            encodings: Some(DnsNameEncoding {
                inner_encoding,
            }),
        })
    }
}

impl Serialize for ExUnitPrices {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().to_len_sz(2, force_canonical))?;
        self.mem_price.serialize(serializer, force_canonical)?;
        self.step_price.serialize(serializer, force_canonical)?;
        self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().end(serializer, force_canonical)
    }
}

impl Deserialize for ExUnitPrices {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array_sz()?;
            let len_encoding: LenEncoding = len.into();
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(2)?;
            let mem_price = (|| -> Result<_, DeserializeError> {
                Ok(PositiveInterval::deserialize(raw)?)
            })().map_err(|e| e.annotate("mem_price"))?;
            let step_price = (|| -> Result<_, DeserializeError> {
                Ok(PositiveInterval::deserialize(raw)?)
            })().map_err(|e| e.annotate("step_price"))?;
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(ExUnitPrices {
                mem_price,
                step_price,
                encodings: Some(ExUnitPricesEncoding {
                    len_encoding,
                }),
            })
        })().map_err(|e| e.annotate("ExUnitPrices"))
    }
}

impl Serialize for ExUnits {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().to_len_sz(2, force_canonical))?;
        serializer.write_unsigned_integer_sz(self.mem, fit_sz(self.mem, self.encodings.as_ref().map(|encs| encs.mem_encoding.clone()).unwrap_or_default(), force_canonical))?;
        serializer.write_unsigned_integer_sz(self.steps, fit_sz(self.steps, self.encodings.as_ref().map(|encs| encs.steps_encoding.clone()).unwrap_or_default(), force_canonical))?;
        self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().end(serializer, force_canonical)
    }
}

impl Deserialize for ExUnits {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array_sz()?;
            let len_encoding: LenEncoding = len.into();
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(2)?;
            let (mem, mem_encoding) = (|| -> Result<_, DeserializeError> {
                Ok(raw.unsigned_integer_sz().map(|(x, enc)| (x, Some(enc)))?)
            })().map_err(|e| e.annotate("mem"))?;
            let (steps, steps_encoding) = (|| -> Result<_, DeserializeError> {
                Ok(raw.unsigned_integer_sz().map(|(x, enc)| (x, Some(enc)))?)
            })().map_err(|e| e.annotate("steps"))?;
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
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
        })().map_err(|e| e.annotate("ExUnits"))
    }
}

impl Serialize for GenesisKeyDelegation {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().to_len_sz(4, force_canonical))?;
        self.serialize_as_embedded_group(serializer, force_canonical)
    }
}

impl SerializeEmbeddedGroup for GenesisKeyDelegation {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer_sz(5u64, fit_sz(5u64, self.encodings.as_ref().map(|encs| encs.index_0_encoding.clone()).unwrap_or_default(), force_canonical))?;
        self.genesishash.serialize(serializer, force_canonical)?;
        self.genesis_delegate_hash.serialize(serializer, force_canonical)?;
        self.vrf_keyhash.serialize(serializer, force_canonical)?;
        self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().end(serializer, force_canonical)
    }
}

impl Deserialize for GenesisKeyDelegation {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array_sz()?;
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(4)?;
            let ret = Self::deserialize_as_embedded_group(raw, &mut read_len, len);
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("GenesisKeyDelegation"))
    }
}

impl DeserializeEmbeddedGroup for GenesisKeyDelegation {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, read_len: &mut CBORReadLen, len: cbor_event::LenSz) -> Result<Self, DeserializeError> {
        let len_encoding = len.into();
        let index_0_encoding = (|| -> Result<_, DeserializeError> {
            let (index_0_value, index_0_encoding) = raw.unsigned_integer_sz()?;
            if index_0_value != 5 {
                return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(index_0_value), expected: Key::Uint(5) }.into());
            }
            Ok(Some(index_0_encoding))
        })().map_err(|e| e.annotate("index_0"))?;
        let genesishash = (|| -> Result<_, DeserializeError> {
            Ok(GenesisHash::deserialize(raw)?)
        })().map_err(|e| e.annotate("genesishash"))?;
        let genesis_delegate_hash = (|| -> Result<_, DeserializeError> {
            Ok(GenesisDelegateHash::deserialize(raw)?)
        })().map_err(|e| e.annotate("genesis_delegate_hash"))?;
        let vrf_keyhash = (|| -> Result<_, DeserializeError> {
            Ok(VRFKeyHash::deserialize(raw)?)
        })().map_err(|e| e.annotate("vrf_keyhash"))?;
        Ok(GenesisKeyDelegation {
            genesishash,
            genesis_delegate_hash,
            vrf_keyhash,
            encodings: Some(GenesisKeyDelegationEncoding {
                len_encoding,
                index_0_encoding,
            }),
        })
    }
}

impl Serialize for Header {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().to_len_sz(2, force_canonical))?;
        self.header_body.serialize(serializer, force_canonical)?;
        self.body_signature.serialize(serializer, force_canonical)?;
        self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().end(serializer, force_canonical)
    }
}

impl Deserialize for Header {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array_sz()?;
            let len_encoding: LenEncoding = len.into();
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(2)?;
            let header_body = (|| -> Result<_, DeserializeError> {
                Ok(HeaderBody::deserialize(raw)?)
            })().map_err(|e| e.annotate("header_body"))?;
            let body_signature = (|| -> Result<_, DeserializeError> {
                Ok(KesSignature::deserialize(raw)?)
            })().map_err(|e| e.annotate("body_signature"))?;
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(Header {
                header_body,
                body_signature,
                encodings: Some(HeaderEncoding {
                    len_encoding,
                }),
            })
        })().map_err(|e| e.annotate("Header"))
    }
}

impl Serialize for HeaderBody {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().to_len_sz(14, force_canonical))?;
        serializer.write_unsigned_integer_sz(self.block_number, fit_sz(self.block_number, self.encodings.as_ref().map(|encs| encs.block_number_encoding.clone()).unwrap_or_default(), force_canonical))?;
        serializer.write_unsigned_integer_sz(self.slot, fit_sz(self.slot, self.encodings.as_ref().map(|encs| encs.slot_encoding.clone()).unwrap_or_default(), force_canonical))?;
        match &self.prev_hash {
            Some(x) => {
                x.serialize(serializer, force_canonical)
            },
            None => serializer.write_special(CBORSpecial::Null),
        }?;
        self.issuer_vkey.serialize(serializer, force_canonical)?;
        self.vrf_vkey.serialize(serializer, force_canonical)?;
        self.vrf_result.serialize(serializer, force_canonical)?;
        serializer.write_unsigned_integer_sz(self.block_body_size, fit_sz(self.block_body_size, self.encodings.as_ref().map(|encs| encs.block_body_size_encoding.clone()).unwrap_or_default(), force_canonical))?;
        self.block_body_hash.serialize(serializer, force_canonical)?;
        self.operational_cert.serialize_as_embedded_group(serializer, force_canonical)?;
        self.protocol_version.serialize_as_embedded_group(serializer, force_canonical)?;
        self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().end(serializer, force_canonical)
    }
}

impl Deserialize for HeaderBody {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array_sz()?;
            let len_encoding: LenEncoding = len.into();
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(14)?;
            let (block_number, block_number_encoding) = (|| -> Result<_, DeserializeError> {
                Ok(raw.unsigned_integer_sz().map(|(x, enc)| (x, Some(enc)))?)
            })().map_err(|e| e.annotate("block_number"))?;
            let (slot, slot_encoding) = (|| -> Result<_, DeserializeError> {
                Ok(raw.unsigned_integer_sz().map(|(x, enc)| (x, Some(enc)))?)
            })().map_err(|e| e.annotate("slot"))?;
            let prev_hash = (|| -> Result<_, DeserializeError> {
                Ok(match raw.cbor_type()? != CBORType::Special {
                    true => {
                        Some(BlockHeaderHash::deserialize(raw)?)
                    },
                    false => {
                        if raw.special()? != CBORSpecial::Null {
                            return Err(DeserializeFailure::ExpectedNull.into());
                        }
                        None
                    }
                })
            })().map_err(|e| e.annotate("prev_hash"))?;
            let issuer_vkey = (|| -> Result<_, DeserializeError> {
                Ok(Vkey::deserialize(raw)?)
            })().map_err(|e| e.annotate("issuer_vkey"))?;
            let vrf_vkey = (|| -> Result<_, DeserializeError> {
                Ok(VRFVKey::deserialize(raw)?)
            })().map_err(|e| e.annotate("vrf_vkey"))?;
            let vrf_result = (|| -> Result<_, DeserializeError> {
                Ok(VrfCert::deserialize(raw)?)
            })().map_err(|e| e.annotate("vrf_result"))?;
            let (block_body_size, block_body_size_encoding) = (|| -> Result<_, DeserializeError> {
                Ok(raw.unsigned_integer_sz().map(|(x, enc)| (x, Some(enc)))?)
            })().map_err(|e| e.annotate("block_body_size"))?;
            let block_body_hash = (|| -> Result<_, DeserializeError> {
                Ok(BlockBodyHash::deserialize(raw)?)
            })().map_err(|e| e.annotate("block_body_hash"))?;
            let operational_cert = (|| -> Result<_, DeserializeError> {
                Ok(OperationalCert::deserialize_as_embedded_group(raw, &mut read_len, len)?)
            })().map_err(|e| e.annotate("operational_cert"))?;
            let protocol_version = (|| -> Result<_, DeserializeError> {
                Ok(ProtocolVersion::deserialize_as_embedded_group(raw, &mut read_len, len)?)
            })().map_err(|e| e.annotate("protocol_version"))?;
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
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
                    block_body_size_encoding,
                }),
            })
        })().map_err(|e| e.annotate("HeaderBody"))
    }
}

impl Serialize for I0OrI1 {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            I0OrI1::I0{ i0_encoding } => {
                serializer.write_unsigned_integer_sz(0u64, fit_sz(0u64, *i0_encoding, force_canonical))
            },
            I0OrI1::I1{ i1_encoding } => {
                serializer.write_unsigned_integer_sz(1u64, fit_sz(1u64, *i1_encoding, force_canonical))
            },
        }
    }
}

impl Deserialize for I0OrI1 {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let initial_position = raw.as_mut_ref().seek(SeekFrom::Current(0)).unwrap();
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let (i0_value, i0_encoding) = raw.unsigned_integer_sz()?;
                if i0_value != 0 {
                    return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(i0_value), expected: Key::Uint(0) }.into());
                }
                Ok(Some(i0_encoding))
            })(raw)
            {
                Ok(i0_encoding) => return Ok(Self::I0 {
                    i0_encoding,
                }),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let (i1_value, i1_encoding) = raw.unsigned_integer_sz()?;
                if i1_value != 1 {
                    return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(i1_value), expected: Key::Uint(1) }.into());
                }
                Ok(Some(i1_encoding))
            })(raw)
            {
                Ok(i1_encoding) => return Ok(Self::I1 {
                    i1_encoding,
                }),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            Err(DeserializeError::new("I0OrI1", DeserializeFailure::NoVariantMatched.into()))
        })().map_err(|e| e.annotate("I0OrI1"))
    }
}

impl Serialize for InvalidBefore {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().to_len_sz(2, force_canonical))?;
        self.serialize_as_embedded_group(serializer, force_canonical)
    }
}

impl SerializeEmbeddedGroup for InvalidBefore {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer_sz(4u64, fit_sz(4u64, self.encodings.as_ref().map(|encs| encs.index_0_encoding.clone()).unwrap_or_default(), force_canonical))?;
        serializer.write_unsigned_integer_sz(self.index_1, fit_sz(self.index_1, self.encodings.as_ref().map(|encs| encs.index_1_encoding.clone()).unwrap_or_default(), force_canonical))?;
        self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().end(serializer, force_canonical)
    }
}

impl Deserialize for InvalidBefore {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array_sz()?;
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(2)?;
            let ret = Self::deserialize_as_embedded_group(raw, &mut read_len, len);
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("InvalidBefore"))
    }
}

impl DeserializeEmbeddedGroup for InvalidBefore {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, read_len: &mut CBORReadLen, len: cbor_event::LenSz) -> Result<Self, DeserializeError> {
        let len_encoding = len.into();
        let index_0_encoding = (|| -> Result<_, DeserializeError> {
            let (index_0_value, index_0_encoding) = raw.unsigned_integer_sz()?;
            if index_0_value != 4 {
                return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(index_0_value), expected: Key::Uint(4) }.into());
            }
            Ok(Some(index_0_encoding))
        })().map_err(|e| e.annotate("index_0"))?;
        let (index_1, index_1_encoding) = (|| -> Result<_, DeserializeError> {
            Ok(raw.unsigned_integer_sz().map(|(x, enc)| (x, Some(enc)))?)
        })().map_err(|e| e.annotate("index_1"))?;
        Ok(InvalidBefore {
            index_1,
            encodings: Some(InvalidBeforeEncoding {
                len_encoding,
                index_0_encoding,
                index_1_encoding,
            }),
        })
    }
}

impl Serialize for InvalidHereafter {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().to_len_sz(2, force_canonical))?;
        self.serialize_as_embedded_group(serializer, force_canonical)
    }
}

impl SerializeEmbeddedGroup for InvalidHereafter {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer_sz(5u64, fit_sz(5u64, self.encodings.as_ref().map(|encs| encs.index_0_encoding.clone()).unwrap_or_default(), force_canonical))?;
        serializer.write_unsigned_integer_sz(self.index_1, fit_sz(self.index_1, self.encodings.as_ref().map(|encs| encs.index_1_encoding.clone()).unwrap_or_default(), force_canonical))?;
        self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().end(serializer, force_canonical)
    }
}

impl Deserialize for InvalidHereafter {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array_sz()?;
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(2)?;
            let ret = Self::deserialize_as_embedded_group(raw, &mut read_len, len);
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("InvalidHereafter"))
    }
}

impl DeserializeEmbeddedGroup for InvalidHereafter {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, read_len: &mut CBORReadLen, len: cbor_event::LenSz) -> Result<Self, DeserializeError> {
        let len_encoding = len.into();
        let index_0_encoding = (|| -> Result<_, DeserializeError> {
            let (index_0_value, index_0_encoding) = raw.unsigned_integer_sz()?;
            if index_0_value != 5 {
                return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(index_0_value), expected: Key::Uint(5) }.into());
            }
            Ok(Some(index_0_encoding))
        })().map_err(|e| e.annotate("index_0"))?;
        let (index_1, index_1_encoding) = (|| -> Result<_, DeserializeError> {
            Ok(raw.unsigned_integer_sz().map(|(x, enc)| (x, Some(enc)))?)
        })().map_err(|e| e.annotate("index_1"))?;
        Ok(InvalidHereafter {
            index_1,
            encodings: Some(InvalidHereafterEncoding {
                len_encoding,
                index_0_encoding,
                index_1_encoding,
            }),
        })
    }
}

impl Serialize for Ipv4 {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_bytes_sz(&self.inner, self.encodings.as_ref().map(|encs| encs.inner_encoding.clone()).unwrap_or_default().to_str_len_sz(self.inner.len() as u64, force_canonical))
    }
}

impl Deserialize for Ipv4 {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let (inner, inner_encoding) = raw.bytes_sz().map(|(bytes, enc)| (bytes, StringEncoding::from(enc)))?;
        if inner.len() != 4 {
            return Err(DeserializeError::new("Ipv4", DeserializeFailure::RangeCheck{ found: inner.len(), min: Some(4), max: Some(4) }));
        }
        Ok(Self {
            inner,
            encodings: Some(Ipv4Encoding {
                inner_encoding,
            }),
        })
    }
}

impl Serialize for Ipv6 {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_bytes_sz(&self.inner, self.encodings.as_ref().map(|encs| encs.inner_encoding.clone()).unwrap_or_default().to_str_len_sz(self.inner.len() as u64, force_canonical))
    }
}

impl Deserialize for Ipv6 {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let (inner, inner_encoding) = raw.bytes_sz().map(|(bytes, enc)| (bytes, StringEncoding::from(enc)))?;
        if inner.len() != 16 {
            return Err(DeserializeError::new("Ipv6", DeserializeFailure::RangeCheck{ found: inner.len(), min: Some(16), max: Some(16) }));
        }
        Ok(Self {
            inner,
            encodings: Some(Ipv6Encoding {
                inner_encoding,
            }),
        })
    }
}

impl Serialize for Language {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            Language::I0{ i0_encoding } => {
                serializer.write_unsigned_integer_sz(0u64, fit_sz(0u64, *i0_encoding, force_canonical))
            },
            Language::I1{ i1_encoding } => {
                serializer.write_unsigned_integer_sz(1u64, fit_sz(1u64, *i1_encoding, force_canonical))
            },
        }
    }
}

impl Deserialize for Language {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let initial_position = raw.as_mut_ref().seek(SeekFrom::Current(0)).unwrap();
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let (i0_value, i0_encoding) = raw.unsigned_integer_sz()?;
                if i0_value != 0 {
                    return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(i0_value), expected: Key::Uint(0) }.into());
                }
                Ok(Some(i0_encoding))
            })(raw)
            {
                Ok(i0_encoding) => return Ok(Self::I0 {
                    i0_encoding,
                }),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let (i1_value, i1_encoding) = raw.unsigned_integer_sz()?;
                if i1_value != 1 {
                    return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(i1_value), expected: Key::Uint(1) }.into());
                }
                Ok(Some(i1_encoding))
            })(raw)
            {
                Ok(i1_encoding) => return Ok(Self::I1 {
                    i1_encoding,
                }),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            Err(DeserializeError::new("Language", DeserializeFailure::NoVariantMatched.into()))
        })().map_err(|e| e.annotate("Language"))
    }
}

impl Serialize for MoveInstantaneousReward {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().to_len_sz(3, force_canonical))?;
        self.index_0.serialize(serializer, force_canonical)?;
        serializer.write_map_sz(self.encodings.as_ref().map(|encs| encs.index_1_encoding.clone()).unwrap_or_default().to_len_sz(self.index_1.len() as u64, force_canonical))?;
        let mut key_order = self.index_1.iter().map(|(k, v)| {
            let mut buf = cbor_event::se::Serializer::new_vec();
            k.serialize(&mut buf, force_canonical)?;
            Ok((buf.finalize(), k, v))
        }).collect::<Result<Vec<(Vec<u8>, &_, &_)>, cbor_event::Error>>()?;
        if force_canonical {
            key_order.sort_by(|(lhs_bytes, _, _), (rhs_bytes, _, _)| {
                match lhs_bytes.len().cmp(&rhs_bytes.len()) {
                    std::cmp::Ordering::Equal => lhs_bytes.cmp(&rhs_bytes),
                    diff_ord => diff_ord,
                }
            });
        }
        for (key_bytes, key, value) in key_order {
            serializer.write_raw_bytes(&key_bytes)?;
            value.serialize(serializer, force_canonical)?;
        }
        self.encodings.as_ref().map(|encs| encs.index_1_encoding.clone()).unwrap_or_default().end(serializer, force_canonical)?;
        serializer.write_unsigned_integer_sz(self.coin, fit_sz(self.coin, self.encodings.as_ref().map(|encs| encs.coin_encoding.clone()).unwrap_or_default(), force_canonical))?;
        self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().end(serializer, force_canonical)
    }
}

impl Deserialize for MoveInstantaneousReward {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array_sz()?;
            let len_encoding: LenEncoding = len.into();
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(3)?;
            let index_0 = (|| -> Result<_, DeserializeError> {
                Ok(I0OrI1::deserialize(raw)?)
            })().map_err(|e| e.annotate("index_0"))?;
            let (index_1, index_1_encoding) = (|| -> Result<_, DeserializeError> {
                let mut index_1_table = OrderedHashMap::new();
                let index_1_len = raw.map_sz()?;
                let index_1_encoding = index_1_len.into();
                while match index_1_len { cbor_event::LenSz::Len(n, _) => index_1_table.len() < n as usize, cbor_event::LenSz::Indefinite => true, } {
                    if raw.cbor_type()? == CBORType::Special {
                        assert_eq!(raw.special()?, CBORSpecial::Break);
                        break;
                    }
                    let index_1_key = StakeCredential::deserialize(raw)?;
                    let index_1_value = Int::deserialize(raw)?;
                    if index_1_table.insert(index_1_key.clone(), index_1_value).is_some() {
                        return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from("some complicated/unsupported type"))).into());
                    }
                }
                Ok((index_1_table, index_1_encoding))
            })().map_err(|e| e.annotate("index_1"))?;
            let (coin, coin_encoding) = (|| -> Result<_, DeserializeError> {
                Ok(raw.unsigned_integer_sz().map(|(x, enc)| (x, Some(enc)))?)
            })().map_err(|e| e.annotate("coin"))?;
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(MoveInstantaneousReward {
                index_0,
                index_1,
                coin,
                encodings: Some(MoveInstantaneousRewardEncoding {
                    len_encoding,
                    index_1_encoding,
                    coin_encoding,
                }),
            })
        })().map_err(|e| e.annotate("MoveInstantaneousReward"))
    }
}

impl Serialize for MoveInstantaneousRewardsCert {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().to_len_sz(2, force_canonical))?;
        self.serialize_as_embedded_group(serializer, force_canonical)
    }
}

impl SerializeEmbeddedGroup for MoveInstantaneousRewardsCert {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer_sz(6u64, fit_sz(6u64, self.encodings.as_ref().map(|encs| encs.index_0_encoding.clone()).unwrap_or_default(), force_canonical))?;
        self.move_instantaneous_reward.serialize(serializer, force_canonical)?;
        self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().end(serializer, force_canonical)
    }
}

impl Deserialize for MoveInstantaneousRewardsCert {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array_sz()?;
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(2)?;
            let ret = Self::deserialize_as_embedded_group(raw, &mut read_len, len);
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("MoveInstantaneousRewardsCert"))
    }
}

impl DeserializeEmbeddedGroup for MoveInstantaneousRewardsCert {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, read_len: &mut CBORReadLen, len: cbor_event::LenSz) -> Result<Self, DeserializeError> {
        let len_encoding = len.into();
        let index_0_encoding = (|| -> Result<_, DeserializeError> {
            let (index_0_value, index_0_encoding) = raw.unsigned_integer_sz()?;
            if index_0_value != 6 {
                return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(index_0_value), expected: Key::Uint(6) }.into());
            }
            Ok(Some(index_0_encoding))
        })().map_err(|e| e.annotate("index_0"))?;
        let move_instantaneous_reward = (|| -> Result<_, DeserializeError> {
            Ok(MoveInstantaneousReward::deserialize(raw)?)
        })().map_err(|e| e.annotate("move_instantaneous_reward"))?;
        Ok(MoveInstantaneousRewardsCert {
            move_instantaneous_reward,
            encodings: Some(MoveInstantaneousRewardsCertEncoding {
                len_encoding,
                index_0_encoding,
            }),
        })
    }
}

impl Serialize for MultiHostName {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().to_len_sz(2, force_canonical))?;
        self.serialize_as_embedded_group(serializer, force_canonical)
    }
}

impl SerializeEmbeddedGroup for MultiHostName {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer_sz(2u64, fit_sz(2u64, self.encodings.as_ref().map(|encs| encs.index_0_encoding.clone()).unwrap_or_default(), force_canonical))?;
        self.dns_name.serialize(serializer, force_canonical)?;
        self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().end(serializer, force_canonical)
    }
}

impl Deserialize for MultiHostName {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array_sz()?;
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(2)?;
            let ret = Self::deserialize_as_embedded_group(raw, &mut read_len, len);
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("MultiHostName"))
    }
}

impl DeserializeEmbeddedGroup for MultiHostName {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, read_len: &mut CBORReadLen, len: cbor_event::LenSz) -> Result<Self, DeserializeError> {
        let len_encoding = len.into();
        let index_0_encoding = (|| -> Result<_, DeserializeError> {
            let (index_0_value, index_0_encoding) = raw.unsigned_integer_sz()?;
            if index_0_value != 2 {
                return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(index_0_value), expected: Key::Uint(2) }.into());
            }
            Ok(Some(index_0_encoding))
        })().map_err(|e| e.annotate("index_0"))?;
        let dns_name = (|| -> Result<_, DeserializeError> {
            Ok(DnsName::deserialize(raw)?)
        })().map_err(|e| e.annotate("dns_name"))?;
        Ok(MultiHostName {
            dns_name,
            encodings: Some(MultiHostNameEncoding {
                len_encoding,
                index_0_encoding,
            }),
        })
    }
}

impl Serialize for NativeScript {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            NativeScript::ScriptPubkey(script_pubkey) => script_pubkey.serialize(serializer, force_canonical),
            NativeScript::ScriptAll(script_all) => script_all.serialize(serializer, force_canonical),
            NativeScript::ScriptAny(script_any) => script_any.serialize(serializer, force_canonical),
            NativeScript::ScriptNOfK(script_n_of_k) => script_n_of_k.serialize(serializer, force_canonical),
            NativeScript::InvalidBefore(invalid_before) => invalid_before.serialize(serializer, force_canonical),
            NativeScript::InvalidHereafter(invalid_hereafter) => invalid_hereafter.serialize(serializer, force_canonical),
        }
    }
}

impl Deserialize for NativeScript {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array_sz()?;
            let outer_len_encoding: LenEncoding = len.into();
            let mut read_len = CBORReadLen::new(len);
            let initial_position = raw.as_mut_ref().seek(SeekFrom::Current(0)).unwrap();
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(ScriptPubkey::deserialize_as_embedded_group(raw, &mut read_len, len)?)
            })(raw)
            {
                Ok(script_pubkey) => return Ok(Self::ScriptPubkey(script_pubkey)),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(ScriptAll::deserialize_as_embedded_group(raw, &mut read_len, len)?)
            })(raw)
            {
                Ok(script_all) => return Ok(Self::ScriptAll(script_all)),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(ScriptAny::deserialize_as_embedded_group(raw, &mut read_len, len)?)
            })(raw)
            {
                Ok(script_any) => return Ok(Self::ScriptAny(script_any)),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(ScriptNOfK::deserialize_as_embedded_group(raw, &mut read_len, len)?)
            })(raw)
            {
                Ok(script_n_of_k) => return Ok(Self::ScriptNOfK(script_n_of_k)),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(InvalidBefore::deserialize_as_embedded_group(raw, &mut read_len, len)?)
            })(raw)
            {
                Ok(invalid_before) => return Ok(Self::InvalidBefore(invalid_before)),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(InvalidHereafter::deserialize_as_embedded_group(raw, &mut read_len, len)?)
            })(raw)
            {
                Ok(invalid_hereafter) => return Ok(Self::InvalidHereafter(invalid_hereafter)),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Err(DeserializeError::new("NativeScript", DeserializeFailure::NoVariantMatched.into()))
        })().map_err(|e| e.annotate("NativeScript"))
    }
}

impl Serialize for NetworkId {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            NetworkId::I0{ i0_encoding } => {
                serializer.write_unsigned_integer_sz(0u64, fit_sz(0u64, *i0_encoding, force_canonical))
            },
            NetworkId::I1{ i1_encoding } => {
                serializer.write_unsigned_integer_sz(1u64, fit_sz(1u64, *i1_encoding, force_canonical))
            },
        }
    }
}

impl Deserialize for NetworkId {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let initial_position = raw.as_mut_ref().seek(SeekFrom::Current(0)).unwrap();
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let (i0_value, i0_encoding) = raw.unsigned_integer_sz()?;
                if i0_value != 0 {
                    return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(i0_value), expected: Key::Uint(0) }.into());
                }
                Ok(Some(i0_encoding))
            })(raw)
            {
                Ok(i0_encoding) => return Ok(Self::I0 {
                    i0_encoding,
                }),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let (i1_value, i1_encoding) = raw.unsigned_integer_sz()?;
                if i1_value != 1 {
                    return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(i1_value), expected: Key::Uint(1) }.into());
                }
                Ok(Some(i1_encoding))
            })(raw)
            {
                Ok(i1_encoding) => return Ok(Self::I1 {
                    i1_encoding,
                }),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            Err(DeserializeError::new("NetworkId", DeserializeFailure::NoVariantMatched.into()))
        })().map_err(|e| e.annotate("NetworkId"))
    }
}

impl Serialize for OperationalCert {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().to_len_sz(4, force_canonical))?;
        self.serialize_as_embedded_group(serializer, force_canonical)
    }
}

impl SerializeEmbeddedGroup for OperationalCert {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.hot_vkey.serialize(serializer, force_canonical)?;
        serializer.write_unsigned_integer_sz(self.sequence_number, fit_sz(self.sequence_number, self.encodings.as_ref().map(|encs| encs.sequence_number_encoding.clone()).unwrap_or_default(), force_canonical))?;
        serializer.write_unsigned_integer_sz(self.kes_period, fit_sz(self.kes_period, self.encodings.as_ref().map(|encs| encs.kes_period_encoding.clone()).unwrap_or_default(), force_canonical))?;
        self.sigma.serialize(serializer, force_canonical)?;
        self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().end(serializer, force_canonical)
    }
}

impl Deserialize for OperationalCert {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array_sz()?;
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(4)?;
            let ret = Self::deserialize_as_embedded_group(raw, &mut read_len, len);
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("OperationalCert"))
    }
}

impl DeserializeEmbeddedGroup for OperationalCert {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, read_len: &mut CBORReadLen, len: cbor_event::LenSz) -> Result<Self, DeserializeError> {
        let len_encoding = len.into();
        let hot_vkey = (|| -> Result<_, DeserializeError> {
            Ok(KESVKey::deserialize(raw)?)
        })().map_err(|e| e.annotate("hot_vkey"))?;
        let (sequence_number, sequence_number_encoding) = (|| -> Result<_, DeserializeError> {
            Ok(raw.unsigned_integer_sz().map(|(x, enc)| (x, Some(enc)))?)
        })().map_err(|e| e.annotate("sequence_number"))?;
        let (kes_period, kes_period_encoding) = (|| -> Result<_, DeserializeError> {
            Ok(raw.unsigned_integer_sz().map(|(x, enc)| (x, Some(enc)))?)
        })().map_err(|e| e.annotate("kes_period"))?;
        let sigma = (|| -> Result<_, DeserializeError> {
            Ok(Ed25519Signature::deserialize(raw)?)
        })().map_err(|e| e.annotate("sigma"))?;
        Ok(OperationalCert {
            hot_vkey,
            sequence_number,
            kes_period,
            sigma,
            encodings: Some(OperationalCertEncoding {
                len_encoding,
                sequence_number_encoding,
                kes_period_encoding,
            }),
        })
    }
}

impl Serialize for PlutusData {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            PlutusData::ConstrPlutusData(constr_plutus_data) => {
                constr_plutus_data.serialize(serializer, force_canonical)
            },
            PlutusData::MapPlutusDataToPlutusData{ map_plutus_data_to_plutus_data, map_plutus_data_to_plutus_data_encoding } => {
                serializer.write_map_sz(map_plutus_data_to_plutus_data_encoding.to_len_sz(map_plutus_data_to_plutus_data.len() as u64, force_canonical))?;
                let mut key_order = map_plutus_data_to_plutus_data.iter().map(|(k, v)| {
                    let mut buf = cbor_event::se::Serializer::new_vec();
                    k.serialize(&mut buf, force_canonical)?;
                    Ok((buf.finalize(), k, v))
                }).collect::<Result<Vec<(Vec<u8>, &_, &_)>, cbor_event::Error>>()?;
                if force_canonical {
                    key_order.sort_by(|(lhs_bytes, _, _), (rhs_bytes, _, _)| {
                        match lhs_bytes.len().cmp(&rhs_bytes.len()) {
                            std::cmp::Ordering::Equal => lhs_bytes.cmp(&rhs_bytes),
                            diff_ord => diff_ord,
                        }
                    });
                }
                for (key_bytes, key, value) in key_order {
                    serializer.write_raw_bytes(&key_bytes)?;
                    value.serialize(serializer, force_canonical)?;
                }
                map_plutus_data_to_plutus_data_encoding.end(serializer, force_canonical)
            },
            PlutusData::ArrPlutusData{ arr_plutus_data, arr_plutus_data_encoding } => {
                serializer.write_array_sz(arr_plutus_data_encoding.to_len_sz(arr_plutus_data.len() as u64, force_canonical))?;
                for element in arr_plutus_data.iter() {
                    element.serialize(serializer, force_canonical)?;
                }
                arr_plutus_data_encoding.end(serializer, force_canonical)
            },
            PlutusData::BigInt(big_int) => {
                big_int.serialize(serializer, force_canonical)
            },
            PlutusData::BoundedBytes{ bounded_bytes, bounded_bytes_encoding } => {
                serializer.write_bytes_sz(&bounded_bytes, bounded_bytes_encoding.to_str_len_sz(bounded_bytes.len() as u64, force_canonical))
            },
        }
    }
}

impl Deserialize for PlutusData {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let initial_position = raw.as_mut_ref().seek(SeekFrom::Current(0)).unwrap();
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(ConstrPlutusData::deserialize(raw)?)
            })(raw)
            {
                Ok(constr_plutus_data) => return Ok(Self::ConstrPlutusData(constr_plutus_data)),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let mut map_plutus_data_to_plutus_data_table = OrderedHashMap::new();
                let map_plutus_data_to_plutus_data_len = raw.map_sz()?;
                let map_plutus_data_to_plutus_data_encoding = map_plutus_data_to_plutus_data_len.into();
                while match map_plutus_data_to_plutus_data_len { cbor_event::LenSz::Len(n, _) => map_plutus_data_to_plutus_data_table.len() < n as usize, cbor_event::LenSz::Indefinite => true, } {
                    if raw.cbor_type()? == CBORType::Special {
                        assert_eq!(raw.special()?, CBORSpecial::Break);
                        break;
                    }
                    let map_plutus_data_to_plutus_data_key = PlutusData::deserialize(raw)?;
                    let map_plutus_data_to_plutus_data_value = PlutusData::deserialize(raw)?;
                    if map_plutus_data_to_plutus_data_table.insert(map_plutus_data_to_plutus_data_key.clone(), map_plutus_data_to_plutus_data_value).is_some() {
                        return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from("some complicated/unsupported type"))).into());
                    }
                }
                Ok((map_plutus_data_to_plutus_data_table, map_plutus_data_to_plutus_data_encoding))
            })(raw)
            {
                Ok((map_plutus_data_to_plutus_data, map_plutus_data_to_plutus_data_encoding)) => return Ok(Self::MapPlutusDataToPlutusData {
                    map_plutus_data_to_plutus_data,
                    map_plutus_data_to_plutus_data_encoding,
                }),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let mut arr_plutus_data_arr = Vec::new();
                let len = raw.array_sz()?;
                let arr_plutus_data_encoding = len.into();
                while match len { cbor_event::LenSz::Len(n, _) => arr_plutus_data_arr.len() < n as usize, cbor_event::LenSz::Indefinite => true, } {
                    if raw.cbor_type()? == CBORType::Special {
                        assert_eq!(raw.special()?, CBORSpecial::Break);
                        break;
                    }
                    arr_plutus_data_arr.push(PlutusData::deserialize(raw)?);
                }
                Ok((arr_plutus_data_arr, arr_plutus_data_encoding))
            })(raw)
            {
                Ok((arr_plutus_data, arr_plutus_data_encoding)) => return Ok(Self::ArrPlutusData {
                    arr_plutus_data,
                    arr_plutus_data_encoding,
                }),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(BigInt::deserialize(raw)?)
            })(raw)
            {
                Ok(big_int) => return Ok(Self::BigInt(big_int)),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(raw.bytes_sz().map(|(bytes, enc)| (bytes, StringEncoding::from(enc)))?)
            })(raw)
            {
                Ok((bounded_bytes, bounded_bytes_encoding)) => return Ok(Self::BoundedBytes {
                    bounded_bytes,
                    bounded_bytes_encoding,
                }),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            Err(DeserializeError::new("PlutusData", DeserializeFailure::NoVariantMatched.into()))
        })().map_err(|e| e.annotate("PlutusData"))
    }
}

impl Serialize for PoolMetadata {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().to_len_sz(2, force_canonical))?;
        self.url.serialize(serializer, force_canonical)?;
        self.pool_metadata_hash.serialize(serializer, force_canonical)?;
        self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().end(serializer, force_canonical)
    }
}

impl Deserialize for PoolMetadata {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array_sz()?;
            let len_encoding: LenEncoding = len.into();
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(2)?;
            let url = (|| -> Result<_, DeserializeError> {
                Ok(Url::deserialize(raw)?)
            })().map_err(|e| e.annotate("url"))?;
            let pool_metadata_hash = (|| -> Result<_, DeserializeError> {
                Ok(PoolMetadataHash::deserialize(raw)?)
            })().map_err(|e| e.annotate("pool_metadata_hash"))?;
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(PoolMetadata {
                url,
                pool_metadata_hash,
                encodings: Some(PoolMetadataEncoding {
                    len_encoding,
                }),
            })
        })().map_err(|e| e.annotate("PoolMetadata"))
    }
}

impl Serialize for PoolParams {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().to_len_sz(9, force_canonical))?;
        self.serialize_as_embedded_group(serializer, force_canonical)
    }
}

impl SerializeEmbeddedGroup for PoolParams {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.operator.serialize(serializer, force_canonical)?;
        self.vrf_keyhash.serialize(serializer, force_canonical)?;
        serializer.write_unsigned_integer_sz(self.pledge, fit_sz(self.pledge, self.encodings.as_ref().map(|encs| encs.pledge_encoding.clone()).unwrap_or_default(), force_canonical))?;
        serializer.write_unsigned_integer_sz(self.cost, fit_sz(self.cost, self.encodings.as_ref().map(|encs| encs.cost_encoding.clone()).unwrap_or_default(), force_canonical))?;
        self.margin.serialize(serializer, force_canonical)?;
        self.reward_account.serialize(serializer, force_canonical)?;
        serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.pool_owners_encoding.clone()).unwrap_or_default().to_len_sz(self.pool_owners.len() as u64, force_canonical))?;
        for element in self.pool_owners.iter() {
            element.serialize(serializer, force_canonical)?;
        }
        self.encodings.as_ref().map(|encs| encs.pool_owners_encoding.clone()).unwrap_or_default().end(serializer, force_canonical)?;
        serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.relays_encoding.clone()).unwrap_or_default().to_len_sz(self.relays.len() as u64, force_canonical))?;
        for element in self.relays.iter() {
            element.serialize(serializer, force_canonical)?;
        }
        self.encodings.as_ref().map(|encs| encs.relays_encoding.clone()).unwrap_or_default().end(serializer, force_canonical)?;
        match &self.pool_metadata {
            Some(x) => {
                x.serialize(serializer, force_canonical)
            },
            None => serializer.write_special(CBORSpecial::Null),
        }?;
        self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().end(serializer, force_canonical)
    }
}

impl Deserialize for PoolParams {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array_sz()?;
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(9)?;
            let ret = Self::deserialize_as_embedded_group(raw, &mut read_len, len);
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("PoolParams"))
    }
}

impl DeserializeEmbeddedGroup for PoolParams {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, read_len: &mut CBORReadLen, len: cbor_event::LenSz) -> Result<Self, DeserializeError> {
        let len_encoding = len.into();
        let operator = (|| -> Result<_, DeserializeError> {
            Ok(Ed25519KeyHash::deserialize(raw)?)
        })().map_err(|e| e.annotate("operator"))?;
        let vrf_keyhash = (|| -> Result<_, DeserializeError> {
            Ok(VRFKeyHash::deserialize(raw)?)
        })().map_err(|e| e.annotate("vrf_keyhash"))?;
        let (pledge, pledge_encoding) = (|| -> Result<_, DeserializeError> {
            Ok(raw.unsigned_integer_sz().map(|(x, enc)| (x, Some(enc)))?)
        })().map_err(|e| e.annotate("pledge"))?;
        let (cost, cost_encoding) = (|| -> Result<_, DeserializeError> {
            Ok(raw.unsigned_integer_sz().map(|(x, enc)| (x, Some(enc)))?)
        })().map_err(|e| e.annotate("cost"))?;
        let margin = (|| -> Result<_, DeserializeError> {
            Ok(UnitInterval::deserialize(raw)?)
        })().map_err(|e| e.annotate("margin"))?;
        let reward_account = (|| -> Result<_, DeserializeError> {
            Ok(RewardAccount::deserialize(raw)?)
        })().map_err(|e| e.annotate("reward_account"))?;
        let (pool_owners, pool_owners_encoding) = (|| -> Result<_, DeserializeError> {
            let mut pool_owners_arr = Vec::new();
            let len = raw.array_sz()?;
            let pool_owners_encoding = len.into();
            while match len { cbor_event::LenSz::Len(n, _) => pool_owners_arr.len() < n as usize, cbor_event::LenSz::Indefinite => true, } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                pool_owners_arr.push(Ed25519KeyHash::deserialize(raw)?);
            }
            Ok((pool_owners_arr, pool_owners_encoding))
        })().map_err(|e| e.annotate("pool_owners"))?;
        let (relays, relays_encoding) = (|| -> Result<_, DeserializeError> {
            let mut relays_arr = Vec::new();
            let len = raw.array_sz()?;
            let relays_encoding = len.into();
            while match len { cbor_event::LenSz::Len(n, _) => relays_arr.len() < n as usize, cbor_event::LenSz::Indefinite => true, } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                relays_arr.push(Relay::deserialize(raw)?);
            }
            Ok((relays_arr, relays_encoding))
        })().map_err(|e| e.annotate("relays"))?;
        let pool_metadata = (|| -> Result<_, DeserializeError> {
            Ok(match raw.cbor_type()? != CBORType::Special {
                true => {
                    Some(PoolMetadata::deserialize(raw)?)
                },
                false => {
                    if raw.special()? != CBORSpecial::Null {
                        return Err(DeserializeFailure::ExpectedNull.into());
                    }
                    None
                }
            })
        })().map_err(|e| e.annotate("pool_metadata"))?;
        Ok(PoolParams {
            operator,
            vrf_keyhash,
            pledge,
            cost,
            margin,
            reward_account,
            pool_owners,
            relays,
            pool_metadata,
            encodings: Some(PoolParamsEncoding {
                len_encoding,
                pledge_encoding,
                cost_encoding,
                pool_owners_encoding,
                relays_encoding,
            }),
        })
    }
}

impl Serialize for PoolRegistration {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().to_len_sz(10, force_canonical))?;
        self.serialize_as_embedded_group(serializer, force_canonical)
    }
}

impl SerializeEmbeddedGroup for PoolRegistration {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer_sz(3u64, fit_sz(3u64, self.encodings.as_ref().map(|encs| encs.index_0_encoding.clone()).unwrap_or_default(), force_canonical))?;
        self.pool_params.serialize_as_embedded_group(serializer, force_canonical)?;
        self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().end(serializer, force_canonical)
    }
}

impl Deserialize for PoolRegistration {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array_sz()?;
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(10)?;
            let ret = Self::deserialize_as_embedded_group(raw, &mut read_len, len);
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("PoolRegistration"))
    }
}

impl DeserializeEmbeddedGroup for PoolRegistration {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, read_len: &mut CBORReadLen, len: cbor_event::LenSz) -> Result<Self, DeserializeError> {
        let len_encoding = len.into();
        let index_0_encoding = (|| -> Result<_, DeserializeError> {
            let (index_0_value, index_0_encoding) = raw.unsigned_integer_sz()?;
            if index_0_value != 3 {
                return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(index_0_value), expected: Key::Uint(3) }.into());
            }
            Ok(Some(index_0_encoding))
        })().map_err(|e| e.annotate("index_0"))?;
        let pool_params = (|| -> Result<_, DeserializeError> {
            Ok(PoolParams::deserialize_as_embedded_group(raw, read_len, len)?)
        })().map_err(|e| e.annotate("pool_params"))?;
        Ok(PoolRegistration {
            pool_params,
            encodings: Some(PoolRegistrationEncoding {
                len_encoding,
                index_0_encoding,
            }),
        })
    }
}

impl Serialize for PoolRetirement {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().to_len_sz(3, force_canonical))?;
        self.serialize_as_embedded_group(serializer, force_canonical)
    }
}

impl SerializeEmbeddedGroup for PoolRetirement {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer_sz(4u64, fit_sz(4u64, self.encodings.as_ref().map(|encs| encs.index_0_encoding.clone()).unwrap_or_default(), force_canonical))?;
        self.pool_keyhash.serialize(serializer, force_canonical)?;
        serializer.write_unsigned_integer_sz(self.epoch, fit_sz(self.epoch, self.encodings.as_ref().map(|encs| encs.epoch_encoding.clone()).unwrap_or_default(), force_canonical))?;
        self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().end(serializer, force_canonical)
    }
}

impl Deserialize for PoolRetirement {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array_sz()?;
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(3)?;
            let ret = Self::deserialize_as_embedded_group(raw, &mut read_len, len);
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("PoolRetirement"))
    }
}

impl DeserializeEmbeddedGroup for PoolRetirement {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, read_len: &mut CBORReadLen, len: cbor_event::LenSz) -> Result<Self, DeserializeError> {
        let len_encoding = len.into();
        let index_0_encoding = (|| -> Result<_, DeserializeError> {
            let (index_0_value, index_0_encoding) = raw.unsigned_integer_sz()?;
            if index_0_value != 4 {
                return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(index_0_value), expected: Key::Uint(4) }.into());
            }
            Ok(Some(index_0_encoding))
        })().map_err(|e| e.annotate("index_0"))?;
        let pool_keyhash = (|| -> Result<_, DeserializeError> {
            Ok(Ed25519KeyHash::deserialize(raw)?)
        })().map_err(|e| e.annotate("pool_keyhash"))?;
        let (epoch, epoch_encoding) = (|| -> Result<_, DeserializeError> {
            Ok(raw.unsigned_integer_sz().map(|(x, enc)| (x, Some(enc)))?)
        })().map_err(|e| e.annotate("epoch"))?;
        Ok(PoolRetirement {
            pool_keyhash,
            epoch,
            encodings: Some(PoolRetirementEncoding {
                len_encoding,
                index_0_encoding,
                epoch_encoding,
            }),
        })
    }
}

impl Serialize for PositiveInterval {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_tag_sz(30u64, fit_sz(30u64, self.encodings.as_ref().map(|encs| encs.tag_encoding).unwrap_or_default(), force_canonical))?;
        serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().to_len_sz(2, force_canonical))?;
        serializer.write_unsigned_integer_sz(1u64, fit_sz(1u64, self.encodings.as_ref().map(|encs| encs.index_0_encoding.clone()).unwrap_or_default(), force_canonical))?;
        serializer.write_unsigned_integer_sz(2u64, fit_sz(2u64, self.encodings.as_ref().map(|encs| encs.index_1_encoding.clone()).unwrap_or_default(), force_canonical))?;
        self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().end(serializer, force_canonical)
    }
}

impl Deserialize for PositiveInterval {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let (tag, tag_encoding) = raw.tag_sz()?;
            if tag != 30 {
                return Err(DeserializeError::new("PositiveInterval", DeserializeFailure::TagMismatch{ found: tag, expected: 30 }));
            }
            let len = raw.array_sz()?;
            let len_encoding: LenEncoding = len.into();
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(2)?;
            let index_0_encoding = (|| -> Result<_, DeserializeError> {
                let (index_0_value, index_0_encoding) = raw.unsigned_integer_sz()?;
                if index_0_value != 1 {
                    return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(index_0_value), expected: Key::Uint(1) }.into());
                }
                Ok(Some(index_0_encoding))
            })().map_err(|e| e.annotate("index_0"))?;
            let index_1_encoding = (|| -> Result<_, DeserializeError> {
                let (index_1_value, index_1_encoding) = raw.unsigned_integer_sz()?;
                if index_1_value != 2 {
                    return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(index_1_value), expected: Key::Uint(2) }.into());
                }
                Ok(Some(index_1_encoding))
            })().map_err(|e| e.annotate("index_1"))?;
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(PositiveInterval {
                encodings: Some(PositiveIntervalEncoding {
                    len_encoding,
                    tag_encoding: Some(tag_encoding),
                    index_0_encoding,
                    index_1_encoding,
                }),
            })
        })().map_err(|e| e.annotate("PositiveInterval"))
    }
}

impl Serialize for ProtocolParamUpdate {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_map_sz(self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().to_len_sz(match &self.key_0 { Some(_) => 1, None => 0 } + match &self.key_1 { Some(_) => 1, None => 0 } + match &self.key_2 { Some(_) => 1, None => 0 } + match &self.key_3 { Some(_) => 1, None => 0 } + match &self.key_4 { Some(_) => 1, None => 0 } + match &self.key_5 { Some(_) => 1, None => 0 } + match &self.key_6 { Some(_) => 1, None => 0 } + match &self.key_7 { Some(_) => 1, None => 0 } + match &self.key_8 { Some(_) => 1, None => 0 } + match &self.key_9 { Some(_) => 1, None => 0 } + match &self.key_10 { Some(_) => 1, None => 0 } + match &self.key_11 { Some(_) => 1, None => 0 } + match &self.key_14 { Some(_) => 1, None => 0 } + match &self.key_16 { Some(_) => 1, None => 0 } + match &self.key_17 { Some(_) => 1, None => 0 } + match &self.key_18 { Some(_) => 1, None => 0 } + match &self.key_19 { Some(_) => 1, None => 0 } + match &self.key_20 { Some(_) => 1, None => 0 } + match &self.key_21 { Some(_) => 1, None => 0 } + match &self.key_22 { Some(_) => 1, None => 0 } + match &self.key_23 { Some(_) => 1, None => 0 } + match &self.key_24 { Some(_) => 1, None => 0 }, force_canonical))?;
        let deser_order = self.encodings.as_ref().filter(|encs| !force_canonical && encs.orig_deser_order.len() == match &self.key_0 { Some(_) => 1, None => 0 } + match &self.key_1 { Some(_) => 1, None => 0 } + match &self.key_2 { Some(_) => 1, None => 0 } + match &self.key_3 { Some(_) => 1, None => 0 } + match &self.key_4 { Some(_) => 1, None => 0 } + match &self.key_5 { Some(_) => 1, None => 0 } + match &self.key_6 { Some(_) => 1, None => 0 } + match &self.key_7 { Some(_) => 1, None => 0 } + match &self.key_8 { Some(_) => 1, None => 0 } + match &self.key_9 { Some(_) => 1, None => 0 } + match &self.key_10 { Some(_) => 1, None => 0 } + match &self.key_11 { Some(_) => 1, None => 0 } + match &self.key_14 { Some(_) => 1, None => 0 } + match &self.key_16 { Some(_) => 1, None => 0 } + match &self.key_17 { Some(_) => 1, None => 0 } + match &self.key_18 { Some(_) => 1, None => 0 } + match &self.key_19 { Some(_) => 1, None => 0 } + match &self.key_20 { Some(_) => 1, None => 0 } + match &self.key_21 { Some(_) => 1, None => 0 } + match &self.key_22 { Some(_) => 1, None => 0 } + match &self.key_23 { Some(_) => 1, None => 0 } + match &self.key_24 { Some(_) => 1, None => 0 }).map(|encs| encs.orig_deser_order.clone()).unwrap_or_else(|| vec![0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21]);
        for field_index in deser_order {
            match field_index {
                0 => if let Some(field) = &self.key_0 {
                    serializer.write_unsigned_integer_sz(0u64, fit_sz(0u64, self.encodings.as_ref().map(|encs| encs.key_0_key_encoding.clone()).unwrap_or_default(), force_canonical))?;
                    serializer.write_unsigned_integer_sz(*field, fit_sz(*field, self.encodings.as_ref().map(|encs| encs.key_0_encoding.clone()).unwrap_or_default(), force_canonical))?;
                }
                1 => if let Some(field) = &self.key_1 {
                    serializer.write_unsigned_integer_sz(1u64, fit_sz(1u64, self.encodings.as_ref().map(|encs| encs.key_1_key_encoding.clone()).unwrap_or_default(), force_canonical))?;
                    serializer.write_unsigned_integer_sz(*field, fit_sz(*field, self.encodings.as_ref().map(|encs| encs.key_1_encoding.clone()).unwrap_or_default(), force_canonical))?;
                }
                2 => if let Some(field) = &self.key_2 {
                    serializer.write_unsigned_integer_sz(2u64, fit_sz(2u64, self.encodings.as_ref().map(|encs| encs.key_2_key_encoding.clone()).unwrap_or_default(), force_canonical))?;
                    serializer.write_unsigned_integer_sz(*field, fit_sz(*field, self.encodings.as_ref().map(|encs| encs.key_2_encoding.clone()).unwrap_or_default(), force_canonical))?;
                }
                3 => if let Some(field) = &self.key_3 {
                    serializer.write_unsigned_integer_sz(3u64, fit_sz(3u64, self.encodings.as_ref().map(|encs| encs.key_3_key_encoding.clone()).unwrap_or_default(), force_canonical))?;
                    serializer.write_unsigned_integer_sz(*field, fit_sz(*field, self.encodings.as_ref().map(|encs| encs.key_3_encoding.clone()).unwrap_or_default(), force_canonical))?;
                }
                4 => if let Some(field) = &self.key_4 {
                    serializer.write_unsigned_integer_sz(4u64, fit_sz(4u64, self.encodings.as_ref().map(|encs| encs.key_4_key_encoding.clone()).unwrap_or_default(), force_canonical))?;
                    serializer.write_unsigned_integer_sz(*field, fit_sz(*field, self.encodings.as_ref().map(|encs| encs.key_4_encoding.clone()).unwrap_or_default(), force_canonical))?;
                }
                5 => if let Some(field) = &self.key_5 {
                    serializer.write_unsigned_integer_sz(5u64, fit_sz(5u64, self.encodings.as_ref().map(|encs| encs.key_5_key_encoding.clone()).unwrap_or_default(), force_canonical))?;
                    serializer.write_unsigned_integer_sz(*field, fit_sz(*field, self.encodings.as_ref().map(|encs| encs.key_5_encoding.clone()).unwrap_or_default(), force_canonical))?;
                }
                6 => if let Some(field) = &self.key_6 {
                    serializer.write_unsigned_integer_sz(6u64, fit_sz(6u64, self.encodings.as_ref().map(|encs| encs.key_6_key_encoding.clone()).unwrap_or_default(), force_canonical))?;
                    serializer.write_unsigned_integer_sz(*field, fit_sz(*field, self.encodings.as_ref().map(|encs| encs.key_6_encoding.clone()).unwrap_or_default(), force_canonical))?;
                }
                7 => if let Some(field) = &self.key_7 {
                    serializer.write_unsigned_integer_sz(7u64, fit_sz(7u64, self.encodings.as_ref().map(|encs| encs.key_7_key_encoding.clone()).unwrap_or_default(), force_canonical))?;
                    serializer.write_unsigned_integer_sz(*field, fit_sz(*field, self.encodings.as_ref().map(|encs| encs.key_7_encoding.clone()).unwrap_or_default(), force_canonical))?;
                }
                8 => if let Some(field) = &self.key_8 {
                    serializer.write_unsigned_integer_sz(8u64, fit_sz(8u64, self.encodings.as_ref().map(|encs| encs.key_8_key_encoding.clone()).unwrap_or_default(), force_canonical))?;
                    serializer.write_unsigned_integer_sz(*field, fit_sz(*field, self.encodings.as_ref().map(|encs| encs.key_8_encoding.clone()).unwrap_or_default(), force_canonical))?;
                }
                9 => if let Some(field) = &self.key_9 {
                    serializer.write_unsigned_integer_sz(9u64, fit_sz(9u64, self.encodings.as_ref().map(|encs| encs.key_9_key_encoding.clone()).unwrap_or_default(), force_canonical))?;
                    field.serialize(serializer, force_canonical)?;
                }
                10 => if let Some(field) = &self.key_10 {
                    serializer.write_unsigned_integer_sz(10u64, fit_sz(10u64, self.encodings.as_ref().map(|encs| encs.key_10_key_encoding.clone()).unwrap_or_default(), force_canonical))?;
                    field.serialize(serializer, force_canonical)?;
                }
                11 => if let Some(field) = &self.key_11 {
                    serializer.write_unsigned_integer_sz(11u64, fit_sz(11u64, self.encodings.as_ref().map(|encs| encs.key_11_key_encoding.clone()).unwrap_or_default(), force_canonical))?;
                    field.serialize(serializer, force_canonical)?;
                }
                12 => if let Some(field) = &self.key_14 {
                    serializer.write_unsigned_integer_sz(14u64, fit_sz(14u64, self.encodings.as_ref().map(|encs| encs.key_14_key_encoding.clone()).unwrap_or_default(), force_canonical))?;
                    field.serialize(serializer, force_canonical)?;
                }
                13 => if let Some(field) = &self.key_16 {
                    serializer.write_unsigned_integer_sz(16u64, fit_sz(16u64, self.encodings.as_ref().map(|encs| encs.key_16_key_encoding.clone()).unwrap_or_default(), force_canonical))?;
                    serializer.write_unsigned_integer_sz(*field, fit_sz(*field, self.encodings.as_ref().map(|encs| encs.key_16_encoding.clone()).unwrap_or_default(), force_canonical))?;
                }
                14 => if let Some(field) = &self.key_17 {
                    serializer.write_unsigned_integer_sz(17u64, fit_sz(17u64, self.encodings.as_ref().map(|encs| encs.key_17_key_encoding.clone()).unwrap_or_default(), force_canonical))?;
                    serializer.write_unsigned_integer_sz(*field, fit_sz(*field, self.encodings.as_ref().map(|encs| encs.key_17_encoding.clone()).unwrap_or_default(), force_canonical))?;
                }
                15 => if let Some(field) = &self.key_18 {
                    serializer.write_unsigned_integer_sz(18u64, fit_sz(18u64, self.encodings.as_ref().map(|encs| encs.key_18_key_encoding.clone()).unwrap_or_default(), force_canonical))?;
                    field.serialize(serializer, force_canonical)?;
                }
                16 => if let Some(field) = &self.key_19 {
                    serializer.write_unsigned_integer_sz(19u64, fit_sz(19u64, self.encodings.as_ref().map(|encs| encs.key_19_key_encoding.clone()).unwrap_or_default(), force_canonical))?;
                    field.serialize(serializer, force_canonical)?;
                }
                17 => if let Some(field) = &self.key_20 {
                    serializer.write_unsigned_integer_sz(20u64, fit_sz(20u64, self.encodings.as_ref().map(|encs| encs.key_20_key_encoding.clone()).unwrap_or_default(), force_canonical))?;
                    field.serialize(serializer, force_canonical)?;
                }
                18 => if let Some(field) = &self.key_21 {
                    serializer.write_unsigned_integer_sz(21u64, fit_sz(21u64, self.encodings.as_ref().map(|encs| encs.key_21_key_encoding.clone()).unwrap_or_default(), force_canonical))?;
                    field.serialize(serializer, force_canonical)?;
                }
                19 => if let Some(field) = &self.key_22 {
                    serializer.write_unsigned_integer_sz(22u64, fit_sz(22u64, self.encodings.as_ref().map(|encs| encs.key_22_key_encoding.clone()).unwrap_or_default(), force_canonical))?;
                    serializer.write_unsigned_integer_sz(*field, fit_sz(*field, self.encodings.as_ref().map(|encs| encs.key_22_encoding.clone()).unwrap_or_default(), force_canonical))?;
                }
                20 => if let Some(field) = &self.key_23 {
                    serializer.write_unsigned_integer_sz(23u64, fit_sz(23u64, self.encodings.as_ref().map(|encs| encs.key_23_key_encoding.clone()).unwrap_or_default(), force_canonical))?;
                    serializer.write_unsigned_integer_sz(*field, fit_sz(*field, self.encodings.as_ref().map(|encs| encs.key_23_encoding.clone()).unwrap_or_default(), force_canonical))?;
                }
                21 => if let Some(field) = &self.key_24 {
                    serializer.write_unsigned_integer_sz(24u64, fit_sz(24u64, self.encodings.as_ref().map(|encs| encs.key_24_key_encoding.clone()).unwrap_or_default(), force_canonical))?;
                    serializer.write_unsigned_integer_sz(*field, fit_sz(*field, self.encodings.as_ref().map(|encs| encs.key_24_encoding.clone()).unwrap_or_default(), force_canonical))?;
                }
                _ => unreachable!()
            };
        }
        self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().end(serializer, force_canonical)
    }
}

impl Deserialize for ProtocolParamUpdate {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.map_sz()?;
            let len_encoding: LenEncoding = len.into();
            let mut read_len = CBORReadLen::new(len);
            let mut orig_deser_order = Vec::new();
            let mut key_0_encoding = None;
            let mut key_0_key_encoding = None;
            let mut key_0 = None;
            let mut key_1_encoding = None;
            let mut key_1_key_encoding = None;
            let mut key_1 = None;
            let mut key_2_encoding = None;
            let mut key_2_key_encoding = None;
            let mut key_2 = None;
            let mut key_3_encoding = None;
            let mut key_3_key_encoding = None;
            let mut key_3 = None;
            let mut key_4_encoding = None;
            let mut key_4_key_encoding = None;
            let mut key_4 = None;
            let mut key_5_encoding = None;
            let mut key_5_key_encoding = None;
            let mut key_5 = None;
            let mut key_6_encoding = None;
            let mut key_6_key_encoding = None;
            let mut key_6 = None;
            let mut key_7_encoding = None;
            let mut key_7_key_encoding = None;
            let mut key_7 = None;
            let mut key_8_encoding = None;
            let mut key_8_key_encoding = None;
            let mut key_8 = None;
            let mut key_9_key_encoding = None;
            let mut key_9 = None;
            let mut key_10_key_encoding = None;
            let mut key_10 = None;
            let mut key_11_key_encoding = None;
            let mut key_11 = None;
            let mut key_14_key_encoding = None;
            let mut key_14 = None;
            let mut key_16_encoding = None;
            let mut key_16_key_encoding = None;
            let mut key_16 = None;
            let mut key_17_encoding = None;
            let mut key_17_key_encoding = None;
            let mut key_17 = None;
            let mut key_18_key_encoding = None;
            let mut key_18 = None;
            let mut key_19_key_encoding = None;
            let mut key_19 = None;
            let mut key_20_key_encoding = None;
            let mut key_20 = None;
            let mut key_21_key_encoding = None;
            let mut key_21 = None;
            let mut key_22_encoding = None;
            let mut key_22_key_encoding = None;
            let mut key_22 = None;
            let mut key_23_encoding = None;
            let mut key_23_key_encoding = None;
            let mut key_23 = None;
            let mut key_24_encoding = None;
            let mut key_24_key_encoding = None;
            let mut key_24 = None;
            let mut read = 0;
            while match len { cbor_event::LenSz::Len(n, _) => read < n as usize, cbor_event::LenSz::Indefinite => true, } {
                match raw.cbor_type()? {
                    CBORType::UnsignedInteger => match raw.unsigned_integer_sz()? {
                        (0, key_enc) =>  {
                            if key_0.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(0)).into());
                            }
                            let (tmp_key_0, tmp_key_0_encoding) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                Ok(raw.unsigned_integer_sz().map(|(x, enc)| (x, Some(enc)))?)
                            })().map_err(|e| e.annotate("key_0"))?;
                            key_0 = Some(tmp_key_0);
                            key_0_encoding = tmp_key_0_encoding;
                            key_0_key_encoding = Some(key_enc);
                            orig_deser_order.push(0);
                        },
                        (1, key_enc) =>  {
                            if key_1.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(1)).into());
                            }
                            let (tmp_key_1, tmp_key_1_encoding) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                Ok(raw.unsigned_integer_sz().map(|(x, enc)| (x, Some(enc)))?)
                            })().map_err(|e| e.annotate("key_1"))?;
                            key_1 = Some(tmp_key_1);
                            key_1_encoding = tmp_key_1_encoding;
                            key_1_key_encoding = Some(key_enc);
                            orig_deser_order.push(1);
                        },
                        (2, key_enc) =>  {
                            if key_2.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(2)).into());
                            }
                            let (tmp_key_2, tmp_key_2_encoding) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                Ok(raw.unsigned_integer_sz().map(|(x, enc)| (x, Some(enc)))?)
                            })().map_err(|e| e.annotate("key_2"))?;
                            key_2 = Some(tmp_key_2);
                            key_2_encoding = tmp_key_2_encoding;
                            key_2_key_encoding = Some(key_enc);
                            orig_deser_order.push(2);
                        },
                        (3, key_enc) =>  {
                            if key_3.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(3)).into());
                            }
                            let (tmp_key_3, tmp_key_3_encoding) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                Ok(raw.unsigned_integer_sz().map(|(x, enc)| (x, Some(enc)))?)
                            })().map_err(|e| e.annotate("key_3"))?;
                            key_3 = Some(tmp_key_3);
                            key_3_encoding = tmp_key_3_encoding;
                            key_3_key_encoding = Some(key_enc);
                            orig_deser_order.push(3);
                        },
                        (4, key_enc) =>  {
                            if key_4.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(4)).into());
                            }
                            let (tmp_key_4, tmp_key_4_encoding) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                Ok(raw.unsigned_integer_sz().map(|(x, enc)| (x, Some(enc)))?)
                            })().map_err(|e| e.annotate("key_4"))?;
                            key_4 = Some(tmp_key_4);
                            key_4_encoding = tmp_key_4_encoding;
                            key_4_key_encoding = Some(key_enc);
                            orig_deser_order.push(4);
                        },
                        (5, key_enc) =>  {
                            if key_5.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(5)).into());
                            }
                            let (tmp_key_5, tmp_key_5_encoding) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                Ok(raw.unsigned_integer_sz().map(|(x, enc)| (x, Some(enc)))?)
                            })().map_err(|e| e.annotate("key_5"))?;
                            key_5 = Some(tmp_key_5);
                            key_5_encoding = tmp_key_5_encoding;
                            key_5_key_encoding = Some(key_enc);
                            orig_deser_order.push(5);
                        },
                        (6, key_enc) =>  {
                            if key_6.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(6)).into());
                            }
                            let (tmp_key_6, tmp_key_6_encoding) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                Ok(raw.unsigned_integer_sz().map(|(x, enc)| (x, Some(enc)))?)
                            })().map_err(|e| e.annotate("key_6"))?;
                            key_6 = Some(tmp_key_6);
                            key_6_encoding = tmp_key_6_encoding;
                            key_6_key_encoding = Some(key_enc);
                            orig_deser_order.push(6);
                        },
                        (7, key_enc) =>  {
                            if key_7.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(7)).into());
                            }
                            let (tmp_key_7, tmp_key_7_encoding) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                Ok(raw.unsigned_integer_sz().map(|(x, enc)| (x, Some(enc)))?)
                            })().map_err(|e| e.annotate("key_7"))?;
                            key_7 = Some(tmp_key_7);
                            key_7_encoding = tmp_key_7_encoding;
                            key_7_key_encoding = Some(key_enc);
                            orig_deser_order.push(7);
                        },
                        (8, key_enc) =>  {
                            if key_8.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(8)).into());
                            }
                            let (tmp_key_8, tmp_key_8_encoding) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                Ok(raw.unsigned_integer_sz().map(|(x, enc)| (x, Some(enc)))?)
                            })().map_err(|e| e.annotate("key_8"))?;
                            key_8 = Some(tmp_key_8);
                            key_8_encoding = tmp_key_8_encoding;
                            key_8_key_encoding = Some(key_enc);
                            orig_deser_order.push(8);
                        },
                        (9, key_enc) =>  {
                            if key_9.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(9)).into());
                            }
                            let tmp_key_9 = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                Ok(Rational::deserialize(raw)?)
                            })().map_err(|e| e.annotate("key_9"))?;
                            key_9 = Some(tmp_key_9);
                            key_9_key_encoding = Some(key_enc);
                            orig_deser_order.push(9);
                        },
                        (10, key_enc) =>  {
                            if key_10.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(10)).into());
                            }
                            let tmp_key_10 = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                Ok(UnitInterval::deserialize(raw)?)
                            })().map_err(|e| e.annotate("key_10"))?;
                            key_10 = Some(tmp_key_10);
                            key_10_key_encoding = Some(key_enc);
                            orig_deser_order.push(10);
                        },
                        (11, key_enc) =>  {
                            if key_11.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(11)).into());
                            }
                            let tmp_key_11 = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                Ok(UnitInterval::deserialize(raw)?)
                            })().map_err(|e| e.annotate("key_11"))?;
                            key_11 = Some(tmp_key_11);
                            key_11_key_encoding = Some(key_enc);
                            orig_deser_order.push(11);
                        },
                        (14, key_enc) =>  {
                            if key_14.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(14)).into());
                            }
                            let tmp_key_14 = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                Ok(ProtocolVersionStruct::deserialize(raw)?)
                            })().map_err(|e| e.annotate("key_14"))?;
                            key_14 = Some(tmp_key_14);
                            key_14_key_encoding = Some(key_enc);
                            orig_deser_order.push(12);
                        },
                        (16, key_enc) =>  {
                            if key_16.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(16)).into());
                            }
                            let (tmp_key_16, tmp_key_16_encoding) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                Ok(raw.unsigned_integer_sz().map(|(x, enc)| (x, Some(enc)))?)
                            })().map_err(|e| e.annotate("key_16"))?;
                            key_16 = Some(tmp_key_16);
                            key_16_encoding = tmp_key_16_encoding;
                            key_16_key_encoding = Some(key_enc);
                            orig_deser_order.push(13);
                        },
                        (17, key_enc) =>  {
                            if key_17.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(17)).into());
                            }
                            let (tmp_key_17, tmp_key_17_encoding) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                Ok(raw.unsigned_integer_sz().map(|(x, enc)| (x, Some(enc)))?)
                            })().map_err(|e| e.annotate("key_17"))?;
                            key_17 = Some(tmp_key_17);
                            key_17_encoding = tmp_key_17_encoding;
                            key_17_key_encoding = Some(key_enc);
                            orig_deser_order.push(14);
                        },
                        (18, key_enc) =>  {
                            if key_18.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(18)).into());
                            }
                            let tmp_key_18 = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                Ok(Costmdls::deserialize(raw)?)
                            })().map_err(|e| e.annotate("key_18"))?;
                            key_18 = Some(tmp_key_18);
                            key_18_key_encoding = Some(key_enc);
                            orig_deser_order.push(15);
                        },
                        (19, key_enc) =>  {
                            if key_19.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(19)).into());
                            }
                            let tmp_key_19 = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                Ok(ExUnitPrices::deserialize(raw)?)
                            })().map_err(|e| e.annotate("key_19"))?;
                            key_19 = Some(tmp_key_19);
                            key_19_key_encoding = Some(key_enc);
                            orig_deser_order.push(16);
                        },
                        (20, key_enc) =>  {
                            if key_20.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(20)).into());
                            }
                            let tmp_key_20 = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                Ok(ExUnits::deserialize(raw)?)
                            })().map_err(|e| e.annotate("key_20"))?;
                            key_20 = Some(tmp_key_20);
                            key_20_key_encoding = Some(key_enc);
                            orig_deser_order.push(17);
                        },
                        (21, key_enc) =>  {
                            if key_21.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(21)).into());
                            }
                            let tmp_key_21 = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                Ok(ExUnits::deserialize(raw)?)
                            })().map_err(|e| e.annotate("key_21"))?;
                            key_21 = Some(tmp_key_21);
                            key_21_key_encoding = Some(key_enc);
                            orig_deser_order.push(18);
                        },
                        (22, key_enc) =>  {
                            if key_22.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(22)).into());
                            }
                            let (tmp_key_22, tmp_key_22_encoding) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                Ok(raw.unsigned_integer_sz().map(|(x, enc)| (x, Some(enc)))?)
                            })().map_err(|e| e.annotate("key_22"))?;
                            key_22 = Some(tmp_key_22);
                            key_22_encoding = tmp_key_22_encoding;
                            key_22_key_encoding = Some(key_enc);
                            orig_deser_order.push(19);
                        },
                        (23, key_enc) =>  {
                            if key_23.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(23)).into());
                            }
                            let (tmp_key_23, tmp_key_23_encoding) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                Ok(raw.unsigned_integer_sz().map(|(x, enc)| (x, Some(enc)))?)
                            })().map_err(|e| e.annotate("key_23"))?;
                            key_23 = Some(tmp_key_23);
                            key_23_encoding = tmp_key_23_encoding;
                            key_23_key_encoding = Some(key_enc);
                            orig_deser_order.push(20);
                        },
                        (24, key_enc) =>  {
                            if key_24.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(24)).into());
                            }
                            let (tmp_key_24, tmp_key_24_encoding) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                Ok(raw.unsigned_integer_sz().map(|(x, enc)| (x, Some(enc)))?)
                            })().map_err(|e| e.annotate("key_24"))?;
                            key_24 = Some(tmp_key_24);
                            key_24_encoding = tmp_key_24_encoding;
                            key_24_key_encoding = Some(key_enc);
                            orig_deser_order.push(21);
                        },
                        (unknown_key, _enc) => return Err(DeserializeFailure::UnknownKey(Key::Uint(unknown_key)).into()),
                    },
                    CBORType::Text => {
                        let (text_key, key_enc) = raw.text_sz()?;
                        match text_key.as_str() {
                            unknown_key => return Err(DeserializeFailure::UnknownKey(Key::Str(unknown_key.to_owned())).into()),
                        }
                    },
                    CBORType::Special => match len {
                        cbor_event::LenSz::Len(_, _) => return Err(DeserializeFailure::BreakInDefiniteLen.into()),
                        cbor_event::LenSz::Indefinite => match raw.special()? {
                            CBORSpecial::Break => break,
                            _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                        },
                    },
                    other_type => return Err(DeserializeFailure::UnexpectedKeyType(other_type).into()),
                }
                read += 1;
            }
            read_len.finish()?;
            Ok(Self {
                key_0,
                key_1,
                key_2,
                key_3,
                key_4,
                key_5,
                key_6,
                key_7,
                key_8,
                key_9,
                key_10,
                key_11,
                key_14,
                key_16,
                key_17,
                key_18,
                key_19,
                key_20,
                key_21,
                key_22,
                key_23,
                key_24,
                encodings: Some(ProtocolParamUpdateEncoding {
                    len_encoding,
                    orig_deser_order,
                    key_0_key_encoding,
                    key_0_encoding,
                    key_1_key_encoding,
                    key_1_encoding,
                    key_2_key_encoding,
                    key_2_encoding,
                    key_3_key_encoding,
                    key_3_encoding,
                    key_4_key_encoding,
                    key_4_encoding,
                    key_5_key_encoding,
                    key_5_encoding,
                    key_6_key_encoding,
                    key_6_encoding,
                    key_7_key_encoding,
                    key_7_encoding,
                    key_8_key_encoding,
                    key_8_encoding,
                    key_9_key_encoding,
                    key_10_key_encoding,
                    key_11_key_encoding,
                    key_14_key_encoding,
                    key_16_key_encoding,
                    key_16_encoding,
                    key_17_key_encoding,
                    key_17_encoding,
                    key_18_key_encoding,
                    key_19_key_encoding,
                    key_20_key_encoding,
                    key_21_key_encoding,
                    key_22_key_encoding,
                    key_22_encoding,
                    key_23_key_encoding,
                    key_23_encoding,
                    key_24_key_encoding,
                    key_24_encoding,
                }),
            })
        })().map_err(|e| e.annotate("ProtocolParamUpdate"))
    }
}

impl Serialize for ProtocolVersion {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().to_len_sz(2, force_canonical))?;
        self.serialize_as_embedded_group(serializer, force_canonical)
    }
}

impl SerializeEmbeddedGroup for ProtocolVersion {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer_sz(self.index_0, fit_sz(self.index_0, self.encodings.as_ref().map(|encs| encs.index_0_encoding.clone()).unwrap_or_default(), force_canonical))?;
        serializer.write_unsigned_integer_sz(self.index_1, fit_sz(self.index_1, self.encodings.as_ref().map(|encs| encs.index_1_encoding.clone()).unwrap_or_default(), force_canonical))?;
        self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().end(serializer, force_canonical)
    }
}

impl Deserialize for ProtocolVersion {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array_sz()?;
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(2)?;
            let ret = Self::deserialize_as_embedded_group(raw, &mut read_len, len);
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("ProtocolVersion"))
    }
}

impl DeserializeEmbeddedGroup for ProtocolVersion {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, read_len: &mut CBORReadLen, len: cbor_event::LenSz) -> Result<Self, DeserializeError> {
        let len_encoding = len.into();
        let (index_0, index_0_encoding) = (|| -> Result<_, DeserializeError> {
            Ok(raw.unsigned_integer_sz().map(|(x, enc)| (x, Some(enc)))?)
        })().map_err(|e| e.annotate("index_0"))?;
        let (index_1, index_1_encoding) = (|| -> Result<_, DeserializeError> {
            Ok(raw.unsigned_integer_sz().map(|(x, enc)| (x, Some(enc)))?)
        })().map_err(|e| e.annotate("index_1"))?;
        Ok(ProtocolVersion {
            index_0,
            index_1,
            encodings: Some(ProtocolVersionEncoding {
                len_encoding,
                index_0_encoding,
                index_1_encoding,
            }),
        })
    }
}

impl Serialize for ProtocolVersionStruct {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().to_len_sz(2, force_canonical))?;
        self.protocol_version.serialize_as_embedded_group(serializer, force_canonical)?;
        self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().end(serializer, force_canonical)
    }
}

impl Deserialize for ProtocolVersionStruct {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array_sz()?;
            let len_encoding: LenEncoding = len.into();
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(2)?;
            let protocol_version = (|| -> Result<_, DeserializeError> {
                Ok(ProtocolVersion::deserialize_as_embedded_group(raw, &mut read_len, len)?)
            })().map_err(|e| e.annotate("protocol_version"))?;
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(ProtocolVersionStruct {
                protocol_version,
                encodings: Some(ProtocolVersionStructEncoding {
                    len_encoding,
                }),
            })
        })().map_err(|e| e.annotate("ProtocolVersionStruct"))
    }
}

impl Serialize for Rational {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_tag_sz(30u64, fit_sz(30u64, self.encodings.as_ref().map(|encs| encs.tag_encoding).unwrap_or_default(), force_canonical))?;
        serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().to_len_sz(2, force_canonical))?;
        serializer.write_unsigned_integer_sz(self.numerator, fit_sz(self.numerator, self.encodings.as_ref().map(|encs| encs.numerator_encoding.clone()).unwrap_or_default(), force_canonical))?;
        serializer.write_unsigned_integer_sz(self.denominator, fit_sz(self.denominator, self.encodings.as_ref().map(|encs| encs.denominator_encoding.clone()).unwrap_or_default(), force_canonical))?;
        self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().end(serializer, force_canonical)
    }
}

impl Deserialize for Rational {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let (tag, tag_encoding) = raw.tag_sz()?;
            if tag != 30 {
                return Err(DeserializeError::new("Rational", DeserializeFailure::TagMismatch{ found: tag, expected: 30 }));
            }
            let len = raw.array_sz()?;
            let len_encoding: LenEncoding = len.into();
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(2)?;
            let (numerator, numerator_encoding) = (|| -> Result<_, DeserializeError> {
                Ok(raw.unsigned_integer_sz().map(|(x, enc)| (x, Some(enc)))?)
            })().map_err(|e| e.annotate("numerator"))?;
            let (denominator, denominator_encoding) = (|| -> Result<_, DeserializeError> {
                Ok(raw.unsigned_integer_sz().map(|(x, enc)| (x, Some(enc)))?)
            })().map_err(|e| e.annotate("denominator"))?;
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(Rational {
                numerator,
                denominator,
                encodings: Some(RationalEncoding {
                    len_encoding,
                    tag_encoding: Some(tag_encoding),
                    numerator_encoding,
                    denominator_encoding,
                }),
            })
        })().map_err(|e| e.annotate("Rational"))
    }
}

impl Serialize for Redeemer {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().to_len_sz(4, force_canonical))?;
        self.tag.serialize(serializer, force_canonical)?;
        serializer.write_unsigned_integer_sz(self.index, fit_sz(self.index, self.encodings.as_ref().map(|encs| encs.index_encoding.clone()).unwrap_or_default(), force_canonical))?;
        self.data.serialize(serializer, force_canonical)?;
        self.ex_units.serialize(serializer, force_canonical)?;
        self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().end(serializer, force_canonical)
    }
}

impl Deserialize for Redeemer {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array_sz()?;
            let len_encoding: LenEncoding = len.into();
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(4)?;
            let tag = (|| -> Result<_, DeserializeError> {
                Ok(RedeemerTag::deserialize(raw)?)
            })().map_err(|e| e.annotate("tag"))?;
            let (index, index_encoding) = (|| -> Result<_, DeserializeError> {
                Ok(raw.unsigned_integer_sz().map(|(x, enc)| (x, Some(enc)))?)
            })().map_err(|e| e.annotate("index"))?;
            let data = (|| -> Result<_, DeserializeError> {
                Ok(PlutusData::deserialize(raw)?)
            })().map_err(|e| e.annotate("data"))?;
            let ex_units = (|| -> Result<_, DeserializeError> {
                Ok(ExUnits::deserialize(raw)?)
            })().map_err(|e| e.annotate("ex_units"))?;
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
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
                    index_encoding,
                }),
            })
        })().map_err(|e| e.annotate("Redeemer"))
    }
}

impl Serialize for RedeemerTag {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            RedeemerTag::I0{ i0_encoding } => {
                serializer.write_unsigned_integer_sz(0u64, fit_sz(0u64, *i0_encoding, force_canonical))
            },
            RedeemerTag::I1{ i1_encoding } => {
                serializer.write_unsigned_integer_sz(1u64, fit_sz(1u64, *i1_encoding, force_canonical))
            },
            RedeemerTag::I2{ i2_encoding } => {
                serializer.write_unsigned_integer_sz(2u64, fit_sz(2u64, *i2_encoding, force_canonical))
            },
            RedeemerTag::I3{ i3_encoding } => {
                serializer.write_unsigned_integer_sz(3u64, fit_sz(3u64, *i3_encoding, force_canonical))
            },
        }
    }
}

impl Deserialize for RedeemerTag {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let initial_position = raw.as_mut_ref().seek(SeekFrom::Current(0)).unwrap();
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let (i0_value, i0_encoding) = raw.unsigned_integer_sz()?;
                if i0_value != 0 {
                    return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(i0_value), expected: Key::Uint(0) }.into());
                }
                Ok(Some(i0_encoding))
            })(raw)
            {
                Ok(i0_encoding) => return Ok(Self::I0 {
                    i0_encoding,
                }),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let (i1_value, i1_encoding) = raw.unsigned_integer_sz()?;
                if i1_value != 1 {
                    return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(i1_value), expected: Key::Uint(1) }.into());
                }
                Ok(Some(i1_encoding))
            })(raw)
            {
                Ok(i1_encoding) => return Ok(Self::I1 {
                    i1_encoding,
                }),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let (i2_value, i2_encoding) = raw.unsigned_integer_sz()?;
                if i2_value != 2 {
                    return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(i2_value), expected: Key::Uint(2) }.into());
                }
                Ok(Some(i2_encoding))
            })(raw)
            {
                Ok(i2_encoding) => return Ok(Self::I2 {
                    i2_encoding,
                }),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let (i3_value, i3_encoding) = raw.unsigned_integer_sz()?;
                if i3_value != 3 {
                    return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(i3_value), expected: Key::Uint(3) }.into());
                }
                Ok(Some(i3_encoding))
            })(raw)
            {
                Ok(i3_encoding) => return Ok(Self::I3 {
                    i3_encoding,
                }),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            Err(DeserializeError::new("RedeemerTag", DeserializeFailure::NoVariantMatched.into()))
        })().map_err(|e| e.annotate("RedeemerTag"))
    }
}

impl Serialize for Relay {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            Relay::SingleHostAddr(single_host_addr) => single_host_addr.serialize(serializer, force_canonical),
            Relay::SingleHostName(single_host_name) => single_host_name.serialize(serializer, force_canonical),
            Relay::MultiHostName(multi_host_name) => multi_host_name.serialize(serializer, force_canonical),
        }
    }
}

impl Deserialize for Relay {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array_sz()?;
            let outer_len_encoding: LenEncoding = len.into();
            let mut read_len = CBORReadLen::new(len);
            let initial_position = raw.as_mut_ref().seek(SeekFrom::Current(0)).unwrap();
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(SingleHostAddr::deserialize_as_embedded_group(raw, &mut read_len, len)?)
            })(raw)
            {
                Ok(single_host_addr) => return Ok(Self::SingleHostAddr(single_host_addr)),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(SingleHostName::deserialize_as_embedded_group(raw, &mut read_len, len)?)
            })(raw)
            {
                Ok(single_host_name) => return Ok(Self::SingleHostName(single_host_name)),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(MultiHostName::deserialize_as_embedded_group(raw, &mut read_len, len)?)
            })(raw)
            {
                Ok(multi_host_name) => return Ok(Self::MultiHostName(multi_host_name)),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Err(DeserializeError::new("Relay", DeserializeFailure::NoVariantMatched.into()))
        })().map_err(|e| e.annotate("Relay"))
    }
}

impl Serialize for RequiredSigners {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().to_len_sz(1, force_canonical))?;
        self.addr_keyhash.serialize(serializer, force_canonical)?;
        self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().end(serializer, force_canonical)
    }
}

impl Deserialize for RequiredSigners {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array_sz()?;
            let len_encoding: LenEncoding = len.into();
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(1)?;
            let addr_keyhash = (|| -> Result<_, DeserializeError> {
                Ok(Ed25519KeyHash::deserialize(raw)?)
            })().map_err(|e| e.annotate("addr_keyhash"))?;
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(RequiredSigners {
                addr_keyhash,
                encodings: Some(RequiredSignersEncoding {
                    len_encoding,
                }),
            })
        })().map_err(|e| e.annotate("RequiredSigners"))
    }
}

impl Serialize for Script {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            Script::Script0(script0) => script0.serialize(serializer, force_canonical),
            Script::Script1(script1) => script1.serialize(serializer, force_canonical),
            Script::Script2(script2) => script2.serialize(serializer, force_canonical),
        }
    }
}

impl Deserialize for Script {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array_sz()?;
            let outer_len_encoding: LenEncoding = len.into();
            let mut read_len = CBORReadLen::new(len);
            let initial_position = raw.as_mut_ref().seek(SeekFrom::Current(0)).unwrap();
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(Script0::deserialize_as_embedded_group(raw, &mut read_len, len)?)
            })(raw)
            {
                Ok(script0) => return Ok(Self::Script0(script0)),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(Script1::deserialize_as_embedded_group(raw, &mut read_len, len)?)
            })(raw)
            {
                Ok(script1) => return Ok(Self::Script1(script1)),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(Script2::deserialize_as_embedded_group(raw, &mut read_len, len)?)
            })(raw)
            {
                Ok(script2) => return Ok(Self::Script2(script2)),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Err(DeserializeError::new("Script", DeserializeFailure::NoVariantMatched.into()))
        })().map_err(|e| e.annotate("Script"))
    }
}

impl Serialize for Script0 {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().to_len_sz(2, force_canonical))?;
        self.serialize_as_embedded_group(serializer, force_canonical)
    }
}

impl SerializeEmbeddedGroup for Script0 {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer_sz(0u64, fit_sz(0u64, self.encodings.as_ref().map(|encs| encs.index_0_encoding.clone()).unwrap_or_default(), force_canonical))?;
        self.native_script.serialize(serializer, force_canonical)?;
        self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().end(serializer, force_canonical)
    }
}

impl Deserialize for Script0 {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array_sz()?;
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(2)?;
            let ret = Self::deserialize_as_embedded_group(raw, &mut read_len, len);
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("Script0"))
    }
}

impl DeserializeEmbeddedGroup for Script0 {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, read_len: &mut CBORReadLen, len: cbor_event::LenSz) -> Result<Self, DeserializeError> {
        let len_encoding = len.into();
        let index_0_encoding = (|| -> Result<_, DeserializeError> {
            let (index_0_value, index_0_encoding) = raw.unsigned_integer_sz()?;
            if index_0_value != 0 {
                return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(index_0_value), expected: Key::Uint(0) }.into());
            }
            Ok(Some(index_0_encoding))
        })().map_err(|e| e.annotate("index_0"))?;
        let native_script = (|| -> Result<_, DeserializeError> {
            Ok(NativeScript::deserialize(raw)?)
        })().map_err(|e| e.annotate("native_script"))?;
        Ok(Script0 {
            native_script,
            encodings: Some(Script0Encoding {
                len_encoding,
                index_0_encoding,
            }),
        })
    }
}

impl Serialize for Script1 {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().to_len_sz(2, force_canonical))?;
        self.serialize_as_embedded_group(serializer, force_canonical)
    }
}

impl SerializeEmbeddedGroup for Script1 {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer_sz(1u64, fit_sz(1u64, self.encodings.as_ref().map(|encs| encs.index_0_encoding.clone()).unwrap_or_default(), force_canonical))?;
        serializer.write_bytes_sz(&self.plutus_v1_script, self.encodings.as_ref().map(|encs| encs.plutus_v1_script_encoding.clone()).unwrap_or_default().to_str_len_sz(self.plutus_v1_script.len() as u64, force_canonical))?;
        self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().end(serializer, force_canonical)
    }
}

impl Deserialize for Script1 {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array_sz()?;
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(2)?;
            let ret = Self::deserialize_as_embedded_group(raw, &mut read_len, len);
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("Script1"))
    }
}

impl DeserializeEmbeddedGroup for Script1 {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, read_len: &mut CBORReadLen, len: cbor_event::LenSz) -> Result<Self, DeserializeError> {
        let len_encoding = len.into();
        let index_0_encoding = (|| -> Result<_, DeserializeError> {
            let (index_0_value, index_0_encoding) = raw.unsigned_integer_sz()?;
            if index_0_value != 1 {
                return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(index_0_value), expected: Key::Uint(1) }.into());
            }
            Ok(Some(index_0_encoding))
        })().map_err(|e| e.annotate("index_0"))?;
        let (plutus_v1_script, plutus_v1_script_encoding) = (|| -> Result<_, DeserializeError> {
            Ok(raw.bytes_sz().map(|(bytes, enc)| (bytes, StringEncoding::from(enc)))?)
        })().map_err(|e| e.annotate("plutus_v1_script"))?;
        Ok(Script1 {
            plutus_v1_script,
            encodings: Some(Script1Encoding {
                len_encoding,
                index_0_encoding,
                plutus_v1_script_encoding,
            }),
        })
    }
}

impl Serialize for Script2 {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().to_len_sz(2, force_canonical))?;
        self.serialize_as_embedded_group(serializer, force_canonical)
    }
}

impl SerializeEmbeddedGroup for Script2 {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer_sz(2u64, fit_sz(2u64, self.encodings.as_ref().map(|encs| encs.index_0_encoding.clone()).unwrap_or_default(), force_canonical))?;
        serializer.write_bytes_sz(&self.plutus_v2_script, self.encodings.as_ref().map(|encs| encs.plutus_v2_script_encoding.clone()).unwrap_or_default().to_str_len_sz(self.plutus_v2_script.len() as u64, force_canonical))?;
        self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().end(serializer, force_canonical)
    }
}

impl Deserialize for Script2 {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array_sz()?;
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(2)?;
            let ret = Self::deserialize_as_embedded_group(raw, &mut read_len, len);
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("Script2"))
    }
}

impl DeserializeEmbeddedGroup for Script2 {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, read_len: &mut CBORReadLen, len: cbor_event::LenSz) -> Result<Self, DeserializeError> {
        let len_encoding = len.into();
        let index_0_encoding = (|| -> Result<_, DeserializeError> {
            let (index_0_value, index_0_encoding) = raw.unsigned_integer_sz()?;
            if index_0_value != 2 {
                return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(index_0_value), expected: Key::Uint(2) }.into());
            }
            Ok(Some(index_0_encoding))
        })().map_err(|e| e.annotate("index_0"))?;
        let (plutus_v2_script, plutus_v2_script_encoding) = (|| -> Result<_, DeserializeError> {
            Ok(raw.bytes_sz().map(|(bytes, enc)| (bytes, StringEncoding::from(enc)))?)
        })().map_err(|e| e.annotate("plutus_v2_script"))?;
        Ok(Script2 {
            plutus_v2_script,
            encodings: Some(Script2Encoding {
                len_encoding,
                index_0_encoding,
                plutus_v2_script_encoding,
            }),
        })
    }
}

impl Serialize for ScriptAll {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().to_len_sz(2, force_canonical))?;
        self.serialize_as_embedded_group(serializer, force_canonical)
    }
}

impl SerializeEmbeddedGroup for ScriptAll {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer_sz(1u64, fit_sz(1u64, self.encodings.as_ref().map(|encs| encs.index_0_encoding.clone()).unwrap_or_default(), force_canonical))?;
        serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.native_scripts_encoding.clone()).unwrap_or_default().to_len_sz(self.native_scripts.len() as u64, force_canonical))?;
        for element in self.native_scripts.iter() {
            element.serialize(serializer, force_canonical)?;
        }
        self.encodings.as_ref().map(|encs| encs.native_scripts_encoding.clone()).unwrap_or_default().end(serializer, force_canonical)?;
        self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().end(serializer, force_canonical)
    }
}

impl Deserialize for ScriptAll {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array_sz()?;
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(2)?;
            let ret = Self::deserialize_as_embedded_group(raw, &mut read_len, len);
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("ScriptAll"))
    }
}

impl DeserializeEmbeddedGroup for ScriptAll {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, read_len: &mut CBORReadLen, len: cbor_event::LenSz) -> Result<Self, DeserializeError> {
        let len_encoding = len.into();
        let index_0_encoding = (|| -> Result<_, DeserializeError> {
            let (index_0_value, index_0_encoding) = raw.unsigned_integer_sz()?;
            if index_0_value != 1 {
                return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(index_0_value), expected: Key::Uint(1) }.into());
            }
            Ok(Some(index_0_encoding))
        })().map_err(|e| e.annotate("index_0"))?;
        let (native_scripts, native_scripts_encoding) = (|| -> Result<_, DeserializeError> {
            let mut native_scripts_arr = Vec::new();
            let len = raw.array_sz()?;
            let native_scripts_encoding = len.into();
            while match len { cbor_event::LenSz::Len(n, _) => native_scripts_arr.len() < n as usize, cbor_event::LenSz::Indefinite => true, } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                native_scripts_arr.push(NativeScript::deserialize(raw)?);
            }
            Ok((native_scripts_arr, native_scripts_encoding))
        })().map_err(|e| e.annotate("native_scripts"))?;
        Ok(ScriptAll {
            native_scripts,
            encodings: Some(ScriptAllEncoding {
                len_encoding,
                index_0_encoding,
                native_scripts_encoding,
            }),
        })
    }
}

impl Serialize for ScriptAny {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().to_len_sz(2, force_canonical))?;
        self.serialize_as_embedded_group(serializer, force_canonical)
    }
}

impl SerializeEmbeddedGroup for ScriptAny {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer_sz(2u64, fit_sz(2u64, self.encodings.as_ref().map(|encs| encs.index_0_encoding.clone()).unwrap_or_default(), force_canonical))?;
        serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.native_scripts_encoding.clone()).unwrap_or_default().to_len_sz(self.native_scripts.len() as u64, force_canonical))?;
        for element in self.native_scripts.iter() {
            element.serialize(serializer, force_canonical)?;
        }
        self.encodings.as_ref().map(|encs| encs.native_scripts_encoding.clone()).unwrap_or_default().end(serializer, force_canonical)?;
        self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().end(serializer, force_canonical)
    }
}

impl Deserialize for ScriptAny {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array_sz()?;
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(2)?;
            let ret = Self::deserialize_as_embedded_group(raw, &mut read_len, len);
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("ScriptAny"))
    }
}

impl DeserializeEmbeddedGroup for ScriptAny {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, read_len: &mut CBORReadLen, len: cbor_event::LenSz) -> Result<Self, DeserializeError> {
        let len_encoding = len.into();
        let index_0_encoding = (|| -> Result<_, DeserializeError> {
            let (index_0_value, index_0_encoding) = raw.unsigned_integer_sz()?;
            if index_0_value != 2 {
                return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(index_0_value), expected: Key::Uint(2) }.into());
            }
            Ok(Some(index_0_encoding))
        })().map_err(|e| e.annotate("index_0"))?;
        let (native_scripts, native_scripts_encoding) = (|| -> Result<_, DeserializeError> {
            let mut native_scripts_arr = Vec::new();
            let len = raw.array_sz()?;
            let native_scripts_encoding = len.into();
            while match len { cbor_event::LenSz::Len(n, _) => native_scripts_arr.len() < n as usize, cbor_event::LenSz::Indefinite => true, } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                native_scripts_arr.push(NativeScript::deserialize(raw)?);
            }
            Ok((native_scripts_arr, native_scripts_encoding))
        })().map_err(|e| e.annotate("native_scripts"))?;
        Ok(ScriptAny {
            native_scripts,
            encodings: Some(ScriptAnyEncoding {
                len_encoding,
                index_0_encoding,
                native_scripts_encoding,
            }),
        })
    }
}

impl Serialize for ScriptNOfK {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().to_len_sz(3, force_canonical))?;
        self.serialize_as_embedded_group(serializer, force_canonical)
    }
}

impl SerializeEmbeddedGroup for ScriptNOfK {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer_sz(3u64, fit_sz(3u64, self.encodings.as_ref().map(|encs| encs.index_0_encoding.clone()).unwrap_or_default(), force_canonical))?;
        serializer.write_unsigned_integer_sz(self.n, fit_sz(self.n, self.encodings.as_ref().map(|encs| encs.n_encoding.clone()).unwrap_or_default(), force_canonical))?;
        serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.native_scripts_encoding.clone()).unwrap_or_default().to_len_sz(self.native_scripts.len() as u64, force_canonical))?;
        for element in self.native_scripts.iter() {
            element.serialize(serializer, force_canonical)?;
        }
        self.encodings.as_ref().map(|encs| encs.native_scripts_encoding.clone()).unwrap_or_default().end(serializer, force_canonical)?;
        self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().end(serializer, force_canonical)
    }
}

impl Deserialize for ScriptNOfK {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array_sz()?;
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(3)?;
            let ret = Self::deserialize_as_embedded_group(raw, &mut read_len, len);
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("ScriptNOfK"))
    }
}

impl DeserializeEmbeddedGroup for ScriptNOfK {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, read_len: &mut CBORReadLen, len: cbor_event::LenSz) -> Result<Self, DeserializeError> {
        let len_encoding = len.into();
        let index_0_encoding = (|| -> Result<_, DeserializeError> {
            let (index_0_value, index_0_encoding) = raw.unsigned_integer_sz()?;
            if index_0_value != 3 {
                return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(index_0_value), expected: Key::Uint(3) }.into());
            }
            Ok(Some(index_0_encoding))
        })().map_err(|e| e.annotate("index_0"))?;
        let (n, n_encoding) = (|| -> Result<_, DeserializeError> {
            Ok(raw.unsigned_integer_sz().map(|(x, enc)| (x, Some(enc)))?)
        })().map_err(|e| e.annotate("n"))?;
        let (native_scripts, native_scripts_encoding) = (|| -> Result<_, DeserializeError> {
            let mut native_scripts_arr = Vec::new();
            let len = raw.array_sz()?;
            let native_scripts_encoding = len.into();
            while match len { cbor_event::LenSz::Len(n, _) => native_scripts_arr.len() < n as usize, cbor_event::LenSz::Indefinite => true, } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                native_scripts_arr.push(NativeScript::deserialize(raw)?);
            }
            Ok((native_scripts_arr, native_scripts_encoding))
        })().map_err(|e| e.annotate("native_scripts"))?;
        Ok(ScriptNOfK {
            n,
            native_scripts,
            encodings: Some(ScriptNOfKEncoding {
                len_encoding,
                index_0_encoding,
                n_encoding,
                native_scripts_encoding,
            }),
        })
    }
}

impl Serialize for ScriptPubkey {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().to_len_sz(2, force_canonical))?;
        self.serialize_as_embedded_group(serializer, force_canonical)
    }
}

impl SerializeEmbeddedGroup for ScriptPubkey {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer_sz(0u64, fit_sz(0u64, self.encodings.as_ref().map(|encs| encs.index_0_encoding.clone()).unwrap_or_default(), force_canonical))?;
        self.addr_keyhash.serialize(serializer, force_canonical)?;
        self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().end(serializer, force_canonical)
    }
}

impl Deserialize for ScriptPubkey {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array_sz()?;
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(2)?;
            let ret = Self::deserialize_as_embedded_group(raw, &mut read_len, len);
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("ScriptPubkey"))
    }
}

impl DeserializeEmbeddedGroup for ScriptPubkey {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, read_len: &mut CBORReadLen, len: cbor_event::LenSz) -> Result<Self, DeserializeError> {
        let len_encoding = len.into();
        let index_0_encoding = (|| -> Result<_, DeserializeError> {
            let (index_0_value, index_0_encoding) = raw.unsigned_integer_sz()?;
            if index_0_value != 0 {
                return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(index_0_value), expected: Key::Uint(0) }.into());
            }
            Ok(Some(index_0_encoding))
        })().map_err(|e| e.annotate("index_0"))?;
        let addr_keyhash = (|| -> Result<_, DeserializeError> {
            Ok(Ed25519KeyHash::deserialize(raw)?)
        })().map_err(|e| e.annotate("addr_keyhash"))?;
        Ok(ScriptPubkey {
            addr_keyhash,
            encodings: Some(ScriptPubkeyEncoding {
                len_encoding,
                index_0_encoding,
            }),
        })
    }
}

impl Serialize for ShelleyMaAuxData {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().to_len_sz(2, force_canonical))?;
        serializer.write_map_sz(self.encodings.as_ref().map(|encs| encs.transaction_metadata_encoding.clone()).unwrap_or_default().to_len_sz(self.transaction_metadata.len() as u64, force_canonical))?;
        let mut key_order = self.transaction_metadata.iter().map(|(k, v)| {
            let mut buf = cbor_event::se::Serializer::new_vec();
            let transaction_metadata_key_encoding = self.encodings.as_ref().and_then(|encs| encs.transaction_metadata_key_encodings.get(k)).map(|e| e.clone()).unwrap_or_else(|| None);
            buf.write_unsigned_integer_sz(*k, fit_sz(*k, transaction_metadata_key_encoding, force_canonical))?;
            Ok((buf.finalize(), k, v))
        }).collect::<Result<Vec<(Vec<u8>, &_, &_)>, cbor_event::Error>>()?;
        if force_canonical {
            key_order.sort_by(|(lhs_bytes, _, _), (rhs_bytes, _, _)| {
                match lhs_bytes.len().cmp(&rhs_bytes.len()) {
                    std::cmp::Ordering::Equal => lhs_bytes.cmp(&rhs_bytes),
                    diff_ord => diff_ord,
                }
            });
        }
        for (key_bytes, key, value) in key_order {
            serializer.write_raw_bytes(&key_bytes)?;
            value.serialize(serializer, force_canonical)?;
        }
        self.encodings.as_ref().map(|encs| encs.transaction_metadata_encoding.clone()).unwrap_or_default().end(serializer, force_canonical)?;
        serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.auxiliary_scripts_encoding.clone()).unwrap_or_default().to_len_sz(self.auxiliary_scripts.len() as u64, force_canonical))?;
        for element in self.auxiliary_scripts.iter() {
            element.serialize(serializer, force_canonical)?;
        }
        self.encodings.as_ref().map(|encs| encs.auxiliary_scripts_encoding.clone()).unwrap_or_default().end(serializer, force_canonical)?;
        self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().end(serializer, force_canonical)
    }
}

impl Deserialize for ShelleyMaAuxData {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array_sz()?;
            let len_encoding: LenEncoding = len.into();
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(2)?;
            let (transaction_metadata, transaction_metadata_encoding, transaction_metadata_key_encodings) = (|| -> Result<_, DeserializeError> {
                let mut transaction_metadata_table = OrderedHashMap::new();
                let transaction_metadata_len = raw.map_sz()?;
                let transaction_metadata_encoding = transaction_metadata_len.into();
                let mut transaction_metadata_key_encodings = BTreeMap::new();
                while match transaction_metadata_len { cbor_event::LenSz::Len(n, _) => transaction_metadata_table.len() < n as usize, cbor_event::LenSz::Indefinite => true, } {
                    if raw.cbor_type()? == CBORType::Special {
                        assert_eq!(raw.special()?, CBORSpecial::Break);
                        break;
                    }
                    let (transaction_metadata_key, transaction_metadata_key_encoding) = raw.unsigned_integer_sz().map(|(x, enc)| (x, Some(enc)))?;
                    let transaction_metadata_value = TransactionMetadatum::deserialize(raw)?;
                    if transaction_metadata_table.insert(transaction_metadata_key.clone(), transaction_metadata_value).is_some() {
                        return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from("some complicated/unsupported type"))).into());
                    }
                    transaction_metadata_key_encodings.insert(transaction_metadata_key.clone(), transaction_metadata_key_encoding);
                }
                Ok((transaction_metadata_table, transaction_metadata_encoding, transaction_metadata_key_encodings))
            })().map_err(|e| e.annotate("transaction_metadata"))?;
            let (auxiliary_scripts, auxiliary_scripts_encoding) = (|| -> Result<_, DeserializeError> {
                let mut auxiliary_scripts_arr = Vec::new();
                let len = raw.array_sz()?;
                let auxiliary_scripts_encoding = len.into();
                while match len { cbor_event::LenSz::Len(n, _) => auxiliary_scripts_arr.len() < n as usize, cbor_event::LenSz::Indefinite => true, } {
                    if raw.cbor_type()? == CBORType::Special {
                        assert_eq!(raw.special()?, CBORSpecial::Break);
                        break;
                    }
                    auxiliary_scripts_arr.push(NativeScript::deserialize(raw)?);
                }
                Ok((auxiliary_scripts_arr, auxiliary_scripts_encoding))
            })().map_err(|e| e.annotate("auxiliary_scripts"))?;
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(ShelleyMaAuxData {
                transaction_metadata,
                auxiliary_scripts,
                encodings: Some(ShelleyMaAuxDataEncoding {
                    len_encoding,
                    transaction_metadata_encoding,
                    transaction_metadata_key_encodings,
                    auxiliary_scripts_encoding,
                }),
            })
        })().map_err(|e| e.annotate("ShelleyMaAuxData"))
    }
}

impl Serialize for ShelleyTxOut {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().to_len_sz(2, force_canonical))?;
        self.address.serialize(serializer, force_canonical)?;
        self.amount.serialize(serializer, force_canonical)?;
        self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().end(serializer, force_canonical)
    }
}

impl Deserialize for ShelleyTxOut {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array_sz()?;
            let len_encoding: LenEncoding = len.into();
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(2)?;
            let address = (|| -> Result<_, DeserializeError> {
                Ok(Address::deserialize(raw)?)
            })().map_err(|e| e.annotate("address"))?;
            let amount = (|| -> Result<_, DeserializeError> {
                Ok(Value::deserialize(raw)?)
            })().map_err(|e| e.annotate("amount"))?;
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(ShelleyTxOut {
                address,
                amount,
                encodings: Some(ShelleyTxOutEncoding {
                    len_encoding,
                }),
            })
        })().map_err(|e| e.annotate("ShelleyTxOut"))
    }
}

impl Serialize for SingleHostAddr {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().to_len_sz(4, force_canonical))?;
        self.serialize_as_embedded_group(serializer, force_canonical)
    }
}

impl SerializeEmbeddedGroup for SingleHostAddr {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer_sz(0u64, fit_sz(0u64, self.encodings.as_ref().map(|encs| encs.index_0_encoding.clone()).unwrap_or_default(), force_canonical))?;
        match &self.port {
            Some(x) => {
                serializer.write_unsigned_integer_sz(*x as u64, fit_sz(*x as u64, self.encodings.as_ref().map(|encs| encs.port_encoding.clone()).unwrap_or_default(), force_canonical))
            },
            None => serializer.write_special(CBORSpecial::Null),
        }?;
        match &self.ipv4 {
            Some(x) => {
                x.serialize(serializer, force_canonical)
            },
            None => serializer.write_special(CBORSpecial::Null),
        }?;
        match &self.ipv6 {
            Some(x) => {
                x.serialize(serializer, force_canonical)
            },
            None => serializer.write_special(CBORSpecial::Null),
        }?;
        self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().end(serializer, force_canonical)
    }
}

impl Deserialize for SingleHostAddr {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array_sz()?;
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(4)?;
            let ret = Self::deserialize_as_embedded_group(raw, &mut read_len, len);
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("SingleHostAddr"))
    }
}

impl DeserializeEmbeddedGroup for SingleHostAddr {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, read_len: &mut CBORReadLen, len: cbor_event::LenSz) -> Result<Self, DeserializeError> {
        let len_encoding = len.into();
        let index_0_encoding = (|| -> Result<_, DeserializeError> {
            let (index_0_value, index_0_encoding) = raw.unsigned_integer_sz()?;
            if index_0_value != 0 {
                return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(index_0_value), expected: Key::Uint(0) }.into());
            }
            Ok(Some(index_0_encoding))
        })().map_err(|e| e.annotate("index_0"))?;
        let (port, port_encoding) = (|| -> Result<_, DeserializeError> {
            Ok(match raw.cbor_type()? != CBORType::Special {
                true => {
                    Result::<_, DeserializeError>::Ok(raw.unsigned_integer_sz().map(|(x, enc)| (x as u16, Some(enc)))?).map(|(x, port_encoding)| (Some(x), port_encoding))?
                },
                false => {
                    if raw.special()? != CBORSpecial::Null {
                        return Err(DeserializeFailure::ExpectedNull.into());
                    }
                    (None, None)
                }
            })
        })().map_err(|e| e.annotate("port"))?;
        let ipv4 = (|| -> Result<_, DeserializeError> {
            Ok(match raw.cbor_type()? != CBORType::Special {
                true => {
                    Some(Ipv4::deserialize(raw)?)
                },
                false => {
                    if raw.special()? != CBORSpecial::Null {
                        return Err(DeserializeFailure::ExpectedNull.into());
                    }
                    None
                }
            })
        })().map_err(|e| e.annotate("ipv4"))?;
        let ipv6 = (|| -> Result<_, DeserializeError> {
            Ok(match raw.cbor_type()? != CBORType::Special {
                true => {
                    Some(Ipv6::deserialize(raw)?)
                },
                false => {
                    if raw.special()? != CBORSpecial::Null {
                        return Err(DeserializeFailure::ExpectedNull.into());
                    }
                    None
                }
            })
        })().map_err(|e| e.annotate("ipv6"))?;
        Ok(SingleHostAddr {
            port,
            ipv4,
            ipv6,
            encodings: Some(SingleHostAddrEncoding {
                len_encoding,
                index_0_encoding,
                port_encoding,
            }),
        })
    }
}

impl Serialize for SingleHostName {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().to_len_sz(3, force_canonical))?;
        self.serialize_as_embedded_group(serializer, force_canonical)
    }
}

impl SerializeEmbeddedGroup for SingleHostName {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer_sz(1u64, fit_sz(1u64, self.encodings.as_ref().map(|encs| encs.index_0_encoding.clone()).unwrap_or_default(), force_canonical))?;
        match &self.port {
            Some(x) => {
                serializer.write_unsigned_integer_sz(*x as u64, fit_sz(*x as u64, self.encodings.as_ref().map(|encs| encs.port_encoding.clone()).unwrap_or_default(), force_canonical))
            },
            None => serializer.write_special(CBORSpecial::Null),
        }?;
        self.dns_name.serialize(serializer, force_canonical)?;
        self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().end(serializer, force_canonical)
    }
}

impl Deserialize for SingleHostName {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array_sz()?;
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(3)?;
            let ret = Self::deserialize_as_embedded_group(raw, &mut read_len, len);
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("SingleHostName"))
    }
}

impl DeserializeEmbeddedGroup for SingleHostName {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, read_len: &mut CBORReadLen, len: cbor_event::LenSz) -> Result<Self, DeserializeError> {
        let len_encoding = len.into();
        let index_0_encoding = (|| -> Result<_, DeserializeError> {
            let (index_0_value, index_0_encoding) = raw.unsigned_integer_sz()?;
            if index_0_value != 1 {
                return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(index_0_value), expected: Key::Uint(1) }.into());
            }
            Ok(Some(index_0_encoding))
        })().map_err(|e| e.annotate("index_0"))?;
        let (port, port_encoding) = (|| -> Result<_, DeserializeError> {
            Ok(match raw.cbor_type()? != CBORType::Special {
                true => {
                    Result::<_, DeserializeError>::Ok(raw.unsigned_integer_sz().map(|(x, enc)| (x as u16, Some(enc)))?).map(|(x, port_encoding)| (Some(x), port_encoding))?
                },
                false => {
                    if raw.special()? != CBORSpecial::Null {
                        return Err(DeserializeFailure::ExpectedNull.into());
                    }
                    (None, None)
                }
            })
        })().map_err(|e| e.annotate("port"))?;
        let dns_name = (|| -> Result<_, DeserializeError> {
            Ok(DnsName::deserialize(raw)?)
        })().map_err(|e| e.annotate("dns_name"))?;
        Ok(SingleHostName {
            port,
            dns_name,
            encodings: Some(SingleHostNameEncoding {
                len_encoding,
                index_0_encoding,
                port_encoding,
            }),
        })
    }
}

impl Serialize for StakeDelegation {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().to_len_sz(3, force_canonical))?;
        self.serialize_as_embedded_group(serializer, force_canonical)
    }
}

impl SerializeEmbeddedGroup for StakeDelegation {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer_sz(2u64, fit_sz(2u64, self.encodings.as_ref().map(|encs| encs.index_0_encoding.clone()).unwrap_or_default(), force_canonical))?;
        self.stake_credential.serialize(serializer, force_canonical)?;
        self.pool_keyhash.serialize(serializer, force_canonical)?;
        self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().end(serializer, force_canonical)
    }
}

impl Deserialize for StakeDelegation {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array_sz()?;
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(3)?;
            let ret = Self::deserialize_as_embedded_group(raw, &mut read_len, len);
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("StakeDelegation"))
    }
}

impl DeserializeEmbeddedGroup for StakeDelegation {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, read_len: &mut CBORReadLen, len: cbor_event::LenSz) -> Result<Self, DeserializeError> {
        let len_encoding = len.into();
        let index_0_encoding = (|| -> Result<_, DeserializeError> {
            let (index_0_value, index_0_encoding) = raw.unsigned_integer_sz()?;
            if index_0_value != 2 {
                return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(index_0_value), expected: Key::Uint(2) }.into());
            }
            Ok(Some(index_0_encoding))
        })().map_err(|e| e.annotate("index_0"))?;
        let stake_credential = (|| -> Result<_, DeserializeError> {
            Ok(StakeCredential::deserialize(raw)?)
        })().map_err(|e| e.annotate("stake_credential"))?;
        let pool_keyhash = (|| -> Result<_, DeserializeError> {
            Ok(Ed25519KeyHash::deserialize(raw)?)
        })().map_err(|e| e.annotate("pool_keyhash"))?;
        Ok(StakeDelegation {
            stake_credential,
            pool_keyhash,
            encodings: Some(StakeDelegationEncoding {
                len_encoding,
                index_0_encoding,
            }),
        })
    }
}

impl Serialize for StakeDeregistration {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().to_len_sz(2, force_canonical))?;
        self.serialize_as_embedded_group(serializer, force_canonical)
    }
}

impl SerializeEmbeddedGroup for StakeDeregistration {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer_sz(1u64, fit_sz(1u64, self.encodings.as_ref().map(|encs| encs.index_0_encoding.clone()).unwrap_or_default(), force_canonical))?;
        self.stake_credential.serialize(serializer, force_canonical)?;
        self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().end(serializer, force_canonical)
    }
}

impl Deserialize for StakeDeregistration {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array_sz()?;
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(2)?;
            let ret = Self::deserialize_as_embedded_group(raw, &mut read_len, len);
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("StakeDeregistration"))
    }
}

impl DeserializeEmbeddedGroup for StakeDeregistration {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, read_len: &mut CBORReadLen, len: cbor_event::LenSz) -> Result<Self, DeserializeError> {
        let len_encoding = len.into();
        let index_0_encoding = (|| -> Result<_, DeserializeError> {
            let (index_0_value, index_0_encoding) = raw.unsigned_integer_sz()?;
            if index_0_value != 1 {
                return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(index_0_value), expected: Key::Uint(1) }.into());
            }
            Ok(Some(index_0_encoding))
        })().map_err(|e| e.annotate("index_0"))?;
        let stake_credential = (|| -> Result<_, DeserializeError> {
            Ok(StakeCredential::deserialize(raw)?)
        })().map_err(|e| e.annotate("stake_credential"))?;
        Ok(StakeDeregistration {
            stake_credential,
            encodings: Some(StakeDeregistrationEncoding {
                len_encoding,
                index_0_encoding,
            }),
        })
    }
}

impl Serialize for StakeRegistration {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().to_len_sz(2, force_canonical))?;
        self.serialize_as_embedded_group(serializer, force_canonical)
    }
}

impl SerializeEmbeddedGroup for StakeRegistration {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer_sz(0u64, fit_sz(0u64, self.encodings.as_ref().map(|encs| encs.index_0_encoding.clone()).unwrap_or_default(), force_canonical))?;
        self.stake_credential.serialize(serializer, force_canonical)?;
        self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().end(serializer, force_canonical)
    }
}

impl Deserialize for StakeRegistration {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array_sz()?;
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(2)?;
            let ret = Self::deserialize_as_embedded_group(raw, &mut read_len, len);
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("StakeRegistration"))
    }
}

impl DeserializeEmbeddedGroup for StakeRegistration {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, read_len: &mut CBORReadLen, len: cbor_event::LenSz) -> Result<Self, DeserializeError> {
        let len_encoding = len.into();
        let index_0_encoding = (|| -> Result<_, DeserializeError> {
            let (index_0_value, index_0_encoding) = raw.unsigned_integer_sz()?;
            if index_0_value != 0 {
                return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(index_0_value), expected: Key::Uint(0) }.into());
            }
            Ok(Some(index_0_encoding))
        })().map_err(|e| e.annotate("index_0"))?;
        let stake_credential = (|| -> Result<_, DeserializeError> {
            Ok(StakeCredential::deserialize(raw)?)
        })().map_err(|e| e.annotate("stake_credential"))?;
        Ok(StakeRegistration {
            stake_credential,
            encodings: Some(StakeRegistrationEncoding {
                len_encoding,
                index_0_encoding,
            }),
        })
    }
}

impl Serialize for Transaction {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().to_len_sz(4, force_canonical))?;
        self.transaction_body.serialize(serializer, force_canonical)?;
        self.transaction_witness_set.serialize(serializer, force_canonical)?;
        serializer.write_special(cbor_event::Special::Bool(self.index_2))?;
        match &self.auxiliary_data {
            Some(x) => {
                x.serialize(serializer, force_canonical)
            },
            None => serializer.write_special(CBORSpecial::Null),
        }?;
        self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().end(serializer, force_canonical)
    }
}

impl Deserialize for Transaction {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array_sz()?;
            let len_encoding: LenEncoding = len.into();
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(4)?;
            let transaction_body = (|| -> Result<_, DeserializeError> {
                Ok(TransactionBody::deserialize(raw)?)
            })().map_err(|e| e.annotate("transaction_body"))?;
            let transaction_witness_set = (|| -> Result<_, DeserializeError> {
                Ok(TransactionWitnessSet::deserialize(raw)?)
            })().map_err(|e| e.annotate("transaction_witness_set"))?;
            let index_2 = (|| -> Result<_, DeserializeError> {
                Ok(bool::deserialize(raw)?)
            })().map_err(|e| e.annotate("index_2"))?;
            let auxiliary_data = (|| -> Result<_, DeserializeError> {
                Ok(match raw.cbor_type()? != CBORType::Special {
                    true => {
                        Some(AuxiliaryData::deserialize(raw)?)
                    },
                    false => {
                        if raw.special()? != CBORSpecial::Null {
                            return Err(DeserializeFailure::ExpectedNull.into());
                        }
                        None
                    }
                })
            })().map_err(|e| e.annotate("auxiliary_data"))?;
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(Transaction {
                transaction_body,
                transaction_witness_set,
                index_2,
                auxiliary_data,
                encodings: Some(TransactionEncoding {
                    len_encoding,
                }),
            })
        })().map_err(|e| e.annotate("Transaction"))
    }
}

impl Serialize for TransactionBody {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_map_sz(self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().to_len_sz(3 + match &self.key_3 { Some(_) => 1, None => 0 } + match &self.key_4 { Some(_) => 1, None => 0 } + match &self.key_5 { Some(_) => 1, None => 0 } + match &self.key_6 { Some(_) => 1, None => 0 } + match &self.key_7 { Some(_) => 1, None => 0 } + match &self.key_8 { Some(_) => 1, None => 0 } + match &self.key_9 { Some(_) => 1, None => 0 } + match &self.key_11 { Some(_) => 1, None => 0 } + match &self.key_13 { Some(_) => 1, None => 0 } + match &self.key_14 { Some(_) => 1, None => 0 } + match &self.key_15 { Some(_) => 1, None => 0 } + match &self.key_16 { Some(_) => 1, None => 0 } + match &self.key_17 { Some(_) => 1, None => 0 } + match &self.key_18 { Some(_) => 1, None => 0 }, force_canonical))?;
        let deser_order = self.encodings.as_ref().filter(|encs| !force_canonical && encs.orig_deser_order.len() == 3 + match &self.key_3 { Some(_) => 1, None => 0 } + match &self.key_4 { Some(_) => 1, None => 0 } + match &self.key_5 { Some(_) => 1, None => 0 } + match &self.key_6 { Some(_) => 1, None => 0 } + match &self.key_7 { Some(_) => 1, None => 0 } + match &self.key_8 { Some(_) => 1, None => 0 } + match &self.key_9 { Some(_) => 1, None => 0 } + match &self.key_11 { Some(_) => 1, None => 0 } + match &self.key_13 { Some(_) => 1, None => 0 } + match &self.key_14 { Some(_) => 1, None => 0 } + match &self.key_15 { Some(_) => 1, None => 0 } + match &self.key_16 { Some(_) => 1, None => 0 } + match &self.key_17 { Some(_) => 1, None => 0 } + match &self.key_18 { Some(_) => 1, None => 0 }).map(|encs| encs.orig_deser_order.clone()).unwrap_or_else(|| vec![0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16]);
        for field_index in deser_order {
            match field_index {
                0 => {
                    serializer.write_unsigned_integer_sz(0u64, fit_sz(0u64, self.encodings.as_ref().map(|encs| encs.key_0_key_encoding.clone()).unwrap_or_default(), force_canonical))?;
                    serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.key_0_encoding.clone()).unwrap_or_default().to_len_sz(self.key_0.len() as u64, force_canonical))?;
                    for element in self.key_0.iter() {
                        element.serialize(serializer, force_canonical)?;
                    }
                    self.encodings.as_ref().map(|encs| encs.key_0_encoding.clone()).unwrap_or_default().end(serializer, force_canonical)?;
                }
                1 => {
                    serializer.write_unsigned_integer_sz(1u64, fit_sz(1u64, self.encodings.as_ref().map(|encs| encs.key_1_key_encoding.clone()).unwrap_or_default(), force_canonical))?;
                    serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.key_1_encoding.clone()).unwrap_or_default().to_len_sz(self.key_1.len() as u64, force_canonical))?;
                    for element in self.key_1.iter() {
                        element.serialize(serializer, force_canonical)?;
                    }
                    self.encodings.as_ref().map(|encs| encs.key_1_encoding.clone()).unwrap_or_default().end(serializer, force_canonical)?;
                }
                2 => {
                    serializer.write_unsigned_integer_sz(2u64, fit_sz(2u64, self.encodings.as_ref().map(|encs| encs.key_2_key_encoding.clone()).unwrap_or_default(), force_canonical))?;
                    serializer.write_unsigned_integer_sz(self.key_2, fit_sz(self.key_2, self.encodings.as_ref().map(|encs| encs.key_2_encoding.clone()).unwrap_or_default(), force_canonical))?;
                }
                3 => if let Some(field) = &self.key_3 {
                    serializer.write_unsigned_integer_sz(3u64, fit_sz(3u64, self.encodings.as_ref().map(|encs| encs.key_3_key_encoding.clone()).unwrap_or_default(), force_canonical))?;
                    serializer.write_unsigned_integer_sz(*field, fit_sz(*field, self.encodings.as_ref().map(|encs| encs.key_3_encoding.clone()).unwrap_or_default(), force_canonical))?;
                }
                4 => if let Some(field) = &self.key_4 {
                    serializer.write_unsigned_integer_sz(4u64, fit_sz(4u64, self.encodings.as_ref().map(|encs| encs.key_4_key_encoding.clone()).unwrap_or_default(), force_canonical))?;
                    serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.key_4_encoding.clone()).unwrap_or_default().to_len_sz(field.len() as u64, force_canonical))?;
                    for element in field.iter() {
                        element.serialize(serializer, force_canonical)?;
                    }
                    self.encodings.as_ref().map(|encs| encs.key_4_encoding.clone()).unwrap_or_default().end(serializer, force_canonical)?;
                }
                5 => if let Some(field) = &self.key_5 {
                    serializer.write_unsigned_integer_sz(5u64, fit_sz(5u64, self.encodings.as_ref().map(|encs| encs.key_5_key_encoding.clone()).unwrap_or_default(), force_canonical))?;
                    serializer.write_map_sz(self.encodings.as_ref().map(|encs| encs.key_5_encoding.clone()).unwrap_or_default().to_len_sz(field.len() as u64, force_canonical))?;
                    let mut key_order = field.iter().map(|(k, v)| {
                        let mut buf = cbor_event::se::Serializer::new_vec();
                        k.serialize(&mut buf, force_canonical)?;
                        Ok((buf.finalize(), k, v))
                    }).collect::<Result<Vec<(Vec<u8>, &_, &_)>, cbor_event::Error>>()?;
                    if force_canonical {
                        key_order.sort_by(|(lhs_bytes, _, _), (rhs_bytes, _, _)| {
                            match lhs_bytes.len().cmp(&rhs_bytes.len()) {
                                std::cmp::Ordering::Equal => lhs_bytes.cmp(&rhs_bytes),
                                diff_ord => diff_ord,
                            }
                        });
                    }
                    for (key_bytes, key, value) in key_order {
                        serializer.write_raw_bytes(&key_bytes)?;
                        let key_5_value_encoding = self.encodings.as_ref().and_then(|encs| encs.key_5_value_encodings.get(key)).map(|e| e.clone()).unwrap_or_else(|| None);
                        serializer.write_unsigned_integer_sz(*value, fit_sz(*value, key_5_value_encoding, force_canonical))?;
                    }
                    self.encodings.as_ref().map(|encs| encs.key_5_encoding.clone()).unwrap_or_default().end(serializer, force_canonical)?;
                }
                6 => if let Some(field) = &self.key_6 {
                    serializer.write_unsigned_integer_sz(6u64, fit_sz(6u64, self.encodings.as_ref().map(|encs| encs.key_6_key_encoding.clone()).unwrap_or_default(), force_canonical))?;
                    field.serialize(serializer, force_canonical)?;
                }
                7 => if let Some(field) = &self.key_7 {
                    serializer.write_unsigned_integer_sz(7u64, fit_sz(7u64, self.encodings.as_ref().map(|encs| encs.key_7_key_encoding.clone()).unwrap_or_default(), force_canonical))?;
                    field.serialize(serializer, force_canonical)?;
                }
                8 => if let Some(field) = &self.key_8 {
                    serializer.write_unsigned_integer_sz(8u64, fit_sz(8u64, self.encodings.as_ref().map(|encs| encs.key_8_key_encoding.clone()).unwrap_or_default(), force_canonical))?;
                    serializer.write_unsigned_integer_sz(*field, fit_sz(*field, self.encodings.as_ref().map(|encs| encs.key_8_encoding.clone()).unwrap_or_default(), force_canonical))?;
                }
                9 => if let Some(field) = &self.key_9 {
                    serializer.write_unsigned_integer_sz(9u64, fit_sz(9u64, self.encodings.as_ref().map(|encs| encs.key_9_key_encoding.clone()).unwrap_or_default(), force_canonical))?;
                    serializer.write_map_sz(self.encodings.as_ref().map(|encs| encs.key_9_encoding.clone()).unwrap_or_default().to_len_sz(field.len() as u64, force_canonical))?;
                    let mut key_order = field.iter().map(|(k, v)| {
                        let mut buf = cbor_event::se::Serializer::new_vec();
                        k.serialize(&mut buf, force_canonical)?;
                        Ok((buf.finalize(), k, v))
                    }).collect::<Result<Vec<(Vec<u8>, &_, &_)>, cbor_event::Error>>()?;
                    if force_canonical {
                        key_order.sort_by(|(lhs_bytes, _, _), (rhs_bytes, _, _)| {
                            match lhs_bytes.len().cmp(&rhs_bytes.len()) {
                                std::cmp::Ordering::Equal => lhs_bytes.cmp(&rhs_bytes),
                                diff_ord => diff_ord,
                            }
                        });
                    }
                    for (key_bytes, key, value) in key_order {
                        serializer.write_raw_bytes(&key_bytes)?;
                        let (key_9_value_encoding, key_9_value_value_encodings) = self.encodings.as_ref().and_then(|encs| encs.key_9_value_encodings.get(key)).map(|e| e.clone()).unwrap_or_else(|| (LenEncoding::default(), BTreeMap::new()));
                        serializer.write_map_sz(key_9_value_encoding.to_len_sz(value.len() as u64, force_canonical))?;
                        let mut key_order = value.iter().map(|(k, v)| {
                            let mut buf = cbor_event::se::Serializer::new_vec();
                            k.serialize(&mut buf, force_canonical)?;
                            Ok((buf.finalize(), k, v))
                        }).collect::<Result<Vec<(Vec<u8>, &_, &_)>, cbor_event::Error>>()?;
                        if force_canonical {
                            key_order.sort_by(|(lhs_bytes, _, _), (rhs_bytes, _, _)| {
                                match lhs_bytes.len().cmp(&rhs_bytes.len()) {
                                    std::cmp::Ordering::Equal => lhs_bytes.cmp(&rhs_bytes),
                                    diff_ord => diff_ord,
                                }
                            });
                        }
                        for (key_bytes, key, value) in key_order {
                            serializer.write_raw_bytes(&key_bytes)?;
                            let key_9_value_value_encoding = key_9_value_value_encodings.get(key).map(|e| e.clone()).unwrap_or_else(|| None);
                            if *value >= 0 {
                                serializer.write_unsigned_integer_sz(*value as u64, fit_sz(*value as u64, key_9_value_value_encoding, force_canonical))?;
                            }
                            else {
                                serializer.write_negative_integer_sz(*value as i128, fit_sz((*value + 1).abs() as u64, key_9_value_value_encoding, force_canonical))?;
                            }
                        }
                        key_9_value_encoding.end(serializer, force_canonical)?;
                    }
                    self.encodings.as_ref().map(|encs| encs.key_9_encoding.clone()).unwrap_or_default().end(serializer, force_canonical)?;
                }
                10 => if let Some(field) = &self.key_11 {
                    serializer.write_unsigned_integer_sz(11u64, fit_sz(11u64, self.encodings.as_ref().map(|encs| encs.key_11_key_encoding.clone()).unwrap_or_default(), force_canonical))?;
                    field.serialize(serializer, force_canonical)?;
                }
                11 => if let Some(field) = &self.key_13 {
                    serializer.write_unsigned_integer_sz(13u64, fit_sz(13u64, self.encodings.as_ref().map(|encs| encs.key_13_key_encoding.clone()).unwrap_or_default(), force_canonical))?;
                    serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.key_13_encoding.clone()).unwrap_or_default().to_len_sz(field.len() as u64, force_canonical))?;
                    for element in field.iter() {
                        element.serialize(serializer, force_canonical)?;
                    }
                    self.encodings.as_ref().map(|encs| encs.key_13_encoding.clone()).unwrap_or_default().end(serializer, force_canonical)?;
                }
                12 => if let Some(field) = &self.key_14 {
                    serializer.write_unsigned_integer_sz(14u64, fit_sz(14u64, self.encodings.as_ref().map(|encs| encs.key_14_key_encoding.clone()).unwrap_or_default(), force_canonical))?;
                    field.serialize(serializer, force_canonical)?;
                }
                13 => if let Some(field) = &self.key_15 {
                    serializer.write_unsigned_integer_sz(15u64, fit_sz(15u64, self.encodings.as_ref().map(|encs| encs.key_15_key_encoding.clone()).unwrap_or_default(), force_canonical))?;
                    field.serialize(serializer, force_canonical)?;
                }
                14 => if let Some(field) = &self.key_16 {
                    serializer.write_unsigned_integer_sz(16u64, fit_sz(16u64, self.encodings.as_ref().map(|encs| encs.key_16_key_encoding.clone()).unwrap_or_default(), force_canonical))?;
                    field.serialize(serializer, force_canonical)?;
                }
                15 => if let Some(field) = &self.key_17 {
                    serializer.write_unsigned_integer_sz(17u64, fit_sz(17u64, self.encodings.as_ref().map(|encs| encs.key_17_key_encoding.clone()).unwrap_or_default(), force_canonical))?;
                    serializer.write_unsigned_integer_sz(*field, fit_sz(*field, self.encodings.as_ref().map(|encs| encs.key_17_encoding.clone()).unwrap_or_default(), force_canonical))?;
                }
                16 => if let Some(field) = &self.key_18 {
                    serializer.write_unsigned_integer_sz(18u64, fit_sz(18u64, self.encodings.as_ref().map(|encs| encs.key_18_key_encoding.clone()).unwrap_or_default(), force_canonical))?;
                    serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.key_18_encoding.clone()).unwrap_or_default().to_len_sz(field.len() as u64, force_canonical))?;
                    for element in field.iter() {
                        element.serialize(serializer, force_canonical)?;
                    }
                    self.encodings.as_ref().map(|encs| encs.key_18_encoding.clone()).unwrap_or_default().end(serializer, force_canonical)?;
                }
                _ => unreachable!()
            };
        }
        self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().end(serializer, force_canonical)
    }
}

impl Deserialize for TransactionBody {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.map_sz()?;
            let len_encoding: LenEncoding = len.into();
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(3)?;
            let mut orig_deser_order = Vec::new();
            let mut key_0_encoding = LenEncoding::default();
            let mut key_0_key_encoding = None;
            let mut key_0 = None;
            let mut key_1_encoding = LenEncoding::default();
            let mut key_1_key_encoding = None;
            let mut key_1 = None;
            let mut key_2_encoding = None;
            let mut key_2_key_encoding = None;
            let mut key_2 = None;
            let mut key_3_encoding = None;
            let mut key_3_key_encoding = None;
            let mut key_3 = None;
            let mut key_4_encoding = LenEncoding::default();
            let mut key_4_key_encoding = None;
            let mut key_4 = None;
            let mut key_5_encoding = LenEncoding::default();
            let mut key_5_value_encodings = BTreeMap::new();
            let mut key_5_key_encoding = None;
            let mut key_5 = None;
            let mut key_6_key_encoding = None;
            let mut key_6 = None;
            let mut key_7_key_encoding = None;
            let mut key_7 = None;
            let mut key_8_encoding = None;
            let mut key_8_key_encoding = None;
            let mut key_8 = None;
            let mut key_9_encoding = LenEncoding::default();
            let mut key_9_value_encodings = BTreeMap::new();
            let mut key_9_key_encoding = None;
            let mut key_9 = None;
            let mut key_11_key_encoding = None;
            let mut key_11 = None;
            let mut key_13_encoding = LenEncoding::default();
            let mut key_13_key_encoding = None;
            let mut key_13 = None;
            let mut key_14_key_encoding = None;
            let mut key_14 = None;
            let mut key_15_key_encoding = None;
            let mut key_15 = None;
            let mut key_16_key_encoding = None;
            let mut key_16 = None;
            let mut key_17_encoding = None;
            let mut key_17_key_encoding = None;
            let mut key_17 = None;
            let mut key_18_encoding = LenEncoding::default();
            let mut key_18_key_encoding = None;
            let mut key_18 = None;
            let mut read = 0;
            while match len { cbor_event::LenSz::Len(n, _) => read < n as usize, cbor_event::LenSz::Indefinite => true, } {
                match raw.cbor_type()? {
                    CBORType::UnsignedInteger => match raw.unsigned_integer_sz()? {
                        (0, key_enc) =>  {
                            if key_0.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(0)).into());
                            }
                            let (tmp_key_0, tmp_key_0_encoding) = (|| -> Result<_, DeserializeError> {
                                let mut key_0_arr = Vec::new();
                                let len = raw.array_sz()?;
                                let key_0_encoding = len.into();
                                while match len { cbor_event::LenSz::Len(n, _) => key_0_arr.len() < n as usize, cbor_event::LenSz::Indefinite => true, } {
                                    if raw.cbor_type()? == CBORType::Special {
                                        assert_eq!(raw.special()?, CBORSpecial::Break);
                                        break;
                                    }
                                    key_0_arr.push(TransactionInput::deserialize(raw)?);
                                }
                                Ok((key_0_arr, key_0_encoding))
                            })().map_err(|e| e.annotate("key_0"))?;
                            key_0 = Some(tmp_key_0);
                            key_0_encoding = tmp_key_0_encoding;
                            key_0_key_encoding = Some(key_enc);
                            orig_deser_order.push(0);
                        },
                        (1, key_enc) =>  {
                            if key_1.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(1)).into());
                            }
                            let (tmp_key_1, tmp_key_1_encoding) = (|| -> Result<_, DeserializeError> {
                                let mut key_1_arr = Vec::new();
                                let len = raw.array_sz()?;
                                let key_1_encoding = len.into();
                                while match len { cbor_event::LenSz::Len(n, _) => key_1_arr.len() < n as usize, cbor_event::LenSz::Indefinite => true, } {
                                    if raw.cbor_type()? == CBORType::Special {
                                        assert_eq!(raw.special()?, CBORSpecial::Break);
                                        break;
                                    }
                                    key_1_arr.push(TransactionOutput::deserialize(raw)?);
                                }
                                Ok((key_1_arr, key_1_encoding))
                            })().map_err(|e| e.annotate("key_1"))?;
                            key_1 = Some(tmp_key_1);
                            key_1_encoding = tmp_key_1_encoding;
                            key_1_key_encoding = Some(key_enc);
                            orig_deser_order.push(1);
                        },
                        (2, key_enc) =>  {
                            if key_2.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(2)).into());
                            }
                            let (tmp_key_2, tmp_key_2_encoding) = (|| -> Result<_, DeserializeError> {
                                Ok(raw.unsigned_integer_sz().map(|(x, enc)| (x, Some(enc)))?)
                            })().map_err(|e| e.annotate("key_2"))?;
                            key_2 = Some(tmp_key_2);
                            key_2_encoding = tmp_key_2_encoding;
                            key_2_key_encoding = Some(key_enc);
                            orig_deser_order.push(2);
                        },
                        (3, key_enc) =>  {
                            if key_3.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(3)).into());
                            }
                            let (tmp_key_3, tmp_key_3_encoding) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                Ok(raw.unsigned_integer_sz().map(|(x, enc)| (x, Some(enc)))?)
                            })().map_err(|e| e.annotate("key_3"))?;
                            key_3 = Some(tmp_key_3);
                            key_3_encoding = tmp_key_3_encoding;
                            key_3_key_encoding = Some(key_enc);
                            orig_deser_order.push(3);
                        },
                        (4, key_enc) =>  {
                            if key_4.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(4)).into());
                            }
                            let (tmp_key_4, tmp_key_4_encoding) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                let mut key_4_arr = Vec::new();
                                let len = raw.array_sz()?;
                                let key_4_encoding = len.into();
                                while match len { cbor_event::LenSz::Len(n, _) => key_4_arr.len() < n as usize, cbor_event::LenSz::Indefinite => true, } {
                                    if raw.cbor_type()? == CBORType::Special {
                                        assert_eq!(raw.special()?, CBORSpecial::Break);
                                        break;
                                    }
                                    key_4_arr.push(Certificate::deserialize(raw)?);
                                }
                                Ok((key_4_arr, key_4_encoding))
                            })().map_err(|e| e.annotate("key_4"))?;
                            key_4 = Some(tmp_key_4);
                            key_4_encoding = tmp_key_4_encoding;
                            key_4_key_encoding = Some(key_enc);
                            orig_deser_order.push(4);
                        },
                        (5, key_enc) =>  {
                            if key_5.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(5)).into());
                            }
                            let (tmp_key_5, tmp_key_5_encoding, tmp_key_5_value_encodings) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                let mut key_5_table = OrderedHashMap::new();
                                let key_5_len = raw.map_sz()?;
                                let key_5_encoding = key_5_len.into();
                                let mut key_5_value_encodings = BTreeMap::new();
                                while match key_5_len { cbor_event::LenSz::Len(n, _) => key_5_table.len() < n as usize, cbor_event::LenSz::Indefinite => true, } {
                                    if raw.cbor_type()? == CBORType::Special {
                                        assert_eq!(raw.special()?, CBORSpecial::Break);
                                        break;
                                    }
                                    let key_5_key = RewardAccount::deserialize(raw)?;
                                    let (key_5_value, key_5_value_encoding) = raw.unsigned_integer_sz().map(|(x, enc)| (x, Some(enc)))?;
                                    if key_5_table.insert(key_5_key.clone(), key_5_value).is_some() {
                                        return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from("some complicated/unsupported type"))).into());
                                    }
                                    key_5_value_encodings.insert(key_5_key.clone(), key_5_value_encoding);
                                }
                                Ok((key_5_table, key_5_encoding, key_5_value_encodings))
                            })().map_err(|e| e.annotate("key_5"))?;
                            key_5 = Some(tmp_key_5);
                            key_5_encoding = tmp_key_5_encoding;
                            key_5_value_encodings = tmp_key_5_value_encodings;
                            key_5_key_encoding = Some(key_enc);
                            orig_deser_order.push(5);
                        },
                        (6, key_enc) =>  {
                            if key_6.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(6)).into());
                            }
                            let tmp_key_6 = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                Ok(Update::deserialize(raw)?)
                            })().map_err(|e| e.annotate("key_6"))?;
                            key_6 = Some(tmp_key_6);
                            key_6_key_encoding = Some(key_enc);
                            orig_deser_order.push(6);
                        },
                        (7, key_enc) =>  {
                            if key_7.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(7)).into());
                            }
                            let tmp_key_7 = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                Ok(AuxiliaryDataHash::deserialize(raw)?)
                            })().map_err(|e| e.annotate("key_7"))?;
                            key_7 = Some(tmp_key_7);
                            key_7_key_encoding = Some(key_enc);
                            orig_deser_order.push(7);
                        },
                        (8, key_enc) =>  {
                            if key_8.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(8)).into());
                            }
                            let (tmp_key_8, tmp_key_8_encoding) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                Ok(raw.unsigned_integer_sz().map(|(x, enc)| (x, Some(enc)))?)
                            })().map_err(|e| e.annotate("key_8"))?;
                            key_8 = Some(tmp_key_8);
                            key_8_encoding = tmp_key_8_encoding;
                            key_8_key_encoding = Some(key_enc);
                            orig_deser_order.push(8);
                        },
                        (9, key_enc) =>  {
                            if key_9.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(9)).into());
                            }
                            let (tmp_key_9, tmp_key_9_encoding, tmp_key_9_value_encodings) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                let mut key_9_table = OrderedHashMap::new();
                                let key_9_len = raw.map_sz()?;
                                let key_9_encoding = key_9_len.into();
                                let mut key_9_value_encodings = BTreeMap::new();
                                while match key_9_len { cbor_event::LenSz::Len(n, _) => key_9_table.len() < n as usize, cbor_event::LenSz::Indefinite => true, } {
                                    if raw.cbor_type()? == CBORType::Special {
                                        assert_eq!(raw.special()?, CBORSpecial::Break);
                                        break;
                                    }
                                    let key_9_key = PolicyId::deserialize(raw)?;
                                    let mut key_9_value_table = OrderedHashMap::new();
                                    let key_9_value_len = raw.map_sz()?;
                                    let key_9_value_encoding = key_9_value_len.into();
                                    let mut key_9_value_value_encodings = BTreeMap::new();
                                    while match key_9_value_len { cbor_event::LenSz::Len(n, _) => key_9_value_table.len() < n as usize, cbor_event::LenSz::Indefinite => true, } {
                                        if raw.cbor_type()? == CBORType::Special {
                                            assert_eq!(raw.special()?, CBORSpecial::Break);
                                            break;
                                        }
                                        let key_9_value_key = AssetName::deserialize(raw)?;
                                        let (key_9_value_value, key_9_value_value_encoding) = match raw.cbor_type()? {
                                            cbor_event::Type::UnsignedInteger => {
                                                let (x, enc) = raw.unsigned_integer_sz()?;
                                                (x as i64, Some(enc))
                                            },
                                            _ => {
                                                let (x, enc) = raw.negative_integer_sz()?;
                                                (x as i64, Some(enc))
                                            },
                                        };
                                        if key_9_value_table.insert(key_9_value_key.clone(), key_9_value_value).is_some() {
                                            return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from("some complicated/unsupported type"))).into());
                                        }
                                        key_9_value_value_encodings.insert(key_9_value_key.clone(), key_9_value_value_encoding);
                                    }
                                    let (key_9_value, key_9_value_encoding, key_9_value_value_encodings) = (key_9_value_table, key_9_value_encoding, key_9_value_value_encodings);
                                    if key_9_table.insert(key_9_key.clone(), key_9_value).is_some() {
                                        return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from("some complicated/unsupported type"))).into());
                                    }
                                    key_9_value_encodings.insert(key_9_key.clone(), (key_9_value_encoding, key_9_value_value_encodings));
                                }
                                Ok((key_9_table, key_9_encoding, key_9_value_encodings))
                            })().map_err(|e| e.annotate("key_9"))?;
                            key_9 = Some(tmp_key_9);
                            key_9_encoding = tmp_key_9_encoding;
                            key_9_value_encodings = tmp_key_9_value_encodings;
                            key_9_key_encoding = Some(key_enc);
                            orig_deser_order.push(9);
                        },
                        (11, key_enc) =>  {
                            if key_11.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(11)).into());
                            }
                            let tmp_key_11 = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                Ok(ScriptDataHash::deserialize(raw)?)
                            })().map_err(|e| e.annotate("key_11"))?;
                            key_11 = Some(tmp_key_11);
                            key_11_key_encoding = Some(key_enc);
                            orig_deser_order.push(10);
                        },
                        (13, key_enc) =>  {
                            if key_13.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(13)).into());
                            }
                            let (tmp_key_13, tmp_key_13_encoding) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                let mut key_13_arr = Vec::new();
                                let len = raw.array_sz()?;
                                let key_13_encoding = len.into();
                                while match len { cbor_event::LenSz::Len(n, _) => key_13_arr.len() < n as usize, cbor_event::LenSz::Indefinite => true, } {
                                    if raw.cbor_type()? == CBORType::Special {
                                        assert_eq!(raw.special()?, CBORSpecial::Break);
                                        break;
                                    }
                                    key_13_arr.push(TransactionInput::deserialize(raw)?);
                                }
                                Ok((key_13_arr, key_13_encoding))
                            })().map_err(|e| e.annotate("key_13"))?;
                            key_13 = Some(tmp_key_13);
                            key_13_encoding = tmp_key_13_encoding;
                            key_13_key_encoding = Some(key_enc);
                            orig_deser_order.push(11);
                        },
                        (14, key_enc) =>  {
                            if key_14.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(14)).into());
                            }
                            let tmp_key_14 = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                Ok(RequiredSigners::deserialize(raw)?)
                            })().map_err(|e| e.annotate("key_14"))?;
                            key_14 = Some(tmp_key_14);
                            key_14_key_encoding = Some(key_enc);
                            orig_deser_order.push(12);
                        },
                        (15, key_enc) =>  {
                            if key_15.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(15)).into());
                            }
                            let tmp_key_15 = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                Ok(NetworkId::deserialize(raw)?)
                            })().map_err(|e| e.annotate("key_15"))?;
                            key_15 = Some(tmp_key_15);
                            key_15_key_encoding = Some(key_enc);
                            orig_deser_order.push(13);
                        },
                        (16, key_enc) =>  {
                            if key_16.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(16)).into());
                            }
                            let tmp_key_16 = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                Ok(TransactionOutput::deserialize(raw)?)
                            })().map_err(|e| e.annotate("key_16"))?;
                            key_16 = Some(tmp_key_16);
                            key_16_key_encoding = Some(key_enc);
                            orig_deser_order.push(14);
                        },
                        (17, key_enc) =>  {
                            if key_17.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(17)).into());
                            }
                            let (tmp_key_17, tmp_key_17_encoding) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                Ok(raw.unsigned_integer_sz().map(|(x, enc)| (x, Some(enc)))?)
                            })().map_err(|e| e.annotate("key_17"))?;
                            key_17 = Some(tmp_key_17);
                            key_17_encoding = tmp_key_17_encoding;
                            key_17_key_encoding = Some(key_enc);
                            orig_deser_order.push(15);
                        },
                        (18, key_enc) =>  {
                            if key_18.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(18)).into());
                            }
                            let (tmp_key_18, tmp_key_18_encoding) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                let mut key_18_arr = Vec::new();
                                let len = raw.array_sz()?;
                                let key_18_encoding = len.into();
                                while match len { cbor_event::LenSz::Len(n, _) => key_18_arr.len() < n as usize, cbor_event::LenSz::Indefinite => true, } {
                                    if raw.cbor_type()? == CBORType::Special {
                                        assert_eq!(raw.special()?, CBORSpecial::Break);
                                        break;
                                    }
                                    key_18_arr.push(TransactionInput::deserialize(raw)?);
                                }
                                Ok((key_18_arr, key_18_encoding))
                            })().map_err(|e| e.annotate("key_18"))?;
                            key_18 = Some(tmp_key_18);
                            key_18_encoding = tmp_key_18_encoding;
                            key_18_key_encoding = Some(key_enc);
                            orig_deser_order.push(16);
                        },
                        (unknown_key, _enc) => return Err(DeserializeFailure::UnknownKey(Key::Uint(unknown_key)).into()),
                    },
                    CBORType::Text => {
                        let (text_key, key_enc) = raw.text_sz()?;
                        match text_key.as_str() {
                            unknown_key => return Err(DeserializeFailure::UnknownKey(Key::Str(unknown_key.to_owned())).into()),
                        }
                    },
                    CBORType::Special => match len {
                        cbor_event::LenSz::Len(_, _) => return Err(DeserializeFailure::BreakInDefiniteLen.into()),
                        cbor_event::LenSz::Indefinite => match raw.special()? {
                            CBORSpecial::Break => break,
                            _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                        },
                    },
                    other_type => return Err(DeserializeFailure::UnexpectedKeyType(other_type).into()),
                }
                read += 1;
            }
            let key_0 = match key_0 {
                Some(x) => x,
                None => return Err(DeserializeFailure::MandatoryFieldMissing(Key::Uint(0)).into()),
            };
            let key_1 = match key_1 {
                Some(x) => x,
                None => return Err(DeserializeFailure::MandatoryFieldMissing(Key::Uint(1)).into()),
            };
            let key_2 = match key_2 {
                Some(x) => x,
                None => return Err(DeserializeFailure::MandatoryFieldMissing(Key::Uint(2)).into()),
            };
            read_len.finish()?;
            Ok(Self {
                key_0,
                key_1,
                key_2,
                key_3,
                key_4,
                key_5,
                key_6,
                key_7,
                key_8,
                key_9,
                key_11,
                key_13,
                key_14,
                key_15,
                key_16,
                key_17,
                key_18,
                encodings: Some(TransactionBodyEncoding {
                    len_encoding,
                    orig_deser_order,
                    key_0_key_encoding,
                    key_0_encoding,
                    key_1_key_encoding,
                    key_1_encoding,
                    key_2_key_encoding,
                    key_2_encoding,
                    key_3_key_encoding,
                    key_3_encoding,
                    key_4_key_encoding,
                    key_4_encoding,
                    key_5_key_encoding,
                    key_5_encoding,
                    key_5_value_encodings,
                    key_6_key_encoding,
                    key_7_key_encoding,
                    key_8_key_encoding,
                    key_8_encoding,
                    key_9_key_encoding,
                    key_9_encoding,
                    key_9_value_encodings,
                    key_11_key_encoding,
                    key_13_key_encoding,
                    key_13_encoding,
                    key_14_key_encoding,
                    key_15_key_encoding,
                    key_16_key_encoding,
                    key_17_key_encoding,
                    key_17_encoding,
                    key_18_key_encoding,
                    key_18_encoding,
                }),
            })
        })().map_err(|e| e.annotate("TransactionBody"))
    }
}

impl Serialize for TransactionInput {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().to_len_sz(2, force_canonical))?;
        self.transaction_id.serialize(serializer, force_canonical)?;
        serializer.write_unsigned_integer_sz(self.index, fit_sz(self.index, self.encodings.as_ref().map(|encs| encs.index_encoding.clone()).unwrap_or_default(), force_canonical))?;
        self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().end(serializer, force_canonical)
    }
}

impl Deserialize for TransactionInput {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array_sz()?;
            let len_encoding: LenEncoding = len.into();
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(2)?;
            let transaction_id = (|| -> Result<_, DeserializeError> {
                Ok(TransactionHash::deserialize(raw)?)
            })().map_err(|e| e.annotate("transaction_id"))?;
            let (index, index_encoding) = (|| -> Result<_, DeserializeError> {
                Ok(raw.unsigned_integer_sz().map(|(x, enc)| (x, Some(enc)))?)
            })().map_err(|e| e.annotate("index"))?;
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(TransactionInput {
                transaction_id,
                index,
                encodings: Some(TransactionInputEncoding {
                    len_encoding,
                    index_encoding,
                }),
            })
        })().map_err(|e| e.annotate("TransactionInput"))
    }
}

impl Serialize for TransactionOutput {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            TransactionOutput::ShelleyTxOut(shelley_tx_out) => {
                shelley_tx_out.serialize(serializer, force_canonical)
            },
            TransactionOutput::AlonzoTxOut(alonzo_tx_out) => {
                alonzo_tx_out.serialize(serializer, force_canonical)
            },
            TransactionOutput::BabbageTxOut(babbage_tx_out) => {
                babbage_tx_out.serialize(serializer, force_canonical)
            },
        }
    }
}

impl Deserialize for TransactionOutput {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let initial_position = raw.as_mut_ref().seek(SeekFrom::Current(0)).unwrap();
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(ShelleyTxOut::deserialize(raw)?)
            })(raw)
            {
                Ok(shelley_tx_out) => return Ok(Self::ShelleyTxOut(shelley_tx_out)),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(AlonzoTxOut::deserialize(raw)?)
            })(raw)
            {
                Ok(alonzo_tx_out) => return Ok(Self::AlonzoTxOut(alonzo_tx_out)),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(BabbageTxOut::deserialize(raw)?)
            })(raw)
            {
                Ok(babbage_tx_out) => return Ok(Self::BabbageTxOut(babbage_tx_out)),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            Err(DeserializeError::new("TransactionOutput", DeserializeFailure::NoVariantMatched.into()))
        })().map_err(|e| e.annotate("TransactionOutput"))
    }
}

impl Serialize for TransactionWitnessSet {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_map_sz(self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().to_len_sz(match &self.key_0 { Some(_) => 1, None => 0 } + match &self.key_1 { Some(_) => 1, None => 0 } + match &self.key_2 { Some(_) => 1, None => 0 } + match &self.key_3 { Some(_) => 1, None => 0 } + match &self.key_4 { Some(_) => 1, None => 0 } + match &self.key_5 { Some(_) => 1, None => 0 } + match &self.key_6 { Some(_) => 1, None => 0 }, force_canonical))?;
        let deser_order = self.encodings.as_ref().filter(|encs| !force_canonical && encs.orig_deser_order.len() == match &self.key_0 { Some(_) => 1, None => 0 } + match &self.key_1 { Some(_) => 1, None => 0 } + match &self.key_2 { Some(_) => 1, None => 0 } + match &self.key_3 { Some(_) => 1, None => 0 } + match &self.key_4 { Some(_) => 1, None => 0 } + match &self.key_5 { Some(_) => 1, None => 0 } + match &self.key_6 { Some(_) => 1, None => 0 }).map(|encs| encs.orig_deser_order.clone()).unwrap_or_else(|| vec![0,1,2,3,4,5,6]);
        for field_index in deser_order {
            match field_index {
                0 => if let Some(field) = &self.key_0 {
                    serializer.write_unsigned_integer_sz(0u64, fit_sz(0u64, self.encodings.as_ref().map(|encs| encs.key_0_key_encoding.clone()).unwrap_or_default(), force_canonical))?;
                    serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.key_0_encoding.clone()).unwrap_or_default().to_len_sz(field.len() as u64, force_canonical))?;
                    for element in field.iter() {
                        element.serialize(serializer, force_canonical)?;
                    }
                    self.encodings.as_ref().map(|encs| encs.key_0_encoding.clone()).unwrap_or_default().end(serializer, force_canonical)?;
                }
                1 => if let Some(field) = &self.key_1 {
                    serializer.write_unsigned_integer_sz(1u64, fit_sz(1u64, self.encodings.as_ref().map(|encs| encs.key_1_key_encoding.clone()).unwrap_or_default(), force_canonical))?;
                    serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.key_1_encoding.clone()).unwrap_or_default().to_len_sz(field.len() as u64, force_canonical))?;
                    for element in field.iter() {
                        element.serialize(serializer, force_canonical)?;
                    }
                    self.encodings.as_ref().map(|encs| encs.key_1_encoding.clone()).unwrap_or_default().end(serializer, force_canonical)?;
                }
                2 => if let Some(field) = &self.key_2 {
                    serializer.write_unsigned_integer_sz(2u64, fit_sz(2u64, self.encodings.as_ref().map(|encs| encs.key_2_key_encoding.clone()).unwrap_or_default(), force_canonical))?;
                    serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.key_2_encoding.clone()).unwrap_or_default().to_len_sz(field.len() as u64, force_canonical))?;
                    for element in field.iter() {
                        element.serialize(serializer, force_canonical)?;
                    }
                    self.encodings.as_ref().map(|encs| encs.key_2_encoding.clone()).unwrap_or_default().end(serializer, force_canonical)?;
                }
                3 => if let Some(field) = &self.key_3 {
                    serializer.write_unsigned_integer_sz(3u64, fit_sz(3u64, self.encodings.as_ref().map(|encs| encs.key_3_key_encoding.clone()).unwrap_or_default(), force_canonical))?;
                    serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.key_3_encoding.clone()).unwrap_or_default().to_len_sz(field.len() as u64, force_canonical))?;
                    for (i, element) in field.iter().enumerate() {
                        let key_3_elem_encoding = self.encodings.as_ref().and_then(|encs| encs.key_3_elem_encodings.get(i)).map(|e| e.clone()).unwrap_or_else(|| StringEncoding::default());
                        serializer.write_bytes_sz(&element, key_3_elem_encoding.to_str_len_sz(element.len() as u64, force_canonical))?;
                    }
                    self.encodings.as_ref().map(|encs| encs.key_3_encoding.clone()).unwrap_or_default().end(serializer, force_canonical)?;
                }
                4 => if let Some(field) = &self.key_4 {
                    serializer.write_unsigned_integer_sz(4u64, fit_sz(4u64, self.encodings.as_ref().map(|encs| encs.key_4_key_encoding.clone()).unwrap_or_default(), force_canonical))?;
                    serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.key_4_encoding.clone()).unwrap_or_default().to_len_sz(field.len() as u64, force_canonical))?;
                    for element in field.iter() {
                        element.serialize(serializer, force_canonical)?;
                    }
                    self.encodings.as_ref().map(|encs| encs.key_4_encoding.clone()).unwrap_or_default().end(serializer, force_canonical)?;
                }
                5 => if let Some(field) = &self.key_5 {
                    serializer.write_unsigned_integer_sz(5u64, fit_sz(5u64, self.encodings.as_ref().map(|encs| encs.key_5_key_encoding.clone()).unwrap_or_default(), force_canonical))?;
                    serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.key_5_encoding.clone()).unwrap_or_default().to_len_sz(field.len() as u64, force_canonical))?;
                    for element in field.iter() {
                        element.serialize(serializer, force_canonical)?;
                    }
                    self.encodings.as_ref().map(|encs| encs.key_5_encoding.clone()).unwrap_or_default().end(serializer, force_canonical)?;
                }
                6 => if let Some(field) = &self.key_6 {
                    serializer.write_unsigned_integer_sz(6u64, fit_sz(6u64, self.encodings.as_ref().map(|encs| encs.key_6_key_encoding.clone()).unwrap_or_default(), force_canonical))?;
                    serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.key_6_encoding.clone()).unwrap_or_default().to_len_sz(field.len() as u64, force_canonical))?;
                    for (i, element) in field.iter().enumerate() {
                        let key_6_elem_encoding = self.encodings.as_ref().and_then(|encs| encs.key_6_elem_encodings.get(i)).map(|e| e.clone()).unwrap_or_else(|| StringEncoding::default());
                        serializer.write_bytes_sz(&element, key_6_elem_encoding.to_str_len_sz(element.len() as u64, force_canonical))?;
                    }
                    self.encodings.as_ref().map(|encs| encs.key_6_encoding.clone()).unwrap_or_default().end(serializer, force_canonical)?;
                }
                _ => unreachable!()
            };
        }
        self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().end(serializer, force_canonical)
    }
}

impl Deserialize for TransactionWitnessSet {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.map_sz()?;
            let len_encoding: LenEncoding = len.into();
            let mut read_len = CBORReadLen::new(len);
            let mut orig_deser_order = Vec::new();
            let mut key_0_encoding = LenEncoding::default();
            let mut key_0_key_encoding = None;
            let mut key_0 = None;
            let mut key_1_encoding = LenEncoding::default();
            let mut key_1_key_encoding = None;
            let mut key_1 = None;
            let mut key_2_encoding = LenEncoding::default();
            let mut key_2_key_encoding = None;
            let mut key_2 = None;
            let mut key_3_encoding = LenEncoding::default();
            let mut key_3_elem_encodings = Vec::new();
            let mut key_3_key_encoding = None;
            let mut key_3 = None;
            let mut key_4_encoding = LenEncoding::default();
            let mut key_4_key_encoding = None;
            let mut key_4 = None;
            let mut key_5_encoding = LenEncoding::default();
            let mut key_5_key_encoding = None;
            let mut key_5 = None;
            let mut key_6_encoding = LenEncoding::default();
            let mut key_6_elem_encodings = Vec::new();
            let mut key_6_key_encoding = None;
            let mut key_6 = None;
            let mut read = 0;
            while match len { cbor_event::LenSz::Len(n, _) => read < n as usize, cbor_event::LenSz::Indefinite => true, } {
                match raw.cbor_type()? {
                    CBORType::UnsignedInteger => match raw.unsigned_integer_sz()? {
                        (0, key_enc) =>  {
                            if key_0.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(0)).into());
                            }
                            let (tmp_key_0, tmp_key_0_encoding) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                let mut key_0_arr = Vec::new();
                                let len = raw.array_sz()?;
                                let key_0_encoding = len.into();
                                while match len { cbor_event::LenSz::Len(n, _) => key_0_arr.len() < n as usize, cbor_event::LenSz::Indefinite => true, } {
                                    if raw.cbor_type()? == CBORType::Special {
                                        assert_eq!(raw.special()?, CBORSpecial::Break);
                                        break;
                                    }
                                    key_0_arr.push(Vkeywitness::deserialize(raw)?);
                                }
                                Ok((key_0_arr, key_0_encoding))
                            })().map_err(|e| e.annotate("key_0"))?;
                            key_0 = Some(tmp_key_0);
                            key_0_encoding = tmp_key_0_encoding;
                            key_0_key_encoding = Some(key_enc);
                            orig_deser_order.push(0);
                        },
                        (1, key_enc) =>  {
                            if key_1.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(1)).into());
                            }
                            let (tmp_key_1, tmp_key_1_encoding) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                let mut key_1_arr = Vec::new();
                                let len = raw.array_sz()?;
                                let key_1_encoding = len.into();
                                while match len { cbor_event::LenSz::Len(n, _) => key_1_arr.len() < n as usize, cbor_event::LenSz::Indefinite => true, } {
                                    if raw.cbor_type()? == CBORType::Special {
                                        assert_eq!(raw.special()?, CBORSpecial::Break);
                                        break;
                                    }
                                    key_1_arr.push(NativeScript::deserialize(raw)?);
                                }
                                Ok((key_1_arr, key_1_encoding))
                            })().map_err(|e| e.annotate("key_1"))?;
                            key_1 = Some(tmp_key_1);
                            key_1_encoding = tmp_key_1_encoding;
                            key_1_key_encoding = Some(key_enc);
                            orig_deser_order.push(1);
                        },
                        (2, key_enc) =>  {
                            if key_2.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(2)).into());
                            }
                            let (tmp_key_2, tmp_key_2_encoding) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                let mut key_2_arr = Vec::new();
                                let len = raw.array_sz()?;
                                let key_2_encoding = len.into();
                                while match len { cbor_event::LenSz::Len(n, _) => key_2_arr.len() < n as usize, cbor_event::LenSz::Indefinite => true, } {
                                    if raw.cbor_type()? == CBORType::Special {
                                        assert_eq!(raw.special()?, CBORSpecial::Break);
                                        break;
                                    }
                                    key_2_arr.push(BootstrapWitness::deserialize(raw)?);
                                }
                                Ok((key_2_arr, key_2_encoding))
                            })().map_err(|e| e.annotate("key_2"))?;
                            key_2 = Some(tmp_key_2);
                            key_2_encoding = tmp_key_2_encoding;
                            key_2_key_encoding = Some(key_enc);
                            orig_deser_order.push(2);
                        },
                        (3, key_enc) =>  {
                            if key_3.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(3)).into());
                            }
                            let (tmp_key_3, tmp_key_3_encoding, tmp_key_3_elem_encodings) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                let mut key_3_arr = Vec::new();
                                let len = raw.array_sz()?;
                                let key_3_encoding = len.into();
                                let mut key_3_elem_encodings = Vec::new();
                                while match len { cbor_event::LenSz::Len(n, _) => key_3_arr.len() < n as usize, cbor_event::LenSz::Indefinite => true, } {
                                    if raw.cbor_type()? == CBORType::Special {
                                        assert_eq!(raw.special()?, CBORSpecial::Break);
                                        break;
                                    }
                                    let (key_3_elem, key_3_elem_encoding) = raw.bytes_sz().map(|(bytes, enc)| (bytes, StringEncoding::from(enc)))?;
                                    key_3_arr.push(key_3_elem);
                                    key_3_elem_encodings.push(key_3_elem_encoding);
                                }
                                Ok((key_3_arr, key_3_encoding, key_3_elem_encodings))
                            })().map_err(|e| e.annotate("key_3"))?;
                            key_3 = Some(tmp_key_3);
                            key_3_encoding = tmp_key_3_encoding;
                            key_3_elem_encodings = tmp_key_3_elem_encodings;
                            key_3_key_encoding = Some(key_enc);
                            orig_deser_order.push(3);
                        },
                        (4, key_enc) =>  {
                            if key_4.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(4)).into());
                            }
                            let (tmp_key_4, tmp_key_4_encoding) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                let mut key_4_arr = Vec::new();
                                let len = raw.array_sz()?;
                                let key_4_encoding = len.into();
                                while match len { cbor_event::LenSz::Len(n, _) => key_4_arr.len() < n as usize, cbor_event::LenSz::Indefinite => true, } {
                                    if raw.cbor_type()? == CBORType::Special {
                                        assert_eq!(raw.special()?, CBORSpecial::Break);
                                        break;
                                    }
                                    key_4_arr.push(PlutusData::deserialize(raw)?);
                                }
                                Ok((key_4_arr, key_4_encoding))
                            })().map_err(|e| e.annotate("key_4"))?;
                            key_4 = Some(tmp_key_4);
                            key_4_encoding = tmp_key_4_encoding;
                            key_4_key_encoding = Some(key_enc);
                            orig_deser_order.push(4);
                        },
                        (5, key_enc) =>  {
                            if key_5.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(5)).into());
                            }
                            let (tmp_key_5, tmp_key_5_encoding) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                let mut key_5_arr = Vec::new();
                                let len = raw.array_sz()?;
                                let key_5_encoding = len.into();
                                while match len { cbor_event::LenSz::Len(n, _) => key_5_arr.len() < n as usize, cbor_event::LenSz::Indefinite => true, } {
                                    if raw.cbor_type()? == CBORType::Special {
                                        assert_eq!(raw.special()?, CBORSpecial::Break);
                                        break;
                                    }
                                    key_5_arr.push(Redeemer::deserialize(raw)?);
                                }
                                Ok((key_5_arr, key_5_encoding))
                            })().map_err(|e| e.annotate("key_5"))?;
                            key_5 = Some(tmp_key_5);
                            key_5_encoding = tmp_key_5_encoding;
                            key_5_key_encoding = Some(key_enc);
                            orig_deser_order.push(5);
                        },
                        (6, key_enc) =>  {
                            if key_6.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(6)).into());
                            }
                            let (tmp_key_6, tmp_key_6_encoding, tmp_key_6_elem_encodings) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                let mut key_6_arr = Vec::new();
                                let len = raw.array_sz()?;
                                let key_6_encoding = len.into();
                                let mut key_6_elem_encodings = Vec::new();
                                while match len { cbor_event::LenSz::Len(n, _) => key_6_arr.len() < n as usize, cbor_event::LenSz::Indefinite => true, } {
                                    if raw.cbor_type()? == CBORType::Special {
                                        assert_eq!(raw.special()?, CBORSpecial::Break);
                                        break;
                                    }
                                    let (key_6_elem, key_6_elem_encoding) = raw.bytes_sz().map(|(bytes, enc)| (bytes, StringEncoding::from(enc)))?;
                                    key_6_arr.push(key_6_elem);
                                    key_6_elem_encodings.push(key_6_elem_encoding);
                                }
                                Ok((key_6_arr, key_6_encoding, key_6_elem_encodings))
                            })().map_err(|e| e.annotate("key_6"))?;
                            key_6 = Some(tmp_key_6);
                            key_6_encoding = tmp_key_6_encoding;
                            key_6_elem_encodings = tmp_key_6_elem_encodings;
                            key_6_key_encoding = Some(key_enc);
                            orig_deser_order.push(6);
                        },
                        (unknown_key, _enc) => return Err(DeserializeFailure::UnknownKey(Key::Uint(unknown_key)).into()),
                    },
                    CBORType::Text => {
                        let (text_key, key_enc) = raw.text_sz()?;
                        match text_key.as_str() {
                            unknown_key => return Err(DeserializeFailure::UnknownKey(Key::Str(unknown_key.to_owned())).into()),
                        }
                    },
                    CBORType::Special => match len {
                        cbor_event::LenSz::Len(_, _) => return Err(DeserializeFailure::BreakInDefiniteLen.into()),
                        cbor_event::LenSz::Indefinite => match raw.special()? {
                            CBORSpecial::Break => break,
                            _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                        },
                    },
                    other_type => return Err(DeserializeFailure::UnexpectedKeyType(other_type).into()),
                }
                read += 1;
            }
            read_len.finish()?;
            Ok(Self {
                key_0,
                key_1,
                key_2,
                key_3,
                key_4,
                key_5,
                key_6,
                encodings: Some(TransactionWitnessSetEncoding {
                    len_encoding,
                    orig_deser_order,
                    key_0_key_encoding,
                    key_0_encoding,
                    key_1_key_encoding,
                    key_1_encoding,
                    key_2_key_encoding,
                    key_2_encoding,
                    key_3_key_encoding,
                    key_3_encoding,
                    key_3_elem_encodings,
                    key_4_key_encoding,
                    key_4_encoding,
                    key_5_key_encoding,
                    key_5_encoding,
                    key_6_key_encoding,
                    key_6_encoding,
                    key_6_elem_encodings,
                }),
            })
        })().map_err(|e| e.annotate("TransactionWitnessSet"))
    }
}

impl Serialize for UnitInterval {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_tag_sz(30u64, fit_sz(30u64, self.encodings.as_ref().map(|encs| encs.tag_encoding).unwrap_or_default(), force_canonical))?;
        serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().to_len_sz(2, force_canonical))?;
        serializer.write_unsigned_integer_sz(self.index_0, fit_sz(self.index_0, self.encodings.as_ref().map(|encs| encs.index_0_encoding.clone()).unwrap_or_default(), force_canonical))?;
        serializer.write_unsigned_integer_sz(self.index_1, fit_sz(self.index_1, self.encodings.as_ref().map(|encs| encs.index_1_encoding.clone()).unwrap_or_default(), force_canonical))?;
        self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().end(serializer, force_canonical)
    }
}

impl Deserialize for UnitInterval {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let (tag, tag_encoding) = raw.tag_sz()?;
            if tag != 30 {
                return Err(DeserializeError::new("UnitInterval", DeserializeFailure::TagMismatch{ found: tag, expected: 30 }));
            }
            let len = raw.array_sz()?;
            let len_encoding: LenEncoding = len.into();
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(2)?;
            let (index_0, index_0_encoding) = (|| -> Result<_, DeserializeError> {
                Ok(raw.unsigned_integer_sz().map(|(x, enc)| (x, Some(enc)))?)
            })().map_err(|e| e.annotate("index_0"))?;
            let (index_1, index_1_encoding) = (|| -> Result<_, DeserializeError> {
                Ok(raw.unsigned_integer_sz().map(|(x, enc)| (x, Some(enc)))?)
            })().map_err(|e| e.annotate("index_1"))?;
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(UnitInterval {
                index_0,
                index_1,
                encodings: Some(UnitIntervalEncoding {
                    len_encoding,
                    tag_encoding: Some(tag_encoding),
                    index_0_encoding,
                    index_1_encoding,
                }),
            })
        })().map_err(|e| e.annotate("UnitInterval"))
    }
}

impl Serialize for Update {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().to_len_sz(2, force_canonical))?;
        serializer.write_map_sz(self.encodings.as_ref().map(|encs| encs.proposed_protocol_parameter_updates_encoding.clone()).unwrap_or_default().to_len_sz(self.proposed_protocol_parameter_updates.len() as u64, force_canonical))?;
        let mut key_order = self.proposed_protocol_parameter_updates.iter().map(|(k, v)| {
            let mut buf = cbor_event::se::Serializer::new_vec();
            k.serialize(&mut buf, force_canonical)?;
            Ok((buf.finalize(), k, v))
        }).collect::<Result<Vec<(Vec<u8>, &_, &_)>, cbor_event::Error>>()?;
        if force_canonical {
            key_order.sort_by(|(lhs_bytes, _, _), (rhs_bytes, _, _)| {
                match lhs_bytes.len().cmp(&rhs_bytes.len()) {
                    std::cmp::Ordering::Equal => lhs_bytes.cmp(&rhs_bytes),
                    diff_ord => diff_ord,
                }
            });
        }
        for (key_bytes, key, value) in key_order {
            serializer.write_raw_bytes(&key_bytes)?;
            value.serialize(serializer, force_canonical)?;
        }
        self.encodings.as_ref().map(|encs| encs.proposed_protocol_parameter_updates_encoding.clone()).unwrap_or_default().end(serializer, force_canonical)?;
        serializer.write_unsigned_integer_sz(self.epoch, fit_sz(self.epoch, self.encodings.as_ref().map(|encs| encs.epoch_encoding.clone()).unwrap_or_default(), force_canonical))?;
        self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().end(serializer, force_canonical)
    }
}

impl Deserialize for Update {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array_sz()?;
            let len_encoding: LenEncoding = len.into();
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(2)?;
            let (proposed_protocol_parameter_updates, proposed_protocol_parameter_updates_encoding) = (|| -> Result<_, DeserializeError> {
                let mut proposed_protocol_parameter_updates_table = OrderedHashMap::new();
                let proposed_protocol_parameter_updates_len = raw.map_sz()?;
                let proposed_protocol_parameter_updates_encoding = proposed_protocol_parameter_updates_len.into();
                while match proposed_protocol_parameter_updates_len { cbor_event::LenSz::Len(n, _) => proposed_protocol_parameter_updates_table.len() < n as usize, cbor_event::LenSz::Indefinite => true, } {
                    if raw.cbor_type()? == CBORType::Special {
                        assert_eq!(raw.special()?, CBORSpecial::Break);
                        break;
                    }
                    let proposed_protocol_parameter_updates_key = GenesisHash::deserialize(raw)?;
                    let proposed_protocol_parameter_updates_value = ProtocolParamUpdate::deserialize(raw)?;
                    if proposed_protocol_parameter_updates_table.insert(proposed_protocol_parameter_updates_key.clone(), proposed_protocol_parameter_updates_value).is_some() {
                        return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from("some complicated/unsupported type"))).into());
                    }
                }
                Ok((proposed_protocol_parameter_updates_table, proposed_protocol_parameter_updates_encoding))
            })().map_err(|e| e.annotate("proposed_protocol_parameter_updates"))?;
            let (epoch, epoch_encoding) = (|| -> Result<_, DeserializeError> {
                Ok(raw.unsigned_integer_sz().map(|(x, enc)| (x, Some(enc)))?)
            })().map_err(|e| e.annotate("epoch"))?;
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(Update {
                proposed_protocol_parameter_updates,
                epoch,
                encodings: Some(UpdateEncoding {
                    len_encoding,
                    proposed_protocol_parameter_updates_encoding,
                    epoch_encoding,
                }),
            })
        })().map_err(|e| e.annotate("Update"))
    }
}

impl Serialize for Url {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_text_sz(&self.inner, self.encodings.as_ref().map(|encs| encs.inner_encoding.clone()).unwrap_or_default().to_str_len_sz(self.inner.len() as u64, force_canonical))
    }
}

impl Deserialize for Url {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let (inner, inner_encoding) = raw.text_sz().map(|(s, enc)| (s, StringEncoding::from(enc)))?;
        if inner.len() > 64 {
            return Err(DeserializeError::new("Url", DeserializeFailure::RangeCheck{ found: inner.len(), min: Some(0), max: Some(64) }));
        }
        Ok(Self {
            inner,
            encodings: Some(UrlEncoding {
                inner_encoding,
            }),
        })
    }
}

impl Serialize for Value {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().to_len_sz(2, force_canonical))?;
        serializer.write_unsigned_integer_sz(self.coin, fit_sz(self.coin, self.encodings.as_ref().map(|encs| encs.coin_encoding.clone()).unwrap_or_default(), force_canonical))?;
        serializer.write_map_sz(self.encodings.as_ref().map(|encs| encs.multiasset_encoding.clone()).unwrap_or_default().to_len_sz(self.multiasset.len() as u64, force_canonical))?;
        let mut key_order = self.multiasset.iter().map(|(k, v)| {
            let mut buf = cbor_event::se::Serializer::new_vec();
            k.serialize(&mut buf, force_canonical)?;
            Ok((buf.finalize(), k, v))
        }).collect::<Result<Vec<(Vec<u8>, &_, &_)>, cbor_event::Error>>()?;
        if force_canonical {
            key_order.sort_by(|(lhs_bytes, _, _), (rhs_bytes, _, _)| {
                match lhs_bytes.len().cmp(&rhs_bytes.len()) {
                    std::cmp::Ordering::Equal => lhs_bytes.cmp(&rhs_bytes),
                    diff_ord => diff_ord,
                }
            });
        }
        for (key_bytes, key, value) in key_order {
            serializer.write_raw_bytes(&key_bytes)?;
            let (multiasset_value_encoding, multiasset_value_value_encodings) = self.encodings.as_ref().and_then(|encs| encs.multiasset_value_encodings.get(key)).map(|e| e.clone()).unwrap_or_else(|| (LenEncoding::default(), BTreeMap::new()));
            serializer.write_map_sz(multiasset_value_encoding.to_len_sz(value.len() as u64, force_canonical))?;
            let mut key_order = value.iter().map(|(k, v)| {
                let mut buf = cbor_event::se::Serializer::new_vec();
                k.serialize(&mut buf, force_canonical)?;
                Ok((buf.finalize(), k, v))
            }).collect::<Result<Vec<(Vec<u8>, &_, &_)>, cbor_event::Error>>()?;
            if force_canonical {
                key_order.sort_by(|(lhs_bytes, _, _), (rhs_bytes, _, _)| {
                    match lhs_bytes.len().cmp(&rhs_bytes.len()) {
                        std::cmp::Ordering::Equal => lhs_bytes.cmp(&rhs_bytes),
                        diff_ord => diff_ord,
                    }
                });
            }
            for (key_bytes, key, value) in key_order {
                serializer.write_raw_bytes(&key_bytes)?;
                let multiasset_value_value_encoding = multiasset_value_value_encodings.get(key).map(|e| e.clone()).unwrap_or_else(|| None);
                serializer.write_unsigned_integer_sz(*value, fit_sz(*value, multiasset_value_value_encoding, force_canonical))?;
            }
            multiasset_value_encoding.end(serializer, force_canonical)?;
        }
        self.encodings.as_ref().map(|encs| encs.multiasset_encoding.clone()).unwrap_or_default().end(serializer, force_canonical)?;
        self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().end(serializer, force_canonical)
    }
}

impl Deserialize for Value {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array_sz()?;
            let len_encoding: LenEncoding = len.into();
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(2)?;
            let (coin, coin_encoding) = (|| -> Result<_, DeserializeError> {
                Ok(raw.unsigned_integer_sz().map(|(x, enc)| (x, Some(enc)))?)
            })().map_err(|e| e.annotate("coin"))?;
            let (multiasset, multiasset_encoding, multiasset_value_encodings) = (|| -> Result<_, DeserializeError> {
                let mut multiasset_table = OrderedHashMap::new();
                let multiasset_len = raw.map_sz()?;
                let multiasset_encoding = multiasset_len.into();
                let mut multiasset_value_encodings = BTreeMap::new();
                while match multiasset_len { cbor_event::LenSz::Len(n, _) => multiasset_table.len() < n as usize, cbor_event::LenSz::Indefinite => true, } {
                    if raw.cbor_type()? == CBORType::Special {
                        assert_eq!(raw.special()?, CBORSpecial::Break);
                        break;
                    }
                    let multiasset_key = PolicyId::deserialize(raw)?;
                    let mut multiasset_value_table = OrderedHashMap::new();
                    let multiasset_value_len = raw.map_sz()?;
                    let multiasset_value_encoding = multiasset_value_len.into();
                    let mut multiasset_value_value_encodings = BTreeMap::new();
                    while match multiasset_value_len { cbor_event::LenSz::Len(n, _) => multiasset_value_table.len() < n as usize, cbor_event::LenSz::Indefinite => true, } {
                        if raw.cbor_type()? == CBORType::Special {
                            assert_eq!(raw.special()?, CBORSpecial::Break);
                            break;
                        }
                        let multiasset_value_key = AssetName::deserialize(raw)?;
                        let (multiasset_value_value, multiasset_value_value_encoding) = raw.unsigned_integer_sz().map(|(x, enc)| (x, Some(enc)))?;
                        if multiasset_value_table.insert(multiasset_value_key.clone(), multiasset_value_value).is_some() {
                            return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from("some complicated/unsupported type"))).into());
                        }
                        multiasset_value_value_encodings.insert(multiasset_value_key.clone(), multiasset_value_value_encoding);
                    }
                    let (multiasset_value, multiasset_value_encoding, multiasset_value_value_encodings) = (multiasset_value_table, multiasset_value_encoding, multiasset_value_value_encodings);
                    if multiasset_table.insert(multiasset_key.clone(), multiasset_value).is_some() {
                        return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from("some complicated/unsupported type"))).into());
                    }
                    multiasset_value_encodings.insert(multiasset_key.clone(), (multiasset_value_encoding, multiasset_value_value_encodings));
                }
                Ok((multiasset_table, multiasset_encoding, multiasset_value_encodings))
            })().map_err(|e| e.annotate("multiasset"))?;
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(Value {
                coin,
                multiasset,
                encodings: Some(ValueEncoding {
                    len_encoding,
                    coin_encoding,
                    multiasset_encoding,
                    multiasset_value_encodings,
                }),
            })
        })().map_err(|e| e.annotate("Value"))
    }
}


impl Serialize for BootstrapWitness {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().to_len_sz(4, force_canonical))?;
        self.vkey.serialize(serializer, force_canonical)?;
        self.signature.serialize(serializer, force_canonical)?;
        serializer.write_bytes_sz(&self.chain_code, self.encodings.as_ref().map(|encs| encs.chain_code_encoding.clone()).unwrap_or_default().to_str_len_sz(self.chain_code.len() as u64, force_canonical))?;
        serializer.write_bytes_sz(&self.attributes, self.encodings.as_ref().map(|encs| encs.attributes_encoding.clone()).unwrap_or_default().to_str_len_sz(self.attributes.len() as u64, force_canonical))?;
        self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().end(serializer, force_canonical)
    }
}
  
impl Deserialize for BootstrapWitness {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array_sz()?;
            let len_encoding: LenEncoding = len.into();
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(4)?;
            let vkey = (|| -> Result<_, DeserializeError> {
                Ok(Vkey::deserialize(raw)?)
            })().map_err(|e| e.annotate("vkey"))?;
            let signature = (|| -> Result<_, DeserializeError> {
                Ok(Ed25519Signature::deserialize(raw)?)
            })().map_err(|e| e.annotate("signature"))?;
            let (chain_code, chain_code_encoding) = (|| -> Result<_, DeserializeError> {
                Ok(raw.bytes_sz().map(|(bytes, enc)| (bytes, StringEncoding::from(enc)))?)
            })().map_err(|e| e.annotate("chain_code"))?;
            let (attributes, attributes_encoding) = (|| -> Result<_, DeserializeError> {
                Ok(raw.bytes_sz().map(|(bytes, enc)| (bytes, StringEncoding::from(enc)))?)
            })().map_err(|e| e.annotate("attributes"))?;
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(BootstrapWitness {
                vkey,
                signature,
                chain_code,
                attributes,
                encodings: Some(BootstrapWitnessEncoding {
                    len_encoding,
                    chain_code_encoding,
                    attributes_encoding,
                }),
            })
        })().map_err(|e| e.annotate("BootstrapWitness"))
    }
}
  
impl Serialize for KesSignature {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_bytes_sz(&self.inner, self.encodings.as_ref().map(|encs| encs.inner_encoding.clone()).unwrap_or_default().to_str_len_sz(self.inner.len() as u64, force_canonical))
    }
}
  
impl Deserialize for KesSignature {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let (inner, inner_encoding) = raw.bytes_sz().map(|(bytes, enc)| (bytes, StringEncoding::from(enc)))?;
        if inner.len() != 32 {
            return Err(DeserializeError::new("KesSignature", DeserializeFailure::RangeCheck{ found: inner.len(), min: Some(32), max: Some(32) }));
        }
        Ok(Self {
            inner,
            encodings: Some(KesSignatureEncoding {
                inner_encoding,
            }),
        })
    }
}

impl Serialize for SignkeyKES {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_bytes_sz(&self.inner, self.encodings.as_ref().map(|encs| encs.inner_encoding.clone()).unwrap_or_default().to_str_len_sz(self.inner.len() as u64, force_canonical))
    }
}

impl Deserialize for SignkeyKES {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let (inner, inner_encoding) = raw.bytes_sz().map(|(bytes, enc)| (bytes, StringEncoding::from(enc)))?;
        if inner.len() != 16 {
            return Err(DeserializeError::new("SignkeyKES", DeserializeFailure::RangeCheck{ found: inner.len(), min: Some(16), max: Some(16) }));
        }
        Ok(Self {
            inner,
            encodings: Some(SignkeyKESEncoding {
                inner_encoding,
            }),
        })
    }
}

impl Serialize for Nonce {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            Nonce::I0{ i0_encoding, outer_len_encoding } => {
                serializer.write_array_sz(outer_len_encoding.to_len_sz(1, force_canonical))?;
                serializer.write_unsigned_integer_sz(0u64, fit_sz(0u64, *i0_encoding, force_canonical))?;
                outer_len_encoding.end(serializer, force_canonical)?;
                Ok(serializer)
            },
            Nonce::Nonce1(nonce1) => nonce1.serialize(serializer, force_canonical),
        }
    }
}

impl Deserialize for Nonce {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array_sz()?;
            let outer_len_encoding: LenEncoding = len.into();
            let mut read_len = CBORReadLen::new(len);
            let initial_position = raw.as_mut_ref().seek(SeekFrom::Current(0)).unwrap();
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let (i0_value, i0_encoding) = raw.unsigned_integer_sz()?;
                if i0_value != 0 {
                    return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(i0_value), expected: Key::Uint(0) }.into());
                }
                Ok(Some(i0_encoding))
            })(raw)
            {
                Ok(i0_encoding) => return Ok(Self::I0 {
                    i0_encoding,
                    outer_len_encoding,
                }),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(Nonce1::deserialize_as_embedded_group(raw, &mut read_len, len)?)
            })(raw)
            {
                Ok(nonce1) => return Ok(Self::Nonce1(nonce1)),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Err(DeserializeError::new("Nonce", DeserializeFailure::NoVariantMatched.into()))
        })().map_err(|e| e.annotate("Nonce"))
    }
}

impl Serialize for Nonce1 {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().to_len_sz(2, force_canonical))?;
        self.serialize_as_embedded_group(serializer, force_canonical)
    }
}

impl SerializeEmbeddedGroup for Nonce1 {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer_sz(1u64, fit_sz(1u64, self.encodings.as_ref().map(|encs| encs.index_0_encoding.clone()).unwrap_or_default(), force_canonical))?;
        serializer.write_bytes_sz(&self.bytes, self.encodings.as_ref().map(|encs| encs.bytes_encoding.clone()).unwrap_or_default().to_str_len_sz(self.bytes.len() as u64, force_canonical))?;
        self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().end(serializer, force_canonical)
    }
}

impl Deserialize for Nonce1 {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array_sz()?;
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(2)?;
            let ret = Self::deserialize_as_embedded_group(raw, &mut read_len, len);
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("Nonce1"))
    }
}

impl DeserializeEmbeddedGroup for Nonce1 {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, read_len: &mut CBORReadLen, len: cbor_event::LenSz) -> Result<Self, DeserializeError> {
        let len_encoding = len.into();
        let index_0_encoding = (|| -> Result<_, DeserializeError> {
            let (index_0_value, index_0_encoding) = raw.unsigned_integer_sz()?;
            if index_0_value != 1 {
                return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(index_0_value), expected: Key::Uint(1) }.into());
            }
            Ok(Some(index_0_encoding))
        })().map_err(|e| e.annotate("index_0"))?;
        let (bytes, bytes_encoding) = (|| -> Result<_, DeserializeError> {
            Ok(raw.bytes_sz().map(|(bytes, enc)| (bytes, StringEncoding::from(enc)))?)
        })().map_err(|e| e.annotate("bytes"))?;
        Ok(Nonce1 {
            bytes,
            encodings: Some(Nonce1Encoding {
                len_encoding,
                index_0_encoding,
                bytes_encoding,
            }),
        })
    }
}


impl Serialize for Vkeywitness {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().to_len_sz(2, force_canonical))?;
        self.vkey.serialize(serializer, force_canonical)?;
        self.signature.serialize(serializer, force_canonical)?;
        self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().end(serializer, force_canonical)
    }
}

impl Deserialize for Vkeywitness {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array_sz()?;
            let len_encoding: LenEncoding = len.into();
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(2)?;
            let vkey = (|| -> Result<_, DeserializeError> {
                Ok(Vkey::deserialize(raw)?)
            })().map_err(|e| e.annotate("vkey"))?;
            let signature = (|| -> Result<_, DeserializeError> {
                Ok(Ed25519Signature::deserialize(raw)?)
            })().map_err(|e| e.annotate("signature"))?;
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(Vkeywitness {
                vkey,
                signature,
                encodings: Some(VkeywitnessEncoding {
                    len_encoding,
                }),
            })
        })().map_err(|e| e.annotate("Vkeywitness"))
    }
}

impl Serialize for VrfCert {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().to_len_sz(2, force_canonical))?;
        serializer.write_bytes_sz(&self.index_0, self.encodings.as_ref().map(|encs| encs.index_0_encoding.clone()).unwrap_or_default().to_str_len_sz(self.index_0.len() as u64, force_canonical))?;
        serializer.write_bytes_sz(&self.bytes, self.encodings.as_ref().map(|encs| encs.bytes_encoding.clone()).unwrap_or_default().to_str_len_sz(self.bytes.len() as u64, force_canonical))?;
        self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().end(serializer, force_canonical)
    }
}

impl Deserialize for VrfCert {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array_sz()?;
            let len_encoding: LenEncoding = len.into();
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(2)?;
            let (index_0, index_0_encoding) = (|| -> Result<_, DeserializeError> {
                Ok(raw.bytes_sz().map(|(bytes, enc)| (bytes, StringEncoding::from(enc)))?)
            })().map_err(|e| e.annotate("index_0"))?;
            let (bytes, bytes_encoding) = (|| -> Result<_, DeserializeError> {
                Ok(raw.bytes_sz().map(|(bytes, enc)| (bytes, StringEncoding::from(enc)))?)
            })().map_err(|e| e.annotate("bytes"))?;
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(VrfCert {
                index_0,
                bytes,
                encodings: Some(VrfCertEncoding {
                    len_encoding,
                    index_0_encoding,
                    bytes_encoding,
                }),
            })
        })().map_err(|e| e.annotate("VrfCert"))
    }
}
