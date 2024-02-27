// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use super::*;
use cbor_event::de::Deserializer;
use cbor_event::se::Serializer;
use cbor_event::{self, LenSz};
use cml_core::serialization::fit_sz;
use cml_core::{
    error::{DeserializeError, DeserializeFailure, Key},
    serialization::{CBORReadLen, Deserialize},
};
use cml_crypto::RawBytesEncoding;
use std::io::{BufRead, Seek, SeekFrom, Write};

impl cbor_event::se::Serialize for AddrAttributes {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        // NOTE: This was manually modified *slightly*
        // The direct cddl-codegen code will always include the stake distribution
        // but the old byron code seems to only put it in when it's not bootstrap.
        serializer.write_map(cbor_event::Len::Len(
            match &self.stake_distribution {
                Some(StakeDistribution::SingleKey(_)) => 1,
                _ => 0,
            } + match &self.derivation_path {
                Some(_) => 1,
                None => 0,
            } + match &self.protocol_magic {
                Some(_) => 1,
                None => 0,
            },
        ))?;
        if let Some(StakeDistribution::SingleKey(_)) = &self.stake_distribution {
            serializer.write_unsigned_integer(0u64)?;
            let mut stake_distribution_inner_se = Serializer::new_vec();
            self.stake_distribution
                .as_ref()
                .unwrap()
                .serialize(&mut stake_distribution_inner_se)?;
            let stake_distribution_bytes = stake_distribution_inner_se.finalize();
            serializer.write_bytes(&stake_distribution_bytes)?;
        }
        if let Some(field) = &self.derivation_path {
            serializer.write_unsigned_integer(1u64)?;
            let mut derivation_path_inner_se = Serializer::new_vec();
            field.serialize(&mut derivation_path_inner_se)?;
            let derivation_path_bytes = derivation_path_inner_se.finalize();
            serializer.write_bytes(&derivation_path_bytes)?;
        }
        if let Some(field) = &self.protocol_magic {
            serializer.write_unsigned_integer(2u64)?;
            let mut protocol_magic_inner_se = Serializer::new_vec();
            field.serialize(&mut protocol_magic_inner_se)?;
            let protocol_magic_bytes = protocol_magic_inner_se.finalize();
            serializer.write_bytes(&protocol_magic_bytes)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for AddrAttributes {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.map()?;
        let mut read_len = CBORReadLen::from(len);
        (|| -> Result<_, DeserializeError> {
            let mut stake_distribution = None;
            let mut derivation_path = None;
            let mut protocol_magic = None;
            let mut read = 0;
            while match len {
                cbor_event::Len::Len(n) => read < n,
                cbor_event::Len::Indefinite => true,
            } {
                match raw.cbor_type()? {
                    cbor_event::Type::UnsignedInteger => match raw.unsigned_integer()? {
                        0 => {
                            if stake_distribution.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(0)).into());
                            }
                            stake_distribution = Some(
                                (|| -> Result<_, DeserializeError> {
                                    let stake_distribution_bytes = raw.bytes()?;
                                    let inner_de = &mut Deserializer::from(std::io::Cursor::new(
                                        stake_distribution_bytes,
                                    ));
                                    read_len.read_elems(1)?;
                                    StakeDistribution::deserialize(inner_de)
                                })()
                                .map_err(|e| e.annotate("stake_distribution"))?,
                            );
                        }
                        1 => {
                            if derivation_path.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(1)).into());
                            }
                            derivation_path = Some(
                                (|| -> Result<_, DeserializeError> {
                                    let derivation_path_bytes = raw.bytes()?;
                                    let inner_de = &mut Deserializer::from(std::io::Cursor::new(
                                        derivation_path_bytes,
                                    ));
                                    read_len.read_elems(1)?;
                                    HDAddressPayload::deserialize(inner_de)
                                })()
                                .map_err(|e| e.annotate("derivation_path"))?,
                            );
                        }
                        2 => {
                            if protocol_magic.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(2)).into());
                            }
                            protocol_magic = Some(
                                (|| -> Result<_, DeserializeError> {
                                    let protocol_magic_bytes = raw.bytes()?;
                                    let inner_de = &mut Deserializer::from(std::io::Cursor::new(
                                        protocol_magic_bytes,
                                    ));
                                    read_len.read_elems(1)?;
                                    ProtocolMagic::deserialize(inner_de)
                                })()
                                .map_err(|e| e.annotate("protocol_magic"))?,
                            );
                        }
                        unknown_key => {
                            return Err(
                                DeserializeFailure::UnknownKey(Key::Uint(unknown_key)).into()
                            )
                        }
                    },
                    cbor_event::Type::Text => {
                        return Err(DeserializeFailure::UnknownKey(Key::Str(raw.text()?)).into())
                    }
                    cbor_event::Type::Special => match len {
                        cbor_event::Len::Len(_) => {
                            return Err(DeserializeFailure::BreakInDefiniteLen.into())
                        }
                        cbor_event::Len::Indefinite => match raw.special()? {
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
                stake_distribution,
                derivation_path,
                protocol_magic,
            })
        })()
        .map_err(|e| e.annotate("AddrAttributes"))
    }
}

