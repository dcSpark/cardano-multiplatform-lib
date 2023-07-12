// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use super::*;
use cbor_event;
use cbor_event::de::Deserializer;
use cbor_event::se::{Serialize, Serializer};
use cml_core::error::*;
use cml_core::serialization::*;
use std::io::{BufRead, Seek, SeekFrom, Write};

impl cbor_event::se::Serialize for BlockHeaderExtraData {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(4))?;
        self.block_version.serialize(serializer)?;
        self.software_version.serialize(serializer)?;
        serializer.write_map(cbor_event::Len::Len(self.byron_attributes.len() as u64))?;
        for (key, value) in self.byron_attributes.iter() {
            key.serialize(serializer)?;
            value.serialize(serializer)?;
        }
        serializer.write_bytes(&self.extra_proof.to_raw_bytes())?;
        Ok(serializer)
    }
}

impl Deserialize for BlockHeaderExtraData {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(4)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let block_version = ByronBlockVersion::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("block_version"))?;
            let software_version = ByronSoftwareVersion::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("software_version"))?;
            let byron_attributes = (|| -> Result<_, DeserializeError> {
                let mut byron_attributes_table = BTreeMap::new();
                let byron_attributes_len = raw.map()?;
                while match byron_attributes_len {
                    cbor_event::Len::Len(n) => (byron_attributes_table.len() as u64) < n,
                    cbor_event::Len::Indefinite => true,
                } {
                    if raw.cbor_type()? == cbor_event::Type::Special {
                        assert_eq!(raw.special()?, cbor_event::Special::Break);
                        break;
                    }
                    let byron_attributes_key = ByronAny::deserialize(raw)?;
                    let byron_attributes_value = ByronAny::deserialize(raw)?;
                    if byron_attributes_table
                        .insert(byron_attributes_key.clone(), byron_attributes_value)
                        .is_some()
                    {
                        return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from(
                            "some complicated/unsupported type",
                        )))
                        .into());
                    }
                }
                Ok(byron_attributes_table)
            })()
            .map_err(|e| e.annotate("byron_attributes"))?;
            let extra_proof = raw
                .bytes()
                .map_err(Into::<DeserializeError>::into)
                .and_then(|bytes| {
                    Blake2b256::from_raw_bytes(&bytes)
                        .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
                })
                .map_err(|e: DeserializeError| e.annotate("extra_proof"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(BlockHeaderExtraData {
                block_version,
                software_version,
                byron_attributes,
                extra_proof,
            })
        })()
        .map_err(|e| e.annotate("BlockHeaderExtraData"))
    }
}

impl cbor_event::se::Serialize for ByronBlock {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            ByronBlock::EpochBoundary(epoch_boundary) => epoch_boundary.serialize(serializer),
            ByronBlock::Main(main) => main.serialize(serializer),
        }
    }
}

