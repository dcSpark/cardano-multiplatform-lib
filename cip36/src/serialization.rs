use super::*;
use std::io::{Seek, SeekFrom};
use cml_core::{
    serialization::{CBORReadLen, fit_sz, Serialize, Deserialize, SerializeEmbeddedGroup, DeserializeEmbeddedGroup},
    Key
};

impl Serialize for ArrDelegationOrLegacyKeyRegistration {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            ArrDelegationOrLegacyKeyRegistration::ArrDelegation{ arr_delegation, arr_delegation_encoding } => {
                serializer.write_array_sz(arr_delegation_encoding.to_len_sz(2 * arr_delegation.len() as u64, force_canonical))?;
                for element in arr_delegation.iter() {
                    element.serialize_as_embedded_group(serializer, force_canonical)?;
                }
                arr_delegation_encoding.end(serializer, force_canonical)
            },
            ArrDelegationOrLegacyKeyRegistration::LegacyKeyRegistration(legacy_key_registration) => {
                legacy_key_registration.serialize(serializer, force_canonical)
            },
        }
    }
}

impl Deserialize for ArrDelegationOrLegacyKeyRegistration {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let initial_position = raw.as_mut_ref().seek(SeekFrom::Current(0)).unwrap();
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let mut arr_delegation_arr = Vec::new();
                let len = raw.array_sz()?;
                let arr_delegation_encoding = len.into();
                let mut arr_delegation_read_len = CBORReadLen::new(len);
                while match len { cbor_event::LenSz::Len(n, _) => arr_delegation_read_len.read() < n, cbor_event::LenSz::Indefinite => true, } {
                    if raw.cbor_type()? == CBORType::Special {
                        assert_eq!(raw.special()?, CBORSpecial::Break);
                        break;
                    }
                    arr_delegation_read_len.read_elems(2)?;
                    arr_delegation_arr.push(Delegation::deserialize_as_embedded_group(raw, &mut arr_delegation_read_len, len)?);
                }
                Ok((arr_delegation_arr, arr_delegation_encoding))
            })(raw)
            {
                Ok((arr_delegation, arr_delegation_encoding)) => return Ok(Self::ArrDelegation {
                    arr_delegation,
                    arr_delegation_encoding,
                }),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            let deser_variant: Result<_, DeserializeError> = VotingPubKey::deserialize(raw);
            match deser_variant {
                Ok(legacy_key_registration) => return Ok(Self::LegacyKeyRegistration(legacy_key_registration)),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            Err(DeserializeError::new("ArrDelegationOrLegacyKeyRegistration", DeserializeFailure::NoVariantMatched))
        })().map_err(|e| e.annotate("ArrDelegationOrLegacyKeyRegistration"))
    }
}

impl Serialize for Delegation {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().to_len_sz(2, force_canonical))?;
        self.serialize_as_embedded_group(serializer, force_canonical)
    }
}

impl SerializeEmbeddedGroup for Delegation {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.voting_pub_key.serialize(serializer, force_canonical)?;
        serializer.write_unsigned_integer_sz(self.weight as u64, fit_sz(self.weight as u64, self.encodings.as_ref().map(|encs| encs.weight_encoding).unwrap_or_default(), force_canonical))?;
        self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().end(serializer, force_canonical)
    }
}

impl Deserialize for Delegation {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
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
    }
}

impl DeserializeEmbeddedGroup for Delegation {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, _read_len: &mut CBORReadLen, len: cbor_event::LenSz) -> Result<Self, DeserializeError> {
        let len_encoding = len.into();
        (|| -> Result<_, DeserializeError> {
            let voting_pub_key = VotingPubKey::deserialize(raw).map_err(|e: DeserializeError| e.annotate("voting_pub_key"))?;
            let (weight, weight_encoding) = raw.unsigned_integer_sz().map(|(x, enc)| (x as u32, Some(enc))).map_err(Into::<DeserializeError>::into).map_err(|e: DeserializeError| e.annotate("weight"))?;
            Ok(Delegation {
                voting_pub_key,
                weight,
                encodings: Some(DelegationEncoding {
                    len_encoding,
                    weight_encoding,
                }),
            })
        })().map_err(|e| e.annotate("Delegation"))
    }
}