impl cbor_event::se::Serialize for AddressContent {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(3))?;
        serializer.write_bytes(self.address_id.to_raw_bytes())?;
        self.addr_attributes.serialize(serializer)?;
        match &self.addr_type {
            ByronAddrType::PublicKey => serializer.write_unsigned_integer(0u64),
            ByronAddrType::Script => serializer.write_unsigned_integer(1u64),
            ByronAddrType::Redeem => serializer.write_unsigned_integer(2u64),
        }?;
        Ok(serializer)
    }
}

impl Deserialize for AddressContent {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::new(match len {
            cbor_event::Len::Len(n) => LenSz::Len(n, fit_sz(n, None, true)),
            cbor_event::Len::Indefinite => LenSz::Indefinite,
        });
        read_len.read_elems(3)?;
        (|| -> Result<_, DeserializeError> {
            let address_id = raw
                .bytes()
                .map_err(Into::<DeserializeError>::into)
                .and_then(|bytes| {
                    AddressId::from_raw_bytes(&bytes)
                        .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
                })
                .map_err(|e: DeserializeError| e.annotate("address_id"))?;
            let addr_attributes = AddrAttributes::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("addr_attributes"))?;
            let addr_type = (|| -> Result<_, DeserializeError> {
                let initial_position = raw.as_mut_ref().stream_position().unwrap();
                let deser_variant = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                    let public_key_value = raw.unsigned_integer()?;
                    if public_key_value != 0 {
                        return Err(DeserializeFailure::FixedValueMismatch {
                            found: Key::Uint(public_key_value),
                            expected: Key::Uint(0),
                        }
                        .into());
                    }
                    Ok(())
                })(raw);
                match deser_variant {
                    Ok(()) => return Ok(ByronAddrType::PublicKey),
                    Err(_) => raw
                        .as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap(),
                };
                let deser_variant = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                    let script_value = raw.unsigned_integer()?;
                    if script_value != 1 {
                        return Err(DeserializeFailure::FixedValueMismatch {
                            found: Key::Uint(script_value),
                            expected: Key::Uint(1),
                        }
                        .into());
                    }
                    Ok(())
                })(raw);
                match deser_variant {
                    Ok(()) => return Ok(ByronAddrType::Script),
                    Err(_) => raw
                        .as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap(),
                };
                let deser_variant = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                    let redeem_value = raw.unsigned_integer()?;
                    if redeem_value != 2 {
                        return Err(DeserializeFailure::FixedValueMismatch {
                            found: Key::Uint(redeem_value),
                            expected: Key::Uint(2),
                        }
                        .into());
                    }
                    Ok(())
                })(raw);
                match deser_variant {
                    Ok(()) => return Ok(ByronAddrType::Redeem),
                    Err(_) => raw
                        .as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap(),
                };
                Err(DeserializeError::new(
                    "ByronAddrType",
                    DeserializeFailure::NoVariantMatched,
                ))
            })()
            .map_err(|e| e.annotate("addr_type"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(AddressContent {
                address_id,
                addr_attributes,
                addr_type,
            })
        })()
        .map_err(|e| e.annotate("AddressContent"))
    }
}