impl Deserialize for ByronBlock {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let initial_position = raw.as_mut_ref().seek(SeekFrom::Current(0)).unwrap();
            let mut errs = Vec::new();
            let deser_variant: Result<_, DeserializeError> = ByronEbBlock::deserialize(raw);
            match deser_variant {
                Ok(epoch_boundary) => return Ok(Self::EpochBoundary(epoch_boundary)),
                Err(e) => {
                    errs.push(e.annotate("EpochBoundary"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant: Result<_, DeserializeError> = ByronMainBlock::deserialize(raw);
            match deser_variant {
                Ok(main) => return Ok(Self::Main(main)),
                Err(e) => {
                    errs.push(e.annotate("Main"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            Err(DeserializeError::new(
                "ByronBlock",
                DeserializeFailure::NoVariantMatchedWithCauses(errs),
            ))
        })()
        .map_err(|e| e.annotate("ByronBlock"))
    }
}

impl cbor_event::se::Serialize for ByronBlockBody {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(4))?;
        serializer.write_array(cbor_event::Len::Len(self.tx_payload.len() as u64))?;
        for element in self.tx_payload.iter() {
            element.serialize(serializer)?;
        }
        self.ssc_payload.serialize(serializer)?;
        serializer.write_array(cbor_event::Len::Len(self.dlg_payload.len() as u64))?;
        for element in self.dlg_payload.iter() {
            element.serialize(serializer)?;
        }
        self.upd_payload.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for ByronBlockBody {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(4)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let tx_payload = (|| -> Result<_, DeserializeError> {
                let mut tx_payload_arr = Vec::new();
                let len = raw.array()?;
                while match len {
                    cbor_event::Len::Len(n) => (tx_payload_arr.len() as u64) < n,
                    cbor_event::Len::Indefinite => true,
                } {
                    if raw.cbor_type()? == cbor_event::Type::Special {
                        assert_eq!(raw.special()?, cbor_event::Special::Break);
                        break;
                    }
                    tx_payload_arr.push(TxAux::deserialize(raw)?);
                }
                Ok(tx_payload_arr)
            })()
            .map_err(|e| e.annotate("tx_payload"))?;
            let ssc_payload =
                Ssc::deserialize(raw).map_err(|e: DeserializeError| e.annotate("ssc_payload"))?;
            let dlg_payload = (|| -> Result<_, DeserializeError> {
                let mut dlg_payload_arr = Vec::new();
                let len = raw.array()?;
                while match len {
                    cbor_event::Len::Len(n) => (dlg_payload_arr.len() as u64) < n,
                    cbor_event::Len::Indefinite => true,
                } {
                    if raw.cbor_type()? == cbor_event::Type::Special {
                        assert_eq!(raw.special()?, cbor_event::Special::Break);
                        break;
                    }
                    dlg_payload_arr.push(ByronDelegation::deserialize(raw)?);
                }
                Ok(dlg_payload_arr)
            })()
            .map_err(|e| e.annotate("dlg_payload"))?;
            let upd_payload = ByronUpdate::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("upd_payload"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(ByronBlockBody {
                tx_payload,
                ssc_payload,
                dlg_payload,
                upd_payload,
            })
        })()
        .map_err(|e| e.annotate("ByronBlockBody"))
    }
}

impl cbor_event::se::Serialize for ByronBlockConsensusData {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(4))?;
        self.byron_slot_id.serialize(serializer)?;
        serializer.write_bytes(&self.byron_pub_key)?;
        self.byron_difficulty.serialize(serializer)?;
        self.byron_block_signature.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for ByronBlockConsensusData {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(4)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let byron_slot_id = ByronSlotId::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("byron_slot_id"))?;
            let byron_pub_key = Ok(raw.bytes()? as Vec<u8>)
                .map_err(|e: DeserializeError| e.annotate("byron_pub_key"))?;
            let byron_difficulty = ByronDifficulty::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("byron_difficulty"))?;
            let byron_block_signature = ByronBlockSignature::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("byron_block_signature"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(ByronBlockConsensusData {
                byron_slot_id,
                byron_pub_key,
                byron_difficulty,
                byron_block_signature,
            })
        })()
        .map_err(|e| e.annotate("ByronBlockConsensusData"))
    }
}

impl cbor_event::se::Serialize for ByronBlockHeader {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(5))?;
        serializer.write_unsigned_integer(self.protocol_magic as u64)?;
        serializer.write_bytes(&self.prev_block.to_raw_bytes())?;
        self.body_proof.serialize(serializer)?;
        self.consensus_data.serialize(serializer)?;
        self.extra_data.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for ByronBlockHeader {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(5)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let protocol_magic = Ok(raw.unsigned_integer()? as u32)
                .map_err(|e: DeserializeError| e.annotate("protocol_magic"))?;
            let prev_block = raw
                .bytes()
                .map_err(Into::<DeserializeError>::into)
                .and_then(|bytes| {
                    Blake2b256::from_raw_bytes(&bytes)
                        .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
                })
                .map_err(|e: DeserializeError| e.annotate("prev_block"))?;
            let body_proof = ByronBodyProof::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("body_proof"))?;
            let consensus_data = ByronBlockConsensusData::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("consensus_data"))?;
            let extra_data = BlockHeaderExtraData::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("extra_data"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(ByronBlockHeader {
                protocol_magic,
                prev_block,
                body_proof,
                consensus_data,
                extra_data,
            })
        })()
        .map_err(|e| e.annotate("ByronBlockHeader"))
    }
}

