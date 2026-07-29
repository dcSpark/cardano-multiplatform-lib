// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

extern crate alloc;
use super::*;
use alloc::vec::Vec;
use cbor_event::de::Deserializer;
use cbor_event::se::Serializer;
use cml_core::error::*;
use cml_core::serialization::*;

impl Serialize for MultiEraBlock {
    fn serialize<'se>(
        &self,
        serializer: &'se mut Serializer,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer> {
        match self {
            MultiEraBlock::Byron(byron) => byron.serialize(serializer, force_canonical),
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
    fn deserialize(raw: &mut Deserializer) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let initial_position = raw.position();
            let mut errs = Vec::new();
            let deser_variant: Result<_, DeserializeError> = ByronBlock::deserialize(raw);
            match deser_variant {
                Ok(byron) => return Ok(Self::Byron(byron)),
                Err(e) => {
                    errs.push(e.annotate("Byron"));
                    raw.set_position(initial_position).unwrap();
                }
            };
            let deser_variant: Result<_, DeserializeError> = ShelleyBlock::deserialize(raw);
            match deser_variant {
                Ok(shelley) => return Ok(Self::Shelley(shelley)),
                Err(e) => {
                    errs.push(e.annotate("Shelley"));
                    raw.set_position(initial_position).unwrap();
                }
            };
            let deser_variant: Result<_, DeserializeError> = AllegraBlock::deserialize(raw);
            match deser_variant {
                Ok(allegra) => return Ok(Self::Allegra(allegra)),
                Err(e) => {
                    errs.push(e.annotate("Allegra"));
                    raw.set_position(initial_position).unwrap();
                }
            };
            let deser_variant: Result<_, DeserializeError> = MaryBlock::deserialize(raw);
            match deser_variant {
                Ok(mary) => return Ok(Self::Mary(mary)),
                Err(e) => {
                    errs.push(e.annotate("Mary"));
                    raw.set_position(initial_position).unwrap();
                }
            };
            let deser_variant: Result<_, DeserializeError> = AlonzoBlock::deserialize(raw);
            match deser_variant {
                Ok(alonzo) => return Ok(Self::Alonzo(alonzo)),
                Err(e) => {
                    errs.push(e.annotate("Alonzo"));
                    raw.set_position(initial_position).unwrap();
                }
            };
            let deser_variant: Result<_, DeserializeError> = BabbageBlock::deserialize(raw);
            match deser_variant {
                Ok(babbage) => return Ok(Self::Babbage(babbage)),
                Err(e) => {
                    errs.push(e.annotate("Babbage"));
                    raw.set_position(initial_position).unwrap();
                }
            };
            let deser_variant: Result<_, DeserializeError> = Block::deserialize(raw);
            match deser_variant {
                Ok(conway) => return Ok(Self::Conway(conway)),
                Err(e) => {
                    errs.push(e.annotate("Conway"));
                    raw.set_position(initial_position).unwrap();
                }
            };
            Err(DeserializeFailure::NoVariantMatchedWithCauses(errs).into())
        })()
        .map_err(|e| e.annotate("MultiEraBlock"))
    }
}

impl Serialize for MultiEraTransactionBody {
    fn serialize<'se>(
        &self,
        serializer: &'se mut Serializer,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer> {
        match self {
            MultiEraTransactionBody::Byron(byron) => byron.serialize(serializer, force_canonical),
            MultiEraTransactionBody::Shelley(shelley) => {
                shelley.serialize(serializer, force_canonical)
            }
            MultiEraTransactionBody::Allegra(allegra) => {
                allegra.serialize(serializer, force_canonical)
            }
            MultiEraTransactionBody::Mary(mary) => mary.serialize(serializer, force_canonical),
            MultiEraTransactionBody::Alonzo(alonzo) => {
                alonzo.serialize(serializer, force_canonical)
            }
            MultiEraTransactionBody::Babbage(babbage) => {
                babbage.serialize(serializer, force_canonical)
            }
            MultiEraTransactionBody::Conway(conway) => {
                conway.serialize(serializer, force_canonical)
            }
        }
    }
}

impl Deserialize for MultiEraTransactionBody {
    fn deserialize(raw: &mut Deserializer) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let initial_position = raw.position();
            let mut errs = Vec::new();
            let deser_variant: Result<_, DeserializeError> = ByronTx::deserialize(raw);
            match deser_variant {
                Ok(byron) => return Ok(Self::Byron(byron)),
                Err(e) => {
                    errs.push(e.annotate("Byron"));
                    raw.set_position(initial_position).unwrap();
                }
            };
            let deser_variant: Result<_, DeserializeError> =
                ShelleyTransactionBody::deserialize(raw);
            match deser_variant {
                Ok(shelley) => return Ok(Self::Shelley(shelley)),
                Err(e) => {
                    errs.push(e.annotate("Shelley"));
                    raw.set_position(initial_position).unwrap();
                }
            };
            let deser_variant: Result<_, DeserializeError> =
                AllegraTransactionBody::deserialize(raw);
            match deser_variant {
                Ok(allegra) => return Ok(Self::Allegra(allegra)),
                Err(e) => {
                    errs.push(e.annotate("Allegra"));
                    raw.set_position(initial_position).unwrap();
                }
            };
            let deser_variant: Result<_, DeserializeError> = MaryTransactionBody::deserialize(raw);
            match deser_variant {
                Ok(mary) => return Ok(Self::Mary(mary)),
                Err(e) => {
                    errs.push(e.annotate("Mary"));
                    raw.set_position(initial_position).unwrap();
                }
            };
            let deser_variant: Result<_, DeserializeError> =
                AlonzoTransactionBody::deserialize(raw);
            match deser_variant {
                Ok(alonzo) => return Ok(Self::Alonzo(alonzo)),
                Err(e) => {
                    errs.push(e.annotate("Alonzo"));
                    raw.set_position(initial_position).unwrap();
                }
            };
            let deser_variant: Result<_, DeserializeError> =
                BabbageTransactionBody::deserialize(raw);
            match deser_variant {
                Ok(babbage) => return Ok(Self::Babbage(babbage)),
                Err(e) => {
                    errs.push(e.annotate("Babbage"));
                    raw.set_position(initial_position).unwrap();
                }
            };
            let deser_variant: Result<_, DeserializeError> = TransactionBody::deserialize(raw);
            match deser_variant {
                Ok(conway) => return Ok(Self::Conway(conway)),
                Err(e) => {
                    errs.push(e.annotate("Conway"));
                    raw.set_position(initial_position).unwrap();
                }
            };
            Err(DeserializeFailure::NoVariantMatchedWithCauses(errs).into())
        })()
        .map_err(|e| e.annotate("MultiEraTransactionBody"))
    }
}