impl cbor_event::se::Serialize for ByronAddress {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        serializer.write_tag(24u64)?;
        let mut content_inner_se = Serializer::new_vec();
        self.content.serialize(&mut content_inner_se)?;
        let content_bytes = content_inner_se.finalize();
        serializer.write_bytes(&content_bytes)?;
        self.crc.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for ByronAddress {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::from(len);
        read_len.read_elems(2)?;
        (|| -> Result<_, DeserializeError> {
            let (content, content_crc) = (|| -> Result<_, DeserializeError> {
                match raw.tag()? {
                    24 => {
                        let content_bytes = raw.bytes()?;
                        let crc = crate::byron::crc32::crc32(&content_bytes);
                        let inner_de = &mut Deserializer::from(std::io::Cursor::new(content_bytes));
                        Ok((AddressContent::deserialize(inner_de)?, crc))
                    }
                    tag => Err(DeserializeFailure::TagMismatch {
                        found: tag,
                        expected: 24,
                    }
                    .into()),
                }
            })()
            .map_err(|e| e.annotate("content"))?;
            let crc = Crc32::deserialize(raw).map_err(|e: DeserializeError| e.annotate("crc"))?;
            if Into::<u32>::into(crc) != content_crc {
                return Err(DeserializeFailure::InvalidStructure(Box::new(
                    ByronAddressError::InvalidCRC {
                        found: crc,
                        expected: content_crc.into(),
                    },
                ))
                .into());
            }
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(ByronAddress { content, crc })
        })()
        .map_err(|e| e.annotate("ByronAddress"))
    }
}

impl cbor_event::se::Serialize for ByronTxOut {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        self.address.serialize(serializer)?;
        serializer.write_unsigned_integer(self.amount)?;
        Ok(serializer)
    }
}

impl Deserialize for ByronTxOut {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::from(len);
        read_len.read_elems(2)?;
        (|| -> Result<_, DeserializeError> {
            let address = ByronAddress::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("address"))?;
            let amount =
                Ok(raw.unsigned_integer()?).map_err(|e: DeserializeError| e.annotate("amount"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(ByronTxOut { address, amount })
        })()
        .map_err(|e| e.annotate("ByronTxOut"))
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
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        Ok(Self(raw.bytes()? as Vec<u8>))
    }
}

impl cbor_event::se::Serialize for SpendingData {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            SpendingData::SpendingDataPubKey(pubkey) => {
                serializer.write_array(cbor_event::Len::Len(2))?;
                serializer.write_unsigned_integer(0u64)?;
                serializer.write_bytes(pubkey.to_raw_bytes())?;
                Ok(serializer)
            }
            SpendingData::SpendingDataScript(script) => {
                serializer.write_array(cbor_event::Len::Len(2))?;
                serializer.write_unsigned_integer(1u64)?;
                serializer.write_bytes(script.to_raw_bytes())?;
                Ok(serializer)
            }
            SpendingData::SpendingDataRedeem(redeem) => {
                serializer.write_array(cbor_event::Len::Len(2))?;
                serializer.write_unsigned_integer(2u64)?;
                serializer.write_bytes(redeem.to_raw_bytes())?;
                Ok(serializer)
            }
        }
    }
}

impl Deserialize for SpendingData {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let _read_len = CBORReadLen::from(len);
            let initial_position = raw.as_mut_ref().stream_position().unwrap();
            let deser_variant = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                (|| -> Result<_, DeserializeError> {
                    let tag_value = raw.unsigned_integer()?;
                    if tag_value != 0 {
                        return Err(DeserializeFailure::FixedValueMismatch {
                            found: Key::Uint(tag_value),
                            expected: Key::Uint(0),
                        }
                        .into());
                    }
                    Ok(())
                })()
                .map_err(|e| e.annotate("tag"))?;
                let pubkey = raw
                    .bytes()
                    .map_err(Into::<DeserializeError>::into)
                    .and_then(|bytes| {
                        Bip32PublicKey::from_raw_bytes(&bytes)
                            .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
                    })
                    .map_err(|e: DeserializeError| e.annotate("pubkey"))?;
                match len {
                    cbor_event::Len::Len(_) => (),
                    cbor_event::Len::Indefinite => match raw.special()? {
                        cbor_event::Special::Break => (),
                        _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                    },
                }
                Ok(Self::SpendingDataPubKey(pubkey))
            })(raw);
            match deser_variant {
                Ok(variant) => return Ok(variant),
                Err(_) => raw
                    .as_mut_ref()
                    .seek(SeekFrom::Start(initial_position))
                    .unwrap(),
            };
            let deser_variant = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                (|| -> Result<_, DeserializeError> {
                    let tag_value = raw.unsigned_integer()?;
                    if tag_value != 1 {
                        return Err(DeserializeFailure::FixedValueMismatch {
                            found: Key::Uint(tag_value),
                            expected: Key::Uint(1),
                        }
                        .into());
                    }
                    Ok(())
                })()
                .map_err(|e| e.annotate("tag"))?;
                let script = raw
                    .bytes()
                    .map_err(Into::<DeserializeError>::into)
                    .and_then(|bytes| {
                        ByronScript::from_raw_bytes(&bytes)
                            .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
                    })
                    .map_err(|e: DeserializeError| e.annotate("script"))?;
                match len {
                    cbor_event::Len::Len(_) => (),
                    cbor_event::Len::Indefinite => match raw.special()? {
                        cbor_event::Special::Break => (),
                        _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                    },
                }
                Ok(Self::SpendingDataScript(script))
            })(raw);
            match deser_variant {
                Ok(variant) => return Ok(variant),
                Err(_) => raw
                    .as_mut_ref()
                    .seek(SeekFrom::Start(initial_position))
                    .unwrap(),
            };
            let deser_variant = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                (|| -> Result<_, DeserializeError> {
                    let tag_value = raw.unsigned_integer()?;
                    if tag_value != 2 {
                        return Err(DeserializeFailure::FixedValueMismatch {
                            found: Key::Uint(tag_value),
                            expected: Key::Uint(2),
                        }
                        .into());
                    }
                    Ok(())
                })()
                .map_err(|e| e.annotate("tag"))?;
                let redeem = raw
                    .bytes()
                    .map_err(Into::<DeserializeError>::into)
                    .and_then(|bytes| {
                        PublicKey::from_raw_bytes(&bytes)
                            .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
                    })
                    .map_err(|e: DeserializeError| e.annotate("redeem"))?;
                match len {
                    cbor_event::Len::Len(_) => (),
                    cbor_event::Len::Indefinite => match raw.special()? {
                        cbor_event::Special::Break => (),
                        _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                    },
                }
                Ok(Self::SpendingDataRedeem(redeem))
            })(raw);
            match deser_variant {
                Ok(variant) => return Ok(variant),
                Err(_) => raw
                    .as_mut_ref()
                    .seek(SeekFrom::Start(initial_position))
                    .unwrap(),
            };
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Err(DeserializeError::new(
                "SpendingData",
                DeserializeFailure::NoVariantMatched,
            ))
        })()
        .map_err(|e| e.annotate("SpendingData"))
    }
}

