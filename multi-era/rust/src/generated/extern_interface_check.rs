// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

// Compiled self-check for the dep-side extern-interface export
// (`extern-interface/<dep>/**`). Machine-generated from the SAME projection as that
// export, so the two cannot drift. Every exported name is asserted to be a real,
// correctly-typed surface in THIS crate: opaque rows implement `Serialize` (and
// `Deserialize` where the dep generates one), raw-bytes rows `RawBytesEncoding`, and
// transparent rows (aliases, c-style enums, named collections) must simply exist. A
// hand-edited or stale export — or a projection bug — therefore fails THIS crate's own
// build, naming the type. Do not edit.
// Rows carry NO per-row comments by design: a spec change can delete any row, and a
// comment stranded on a deleted row is what the edit-preservation overlay turns into a
// build-breaking sentinel on the next regen. All commentary lives in this fixed banner;
// each row's type path is its own traceability.
#[allow(dead_code)]
fn _assert_serialize<T: cml_core::serialization::Serialize>() {}
#[allow(dead_code)]
fn _assert_deserialize<T: cml_core::serialization::Deserialize>() {}
#[allow(dead_code)]
fn _assert_serialize_embedded_group<T: cml_core::serialization::SerializeEmbeddedGroup>() {}
#[allow(dead_code)]
fn _assert_deserialize_embedded_group<T: cml_core::serialization::DeserializeEmbeddedGroup>() {}
#[allow(unused_imports)]
use crate::generated::allegra::MIRPot as _;
#[allow(unused_imports)]
use crate::generated::alonzo::AlonzoRedeemerTag as _;
#[allow(unused_imports)]
use crate::generated::shelley::ShelleyTransactionIndex as _;
#[allow(dead_code)]
fn _extern_interface_self_check() {
    _assert_serialize::<crate::generated::allegra::AllegraAuxiliaryData>();
    _assert_deserialize::<crate::generated::allegra::AllegraAuxiliaryData>();
    _assert_serialize::<crate::generated::allegra::AllegraBlock>();
    _assert_deserialize::<crate::generated::allegra::AllegraBlock>();
    _assert_serialize::<crate::generated::allegra::AllegraCertificate>();
    _assert_deserialize::<crate::generated::allegra::AllegraCertificate>();
    _assert_serialize::<crate::generated::allegra::AllegraTransaction>();
    _assert_deserialize::<crate::generated::allegra::AllegraTransaction>();
    _assert_serialize::<crate::generated::allegra::AllegraTransactionBody>();
    _assert_deserialize::<crate::generated::allegra::AllegraTransactionBody>();
    _assert_serialize::<crate::generated::allegra::AllegraTransactionWitnessSet>();
    _assert_deserialize::<crate::generated::allegra::AllegraTransactionWitnessSet>();
    _assert_serialize::<crate::generated::alonzo::AlonzoAuxiliaryData>();
    _assert_deserialize::<crate::generated::alonzo::AlonzoAuxiliaryData>();
    _assert_serialize::<crate::generated::alonzo::AlonzoBlock>();
    _assert_deserialize::<crate::generated::alonzo::AlonzoBlock>();
    _assert_serialize::<crate::generated::alonzo::AlonzoFormatAuxData>();
    _assert_deserialize::<crate::generated::alonzo::AlonzoFormatAuxData>();
    _assert_serialize::<crate::generated::alonzo::AlonzoProtocolParamUpdate>();
    _assert_deserialize::<crate::generated::alonzo::AlonzoProtocolParamUpdate>();
    _assert_serialize::<crate::generated::alonzo::AlonzoRedeemer>();
    _assert_deserialize::<crate::generated::alonzo::AlonzoRedeemer>();
    _assert_serialize::<crate::generated::alonzo::AlonzoTransaction>();
    _assert_deserialize::<crate::generated::alonzo::AlonzoTransaction>();
    _assert_serialize::<crate::generated::alonzo::AlonzoTransactionBody>();
    _assert_deserialize::<crate::generated::alonzo::AlonzoTransactionBody>();
    _assert_serialize::<crate::generated::alonzo::AlonzoTransactionWitnessSet>();
    _assert_deserialize::<crate::generated::alonzo::AlonzoTransactionWitnessSet>();
    _assert_serialize::<crate::generated::alonzo::AlonzoUpdate>();
    _assert_deserialize::<crate::generated::alonzo::AlonzoUpdate>();
    _assert_serialize::<crate::generated::babbage::BabbageAuxiliaryData>();
    _assert_deserialize::<crate::generated::babbage::BabbageAuxiliaryData>();
    _assert_serialize::<crate::generated::babbage::BabbageBlock>();
    _assert_deserialize::<crate::generated::babbage::BabbageBlock>();
    _assert_serialize::<crate::generated::babbage::BabbageFormatAuxData>();
    _assert_deserialize::<crate::generated::babbage::BabbageFormatAuxData>();
    _assert_serialize::<crate::generated::babbage::BabbageFormatTxOut>();
    _assert_deserialize::<crate::generated::babbage::BabbageFormatTxOut>();
    _assert_serialize::<crate::generated::babbage::BabbageMint>();
    _assert_deserialize::<crate::generated::babbage::BabbageMint>();
    _assert_serialize::<crate::generated::babbage::BabbageProtocolParamUpdate>();
    _assert_deserialize::<crate::generated::babbage::BabbageProtocolParamUpdate>();
    _assert_serialize::<crate::generated::babbage::BabbageScript>();
    _assert_deserialize::<crate::generated::babbage::BabbageScript>();
    _assert_serialize::<crate::generated::babbage::BabbageScriptRef>();
    _assert_deserialize::<crate::generated::babbage::BabbageScriptRef>();
    _assert_serialize::<crate::generated::babbage::BabbageTransaction>();
    _assert_deserialize::<crate::generated::babbage::BabbageTransaction>();
    _assert_serialize::<crate::generated::babbage::BabbageTransactionBody>();
    _assert_deserialize::<crate::generated::babbage::BabbageTransactionBody>();
    _assert_serialize::<crate::generated::babbage::BabbageTransactionOutput>();
    _assert_deserialize::<crate::generated::babbage::BabbageTransactionOutput>();
    _assert_serialize::<crate::generated::babbage::BabbageTransactionWitnessSet>();
    _assert_deserialize::<crate::generated::babbage::BabbageTransactionWitnessSet>();
    _assert_serialize::<crate::generated::babbage::BabbageUpdate>();
    _assert_deserialize::<crate::generated::babbage::BabbageUpdate>();
    _assert_serialize::<crate::generated::Block>();
    _assert_deserialize::<crate::generated::Block>();
    _assert_serialize::<crate::generated::ByronBlock>();
    _assert_deserialize::<crate::generated::ByronBlock>();
    _assert_serialize::<crate::generated::ByronTx>();
    _assert_deserialize::<crate::generated::ByronTx>();
    _assert_serialize::<crate::generated::allegra::MIRAction>();
    _assert_deserialize::<crate::generated::allegra::MIRAction>();
    _assert_serialize::<crate::generated::mary::MaryBlock>();
    _assert_deserialize::<crate::generated::mary::MaryBlock>();
    _assert_serialize::<crate::generated::mary::MaryTransaction>();
    _assert_deserialize::<crate::generated::mary::MaryTransaction>();
    _assert_serialize::<crate::generated::mary::MaryTransactionBody>();
    _assert_deserialize::<crate::generated::mary::MaryTransactionBody>();
    _assert_serialize::<crate::generated::mary::MaryTransactionOutput>();
    _assert_deserialize::<crate::generated::mary::MaryTransactionOutput>();
    _assert_serialize::<crate::generated::allegra::MoveInstantaneousReward>();
    _assert_deserialize::<crate::generated::allegra::MoveInstantaneousReward>();
    _assert_serialize::<crate::generated::allegra::MoveInstantaneousRewardsCert>();
    _assert_serialize_embedded_group::<crate::generated::allegra::MoveInstantaneousRewardsCert>();
    _assert_deserialize::<crate::generated::allegra::MoveInstantaneousRewardsCert>();
    _assert_deserialize_embedded_group::<crate::generated::allegra::MoveInstantaneousRewardsCert>();
    _assert_serialize::<crate::generated::MultiEraBlock>();
    _assert_deserialize::<crate::generated::MultiEraBlock>();
    _assert_serialize::<crate::generated::MultiEraTransactionBody>();
    _assert_deserialize::<crate::generated::MultiEraTransactionBody>();
    _assert_serialize::<crate::generated::shelley::MultisigAll>();
    _assert_serialize_embedded_group::<crate::generated::shelley::MultisigAll>();
    _assert_deserialize::<crate::generated::shelley::MultisigAll>();
    _assert_deserialize_embedded_group::<crate::generated::shelley::MultisigAll>();
    _assert_serialize::<crate::generated::shelley::MultisigAny>();
    _assert_serialize_embedded_group::<crate::generated::shelley::MultisigAny>();
    _assert_deserialize::<crate::generated::shelley::MultisigAny>();
    _assert_deserialize_embedded_group::<crate::generated::shelley::MultisigAny>();
    _assert_serialize::<crate::generated::shelley::MultisigNOfK>();
    _assert_serialize_embedded_group::<crate::generated::shelley::MultisigNOfK>();
    _assert_deserialize::<crate::generated::shelley::MultisigNOfK>();
    _assert_deserialize_embedded_group::<crate::generated::shelley::MultisigNOfK>();
    _assert_serialize::<crate::generated::shelley::MultisigScript>();
    _assert_deserialize::<crate::generated::shelley::MultisigScript>();
    _assert_serialize::<crate::generated::shelley::ProtocolVersionStruct>();
    _assert_deserialize::<crate::generated::shelley::ProtocolVersionStruct>();
    _assert_serialize::<crate::generated::shelley::ShelleyBlock>();
    _assert_deserialize::<crate::generated::shelley::ShelleyBlock>();
    _assert_serialize::<crate::generated::shelley::ShelleyCertificate>();
    _assert_deserialize::<crate::generated::shelley::ShelleyCertificate>();
    _assert_serialize::<crate::generated::shelley::ShelleyDNSName>();
    _assert_deserialize::<crate::generated::shelley::ShelleyDNSName>();
    _assert_serialize::<crate::generated::shelley::ShelleyHeader>();
    _assert_deserialize::<crate::generated::shelley::ShelleyHeader>();
    _assert_serialize::<crate::generated::shelley::ShelleyHeaderBody>();
    _assert_deserialize::<crate::generated::shelley::ShelleyHeaderBody>();
    _assert_serialize::<crate::generated::shelley::ShelleyMoveInstantaneousReward>();
    _assert_deserialize::<crate::generated::shelley::ShelleyMoveInstantaneousReward>();
    _assert_serialize::<crate::generated::shelley::ShelleyMoveInstantaneousRewardsCert>();
    _assert_serialize_embedded_group::<
        crate::generated::shelley::ShelleyMoveInstantaneousRewardsCert,
    >();
    _assert_deserialize::<crate::generated::shelley::ShelleyMoveInstantaneousRewardsCert>();
    _assert_deserialize_embedded_group::<
        crate::generated::shelley::ShelleyMoveInstantaneousRewardsCert,
    >();
    _assert_serialize::<crate::generated::shelley::ShelleyMultiHostName>();
    _assert_serialize_embedded_group::<crate::generated::shelley::ShelleyMultiHostName>();
    _assert_deserialize::<crate::generated::shelley::ShelleyMultiHostName>();
    _assert_deserialize_embedded_group::<crate::generated::shelley::ShelleyMultiHostName>();
    _assert_serialize::<crate::generated::shelley::ShelleyProtocolParamUpdate>();
    _assert_deserialize::<crate::generated::shelley::ShelleyProtocolParamUpdate>();
    _assert_serialize::<crate::generated::shelley::ShelleyRelay>();
    _assert_deserialize::<crate::generated::shelley::ShelleyRelay>();
    _assert_serialize::<crate::generated::shelley::ShelleyTransaction>();
    _assert_deserialize::<crate::generated::shelley::ShelleyTransaction>();
    _assert_serialize::<crate::generated::shelley::ShelleyTransactionBody>();
    _assert_deserialize::<crate::generated::shelley::ShelleyTransactionBody>();
    _assert_serialize::<crate::generated::shelley::ShelleyTransactionOutput>();
    _assert_deserialize::<crate::generated::shelley::ShelleyTransactionOutput>();
    _assert_serialize::<crate::generated::shelley::ShelleyTransactionWitnessSet>();
    _assert_deserialize::<crate::generated::shelley::ShelleyTransactionWitnessSet>();
    _assert_serialize::<crate::generated::shelley::ShelleyUpdate>();
    _assert_deserialize::<crate::generated::shelley::ShelleyUpdate>();
}