impl Serialize for DeregistrationCbor {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_map_sz(self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().to_len_sz(2, force_canonical))?;
        let deser_order = self.encodings.as_ref().filter(|encs| !force_canonical && encs.orig_deser_order.len() == 2).map(|encs| encs.orig_deser_order.clone()).unwrap_or_else(|| vec![1,0]);
        for field_index in deser_order {
            match field_index {
                1 => {
                    serializer.write_unsigned_integer_sz(61285u64, fit_sz(61285u64, self.encodings.as_ref().map(|encs| encs.deregistration_witness_key_encoding).unwrap_or_default(), force_canonical))?;
                    self.deregistration_witness.serialize(serializer, force_canonical)?;
                }
                0 => {
                    serializer.write_unsigned_integer_sz(61286u64, fit_sz(61286u64, self.encodings.as_ref().map(|encs| encs.key_deregistration_key_encoding).unwrap_or_default(), force_canonical))?;
                    self.key_deregistration.serialize(serializer, force_canonical)?;
                }
                _ => unreachable!()
            };
        }
        self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().end(serializer, force_canonical)
    }
}

impl Deserialize for DeregistrationCbor {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.map_sz()?;
        let len_encoding: LenEncoding = len.into();
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        (|| -> Result<_, DeserializeError> {
            let mut orig_deser_order = Vec::new();
            let mut deregistration_witness_key_encoding = None;
            let mut deregistration_witness = None;
            let mut key_deregistration_key_encoding = None;
            let mut key_deregistration = None;
            let mut read = 0;
            while match len { cbor_event::LenSz::Len(n, _) => read < n, cbor_event::LenSz::Indefinite => true, } {
                match raw.cbor_type()? {
                    CBORType::UnsignedInteger => match raw.unsigned_integer_sz()? {
                        (61285, key_enc) =>  {
                            if deregistration_witness.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(61285)).into());
                            }
                            let tmp_deregistration_witness = DeregistrationWitness::deserialize(raw).map_err(|e: DeserializeError| e.annotate("deregistration_witness"))?;
                            deregistration_witness = Some(tmp_deregistration_witness);
                            deregistration_witness_key_encoding = Some(key_enc);
                            orig_deser_order.push(1);
                        },
                        (61286, key_enc) =>  {
                            if key_deregistration.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(61286)).into());
                            }
                            let tmp_key_deregistration = KeyDeregistration::deserialize(raw).map_err(|e: DeserializeError| e.annotate("key_deregistration"))?;
                            key_deregistration = Some(tmp_key_deregistration);
                            key_deregistration_key_encoding = Some(key_enc);
                            orig_deser_order.push(0);
                        },
                        (unknown_key, _enc) => return Err(DeserializeFailure::UnknownKey(Key::Uint(unknown_key)).into()),
                    },
                    CBORType::Text => return Err(DeserializeFailure::UnknownKey(Key::Str(raw.text()?)).into()),
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
            let key_deregistration = match key_deregistration {
                Some(x) => x,
                None => return Err(DeserializeFailure::MandatoryFieldMissing(Key::Uint(61286)).into()),
            };
            let deregistration_witness = match deregistration_witness {
                Some(x) => x,
                None => return Err(DeserializeFailure::MandatoryFieldMissing(Key::Uint(61285)).into()),
            };
            ();
            Ok(Self {
                key_deregistration,
                deregistration_witness,
                encodings: Some(DeregistrationCborEncoding {
                    len_encoding,
                    orig_deser_order,
                    key_deregistration_key_encoding,
                    deregistration_witness_key_encoding,
                }),
            })
        })().map_err(|e| e.annotate("DeregistrationCbor"))
    }
}

impl Serialize for DeregistrationWitness {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_map_sz(self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().to_len_sz(1, force_canonical))?;
        let deser_order = self.encodings.as_ref().filter(|encs| !force_canonical && encs.orig_deser_order.len() == 1).map(|encs| encs.orig_deser_order.clone()).unwrap_or_else(|| vec![0]);
        for field_index in deser_order {
            match field_index {
                0 => {
                    serializer.write_unsigned_integer_sz(1u64, fit_sz(1u64, self.encodings.as_ref().map(|encs| encs.stake_witness_key_encoding).unwrap_or_default(), force_canonical))?;
                    self.stake_witness.serialize(serializer, force_canonical)?;
                }
                _ => unreachable!()
            };
        }
        self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().end(serializer, force_canonical)
    }
}

