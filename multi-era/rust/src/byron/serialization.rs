// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use super::*;
use cbor_event;
use cbor_event::de::Deserializer;
use cbor_event::se::Serializer;
use cml_core::error::*;
use cml_core::serialization::*;

impl cbor_event::se::Serialize for ByronSlotId {
    fn serialize<'se>(
        &self,
        serializer: &'se mut Serializer,
    ) -> cbor_event::Result<&'se mut Serializer> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        serializer.write_unsigned_integer(self.epoch)?;
        serializer.write_unsigned_integer(self.slot)?;
        Ok(serializer)
    }
}

impl Deserialize for ByronSlotId {
    fn deserialize(raw: &mut Deserializer) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::from(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let epoch =
                Ok(raw.unsigned_integer()?).map_err(|e: DeserializeError| e.annotate("epoch"))?;
            let slot =
                Ok(raw.unsigned_integer()?).map_err(|e: DeserializeError| e.annotate("slot"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(ByronSlotId { epoch, slot })
        })()
        .map_err(|e| e.annotate("ByronSlotId"))
    }
}
