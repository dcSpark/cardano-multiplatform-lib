use crate::{ledger::{common::binary::{Deserialize, CBORReadLen, SerializeEmbeddedGroup, DeserializeEmbeddedGroup}, self}, error::{DeserializeError, DeserializeFailure, Key}};

use super::*;
use std::io::{Seek, SeekFrom};

impl cbor_event::se::Serialize for AddrAttributes {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_map(cbor_event::Len::Len(match &self.stake_distribution { Some(StakeDistribution(StakeDistributionEnum::SingleKeyDistr(_))) => 1, _ => 0 } + match &self.derivation_path { Some(_) => 1, None => 0 } + match &self.protocol_magic { Some(_) => 1, None => 0 }))?;
        if let Some(StakeDistribution(StakeDistributionEnum::SingleKeyDistr(_))) = &self.stake_distribution {
            serializer.write_unsigned_integer(0)?;
            self.stake_distribution.serialize(serializer)?;
        }
        if let Some(field) = &self.derivation_path {
            serializer.write_unsigned_integer(1)?;
            cbor_event::se::serialize_cbor_in_cbor(field.0.as_slice(), serializer)?;
        }
        if let Some(field) = &self.protocol_magic {
            serializer.write_unsigned_integer(2)?;
            cbor_event::se::serialize_cbor_in_cbor(field, serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for AddrAttributes {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.map()?;
            let mut read_len = CBORReadLen::new(len);
            let mut stake_distribution = None;
            let mut derivation_path = None;
            let mut protocol_magic = None;
            let mut read = 0;
            while match len { cbor_event::Len::Len(n) => read < n as usize, cbor_event::Len::Indefinite => true, } {
                match raw.cbor_type()? {
                    CBORType::UnsignedInteger => match raw.unsigned_integer()? {
                        0 =>  {
                            if stake_distribution.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(0)).into());
                            }
                            stake_distribution = Some((|| -> Result<_, DeserializeError> {
                                Ok(StakeDistribution::deserialize(raw)?)
                            })().map_err(|e| e.annotate("stake_distribution"))?);
                        },
                        1 =>  {
                            if derivation_path.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(1)).into());
                            }
                            derivation_path = Some((|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                let bytes = raw.bytes()?;
                                // bytes encoded as CBOR encoded as Bytes in CBOR.
                                let mut inner_cbor = Deserializer::from(std::io::Cursor::new(bytes));
                                Ok(inner_cbor.bytes()?)
                            })().map_err(|e| e.annotate("derivation_path"))?);
                        },
                        2 =>  {
                            if protocol_magic.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(2)).into());
                            }
                            protocol_magic = Some((|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                let bytes = raw.bytes()?;
                                // integer encoded as CBOR encoded as Bytes in CBOR.
                                let n = Deserializer::from(std::io::Cursor::new(bytes)).deserialize::<u32>()?;
                                Ok(n)
                            })().map_err(|e| e.annotate("protocol_magic"))?);
                        },
                        unknown_key => return Err(DeserializeFailure::UnknownKey(Key::Uint(unknown_key)).into()),
                    },
                    CBORType::Text => match raw.text()?.as_str() {
                        unknown_key => return Err(DeserializeFailure::UnknownKey(Key::Str(unknown_key.to_owned())).into()),
                    },
                    CBORType::Special => match len {
                        cbor_event::Len::Len(_) => return Err(DeserializeFailure::BreakInDefiniteLen.into()),
                        cbor_event::Len::Indefinite => match raw.special()? {
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
                stake_distribution,
                derivation_path: derivation_path.map(HDAddressPayload),
                protocol_magic: protocol_magic.map(ProtocolMagic),
            })
        })().map_err(|e| e.annotate("AddrAttributes"))
    }
}

impl cbor_event::se::Serialize for StakeDistributionEnum {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        let mut se = Serializer::new_vec();
        match self {
            StakeDistributionEnum::BootstrapEraDistr(x) => x.serialize(&mut se),
            StakeDistributionEnum::SingleKeyDistr(x) => x.serialize(&mut se),
        }?;
        serializer.write_bytes(&se.finalize())
    }
}

impl Deserialize for StakeDistributionEnum {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let mut inner = &mut Deserializer::from(std::io::Cursor::new(raw.bytes()?));