impl Deserialize for DeregistrationWitness {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.map_sz()?;
        let len_encoding: LenEncoding = len.into();
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(1)?;
        (|| -> Result<_, DeserializeError> {
            let mut orig_deser_order = Vec::new();
            let mut stake_witness_key_encoding = None;
            let mut stake_witness = None;
            let mut read = 0;
            while match len { cbor_event::LenSz::Len(n, _) => read < n, cbor_event::LenSz::Indefinite => true, } {
                match raw.cbor_type()? {
                    CBORType::UnsignedInteger => match raw.unsigned_integer_sz()? {
                        (1, key_enc) =>  {
                            if stake_witness.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(1)).into());
                            }
                            let tmp_stake_witness = Ed25519Signature::deserialize(raw).map_err(|e: DeserializeError| e.annotate("stake_witness"))?;
                            stake_witness = Some(tmp_stake_witness);
                            stake_witness_key_encoding = Some(key_enc);
                            orig_deser_order.push(0);
                        },
                        (unknown_key, _enc) => return Err(DeserializeFailure::UnknownKey(Key::Uint(unknown_key)).into()),
                    },
                    CBORType::Text => return Err(DeserializeFailure::UnknownKey(Key::Str(raw.text()?)).into()),
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
            let stake_witness = match stake_witness {
                Some(x) => x,
                None => return Err(DeserializeFailure::MandatoryFieldMissing(Key::Uint(1)).into()),
            };
            ();
            Ok(Self {
                stake_witness,
                encodings: Some(DeregistrationWitnessEncoding {
                    len_encoding,
                    orig_deser_order,
                    stake_witness_key_encoding,
                }),
            })
        })().map_err(|e| e.annotate("DeregistrationWitness"))
    }
}

impl Serialize for KeyDeregistration {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_map_sz(self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().to_len_sz(2 + if self.voting_purpose != 0 || self.encodings.as_ref().map(|encs| encs.voting_purpose_default_present).unwrap_or(false) { 1 } else { 0 }, force_canonical))?;
        let deser_order = self.encodings.as_ref().filter(|encs| !force_canonical && encs.orig_deser_order.len() == 2 + if self.voting_purpose != 0 || self.encodings.as_ref().map(|encs| encs.voting_purpose_default_present).unwrap_or(false) { 1 } else { 0 }).map(|encs| encs.orig_deser_order.clone()).unwrap_or_else(|| vec![0,1,2]);
        for field_index in deser_order {
            match field_index {
                0 => {
                    serializer.write_unsigned_integer_sz(1u64, fit_sz(1u64, self.encodings.as_ref().map(|encs| encs.stake_credential_key_encoding).unwrap_or_default(), force_canonical))?;
                    self.stake_credential.serialize(serializer, force_canonical)?;
                }
                1 => {
                    serializer.write_unsigned_integer_sz(2u64, fit_sz(2u64, self.encodings.as_ref().map(|encs| encs.nonce_key_encoding).unwrap_or_default(), force_canonical))?;
                    serializer.write_unsigned_integer_sz(self.nonce, fit_sz(self.nonce, self.encodings.as_ref().map(|encs| encs.nonce_encoding).unwrap_or_default(), force_canonical))?;
                }
                2 => {
                    serializer.write_unsigned_integer_sz(3u64, fit_sz(3u64, self.encodings.as_ref().map(|encs| encs.voting_purpose_key_encoding).unwrap_or_default(), force_canonical))?;
                    serializer.write_unsigned_integer_sz(self.voting_purpose, fit_sz(self.voting_purpose, self.encodings.as_ref().map(|encs| encs.voting_purpose_encoding).unwrap_or_default(), force_canonical))?;
                }
                _ => unreachable!()
            };
        }
        self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().end(serializer, force_canonical)
    }
}

