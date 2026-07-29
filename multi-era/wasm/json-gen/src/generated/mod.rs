use cml_core::json_schema_gen::Registrar;
use cml_core::json_schema_gen::check_schema_ref_closure;

pub fn add_schemas(generator: &mut schemars::SchemaGenerator) {
    cml_chain_json_schema_gen::add_schemas(generator);
    cml_cip25_json_schema_gen::add_schemas(generator);
    cml_cip36_json_schema_gen::add_schemas(generator);
    let mut reg = Registrar::new(generator);
    // allegra
    reg.add::<cml_multi_era::allegra::AllegraAuxiliaryData>();
    reg.add::<cml_multi_era::allegra::AllegraBlock>();
    reg.add::<cml_multi_era::allegra::AllegraCertificate>();
    reg.add::<cml_multi_era::allegra::AllegraTransaction>();
    reg.add::<cml_multi_era::allegra::AllegraTransactionBody>();
    reg.add::<cml_multi_era::allegra::AllegraTransactionWitnessSet>();
    reg.add::<cml_multi_era::allegra::MIRAction>();
    reg.add::<cml_multi_era::allegra::MIRPot>();
    reg.add::<cml_multi_era::allegra::MoveInstantaneousReward>();
    reg.add::<cml_multi_era::allegra::MoveInstantaneousRewardsCert>();
    // alonzo
    reg.add::<cml_multi_era::alonzo::AlonzoAuxiliaryData>();
    reg.add::<cml_multi_era::alonzo::AlonzoBlock>();
    reg.add::<cml_multi_era::alonzo::AlonzoFormatAuxData>();
    reg.add::<cml_multi_era::alonzo::AlonzoProtocolParamUpdate>();
    reg.add::<cml_multi_era::alonzo::AlonzoRedeemer>();
    reg.add::<cml_multi_era::alonzo::AlonzoRedeemerTag>();
    reg.add::<cml_multi_era::alonzo::AlonzoTransaction>();
    reg.add::<cml_multi_era::alonzo::AlonzoTransactionBody>();
    reg.add::<cml_multi_era::alonzo::AlonzoTransactionWitnessSet>();
    reg.add::<cml_multi_era::alonzo::AlonzoUpdate>();
    // babbage
    reg.add::<cml_multi_era::babbage::BabbageAuxiliaryData>();
    reg.add::<cml_multi_era::babbage::BabbageBlock>();
    reg.add::<cml_multi_era::babbage::BabbageFormatAuxData>();
    reg.add::<cml_multi_era::babbage::BabbageFormatTxOut>();
    reg.add::<cml_multi_era::babbage::BabbageMint>();
    reg.add::<cml_multi_era::babbage::BabbageProtocolParamUpdate>();
    reg.add::<cml_multi_era::babbage::BabbageScript>();
    reg.add::<cml_multi_era::babbage::BabbageScriptRef>();
    reg.add::<cml_multi_era::babbage::BabbageTransaction>();
    reg.add::<cml_multi_era::babbage::BabbageTransactionBody>();
    reg.add::<cml_multi_era::babbage::BabbageTransactionOutput>();
    reg.add::<cml_multi_era::babbage::BabbageTransactionWitnessSet>();
    reg.add::<cml_multi_era::babbage::BabbageUpdate>();
    // lib
    reg.add::<cml_multi_era::Block>();
    reg.add::<cml_multi_era::ByronBlock>();
    reg.add::<cml_multi_era::ByronTx>();
    reg.add::<cml_multi_era::Int>();
    reg.add::<cml_multi_era::MultiEraBlock>();
    reg.add::<cml_multi_era::MultiEraTransactionBody>();
    // mary
    reg.add::<cml_multi_era::mary::MaryBlock>();
    reg.add::<cml_multi_era::mary::MaryTransaction>();
    reg.add::<cml_multi_era::mary::MaryTransactionBody>();
    reg.add::<cml_multi_era::mary::MaryTransactionOutput>();
    // shelley
    reg.add::<cml_multi_era::shelley::GenesisKeyDelegation>();
    reg.add::<cml_multi_era::shelley::MultisigAll>();
    reg.add::<cml_multi_era::shelley::MultisigAny>();
    reg.add::<cml_multi_era::shelley::MultisigNOfK>();
    reg.add::<cml_multi_era::shelley::MultisigPubkey>();
    reg.add::<cml_multi_era::shelley::MultisigScript>();
    reg.add::<cml_multi_era::shelley::ProtocolVersionStruct>();
    reg.add::<cml_multi_era::shelley::ShelleyBlock>();
    reg.add::<cml_multi_era::shelley::ShelleyCertificate>();
    reg.add::<cml_multi_era::shelley::ShelleyDNSName>();
    reg.add::<cml_multi_era::shelley::ShelleyHeader>();
    reg.add::<cml_multi_era::shelley::ShelleyHeaderBody>();
    reg.add::<cml_multi_era::shelley::ShelleyMoveInstantaneousReward>();
    reg.add::<cml_multi_era::shelley::ShelleyMoveInstantaneousRewardsCert>();
    reg.add::<cml_multi_era::shelley::ShelleyMultiHostName>();
    reg.add::<cml_multi_era::shelley::ShelleyPoolParams>();
    reg.add::<cml_multi_era::shelley::ShelleyPoolRegistration>();
    reg.add::<cml_multi_era::shelley::ShelleyProtocolParamUpdate>();
    reg.add::<cml_multi_era::shelley::ShelleyRelay>();
    reg.add::<cml_multi_era::shelley::ShelleySingleHostName>();
    reg.add::<cml_multi_era::shelley::ShelleyTransaction>();
    reg.add::<cml_multi_era::shelley::ShelleyTransactionBody>();
    reg.add::<cml_multi_era::shelley::ShelleyTransactionOutput>();
    reg.add::<cml_multi_era::shelley::ShelleyTransactionWitnessSet>();
    reg.add::<cml_multi_era::shelley::ShelleyUpdate>();
    reg.add::<cml_multi_era::byron::Blake2b256>();
    reg.add::<cml_multi_era::byron::utils::ByronAny>();
    reg.add::<cml_multi_era::utils::MultiEraBlockHeader>();
    reg.add::<cml_multi_era::utils::MultiEraCertificate>();
    reg.add::<cml_multi_era::utils::MultiEraProtocolParamUpdate>();
}

pub fn export_schemas() {
    let schema_path = std::path::Path::new("schemas");
    if !schema_path.exists() {
        std::fs::create_dir(schema_path).unwrap();
    }
    let mut generator = schemars::SchemaGenerator::default();
    add_schemas(&mut generator);
    let meta_schema = generator.settings().meta_schema.clone();
    let definitions_path = generator.settings().definitions_path.to_string();
    let mut document = serde_json::Map::new();
    if let Some(meta_schema) = meta_schema {
        document.insert("$schema".to_owned(), meta_schema.into_owned().into());
    }
    document.insert("title".to_owned(), "cml_multi_era".into());
    document.insert("$defs".to_owned(), generator.take_definitions(true).into());
    let document = serde_json::Value::Object(document);
    check_schema_ref_closure(&document, &definitions_path);
    std::fs::write(
        schema_path.join("cml_multi_era.schema.json"),
        serde_json::to_string_pretty(&document).unwrap(),
    )
    .unwrap();
}