            let len = inner.array()?;
            let mut read_len = CBORReadLen::new(len);
            let initial_position = inner.as_mut_ref().seek(SeekFrom::Current(0)).unwrap();
            match (|inner: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(BootstrapEraDistr::deserialize_as_embedded_group(inner, len)?)
            })(inner)
            {
                Ok(variant) => return Ok(StakeDistributionEnum::BootstrapEraDistr(variant)),
                Err(_) => inner.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            match (|inner: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(SingleKeyDistr::deserialize_as_embedded_group(inner, len)?)
            })(inner)
            {
                Ok(variant) => return Ok(StakeDistributionEnum::SingleKeyDistr(variant)),
                Err(_) => inner.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            match len {
                cbor_event::Len::Len(_) => read_len.finish()?,
                cbor_event::Len::Indefinite => match inner.special()? {
                    CBORSpecial::Break => read_len.finish()?,
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Err(DeserializeError::new("StakeDistributionEnum", DeserializeFailure::NoVariantMatched.into()))
        })().map_err(|e| e.annotate("StakeDistributionEnum"))
    }
}

impl cbor_event::se::Serialize for StakeDistribution {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.0.serialize(serializer)
    }
}

impl Deserialize for StakeDistribution {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        Ok(Self(StakeDistributionEnum::deserialize(raw)?))
    }
}

impl cbor_event::se::Serialize for ByronAddress {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        serializer.write_tag(24u64)?;
        serializer.write_bytes(&self.addr)?;
        serializer.write_unsigned_integer(self.crc32)?;
        Ok(serializer)
    }
}

impl Deserialize for ByronAddress {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(2)?;
            let addr = (|| -> Result<_, DeserializeError> {
                Ok(match raw.tag()? {
                    24 => {
                        raw.bytes()?
                    },
                    tag => return Err(DeserializeFailure::TagMismatch{ found: tag, expected: 24 }.into()),
                })
            })().map_err(|e| e.annotate("addr"))?;
            let crc32 = (|| -> Result<_, DeserializeError> {
                Ok(u64::deserialize(raw)?)
            })().map_err(|e| e.annotate("crc32"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ByronAddress::new(
                addr.clone(),
                crc32,
            ).map_err(|_| DeserializeError::new("ByronAddress", DeserializeFailure::ChecksumMismatch { found: crc32, expected: crate::byron::crc32::crc32(&addr) as u64 }))
        })().map_err(|e| e.annotate("ByronAddress"))
    }
}

impl cbor_event::se::Serialize for AddressContent {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(3))?;
        self.address_id.serialize(serializer)?;
        self.addr_attr.serialize(serializer)?;
        self.addr_type.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for AddressContent {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(3)?;
            let address_id = (|| -> Result<_, DeserializeError> {
                Ok(AddressId::deserialize(raw)?)
            })().map_err(|e| e.annotate("address_id"))?;
            let addr_attr = (|| -> Result<_, DeserializeError> {
                Ok(AddrAttributes::deserialize(raw)?)
            })().map_err(|e| e.annotate("addr_attr"))?;
            let addr_type = (|| -> Result<_, DeserializeError> {
                Ok(AddrType::deserialize(raw)?)
            })().map_err(|e| e.annotate("addr_type"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(AddressContent {
                address_id,
                addr_attr,
                addr_type,
            })
        })().map_err(|e| e.annotate("AddressContent"))
    }
}

impl cbor_event::se::Serialize for AddrTypeEnum {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            AddrTypeEnum::ATPubKey => {
                serializer.write_unsigned_integer(0u64)
            },
            AddrTypeEnum::ATScript => {
                serializer.write_unsigned_integer(1u64)
            },
            AddrTypeEnum::ATRedeem => {
                serializer.write_unsigned_integer(2u64)
            },
        }
    }
}

impl Deserialize for AddrTypeEnum {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let initial_position = raw.as_mut_ref().seek(SeekFrom::Current(0)).unwrap();
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let ATPubKey_value = raw.unsigned_integer()?;
                if ATPubKey_value != 0 {
                    return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(ATPubKey_value), expected: Key::Uint(0) }.into());
                }
                Ok(())
            })(raw)
            {
                Ok(()) => return Ok(AddrTypeEnum::ATPubKey),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let ATScript_value = raw.unsigned_integer()?;
                if ATScript_value != 1 {
                    return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(ATScript_value), expected: Key::Uint(1) }.into());
                }
                Ok(())
            })(raw)
            {
                Ok(()) => return Ok(AddrTypeEnum::ATScript),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let ATRedeem_value = raw.unsigned_integer()?;
                if ATRedeem_value != 2 {
                    return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(ATRedeem_value), expected: Key::Uint(2) }.into());
                }
                Ok(())
            })(raw)
            {
                Ok(()) => return Ok(AddrTypeEnum::ATRedeem),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            Err(DeserializeError::new("AddrTypeEnum", DeserializeFailure::NoVariantMatched.into()))
        })().map_err(|e| e.annotate("AddrTypeEnum"))
    }
}