impl Deserialize for KeyDeregistration {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.map_sz()?;
        let len_encoding: LenEncoding = len.into();
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        (|| -> Result<_, DeserializeError> {
            let mut orig_deser_order = Vec::new();
            let mut stake_credential_key_encoding = None;
            let mut stake_credential = None;
            let mut nonce_encoding = None;
            let mut nonce_key_encoding = None;
            let mut nonce = None;
            let mut voting_purpose_encoding = None;
            let mut voting_purpose_default_present = false;
            let mut voting_purpose_key_encoding = None;
            let mut voting_purpose = None;
            let mut read = 0;
            while match len { cbor_event::LenSz::Len(n, _) => read < n, cbor_event::LenSz::Indefinite => true, } {
                match raw.cbor_type()? {
                    CBORType::UnsignedInteger => match raw.unsigned_integer_sz()? {
                        (1, key_enc) =>  {
                            if stake_credential.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(1)).into());
                            }
                            let tmp_stake_credential = StakingPubKey::deserialize(raw).map_err(|e: DeserializeError| e.annotate("stake_credential"))?;
                            stake_credential = Some(tmp_stake_credential);
                            stake_credential_key_encoding = Some(key_enc);
                            orig_deser_order.push(0);
                        },
                        (2, key_enc) =>  {
                            if nonce.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(2)).into());
                            }
                            let (tmp_nonce, tmp_nonce_encoding) = raw.unsigned_integer_sz().map(|(x, enc)| (x, Some(enc))).map_err(Into::<DeserializeError>::into).map_err(|e: DeserializeError| e.annotate("nonce"))?;
                            nonce = Some(tmp_nonce);
                            nonce_encoding = tmp_nonce_encoding;
                            nonce_key_encoding = Some(key_enc);
                            orig_deser_order.push(1);
                        },
                        (3, key_enc) =>  {
                            if voting_purpose.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(3)).into());
                            }
                            let (tmp_voting_purpose, tmp_voting_purpose_encoding) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                raw.unsigned_integer_sz().map(|(x, enc)| (x, Some(enc))).map_err(Into::<DeserializeError>::into)
                            })().map_err(|e| e.annotate("voting_purpose"))?;
                            voting_purpose = Some(tmp_voting_purpose);
                            voting_purpose_encoding = tmp_voting_purpose_encoding;
                            voting_purpose_key_encoding = Some(key_enc);
                            orig_deser_order.push(2);
                        },
                        (unknown_key, _enc) => return Err(DeserializeFailure::UnknownKey(Key::Uint(unknown_key)).into()),
                    },
                    CBORType::Text => return Err(DeserializeFailure::UnknownKey(Key::Str(raw.text()?)).into()),
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
            let stake_credential = match stake_credential {
                Some(x) => x,
                None => return Err(DeserializeFailure::MandatoryFieldMissing(Key::Uint(1)).into()),
            };
            let nonce = match nonce {
                Some(x) => x,
                None => return Err(DeserializeFailure::MandatoryFieldMissing(Key::Uint(2)).into()),
            };
            if voting_purpose == Some(0) {
                voting_purpose_default_present = true;
            }
            let voting_purpose = voting_purpose.unwrap_or(0);
            read_len.finish()?;
            Ok(Self {
                stake_credential,
                nonce,
                voting_purpose,
                encodings: Some(KeyDeregistrationEncoding {
                    len_encoding,
                    orig_deser_order,
                    stake_credential_key_encoding,
                    nonce_key_encoding,
                    nonce_encoding,
                    voting_purpose_key_encoding,
                    voting_purpose_encoding,
                    voting_purpose_default_present,
                }),
            })
        })().map_err(|e| e.annotate("KeyDeregistration"))
    }
}