impl cbor_event::se::Serialize for ByronBlockSignature {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            ByronBlockSignature::Signature(signature) => signature.serialize(serializer),
            ByronBlockSignature::ProxyLight(proxy_light) => proxy_light.serialize(serializer),
            ByronBlockSignature::ProxyHeavy(proxy_heavy) => proxy_heavy.serialize(serializer),
        }
    }
}

impl Deserialize for ByronBlockSignature {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let initial_position = raw.as_mut_ref().seek(SeekFrom::Current(0)).unwrap();
            let mut errs = Vec::new();
            let deser_variant: Result<_, DeserializeError> =
                ByronBlockSignatureNormal::deserialize(raw);
            match deser_variant {
                Ok(signature) => return Ok(Self::Signature(signature)),
                Err(e) => {
                    errs.push(e.annotate("Signature"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant: Result<_, DeserializeError> =
                ByronBlockSignatureProxyLight::deserialize(raw);
            match deser_variant {
                Ok(proxy_light) => return Ok(Self::ProxyLight(proxy_light)),
                Err(e) => {
                    errs.push(e.annotate("ProxyLight"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant: Result<_, DeserializeError> =
                ByronBlockSignatureProxyHeavy::deserialize(raw);
            match deser_variant {
                Ok(proxy_heavy) => return Ok(Self::ProxyHeavy(proxy_heavy)),
                Err(e) => {
                    errs.push(e.annotate("ProxyHeavy"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            Err(DeserializeError::new(
                "ByronBlockSignature",
                DeserializeFailure::NoVariantMatchedWithCauses(errs),
            ))
        })()
        .map_err(|e| e.annotate("ByronBlockSignature"))
    }
}

impl cbor_event::se::Serialize for ByronBlockSignatureNormal {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        serializer.write_unsigned_integer(0u64)?;
        serializer.write_bytes(&self.signature)?;
        Ok(serializer)
    }
}

impl Deserialize for ByronBlockSignatureNormal {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
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
            let signature = Ok(raw.bytes()? as Vec<u8>)
                .map_err(|e: DeserializeError| e.annotate("signature"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(ByronBlockSignatureNormal { signature })
        })()
        .map_err(|e| e.annotate("ByronBlockSignatureNormal"))
    }
}

impl cbor_event::se::Serialize for ByronBlockSignatureProxyHeavy {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        serializer.write_unsigned_integer(2u64)?;
        self.signature.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for ByronBlockSignatureProxyHeavy {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
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
            let signature = ByronDelegationSignature::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("signature"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(ByronBlockSignatureProxyHeavy { signature })
        })()
        .map_err(|e| e.annotate("ByronBlockSignatureProxyHeavy"))
    }
}

impl cbor_event::se::Serialize for ByronBlockSignatureProxyLight {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        serializer.write_unsigned_integer(1u64)?;
        self.signature.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for ByronBlockSignatureProxyLight {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
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
            let signature = LightWeightDelegationSignature::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("signature"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(ByronBlockSignatureProxyLight { signature })
        })()
        .map_err(|e| e.annotate("ByronBlockSignatureProxyLight"))
    }
}

impl cbor_event::se::Serialize for ByronBodyProof {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(4))?;
        self.tx_proof.serialize(serializer)?;
        self.ssc_proof.serialize(serializer)?;
        serializer.write_bytes(&self.dlg_proof.to_raw_bytes())?;
        serializer.write_bytes(&self.upd_proof.to_raw_bytes())?;
        Ok(serializer)
    }
}

impl Deserialize for ByronBodyProof {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(4)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let tx_proof = ByronTxProof::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("tx_proof"))?;
            let ssc_proof = SscProof::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("ssc_proof"))?;
            let dlg_proof = raw
                .bytes()
                .map_err(Into::<DeserializeError>::into)
                .and_then(|bytes| {
                    Blake2b256::from_raw_bytes(&bytes)
                        .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
                })
                .map_err(|e: DeserializeError| e.annotate("dlg_proof"))?;
            let upd_proof = raw
                .bytes()
                .map_err(Into::<DeserializeError>::into)
                .and_then(|bytes| {
                    Blake2b256::from_raw_bytes(&bytes)
                        .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
                })
                .map_err(|e: DeserializeError| e.annotate("upd_proof"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(ByronBodyProof {
                tx_proof,
                ssc_proof,
                dlg_proof,
                upd_proof,
            })
        })()
        .map_err(|e| e.annotate("ByronBodyProof"))
    }
}

