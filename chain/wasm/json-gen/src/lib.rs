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

pub fn export_schemas() {
    let schema_path = std::path::Path::new(&"schemas");
    if !schema_path.exists() {
        std::fs::create_dir(schema_path).unwrap();
    }
    // copy over custom ones
    for custom_schema in std::fs::read_dir(
        std::path::Path::new("..")
            .join("..")
            .join("..")
            .join("chain")
            .join("wasm")
            .join("json-gen")
            .join("custom_schemas"),
    )
    .unwrap()
    {
        let old_path = custom_schema.unwrap().path();
        //if let Some("json") = old_path.extension().and_then(|p| p.to_str()) {
        let new_path = std::path::Path::new("schemas").join(old_path.file_name().unwrap());
        println!(
            "MOVING: {}\nTO: {}",
            old_path.as_os_str().to_str().unwrap(),
            new_path.as_os_str().to_str().unwrap()
        );
        std::fs::copy(old_path, new_path).unwrap();
        //}
    }
    // address
    gen_json_schema!(cml_chain::address::Address);
    gen_json_schema!(cml_chain::address::RewardAccount);
    // assets
    gen_json_schema!(cml_chain::assets::AssetName);
    gen_json_schema!(cml_chain::assets::Value);
    // auxdata
    gen_json_schema!(cml_chain::auxdata::AuxiliaryData);
    gen_json_schema!(cml_chain::auxdata::Metadata);
    // block
    gen_json_schema!(cml_chain::block::Block);
    gen_json_schema!(cml_chain::block::Header);
    gen_json_schema!(cml_chain::block::HeaderBody);
    gen_json_schema!(cml_chain::block::OperationalCert);
    gen_json_schema!(cml_chain::block::ProtocolVersion);
    // byron
    gen_json_schema!(cml_chain::byron::AddrAttributes);
    gen_json_schema!(cml_chain::byron::AddressContent);
    gen_json_schema!(cml_chain::byron::ByronAddress);
    gen_json_schema!(cml_chain::byron::ByronAddrType);
    gen_json_schema!(cml_chain::byron::ByronTxOut);
    gen_json_schema!(cml_chain::byron::Crc32);
    gen_json_schema!(cml_chain::byron::HDAddressPayload);
    gen_json_schema!(cml_chain::byron::SpendingData);
    gen_json_schema!(cml_chain::byron::ProtocolMagic);
    gen_json_schema!(cml_chain::byron::StakeDistribution);
    gen_json_schema!(cml_chain::byron::StakeholderId);
    gen_json_schema!(cml_crypto::Bip32PublicKey);
    // certs
    gen_json_schema!(cml_chain::certs::Certificate);
    gen_json_schema!(cml_chain::certs::DnsName);
    gen_json_schema!(cml_chain::certs::Ipv4);
    gen_json_schema!(cml_chain::certs::Ipv6);
    gen_json_schema!(cml_chain::certs::MultiHostName);
    gen_json_schema!(cml_chain::certs::PoolMetadata);
    gen_json_schema!(cml_chain::certs::PoolParams);
    gen_json_schema!(cml_chain::certs::PoolRegistration);
    gen_json_schema!(cml_chain::certs::PoolRetirement);
    gen_json_schema!(cml_chain::certs::Relay);
    gen_json_schema!(cml_chain::certs::SingleHostAddr);
    gen_json_schema!(cml_chain::certs::SingleHostName);
    gen_json_schema!(cml_chain::certs::StakeCredential);
    gen_json_schema!(cml_chain::certs::StakeDelegation);
    gen_json_schema!(cml_chain::certs::StakeDeregistration);
    gen_json_schema!(cml_chain::certs::StakeRegistration);
    gen_json_schema!(cml_chain::certs::Url);
    // crypto
    gen_json_schema!(cml_chain::crypto::AuxiliaryDataHash);
    gen_json_schema!(cml_chain::crypto::BlockBodyHash);
    gen_json_schema!(cml_chain::crypto::BlockHeaderHash);
    gen_json_schema!(cml_chain::crypto::DatumHash);
    gen_json_schema!(cml_chain::crypto::Ed25519KeyHash);
    gen_json_schema!(cml_chain::crypto::Ed25519Signature);
    gen_json_schema!(cml_chain::crypto::GenesisDelegateHash);
    gen_json_schema!(cml_chain::crypto::GenesisHash);
    gen_json_schema!(cml_chain::crypto::KESSignature);
    gen_json_schema!(cml_chain::crypto::KESVkey);
    gen_json_schema!(cml_chain::crypto::Nonce);
    gen_json_schema!(cml_chain::crypto::PoolMetadataHash);
    gen_json_schema!(cml_chain::crypto::ScriptDataHash);
    gen_json_schema!(cml_chain::crypto::ScriptHash);
    gen_json_schema!(cml_chain::crypto::TransactionHash);
    gen_json_schema!(cml_chain::crypto::VRFCert);
    gen_json_schema!(cml_chain::crypto::VRFKeyHash);
    gen_json_schema!(cml_chain::crypto::VRFVkey);
    gen_json_schema!(cml_chain::crypto::Vkey);
    // lib
    gen_json_schema!(cml_chain::crypto::BootstrapWitness);
    gen_json_schema!(cml_chain::Int);
    gen_json_schema!(cml_chain::ProtocolParamUpdate);
    gen_json_schema!(cml_chain::Rational);
    gen_json_schema!(cml_chain::Script);
    gen_json_schema!(cml_chain::UnitInterval);
    gen_json_schema!(cml_chain::Value);
    gen_json_schema!(cml_chain::crypto::Vkeywitness);
    // plutus
    //gen_json_schema!(cml_chain::plutus::ConstrPlutusData);
    gen_json_schema!(cml_chain::plutus::CostModels);
    gen_json_schema!(cml_chain::plutus::ExUnitPrices);
    gen_json_schema!(cml_chain::plutus::ExUnits);
    //gen_json_schema!(cml_chain::plutus::PlutusData);
    //gen_json_schema!(cml_chain::plutus::PlutusMap);
    gen_json_schema!(cml_chain::plutus::PlutusV1Script);
    gen_json_schema!(cml_chain::plutus::PlutusV2Script);
    gen_json_schema!(cml_chain::plutus::Redeemer);
    gen_json_schema!(cml_chain::plutus::RedeemerTag);
    // transaction
    gen_json_schema!(cml_chain::transaction::DatumOption);
    gen_json_schema!(cml_chain::transaction::NativeScript);
    gen_json_schema!(cml_chain::transaction::RequiredSigners);
    gen_json_schema!(cml_chain::transaction::ScriptAll);
    gen_json_schema!(cml_chain::transaction::ScriptAny);
    gen_json_schema!(cml_chain::transaction::ScriptInvalidBefore);
    gen_json_schema!(cml_chain::transaction::ScriptInvalidHereafter);
    gen_json_schema!(cml_chain::transaction::ScriptNOfK);
    gen_json_schema!(cml_chain::transaction::ScriptPubkey);
    gen_json_schema!(cml_chain::transaction::Transaction);
    gen_json_schema!(cml_chain::transaction::TransactionBody);
    gen_json_schema!(cml_chain::transaction::TransactionInput);
    gen_json_schema!(cml_chain::transaction::TransactionOutput);
    gen_json_schema!(cml_chain::transaction::TransactionWitnessSet);
    // utils
    gen_json_schema!(cml_chain::utils::BigInteger);
}