impl Serialize for KeyRegistration {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_map_sz(self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().to_len_sz(4 + if self.voting_purpose != 0 || self.encodings.as_ref().map(|encs| encs.voting_purpose_default_present).unwrap_or(false) { 1 } else { 0 }, force_canonical))?;
        let deser_order = self.encodings.as_ref().filter(|encs| !force_canonical && encs.orig_deser_order.len() == 4 + if self.voting_purpose != 0 || self.encodings.as_ref().map(|encs| encs.voting_purpose_default_present).unwrap_or(false) { 1 } else { 0 }).map(|encs| encs.orig_deser_order.clone()).unwrap_or_else(|| vec![0,1,2,3,4]);
        for field_index in deser_order {
            match field_index {
                0 => {
                    serializer.write_unsigned_integer_sz(1u64, fit_sz(1u64, self.encodings.as_ref().map(|encs| encs.delegation_key_encoding).unwrap_or_default(), force_canonical))?;
                    self.delegation.serialize(serializer, force_canonical)?;
                }
                1 => {
                    serializer.write_unsigned_integer_sz(2u64, fit_sz(2u64, self.encodings.as_ref().map(|encs| encs.stake_credential_key_encoding).unwrap_or_default(), force_canonical))?;
                    self.stake_credential.serialize(serializer, force_canonical)?;
                }
                2 => {
                    serializer.write_unsigned_integer_sz(3u64, fit_sz(3u64, self.encodings.as_ref().map(|encs| encs.reward_address_key_encoding).unwrap_or_default(), force_canonical))?;
                    serializer.write_bytes_sz(&self.reward_address, self.encodings.as_ref().map(|encs| encs.reward_address_encoding.clone()).unwrap_or_default().to_str_len_sz(self.reward_address.len() as u64, force_canonical))?;
                }
                3 => {
                    serializer.write_unsigned_integer_sz(4u64, fit_sz(4u64, self.encodings.as_ref().map(|encs| encs.nonce_key_encoding).unwrap_or_default(), force_canonical))?;
                    serializer.write_unsigned_integer_sz(self.nonce, fit_sz(self.nonce, self.encodings.as_ref().map(|encs| encs.nonce_encoding).unwrap_or_default(), force_canonical))?;
                }
                4 => {
                    serializer.write_unsigned_integer_sz(5u64, fit_sz(5u64, self.encodings.as_ref().map(|encs| encs.voting_purpose_key_encoding).unwrap_or_default(), force_canonical))?;
                    serializer.write_unsigned_integer_sz(self.voting_purpose, fit_sz(self.voting_purpose, self.encodings.as_ref().map(|encs| encs.voting_purpose_encoding).unwrap_or_default(), force_canonical))?;
                }
                _ => unreachable!()
            };
        }
        self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().end(serializer, force_canonical)
    }
}

impl Deserialize for KeyRegistration {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.map_sz()?;
        let len_encoding: LenEncoding = len.into();
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(4)?;
        (|| -> Result<_, DeserializeError> {
            let mut orig_deser_order = Vec::new();
            let mut delegation_key_encoding = None;
            let mut delegation = None;
            let mut stake_credential_key_encoding = None;
            let mut stake_credential = None;
            let mut reward_address_encoding = StringEncoding::default();
            let mut reward_address_key_encoding = None;
            let mut reward_address = None;
            let mut nonce_encoding = None;
            let mut nonce_key_encoding = None;
            let mut nonce = None;
            let mut voting_purpose_encoding = None;
            let mut voting_purpose_default_present = false;
            let mut voting_purpose_key_encoding = None;
            let mut voting_purpose = None;
            let mut read = 0;
            while match len { cbor_event::LenSz::Len(n, _) => read < n, cbor_event::LenSz::Indefinite => true, } {
                match raw.cbor_type()? {
                    CBORType::UnsignedInteger => match raw.unsigned_integer_sz()? {
                        (1, key_enc) =>  {
                            if delegation.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(1)).into());
                            }
                            let tmp_delegation = ArrDelegationOrLegacyKeyRegistration::deserialize(raw).map_err(|e: DeserializeError| e.annotate("delegation"))?;
                            delegation = Some(tmp_delegation);
                            delegation_key_encoding = Some(key_enc);
                            orig_deser_order.push(0);
                        },
                        (2, key_enc) =>  {
                            if stake_credential.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(2)).into());
                            }
                            let tmp_stake_credential = StakingPubKey::deserialize(raw).map_err(|e: DeserializeError| e.annotate("stake_credential"))?;
                            stake_credential = Some(tmp_stake_credential);
                            stake_credential_key_encoding = Some(key_enc);
                            orig_deser_order.push(1);
                        },
                        (3, key_enc) =>  {
                            if reward_address.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(3)).into());
                            }
                            let (tmp_reward_address, tmp_reward_address_encoding) = raw.bytes_sz().map(|(bytes, enc)| (bytes, StringEncoding::from(enc))).map_err(Into::<DeserializeError>::into).map_err(|e: DeserializeError| e.annotate("reward_address"))?;
                            reward_address = Some(tmp_reward_address);
                            reward_address_encoding = tmp_reward_address_encoding;
                            reward_address_key_encoding = Some(key_enc);
                            orig_deser_order.push(2);
                        },
                        (4, key_enc) =>  {
                            if nonce.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(4)).into());
                            }
                            let (tmp_nonce, tmp_nonce_encoding) = raw.unsigned_integer_sz().map(|(x, enc)| (x, Some(enc))).map_err(Into::<DeserializeError>::into).map_err(|e: DeserializeError| e.annotate("nonce"))?;
                            nonce = Some(tmp_nonce);
                            nonce_encoding = tmp_nonce_encoding;
                            nonce_key_encoding = Some(key_enc);
                            orig_deser_order.push(3);
                        },
                        (5, key_enc) =>  {
                            if voting_purpose.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(5)).into());
                            }
                            let (tmp_voting_purpose, tmp_voting_purpose_encoding) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                raw.unsigned_integer_sz().map(|(x, enc)| (x, Some(enc))).map_err(Into::<DeserializeError>::into)
                            })().map_err(|e| e.annotate("voting_purpose"))?;
                            voting_purpose = Some(tmp_voting_purpose);
                            voting_purpose_encoding = tmp_voting_purpose_encoding;
                            voting_purpose_key_encoding = Some(key_enc);
                            orig_deser_order.push(4);
                        },
                        (unknown_key, _enc) => return Err(DeserializeFailure::UnknownKey(Key::Uint(unknown_key)).into()),
                    },
                    CBORType::Text => return Err(DeserializeFailure::UnknownKey(Key::Str(raw.text()?)).into()),
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
            let delegation = match delegation {
                Some(x) => x,
                None => return Err(DeserializeFailure::MandatoryFieldMissing(Key::Uint(1)).into()),
            };
            let stake_credential = match stake_credential {
                Some(x) => x,
                None => return Err(DeserializeFailure::MandatoryFieldMissing(Key::Uint(2)).into()),
            };
            let reward_address = match reward_address {
                Some(x) => x,
                None => return Err(DeserializeFailure::MandatoryFieldMissing(Key::Uint(3)).into()),
            };
            let nonce = match nonce {
                Some(x) => x,
                None => return Err(DeserializeFailure::MandatoryFieldMissing(Key::Uint(4)).into()),
            };
            if voting_purpose == Some(0) {
                voting_purpose_default_present = true;
            }
            let voting_purpose = voting_purpose.unwrap_or(0);
            read_len.finish()?;
            Ok(Self {
                delegation,
                stake_credential,
                reward_address,
                nonce,
                voting_purpose,
                encodings: Some(KeyRegistrationEncoding {
                    len_encoding,
                    orig_deser_order,
                    delegation_key_encoding,
                    stake_credential_key_encoding,
                    reward_address_key_encoding,
                    reward_address_encoding,
                    nonce_key_encoding,
                    nonce_encoding,
                    voting_purpose_key_encoding,
                    voting_purpose_encoding,
                    voting_purpose_default_present,
                }),
            })
        })().map_err(|e| e.annotate("KeyRegistration"))
    }
}