impl cbor_event::se::Serialize for AddrType {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.0.serialize(serializer)
    }
}

impl Deserialize for AddrType {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        Ok(Self(AddrTypeEnum::deserialize(raw)?))
    }
}

impl cbor_event::se::Serialize for BootstrapEraDistr {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(1))?;
        self.serialize_as_embedded_group(serializer)
    }
}

impl SerializeEmbeddedGroup for BootstrapEraDistr {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(1u64)?;
        Ok(serializer)
    }
}

impl Deserialize for BootstrapEraDistr {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(1)?;
            let ret = Self::deserialize_as_embedded_group(raw, len);
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("BootstrapEraDistr"))
    }
}

impl DeserializeEmbeddedGroup for BootstrapEraDistr {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, len: cbor_event::Len) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let index_0_value = raw.unsigned_integer()?;
            if index_0_value != 1 {
                return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(index_0_value), expected: Key::Uint(1) }.into());
            }
            Ok(())
        })().map_err(|e| e.annotate("index_0"))?;
        Ok(BootstrapEraDistr {
        })
    }
}

impl cbor_event::se::Serialize for SingleKeyDistr {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        self.serialize_as_embedded_group(serializer)
    }
}

impl SerializeEmbeddedGroup for SingleKeyDistr {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(0u64)?;
        self.stakeholder_id.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for SingleKeyDistr {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(2)?;
            let ret = Self::deserialize_as_embedded_group(raw, len);
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("SingleKeyDistr"))
    }
}

impl DeserializeEmbeddedGroup for SingleKeyDistr {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, len: cbor_event::Len) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let index_0_value = raw.unsigned_integer()?;
            if index_0_value != 0 {
                return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(index_0_value), expected: Key::Uint(0) }.into());
            }
            Ok(())
        })().map_err(|e| e.annotate("index_0"))?;
        let stakeholder_id = (|| -> Result<_, DeserializeError> {
            Ok(Blake2b224::deserialize(raw)?)
        })().map_err(|e| e.annotate("stakeholder_id"))?;
        Ok(SingleKeyDistr {
            stakeholder_id: StakeholderId(stakeholder_id.as_hash_bytes().clone()),
        })
    }
}


impl cbor_event::se::Serialize for SpendingDataEnum {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            SpendingDataEnum::SpendingDataPubKeyASD(x) => x.serialize(serializer),
            SpendingDataEnum::SpendingDataScriptASD(x) => x.serialize(serializer),
            SpendingDataEnum::SpendingDataRedeemASD(x) => x.serialize(serializer),
        }
    }
}

impl Deserialize for SpendingDataEnum {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let mut read_len = CBORReadLen::new(len);
            let initial_position = raw.as_mut_ref().seek(SeekFrom::Current(0)).unwrap();
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(SpendingDataPubKeyASD::deserialize_as_embedded_group(raw, len)?)
            })(raw)
            {
                Ok(variant) => return Ok(SpendingDataEnum::SpendingDataPubKeyASD(variant)),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(SpendingDataScriptASD::deserialize_as_embedded_group(raw, len)?)
            })(raw)
            {
                Ok(variant) => return Ok(SpendingDataEnum::SpendingDataScriptASD(variant)),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(SpendingDataRedeemASD::deserialize_as_embedded_group(raw, len)?)
            })(raw)
            {
                Ok(variant) => return Ok(SpendingDataEnum::SpendingDataRedeemASD(variant)),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            match len {
                cbor_event::Len::Len(_) => read_len.finish()?,
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => read_len.finish()?,
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Err(DeserializeError::new("SpendingDataEnum", DeserializeFailure::NoVariantMatched.into()))
        })().map_err(|e| e.annotate("SpendingDataEnum"))
    }
}

impl cbor_event::se::Serialize for SpendingData {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.0.serialize(serializer)
    }
}

impl Deserialize for SpendingData {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        Ok(Self(SpendingDataEnum::deserialize(raw)?))
    }
}

