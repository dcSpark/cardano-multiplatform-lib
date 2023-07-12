// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use super::*;
use cbor_event;
use cbor_event::de::Deserializer;
use cbor_event::se::{Serialize, Serializer};
use cml_core::error::*;
use cml_core::serialization::*;
use std::io::{BufRead, Seek, SeekFrom, Write};

impl cbor_event::se::Serialize for ByronPkWitness {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        self.serialize_as_embedded_group(serializer)
    }
}

impl SerializeEmbeddedGroup for ByronPkWitness {
    fn serialize_as_embedded_group<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(0u64)?;
        serializer.write_tag(24u64)?;
        let mut index_1_inner_se = Serializer::new_vec();
        self.index_1.serialize(&mut index_1_inner_se)?;
        let index_1_bytes = index_1_inner_se.finalize();
        serializer.write_bytes(&index_1_bytes)?;
        Ok(serializer)
    }
}

impl Deserialize for ByronPkWitness {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        let ret = Self::deserialize_as_embedded_group(raw, &mut read_len, len);
        match len {
            cbor_event::Len::Len(_) => (),
            cbor_event::Len::Indefinite => match raw.special()? {
                cbor_event::Special::Break => (),
                _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
            },
        }
        ret
    }
}

impl DeserializeEmbeddedGroup for ByronPkWitness {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        _read_len: &mut CBORReadLen,
        len: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            (|| -> Result<_, DeserializeError> {
                let index_0_value = raw.unsigned_integer()?;
                if index_0_value != 0 {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(index_0_value),
                        expected: Key::Uint(0),
                    }
                    .into());
                }
                Ok(())
            })()
            .map_err(|e| e.annotate("index_0"))?;
            let index_1 = (|| -> Result<_, DeserializeError> {
                match raw.tag()? {
                    24 => {
                        let index_1_bytes = raw.bytes()?;
                        let inner_de = &mut Deserializer::from(std::io::Cursor::new(index_1_bytes));
                        ByronPkWitnessEntry::deserialize(inner_de)
                    }
                    tag => Err(DeserializeFailure::TagMismatch {
                        found: tag,
                        expected: 24,
                    }
                    .into()),
                }
            })()
            .map_err(|e| e.annotate("index_1"))?;
            Ok(ByronPkWitness { index_1 })
        })()
        .map_err(|e| e.annotate("ByronPkWitness"))
    }
}

impl cbor_event::se::Serialize for ByronPkWitnessEntry {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        serializer.write_bytes(&self.byron_pub_key)?;
        serializer.write_bytes(&self.byron_signature)?;
        Ok(serializer)
    }
}

impl Deserialize for ByronPkWitnessEntry {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let byron_pub_key = Ok(raw.bytes()? as Vec<u8>)
                .map_err(|e: DeserializeError| e.annotate("byron_pub_key"))?;
            let byron_signature = Ok(raw.bytes()? as Vec<u8>)
                .map_err(|e: DeserializeError| e.annotate("byron_signature"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(ByronPkWitnessEntry {
                byron_pub_key,
                byron_signature,
            })
        })()
        .map_err(|e| e.annotate("ByronPkWitnessEntry"))
    }
}

impl cbor_event::se::Serialize for ByronRedeemWitness {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        self.serialize_as_embedded_group(serializer)
    }
}

impl SerializeEmbeddedGroup for ByronRedeemWitness {
    fn serialize_as_embedded_group<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(2u64)?;
        serializer.write_tag(24u64)?;
        let mut index_1_inner_se = Serializer::new_vec();
        self.index_1.serialize(&mut index_1_inner_se)?;
        let index_1_bytes = index_1_inner_se.finalize();
        serializer.write_bytes(&index_1_bytes)?;
        Ok(serializer)
    }
}

impl Deserialize for ByronRedeemWitness {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        let ret = Self::deserialize_as_embedded_group(raw, &mut read_len, len);
        match len {
            cbor_event::Len::Len(_) => (),
            cbor_event::Len::Indefinite => match raw.special()? {
                cbor_event::Special::Break => (),
                _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
            },
        }
        ret
    }
}