impl Serialize for RegistrationCbor {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_map_sz(self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().to_len_sz(2, force_canonical))?;
        let deser_order = self.encodings.as_ref().filter(|encs| !force_canonical && encs.orig_deser_order.len() == 2).map(|encs| encs.orig_deser_order.clone()).unwrap_or_else(|| vec![0,1]);
        for field_index in deser_order {
            match field_index {
                0 => {
                    serializer.write_unsigned_integer_sz(61284u64, fit_sz(61284u64, self.encodings.as_ref().map(|encs| encs.key_registration_key_encoding).unwrap_or_default(), force_canonical))?;
                    self.key_registration.serialize(serializer, force_canonical)?;
                }
                1 => {
                    serializer.write_unsigned_integer_sz(61285u64, fit_sz(61285u64, self.encodings.as_ref().map(|encs| encs.registration_witness_key_encoding).unwrap_or_default(), force_canonical))?;
                    self.registration_witness.serialize(serializer, force_canonical)?;
                }
                _ => unreachable!()
            };
        }
        self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().end(serializer, force_canonical)
    }
}

impl Deserialize for RegistrationCbor {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.map_sz()?;
        let len_encoding: LenEncoding = len.into();
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        (|| -> Result<_, DeserializeError> {
            let mut orig_deser_order = Vec::new();
            let mut key_registration_key_encoding = None;
            let mut key_registration = None;
            let mut registration_witness_key_encoding = None;
            let mut registration_witness = None;
            let mut read = 0;
            while match len { cbor_event::LenSz::Len(n, _) => read < n, cbor_event::LenSz::Indefinite => true, } {
                match raw.cbor_type()? {
                    CBORType::UnsignedInteger => match raw.unsigned_integer_sz()? {
                        (61284, key_enc) =>  {
                            if key_registration.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(61284)).into());
                            }
                            let tmp_key_registration = KeyRegistration::deserialize(raw).map_err(|e: DeserializeError| e.annotate("key_registration"))?;
                            key_registration = Some(tmp_key_registration);
                            key_registration_key_encoding = Some(key_enc);
                            orig_deser_order.push(0);
                        },
                        (61285, key_enc) =>  {
                            if registration_witness.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(61285)).into());
                            }
                            let tmp_registration_witness = RegistrationWitness::deserialize(raw).map_err(|e: DeserializeError| e.annotate("registration_witness"))?;
                            registration_witness = Some(tmp_registration_witness);
                            registration_witness_key_encoding = Some(key_enc);
                            orig_deser_order.push(1);
                        },
                        (unknown_key, _enc) => return Err(DeserializeFailure::UnknownKey(Key::Uint(unknown_key)).into()),
                    },
                    CBORType::Text => return Err(DeserializeFailure::UnknownKey(Key::Str(raw.text()?)).into()),
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
            let key_registration = match key_registration {
                Some(x) => x,
                None => return Err(DeserializeFailure::MandatoryFieldMissing(Key::Uint(61284)).into()),
            };
            let registration_witness = match registration_witness {
                Some(x) => x,
                None => return Err(DeserializeFailure::MandatoryFieldMissing(Key::Uint(61285)).into()),
            };
            ();
            Ok(Self {
                key_registration,
                registration_witness,
                encodings: Some(RegistrationCborEncoding {
                    len_encoding,
                    orig_deser_order,
                    key_registration_key_encoding,
                    registration_witness_key_encoding,
                }),
            })
        })().map_err(|e| e.annotate("RegistrationCbor"))
    }
}