impl cbor_event::se::Serialize for ByronDifficulty {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(1))?;
        serializer.write_unsigned_integer(self.u64)?;
        Ok(serializer)
    }
}

impl Deserialize for ByronDifficulty {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(1)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let u64 = Ok(raw.unsigned_integer()? as u64)
                .map_err(|e: DeserializeError| e.annotate("u64"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(ByronDifficulty { u64 })
        })()
        .map_err(|e| e.annotate("ByronDifficulty"))
    }
}

impl cbor_event::se::Serialize for ByronEbBlock {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(3))?;
        self.header.serialize(serializer)?;
        serializer.write_array(cbor_event::Len::Len(self.body.len() as u64))?;
        for element in self.body.iter() {
            serializer.write_bytes(&element.to_raw_bytes())?;
        }
        serializer.write_array(cbor_event::Len::Len(self.extra.len() as u64))?;
        for element in self.extra.iter() {
            serializer.write_map(cbor_event::Len::Len(element.len() as u64))?;
            for (key, value) in element.iter() {
                key.serialize(serializer)?;
                value.serialize(serializer)?;
            }
        }
        Ok(serializer)
    }
}

impl Deserialize for ByronEbBlock {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(3)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let header =
                EbbHead::deserialize(raw).map_err(|e: DeserializeError| e.annotate("header"))?;
            let body = (|| -> Result<_, DeserializeError> {
                let mut body_arr = Vec::new();
                let len = raw.array()?;
                while match len {
                    cbor_event::Len::Len(n) => (body_arr.len() as u64) < n,
                    cbor_event::Len::Indefinite => true,
                } {
                    if raw.cbor_type()? == cbor_event::Type::Special {
                        assert_eq!(raw.special()?, cbor_event::Special::Break);
                        break;
                    }
                    body_arr.push(
                        raw.bytes()
                            .map_err(Into::<DeserializeError>::into)
                            .and_then(|bytes| {
                                StakeholderId::from_raw_bytes(&bytes).map_err(|e| {
                                    DeserializeFailure::InvalidStructure(Box::new(e)).into()
                                })
                            })?,
                    );
                }
                Ok(body_arr)
            })()
            .map_err(|e| e.annotate("body"))?;
            let extra = (|| -> Result<_, DeserializeError> {
                let mut extra_arr = Vec::new();
                let len = raw.array()?;
                while match len {
                    cbor_event::Len::Len(n) => (extra_arr.len() as u64) < n,
                    cbor_event::Len::Indefinite => true,
                } {
                    if raw.cbor_type()? == cbor_event::Type::Special {
                        assert_eq!(raw.special()?, cbor_event::Special::Break);
                        break;
                    }
                    let mut extra_elem_table = BTreeMap::new();
                    let extra_elem_len = raw.map()?;
                    while match extra_elem_len {
                        cbor_event::Len::Len(n) => (extra_elem_table.len() as u64) < n,
                        cbor_event::Len::Indefinite => true,
                    } {
                        if raw.cbor_type()? == cbor_event::Type::Special {
                            assert_eq!(raw.special()?, cbor_event::Special::Break);
                            break;
                        }
                        let extra_elem_key = ByronAny::deserialize(raw)?;
                        let extra_elem_value = ByronAny::deserialize(raw)?;
                        if extra_elem_table
                            .insert(extra_elem_key.clone(), extra_elem_value)
                            .is_some()
                        {
                            return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from(
                                "some complicated/unsupported type",
                            )))
                            .into());
                        }
                    }
                    extra_arr.push(extra_elem_table);
                }
                Ok(extra_arr)
            })()
            .map_err(|e| e.annotate("extra"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(ByronEbBlock {
                header,
                body,
                extra,
            })
        })()
        .map_err(|e| e.annotate("ByronEbBlock"))
    }
}