impl DeserializeEmbeddedGroup for ByronRedeemWitness {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        _read_len: &mut CBORReadLen,
        len: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            (|| -> Result<_, DeserializeError> {
                let index_0_value = raw.unsigned_integer()?;
                if index_0_value != 2 {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(index_0_value),
                        expected: Key::Uint(2),
                    }
                    .into());
                }
                Ok(())
            })()
            .map_err(|e| e.annotate("index_0"))?;
            let index_1 = (|| -> Result<_, DeserializeError> {
                match raw.tag()? {
                    24 => {
                        let index_1_bytes = raw.bytes()?;
                        let inner_de = &mut Deserializer::from(std::io::Cursor::new(index_1_bytes));
                        ByronRedeemerWitnessEntry::deserialize(inner_de)
                    }
                    tag => Err(DeserializeFailure::TagMismatch {
                        found: tag,
                        expected: 24,
                    }
                    .into()),
                }
            })()
            .map_err(|e| e.annotate("index_1"))?;
            Ok(ByronRedeemWitness { index_1 })
        })()
        .map_err(|e| e.annotate("ByronRedeemWitness"))
    }
}

impl cbor_event::se::Serialize for ByronRedeemerScript {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        serializer.write_unsigned_integer(self.u16 as u64)?;
        serializer.write_bytes(&self.index_1)?;
        Ok(serializer)
    }
}

impl Deserialize for ByronRedeemerScript {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let u16 = Ok(raw.unsigned_integer()? as u16)
                .map_err(|e: DeserializeError| e.annotate("u16"))?;
            let index_1 =
                Ok(raw.bytes()? as Vec<u8>).map_err(|e: DeserializeError| e.annotate("index_1"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(ByronRedeemerScript { u16, index_1 })
        })()
        .map_err(|e| e.annotate("ByronRedeemerScript"))
    }
}

impl cbor_event::se::Serialize for ByronRedeemerWitnessEntry {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        serializer.write_bytes(&self.byron_pub_key)?;
        serializer.write_bytes(&self.byron_signature)?;
        Ok(serializer)
    }
}

impl Deserialize for ByronRedeemerWitnessEntry {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let byron_pub_key = Ok(raw.bytes()? as Vec<u8>)
                .map_err(|e: DeserializeError| e.annotate("byron_pub_key"))?;
            let byron_signature = Ok(raw.bytes()? as Vec<u8>)
                .map_err(|e: DeserializeError| e.annotate("byron_signature"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(ByronRedeemerWitnessEntry {
                byron_pub_key,
                byron_signature,
            })
        })()
        .map_err(|e| e.annotate("ByronRedeemerWitnessEntry"))
    }
}

impl cbor_event::se::Serialize for ByronScriptWitness {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        self.serialize_as_embedded_group(serializer)
    }
}

impl SerializeEmbeddedGroup for ByronScriptWitness {
    fn serialize_as_embedded_group<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(1u64)?;
        serializer.write_tag(24u64)?;
        let mut index_1_inner_se = Serializer::new_vec();
        self.index_1.serialize(&mut index_1_inner_se)?;
        let index_1_bytes = index_1_inner_se.finalize();
        serializer.write_bytes(&index_1_bytes)?;
        Ok(serializer)
    }
}

impl Deserialize for ByronScriptWitness {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        let ret = Self::deserialize_as_embedded_group(raw, &mut read_len, len);
        match len {
            cbor_event::Len::Len(_) => (),
            cbor_event::Len::Indefinite => match raw.special()? {
                cbor_event::Special::Break => (),
                _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
            },
        }
        ret
    }
}

impl DeserializeEmbeddedGroup for ByronScriptWitness {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        _read_len: &mut CBORReadLen,
        len: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            (|| -> Result<_, DeserializeError> {
                let index_0_value = raw.unsigned_integer()?;
                if index_0_value != 1 {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(index_0_value),
                        expected: Key::Uint(1),
                    }
                    .into());
                }
                Ok(())
            })()
            .map_err(|e| e.annotate("index_0"))?;
            let index_1 = (|| -> Result<_, DeserializeError> {
                match raw.tag()? {
                    24 => {
                        let index_1_bytes = raw.bytes()?;
                        let inner_de = &mut Deserializer::from(std::io::Cursor::new(index_1_bytes));
                        ByronScriptWitnessEntry::deserialize(inner_de)
                    }
                    tag => Err(DeserializeFailure::TagMismatch {
                        found: tag,
                        expected: 24,
                    }
                    .into()),
                }
            })()
            .map_err(|e| e.annotate("index_1"))?;
            Ok(ByronScriptWitness { index_1 })
        })()
        .map_err(|e| e.annotate("ByronScriptWitness"))
    }
}

