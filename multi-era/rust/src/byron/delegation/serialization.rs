// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use super::*;
use cbor_event;
use cbor_event::de::Deserializer;
use cbor_event::se::{Serialize, Serializer};
use cml_core::error::*;
use cml_core::serialization::*;
use std::io::{BufRead, Seek, SeekFrom, Write};

impl cbor_event::se::Serialize for ByronDelegation {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(4))?;
        serializer.write_unsigned_integer(self.epoch)?;
        serializer.write_bytes(&self.issuer)?;
        serializer.write_bytes(&self.delegate)?;
        serializer.write_bytes(&self.certificate)?;
        Ok(serializer)
    }
}

impl Deserialize for ByronDelegation {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(4)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let epoch = Ok(raw.unsigned_integer()? as u64)
                .map_err(|e: DeserializeError| e.annotate("epoch"))?;
            let issuer =
                Ok(raw.bytes()? as Vec<u8>).map_err(|e: DeserializeError| e.annotate("issuer"))?;
            let delegate = Ok(raw.bytes()? as Vec<u8>)
                .map_err(|e: DeserializeError| e.annotate("delegate"))?;
            let certificate = Ok(raw.bytes()? as Vec<u8>)
                .map_err(|e: DeserializeError| e.annotate("certificate"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(ByronDelegation {
                epoch,
                issuer,
                delegate,
                certificate,
            })
        })()
        .map_err(|e| e.annotate("ByronDelegation"))
    }
}

impl cbor_event::se::Serialize for ByronDelegationSignature {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        self.byron_delegation.serialize(serializer)?;
        serializer.write_bytes(&self.byron_signature)?;
        Ok(serializer)
    }
}

impl Deserialize for ByronDelegationSignature {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let byron_delegation = ByronDelegation::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("byron_delegation"))?;
            let byron_signature = Ok(raw.bytes()? as Vec<u8>)
                .map_err(|e: DeserializeError| e.annotate("byron_signature"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(ByronDelegationSignature {
                byron_delegation,
                byron_signature,
            })
        })()
        .map_err(|e| e.annotate("ByronDelegationSignature"))
    }
}

impl cbor_event::se::Serialize for EpochRange {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        serializer.write_unsigned_integer(self.epoch_id)?;
        serializer.write_unsigned_integer(self.epoch_id2)?;
        Ok(serializer)
    }
}

impl Deserialize for EpochRange {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let epoch_id = Ok(raw.unsigned_integer()? as u64)
                .map_err(|e: DeserializeError| e.annotate("epoch_id"))?;
            let epoch_id2 = Ok(raw.unsigned_integer()? as u64)
                .map_err(|e: DeserializeError| e.annotate("epoch_id2"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(EpochRange {
                epoch_id,
                epoch_id2,
            })
        })()
        .map_err(|e| e.annotate("EpochRange"))
    }
}

impl cbor_event::se::Serialize for LightWeightDelegationSignature {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        self.light_weight_dlg.serialize(serializer)?;
        serializer.write_bytes(&self.byron_signature)?;
        Ok(serializer)
    }
}

impl Deserialize for LightWeightDelegationSignature {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let light_weight_dlg = LightWeightDlg::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("light_weight_dlg"))?;
            let byron_signature = Ok(raw.bytes()? as Vec<u8>)
                .map_err(|e: DeserializeError| e.annotate("byron_signature"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(LightWeightDelegationSignature {
                light_weight_dlg,
                byron_signature,
            })
        })()
        .map_err(|e| e.annotate("LightWeightDelegationSignature"))
    }
}

impl cbor_event::se::Serialize for LightWeightDlg {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(4))?;
        self.epoch_range.serialize(serializer)?;
        serializer.write_bytes(&self.issuer)?;
        serializer.write_bytes(&self.delegate)?;
        serializer.write_bytes(&self.certificate)?;
        Ok(serializer)
    }
}

impl Deserialize for LightWeightDlg {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(4)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let epoch_range = EpochRange::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("epoch_range"))?;
            let issuer =
                Ok(raw.bytes()? as Vec<u8>).map_err(|e: DeserializeError| e.annotate("issuer"))?;
            let delegate = Ok(raw.bytes()? as Vec<u8>)
                .map_err(|e: DeserializeError| e.annotate("delegate"))?;
            let certificate = Ok(raw.bytes()? as Vec<u8>)
                .map_err(|e: DeserializeError| e.annotate("certificate"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(LightWeightDlg {
                epoch_range,
                issuer,
                delegate,
                certificate,
            })
        })()
        .map_err(|e| e.annotate("LightWeightDlg"))
    }
}
