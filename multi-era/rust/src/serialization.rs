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
            MultiEraBlock::Byron(byron) => cbor_event::se::Serialize::serialize(byron, serializer),
            MultiEraBlock::Shelley(shelley) => shelley.serialize(serializer, force_canonical),
            MultiEraBlock::Allegra(allegra) => allegra.serialize(serializer, force_canonical),
            MultiEraBlock::Mary(mary) => mary.serialize(serializer, force_canonical),
            MultiEraBlock::Alonzo(alonzo) => alonzo.serialize(serializer, force_canonical),
            MultiEraBlock::Babbage(babbage) => babbage.serialize(serializer, force_canonical),
            MultiEraBlock::Conway(conway) => conway.serialize(serializer, force_canonical),
        }
    }
}

impl Deserialize for MultiEraBlock {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let initial_position = raw.as_mut_ref().stream_position().unwrap();
            let mut errs = Vec::new();
            let deser_variant: Result<_, DeserializeError> = ByronBlock::deserialize(raw);
            match deser_variant {
                Ok(byron) => return Ok(Self::Byron(byron)),
                Err(e) => {
                    errs.push(e.annotate("Byron"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant: Result<_, DeserializeError> = ShelleyBlock::deserialize(raw);
            match deser_variant {
                Ok(shelley) => return Ok(Self::Shelley(shelley)),
                Err(e) => {
                    errs.push(e.annotate("Shelley"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant: Result<_, DeserializeError> = AllegraBlock::deserialize(raw);
            match deser_variant {
                Ok(allegra) => return Ok(Self::Allegra(allegra)),
                Err(e) => {
                    errs.push(e.annotate("Allegra"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant: Result<_, DeserializeError> = MaryBlock::deserialize(raw);
            match deser_variant {
                Ok(mary) => return Ok(Self::Mary(mary)),
                Err(e) => {
                    errs.push(e.annotate("Mary"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant: Result<_, DeserializeError> = AlonzoBlock::deserialize(raw);
            match deser_variant {
                Ok(alonzo) => return Ok(Self::Alonzo(alonzo)),
                Err(e) => {
                    errs.push(e.annotate("Alonzo"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant: Result<_, DeserializeError> = BabbageBlock::deserialize(raw);
            match deser_variant {
                Ok(babbage) => return Ok(Self::Babbage(babbage)),
                Err(e) => {
                    errs.push(e.annotate("Babbage"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant: Result<_, DeserializeError> = Block::deserialize(raw);
            match deser_variant {
                Ok(conway) => return Ok(Self::Conway(conway)),
                Err(e) => {
                    errs.push(e.annotate("Conway"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            Err(DeserializeError::new(
                "MultiEraBlock",
                DeserializeFailure::NoVariantMatchedWithCauses(errs),
            ))
        })()
        .map_err(|e| e.annotate("MultiEraBlock"))
    }
}