impl cbor_event::se::Serialize for ByronMainBlock {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(3))?;
        self.header.serialize(serializer)?;
        self.body.serialize(serializer)?;
        serializer.write_array(cbor_event::Len::Len(self.extra.len() as u64))?;
        for element in self.extra.iter() {
            serializer.write_map(cbor_event::Len::Len(element.len() as u64))?;
            for (key, value) in element.iter() {
                key.serialize(serializer)?;
                value.serialize(serializer)?;
            }
        }
        Ok(serializer)
    }
}

impl Deserialize for ByronMainBlock {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(3)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let header = ByronBlockHeader::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("header"))?;
            let body = ByronBlockBody::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("body"))?;
            let extra = (|| -> Result<_, DeserializeError> {
                let mut extra_arr = Vec::new();
                let len = raw.array()?;
                while match len {
                    cbor_event::Len::Len(n) => (extra_arr.len() as u64) < n,
                    cbor_event::Len::Indefinite => true,
                } {
                    if raw.cbor_type()? == cbor_event::Type::Special {
                        assert_eq!(raw.special()?, cbor_event::Special::Break);
                        break;
                    }
                    let mut extra_elem_table = BTreeMap::new();
                    let extra_elem_len = raw.map()?;
                    while match extra_elem_len {
                        cbor_event::Len::Len(n) => (extra_elem_table.len() as u64) < n,
                        cbor_event::Len::Indefinite => true,
                    } {
                        if raw.cbor_type()? == cbor_event::Type::Special {
                            assert_eq!(raw.special()?, cbor_event::Special::Break);
                            break;
                        }
                        let extra_elem_key = ByronAny::deserialize(raw)?;
                        let extra_elem_value = ByronAny::deserialize(raw)?;
                        if extra_elem_table
                            .insert(extra_elem_key.clone(), extra_elem_value)
                            .is_some()
                        {
                            return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from(
                                "some complicated/unsupported type",
                            )))
                            .into());
                        }
                    }
                    extra_arr.push(extra_elem_table);
                }
                Ok(extra_arr)
            })()
            .map_err(|e| e.annotate("extra"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(ByronMainBlock {
                header,
                body,
                extra,
            })
        })()
        .map_err(|e| e.annotate("ByronMainBlock"))
    }
}

impl cbor_event::se::Serialize for EbbConsensusData {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        serializer.write_unsigned_integer(self.epoch_id)?;
        self.byron_difficulty.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for EbbConsensusData {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let epoch_id = Ok(raw.unsigned_integer()? as u64)
                .map_err(|e: DeserializeError| e.annotate("epoch_id"))?;
            let byron_difficulty = ByronDifficulty::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("byron_difficulty"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(EbbConsensusData {
                epoch_id,
                byron_difficulty,
            })
        })()
        .map_err(|e| e.annotate("EbbConsensusData"))
    }
}

impl cbor_event::se::Serialize for EbbHead {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(5))?;
        serializer.write_unsigned_integer(self.protocol_magic as u64)?;
        serializer.write_bytes(&self.prev_block.to_raw_bytes())?;
        serializer.write_bytes(&self.body_proof.to_raw_bytes())?;
        self.consensus_data.serialize(serializer)?;
        serializer.write_array(cbor_event::Len::Len(self.extra_data.len() as u64))?;
        for element in self.extra_data.iter() {
            serializer.write_map(cbor_event::Len::Len(element.len() as u64))?;
            for (key, value) in element.iter() {
                key.serialize(serializer)?;
                value.serialize(serializer)?;
            }
        }
        Ok(serializer)
    }
}