impl cbor_event::se::Serialize for ByronScriptWitnessEntry {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        self.byron_validator_script.serialize(serializer)?;
        self.byron_redeemer_script.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for ByronScriptWitnessEntry {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let byron_validator_script = ByronValidatorScript::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("byron_validator_script"))?;
            let byron_redeemer_script = ByronRedeemerScript::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("byron_redeemer_script"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(ByronScriptWitnessEntry {
                byron_validator_script,
                byron_redeemer_script,
            })
        })()
        .map_err(|e| e.annotate("ByronScriptWitnessEntry"))
    }
}

impl cbor_event::se::Serialize for ByronTx {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(3))?;
        serializer.write_array(cbor_event::Len::Len(self.inputs.len() as u64))?;
        for element in self.inputs.iter() {
            element.serialize(serializer)?;
        }
        serializer.write_array(cbor_event::Len::Len(self.outputs.len() as u64))?;
        for element in self.outputs.iter() {
            element.serialize(serializer)?;
        }
        serializer.write_map(cbor_event::Len::Len(self.attrs.len() as u64))?;
        for (key, value) in self.attrs.iter() {
            key.serialize(serializer)?;
            value.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for ByronTx {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(3)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let inputs = (|| -> Result<_, DeserializeError> {
                let mut inputs_arr = Vec::new();
                let len = raw.array()?;
                while match len {
                    cbor_event::Len::Len(n) => (inputs_arr.len() as u64) < n,
                    cbor_event::Len::Indefinite => true,
                } {
                    if raw.cbor_type()? == cbor_event::Type::Special {
                        assert_eq!(raw.special()?, cbor_event::Special::Break);
                        break;
                    }
                    inputs_arr.push(ByronTxIn::deserialize(raw)?);
                }
                Ok(inputs_arr)
            })()
            .map_err(|e| e.annotate("inputs"))?;
            let outputs = (|| -> Result<_, DeserializeError> {
                let mut outputs_arr = Vec::new();
                let len = raw.array()?;
                while match len {
                    cbor_event::Len::Len(n) => (outputs_arr.len() as u64) < n,
                    cbor_event::Len::Indefinite => true,
                } {
                    if raw.cbor_type()? == cbor_event::Type::Special {
                        assert_eq!(raw.special()?, cbor_event::Special::Break);
                        break;
                    }
                    outputs_arr.push(ByronTxOut::deserialize(raw)?);
                }
                Ok(outputs_arr)
            })()
            .map_err(|e| e.annotate("outputs"))?;
            let attrs = (|| -> Result<_, DeserializeError> {
                let mut attrs_table = BTreeMap::new();
                let attrs_len = raw.map()?;
                while match attrs_len {
                    cbor_event::Len::Len(n) => (attrs_table.len() as u64) < n,
                    cbor_event::Len::Indefinite => true,
                } {
                    if raw.cbor_type()? == cbor_event::Type::Special {
                        assert_eq!(raw.special()?, cbor_event::Special::Break);
                        break;
                    }
                    let attrs_key = ByronAny::deserialize(raw)?;
                    let attrs_value = ByronAny::deserialize(raw)?;
                    if attrs_table.insert(attrs_key.clone(), attrs_value).is_some() {
                        return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from(
                            "some complicated/unsupported type",
                        )))
                        .into());
                    }
                }
                Ok(attrs_table)
            })()
            .map_err(|e| e.annotate("attrs"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(ByronTx {
                inputs,
                outputs,
                attrs,
            })
        })()
        .map_err(|e| e.annotate("ByronTx"))
    }
}

impl cbor_event::se::Serialize for ByronTxIn {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            ByronTxIn::ByronTxInRegular(byron_tx_in_regular) => {
                byron_tx_in_regular.serialize(serializer)
            }
            ByronTxIn::ByronTxInGenesis(byron_tx_in_genesis) => {
                byron_tx_in_genesis.serialize(serializer)
            }
        }
    }
}

