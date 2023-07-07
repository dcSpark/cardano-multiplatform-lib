use super::*;
use cbor_event::de::Deserializer;
use cbor_event::se::Serializer;
use cml_core::error::*;
use cml_core::serialization::*;
use std::io::{BufRead, Seek, SeekFrom, Write};

impl Serialize for MultiEraBlock {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            MultiEraBlock::Shelley(shelley) => shelley.serialize(serializer, force_canonical),
            MultiEraBlock::Allegra(allegra) => allegra.serialize(serializer, force_canonical),
            MultiEraBlock::Mary(mary) => mary.serialize(serializer, force_canonical),
            MultiEraBlock::Alonzo(alonzo) => alonzo.serialize(serializer, force_canonical),
            MultiEraBlock::Babbage(babbage) => babbage.serialize(serializer, force_canonical),
        }
    }
}

impl Deserialize for MultiEraBlock {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let initial_position = raw.as_mut_ref().seek(SeekFrom::Current(0)).unwrap();
            let deser_variant: Result<_, DeserializeError> = ShelleyBlock::deserialize(raw);
            match deser_variant {
                Ok(shelley) => return Ok(Self::Shelley(shelley)),
                Err(_e) => {
                    raw
                        .as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap()
                },
            };
            let deser_variant: Result<_, DeserializeError> = AllegraBlock::deserialize(raw);
            match deser_variant {
                Ok(allegra) => return Ok(Self::Allegra(allegra)),
                Err(_e) => {
                    raw
                        .as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap()
                },
            };
            let deser_variant: Result<_, DeserializeError> = MaryBlock::deserialize(raw);
            match deser_variant {
                Ok(mary) => return Ok(Self::Mary(mary)),
                Err(_e) => {
                    raw
                        .as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap()
                },
            };
            let deser_variant: Result<_, DeserializeError> = AlonzoBlock::deserialize(raw);
            match deser_variant {
                Ok(alonzo) => return Ok(Self::Alonzo(alonzo)),
                Err(_e) => {
                    raw
                        .as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap()
                },
            };
            let deser_variant: Result<_, DeserializeError> = Block::deserialize(raw);
            match deser_variant {
                Ok(babbage) => return Ok(Self::Babbage(babbage)),
                Err(_e) => {
                    raw
                        .as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap()
                },
            };
            Err(DeserializeError::new(
                "MultiEraBlock",
                DeserializeFailure::NoVariantMatched,
            ))
        })()
        .map_err(|e| e.annotate("MultiEraBlock"))
    }
}