impl Serialize for RegistrationWitness {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_map_sz(self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().to_len_sz(1, force_canonical))?;
        let deser_order = self.encodings.as_ref().filter(|encs| !force_canonical && encs.orig_deser_order.len() == 1).map(|encs| encs.orig_deser_order.clone()).unwrap_or_else(|| vec![0]);
        for field_index in deser_order {
            match field_index {
                0 => {
                    serializer.write_unsigned_integer_sz(1u64, fit_sz(1u64, self.encodings.as_ref().map(|encs| encs.stake_witness_key_encoding).unwrap_or_default(), force_canonical))?;
                    self.stake_witness.serialize(serializer, force_canonical)?;
                }
                _ => unreachable!()
            };
        }
        self.encodings.as_ref().map(|encs| encs.len_encoding).unwrap_or_default().end(serializer, force_canonical)
    }
}

impl Deserialize for RegistrationWitness {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.map_sz()?;
        let len_encoding: LenEncoding = len.into();
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(1)?;
        (|| -> Result<_, DeserializeError> {
            let mut orig_deser_order = Vec::new();
            let mut stake_witness_key_encoding = None;
            let mut stake_witness = None;
            let mut read = 0;
            while match len { cbor_event::LenSz::Len(n, _) => read < n, cbor_event::LenSz::Indefinite => true, } {
                match raw.cbor_type()? {
                    CBORType::UnsignedInteger => match raw.unsigned_integer_sz()? {
                        (1, key_enc) =>  {
                            if stake_witness.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(1)).into());
                            }
                            let tmp_stake_witness = Ed25519Signature::deserialize(raw).map_err(|e: DeserializeError| e.annotate("stake_witness"))?;
                            stake_witness = Some(tmp_stake_witness);
                            stake_witness_key_encoding = Some(key_enc);
                            orig_deser_order.push(0);
                        },
                        (unknown_key, _enc) => return Err(DeserializeFailure::UnknownKey(Key::Uint(unknown_key)).into()),
                    },
                    CBORType::Text => return Err(DeserializeFailure::UnknownKey(Key::Str(raw.text()?)).into()),
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
            let stake_witness = match stake_witness {
                Some(x) => x,
                None => return Err(DeserializeFailure::MandatoryFieldMissing(Key::Uint(1)).into()),
            };
            ();
            Ok(Self {
                stake_witness,
                encodings: Some(RegistrationWitnessEncoding {
                    len_encoding,
                    orig_deser_order,
                    stake_witness_key_encoding,
                }),
            })
        })().map_err(|e| e.annotate("RegistrationWitness"))
    }
}