impl Deserialize for ByronTxIn {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let initial_position = raw.as_mut_ref().seek(SeekFrom::Current(0)).unwrap();
            let mut errs = Vec::new();
            let deser_variant: Result<_, DeserializeError> = ByronTxInRegular::deserialize(raw);
            match deser_variant {
                Ok(byron_tx_in_regular) => return Ok(Self::ByronTxInRegular(byron_tx_in_regular)),
                Err(e) => {
                    errs.push(e.annotate("ByronTxInRegular"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant: Result<_, DeserializeError> = ByronTxInGenesis::deserialize(raw);
            match deser_variant {
                Ok(byron_tx_in_genesis) => return Ok(Self::ByronTxInGenesis(byron_tx_in_genesis)),
                Err(e) => {
                    errs.push(e.annotate("ByronTxInGenesis"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            Err(DeserializeError::new(
                "ByronTxIn",
                DeserializeFailure::NoVariantMatchedWithCauses(errs),
            ))
        })()
        .map_err(|e| e.annotate("ByronTxIn"))
    }
}

impl cbor_event::se::Serialize for ByronTxInGenesis {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        serializer.write_unsigned_integer(self.u8 as u64)?;
        serializer.write_tag(24u64)?;
        serializer.write_bytes(&self.index_1)?;
        Ok(serializer)
    }
}

impl Deserialize for ByronTxInGenesis {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let u8 = Ok(raw.unsigned_integer()? as u8)
                .map_err(|e: DeserializeError| e.annotate("u8"))?;
            let index_1 = (|| -> Result<_, DeserializeError> {
                match raw.tag()? {
                    24 => Ok(raw.bytes()? as Vec<u8>),
                    tag => Err(DeserializeFailure::TagMismatch {
                        found: tag,
                        expected: 24,
                    }
                    .into()),
                }
            })()
            .map_err(|e| e.annotate("index_1"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(ByronTxInGenesis { u8, index_1 })
        })()
        .map_err(|e| e.annotate("ByronTxInGenesis"))
    }
}

impl cbor_event::se::Serialize for ByronTxInRegular {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        serializer.write_unsigned_integer(0u64)?;
        serializer.write_tag(24u64)?;
        let mut index_1_inner_se = Serializer::new_vec();
        self.index_1.serialize(&mut index_1_inner_se)?;
        let index_1_bytes = index_1_inner_se.finalize();
        serializer.write_bytes(&index_1_bytes)?;
        Ok(serializer)
    }
}

impl Deserialize for ByronTxInRegular {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            (|| -> Result<_, DeserializeError> {
                let index_0_value = raw.unsigned_integer()?;
                if index_0_value != 0 {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(index_0_value),
                        expected: Key::Uint(0),
                    }
                    .into());
                }
                Ok(())
            })()
            .map_err(|e| e.annotate("index_0"))?;
            let index_1 = (|| -> Result<_, DeserializeError> {
                match raw.tag()? {
                    24 => {
                        let index_1_bytes = raw.bytes()?;
                        let inner_de = &mut Deserializer::from(std::io::Cursor::new(index_1_bytes));
                        ByronTxOutPtr::deserialize(inner_de)
                    }
                    tag => Err(DeserializeFailure::TagMismatch {
                        found: tag,
                        expected: 24,
                    }
                    .into()),
                }
            })()
            .map_err(|e| e.annotate("index_1"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(ByronTxInRegular { index_1 })
        })()
        .map_err(|e| e.annotate("ByronTxInRegular"))
    }
}

impl cbor_event::se::Serialize for ByronTxOutPtr {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        serializer.write_bytes(&self.byron_tx_id.to_raw_bytes())?;
        serializer.write_unsigned_integer(self.u32 as u64)?;
        Ok(serializer)
    }
}

impl Deserialize for ByronTxOutPtr {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let byron_tx_id = raw
                .bytes()
                .map_err(Into::<DeserializeError>::into)
                .and_then(|bytes| {
                    Blake2b256::from_raw_bytes(&bytes)
                        .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
                })
                .map_err(|e: DeserializeError| e.annotate("byron_tx_id"))?;
            let u32 = Ok(raw.unsigned_integer()? as u32)
                .map_err(|e: DeserializeError| e.annotate("u32"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(ByronTxOutPtr { byron_tx_id, u32 })
        })()
        .map_err(|e| e.annotate("ByronTxOutPtr"))
    }
}