impl cbor_event::se::Serialize for StakeDistribution {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            StakeDistribution::SingleKey(stakeholder_id) => {
                serializer.write_array(cbor_event::Len::Len(2))?;
                serializer.write_unsigned_integer(0u64)?;
                serializer.write_bytes(stakeholder_id.to_raw_bytes())?;
                Ok(serializer)
            }
            StakeDistribution::BootstrapEra => {
                serializer.write_array(cbor_event::Len::Len(1))?;
                serializer.write_unsigned_integer(1u64)
            }
        }
    }
}

impl Deserialize for StakeDistribution {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let _read_len = CBORReadLen::from(len);
            let initial_position = raw.as_mut_ref().stream_position().unwrap();
            let deser_variant = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                (|| -> Result<_, DeserializeError> {
                    let tag_value = raw.unsigned_integer()?;
                    if tag_value != 0 {
                        return Err(DeserializeFailure::FixedValueMismatch {
                            found: Key::Uint(tag_value),
                            expected: Key::Uint(0),
                        }
                        .into());
                    }
                    Ok(())
                })()
                .map_err(|e| e.annotate("tag"))?;
                let stakeholder_id = raw
                    .bytes()
                    .map_err(Into::<DeserializeError>::into)
                    .and_then(|bytes| {
                        StakeholderId::from_raw_bytes(&bytes)
                            .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
                    })
                    .map_err(|e: DeserializeError| e.annotate("stakeholder_id"))?;
                match len {
                    cbor_event::Len::Len(_) => (),
                    cbor_event::Len::Indefinite => match raw.special()? {
                        cbor_event::Special::Break => (),
                        _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                    },
                }
                Ok(Self::SingleKey(stakeholder_id))
            })(raw);
            match deser_variant {
                Ok(variant) => return Ok(variant),
                Err(_) => raw
                    .as_mut_ref()
                    .seek(SeekFrom::Start(initial_position))
                    .unwrap(),
            };
            let deser_variant = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let bootstrap_era_distr_value = raw.unsigned_integer()?;
                if bootstrap_era_distr_value != 1 {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(bootstrap_era_distr_value),
                        expected: Key::Uint(1),
                    }
                    .into());
                }
                Ok(())
            })(raw);
            match deser_variant {
                Ok(()) => return Ok(StakeDistribution::BootstrapEra),
                Err(_) => raw
                    .as_mut_ref()
                    .seek(SeekFrom::Start(initial_position))
                    .unwrap(),
            };
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Err(DeserializeError::new(
                "StakeDistribution",
                DeserializeFailure::NoVariantMatched,
            ))
        })()
        .map_err(|e| e.annotate("StakeDistribution"))
    }
}