impl Deserialize for EbbHead {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(5)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let protocol_magic = Ok(raw.unsigned_integer()? as u32)
                .map_err(|e: DeserializeError| e.annotate("protocol_magic"))?;
            let prev_block = raw
                .bytes()
                .map_err(Into::<DeserializeError>::into)
                .and_then(|bytes| {
                    Blake2b256::from_raw_bytes(&bytes)
                        .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
                })
                .map_err(|e: DeserializeError| e.annotate("prev_block"))?;
            let body_proof = raw
                .bytes()
                .map_err(Into::<DeserializeError>::into)
                .and_then(|bytes| {
                    Blake2b256::from_raw_bytes(&bytes)
                        .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
                })
                .map_err(|e: DeserializeError| e.annotate("body_proof"))?;
            let consensus_data = EbbConsensusData::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("consensus_data"))?;
            let extra_data = (|| -> Result<_, DeserializeError> {
                let mut extra_data_arr = Vec::new();
                let len = raw.array()?;
                while match len {
                    cbor_event::Len::Len(n) => (extra_data_arr.len() as u64) < n,
                    cbor_event::Len::Indefinite => true,
                } {
                    if raw.cbor_type()? == cbor_event::Type::Special {
                        assert_eq!(raw.special()?, cbor_event::Special::Break);
                        break;
                    }
                    let mut extra_data_elem_table = BTreeMap::new();
                    let extra_data_elem_len = raw.map()?;
                    while match extra_data_elem_len {
                        cbor_event::Len::Len(n) => (extra_data_elem_table.len() as u64) < n,
                        cbor_event::Len::Indefinite => true,
                    } {
                        if raw.cbor_type()? == cbor_event::Type::Special {
                            assert_eq!(raw.special()?, cbor_event::Special::Break);
                            break;
                        }
                        let extra_data_elem_key = ByronAny::deserialize(raw)?;
                        let extra_data_elem_value = ByronAny::deserialize(raw)?;
                        if extra_data_elem_table
                            .insert(extra_data_elem_key.clone(), extra_data_elem_value)
                            .is_some()
                        {
                            return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from(
                                "some complicated/unsupported type",
                            )))
                            .into());
                        }
                    }
                    extra_data_arr.push(extra_data_elem_table);
                }
                Ok(extra_data_arr)
            })()
            .map_err(|e| e.annotate("extra_data"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(EbbHead {
                protocol_magic,
                prev_block,
                body_proof,
                consensus_data,
                extra_data,
            })
        })()
        .map_err(|e| e.annotate("EbbHead"))
    }
}

impl cbor_event::se::Serialize for TxAux {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        self.byron_tx.serialize(serializer)?;
        serializer.write_array(cbor_event::Len::Len(self.byron_tx_witnesss.len() as u64))?;
        for element in self.byron_tx_witnesss.iter() {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for TxAux {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let byron_tx =
                ByronTx::deserialize(raw).map_err(|e: DeserializeError| e.annotate("byron_tx"))?;
            let byron_tx_witnesss = (|| -> Result<_, DeserializeError> {
                let mut byron_tx_witnesss_arr = Vec::new();
                let len = raw.array()?;
                while match len {
                    cbor_event::Len::Len(n) => (byron_tx_witnesss_arr.len() as u64) < n,
                    cbor_event::Len::Indefinite => true,
                } {
                    if raw.cbor_type()? == cbor_event::Type::Special {
                        assert_eq!(raw.special()?, cbor_event::Special::Break);
                        break;
                    }
                    byron_tx_witnesss_arr.push(ByronTxWitness::deserialize(raw)?);
                }
                Ok(byron_tx_witnesss_arr)
            })()
            .map_err(|e| e.annotate("byron_tx_witnesss"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(TxAux {
                byron_tx,
                byron_tx_witnesss,
            })
        })()
        .map_err(|e| e.annotate("TxAux"))
    }
}
