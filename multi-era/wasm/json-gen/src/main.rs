fn main() {
    macro_rules! gen_json_schema {
        ($name:ty) => {
            let dest_path =
                std::path::Path::new(&"schemas").join(&format!("{}.json", stringify!($name)));
            std::fs::write(
                &dest_path,
                serde_json::to_string_pretty(&schemars::schema_for!($name)).unwrap(),
            )
            .unwrap();
        };
    }
    let schema_path = std::path::Path::new(&"schemas");
    if !schema_path.exists() {
        std::fs::create_dir(schema_path).unwrap();
    }
    // allegra.rs
    gen_json_schema!(cml_multi_era::allegra::AllegraAuxiliaryData);
    gen_json_schema!(cml_multi_era::allegra::AllegraTransactionBody);
    gen_json_schema!(cml_multi_era::allegra::AllegraTransactionWitnessSet);
    // alonzo.rs
    gen_json_schema!(cml_multi_era::alonzo::AlonzoAuxiliaryData);
    gen_json_schema!(cml_multi_era::alonzo::AlonzoCostmdls);
    gen_json_schema!(cml_multi_era::alonzo::AlonzoOnlyAuxData);
    gen_json_schema!(cml_multi_era::alonzo::AlonzoProtocolParamUpdate);
    gen_json_schema!(cml_multi_era::alonzo::AlonzoTransactionBody);
    gen_json_schema!(cml_multi_era::alonzo::AlonzoTransactionWitnessSet);
    gen_json_schema!(cml_multi_era::alonzo::AlonzoUpdate);
    // lib.rs
    gen_json_schema!(cml_multi_era::Int);
    // mary.rs
    gen_json_schema!(cml_multi_era::mary::MaryTransactionBody);
    // shelley.rs
    gen_json_schema!(cml_multi_era::shelley::MultisigAll);
    gen_json_schema!(cml_multi_era::shelley::MultisigAny);
    gen_json_schema!(cml_multi_era::shelley::MultisigNOfK);
    gen_json_schema!(cml_multi_era::shelley::MultisigPubkey);
    gen_json_schema!(cml_multi_era::shelley::MultisigScript);
    gen_json_schema!(cml_multi_era::shelley::ShelleyHeader);
    gen_json_schema!(cml_multi_era::shelley::ShelleyHeaderBody);
    gen_json_schema!(cml_multi_era::shelley::ShelleyProtocolParamUpdate);
    gen_json_schema!(cml_multi_era::shelley::ShelleyTransactionBody);
    gen_json_schema!(cml_multi_era::shelley::ShelleyTransactionOutput);
    gen_json_schema!(cml_multi_era::shelley::ShelleyTransactionWitnessSet);
    gen_json_schema!(cml_multi_era::shelley::ShelleyUpdate);
}