impl cbor_event::se::Serialize for ByronTxProof {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(3))?;
        serializer.write_unsigned_integer(self.u32 as u64)?;
        serializer.write_bytes(&self.blake2b256.to_raw_bytes())?;
        serializer.write_bytes(&self.blake2b2562.to_raw_bytes())?;
        Ok(serializer)
    }
}

impl Deserialize for ByronTxProof {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(3)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let u32 = Ok(raw.unsigned_integer()? as u32)
                .map_err(|e: DeserializeError| e.annotate("u32"))?;
            let blake2b256 = raw
                .bytes()
                .map_err(Into::<DeserializeError>::into)
                .and_then(|bytes| {
                    Blake2b256::from_raw_bytes(&bytes)
                        .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
                })
                .map_err(|e: DeserializeError| e.annotate("blake2b256"))?;
            let blake2b2562 = raw
                .bytes()
                .map_err(Into::<DeserializeError>::into)
                .and_then(|bytes| {
                    Blake2b256::from_raw_bytes(&bytes)
                        .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
                })
                .map_err(|e: DeserializeError| e.annotate("blake2b2562"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(ByronTxProof {
                u32,
                blake2b256,
                blake2b2562,
            })
        })()
        .map_err(|e| e.annotate("ByronTxProof"))
    }
}

impl cbor_event::se::Serialize for ByronTxWitness {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            ByronTxWitness::ByronPkWitness(byron_pk_witness) => {
                byron_pk_witness.serialize(serializer)
            }
            ByronTxWitness::ByronScriptWitness(byron_script_witness) => {
                byron_script_witness.serialize(serializer)
            }
            ByronTxWitness::ByronRedeemWitness(byron_redeem_witness) => {
                byron_redeem_witness.serialize(serializer)
            }
        }
    }
}

impl Deserialize for ByronTxWitness {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let mut read_len = CBORReadLen::new(len);
            let initial_position = raw.as_mut_ref().seek(SeekFrom::Current(0)).unwrap();
            let mut errs = Vec::new();
            let deser_variant: Result<_, DeserializeError> =
                ByronPkWitness::deserialize_as_embedded_group(raw, &mut read_len, len);
            match deser_variant {
                Ok(byron_pk_witness) => return Ok(Self::ByronPkWitness(byron_pk_witness)),
                Err(e) => {
                    errs.push(e.annotate("ByronPkWitness"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant: Result<_, DeserializeError> =
                ByronScriptWitness::deserialize_as_embedded_group(raw, &mut read_len, len);
            match deser_variant {
                Ok(byron_script_witness) => {
                    return Ok(Self::ByronScriptWitness(byron_script_witness))
                }
                Err(e) => {
                    errs.push(e.annotate("ByronScriptWitness"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant: Result<_, DeserializeError> =
                ByronRedeemWitness::deserialize_as_embedded_group(raw, &mut read_len, len);
            match deser_variant {
                Ok(byron_redeem_witness) => {
                    return Ok(Self::ByronRedeemWitness(byron_redeem_witness))
                }
                Err(e) => {
                    errs.push(e.annotate("ByronRedeemWitness"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Err(DeserializeError::new(
                "ByronTxWitness",
                DeserializeFailure::NoVariantMatchedWithCauses(errs),
            ))
        })()
        .map_err(|e| e.annotate("ByronTxWitness"))
    }
}

impl cbor_event::se::Serialize for ByronValidatorScript {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        serializer.write_unsigned_integer(self.u16 as u64)?;
        serializer.write_bytes(&self.index_1)?;
        Ok(serializer)
    }
}

impl Deserialize for ByronValidatorScript {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let u16 = Ok(raw.unsigned_integer()? as u16)
                .map_err(|e: DeserializeError| e.annotate("u16"))?;
            let index_1 =
                Ok(raw.bytes()? as Vec<u8>).map_err(|e: DeserializeError| e.annotate("index_1"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(ByronValidatorScript { u16, index_1 })
        })()
        .map_err(|e| e.annotate("ByronValidatorScript"))
    }
}