impl cbor_event::se::Serialize for SpendingDataPubKeyASD {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        self.serialize_as_embedded_group(serializer)
    }
}

impl SerializeEmbeddedGroup for SpendingDataPubKeyASD {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(0u64)?;
        self.public_ed25519_bip32.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for SpendingDataPubKeyASD {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(2)?;
            let ret = Self::deserialize_as_embedded_group(raw, len);
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("SpendingDataPubKeyASD"))
    }
}

impl DeserializeEmbeddedGroup for SpendingDataPubKeyASD {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, len: cbor_event::Len) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let index_0_value = raw.unsigned_integer()?;
            if index_0_value != 0 {
                return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(index_0_value), expected: Key::Uint(0) }.into());
            }
            Ok(())
        })().map_err(|e| e.annotate("index_0"))?;
        let public_ed25519_bip32 = (|| -> Result<_, DeserializeError> {
            Ok(chain_crypto::PublicKey::<Ed25519Bip32>::deserialize(raw)?)
        })().map_err(|e| e.annotate("public_ed25519_bip32"))?;
        Ok(SpendingDataPubKeyASD {
            public_ed25519_bip32,
        })
    }
}

impl cbor_event::se::Serialize for SpendingDataRedeemASD {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        self.serialize_as_embedded_group(serializer)
    }
}

impl SerializeEmbeddedGroup for SpendingDataRedeemASD {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(2u64)?;
        self.public_ed25519.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for SpendingDataRedeemASD {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(2)?;
            let ret = Self::deserialize_as_embedded_group(raw, len);
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("SpendingDataRedeemASD"))
    }
}

impl DeserializeEmbeddedGroup for SpendingDataRedeemASD {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, len: cbor_event::Len) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let index_0_value = raw.unsigned_integer()?;
            if index_0_value != 2 {
                return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(index_0_value), expected: Key::Uint(2) }.into());
            }
            Ok(())
        })().map_err(|e| e.annotate("index_0"))?;
        let public_ed25519 = (|| -> Result<_, DeserializeError> {
            Ok(chain_crypto::PublicKey::<Ed25519>::deserialize(raw)?)
        })().map_err(|e| e.annotate("public_ed25519"))?;
        Ok(SpendingDataRedeemASD {
            public_ed25519,
        })
    }
}

impl cbor_event::se::Serialize for SpendingDataScriptASD {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        self.serialize_as_embedded_group(serializer)
    }
}

impl SerializeEmbeddedGroup for SpendingDataScriptASD {
    fn serialize_as_embedded_group<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(1u64)?;
        self.script.0.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for SpendingDataScriptASD {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let mut read_len = CBORReadLen::new(len);
            read_len.read_elems(2)?;
            let ret = Self::deserialize_as_embedded_group(raw, len);
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            ret
        })().map_err(|e| e.annotate("SpendingDataScriptASD"))
    }
}

impl DeserializeEmbeddedGroup for SpendingDataScriptASD {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(raw: &mut Deserializer<R>, len: cbor_event::Len) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let index_0_value = raw.unsigned_integer()?;
            if index_0_value != 1 {
                return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(index_0_value), expected: Key::Uint(1) }.into());
            }
            Ok(())
        })().map_err(|e| e.annotate("index_0"))?;
        let script = (|| -> Result<_, DeserializeError> {
            <[u8; 32]>::deserialize(raw).map(ByronScript)
        })().map_err(|e| e.annotate("script"))?;
        Ok(SpendingDataScriptASD {
            script,
        })
    }
}

impl cbor_event::se::Serialize for ProtocolMagic {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(self.0 as u64)
    }
}
impl Deserialize for ProtocolMagic {
    fn deserialize<R: BufRead + Seek>(reader: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let v = reader.unsigned_integer()? as u32;
        Ok(ProtocolMagic::from(v))
    }
}

impl cbor_event::se::Serialize for HDAddressPayload {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_bytes(&self.0)
    }
}
impl Deserialize for HDAddressPayload {
    fn deserialize<R: BufRead + Seek>(reader: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let v = reader.bytes()?;
        Ok(HDAddressPayload(v))
    }
}
impl cbor_event::se::Serialize for ByronScript {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_bytes(self.0)
    }
}
impl Deserialize for ByronScript {
    fn deserialize<R: BufRead + Seek>(reader: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        <[u8; 32]>::deserialize(reader).map(ByronScript)
    }
}
