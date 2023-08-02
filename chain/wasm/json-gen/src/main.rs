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
    // address.rs
    gen_json_schema!(cml_chain::address::Address);
    gen_json_schema!(cml_chain::address::RewardAccount);
    // assets.rs
    gen_json_schema!(cml_chain::assets::AssetBundle<u64>);
    gen_json_schema!(cml_chain::assets::AssetBundle<i64>);
    // auxdata.rs
    gen_json_schema!(cml_chain::auxdata::AlonzoAuxData);
    gen_json_schema!(cml_chain::auxdata::AuxiliaryData);
    gen_json_schema!(cml_chain::auxdata::ShelleyMaAuxData);
    gen_json_schema!(cml_chain::auxdata::TransactionMetadatum);
    gen_json_schema!(cml_chain::auxdata::Metadata);
    gen_json_schema!(cml_chain::auxdata::MetadatumMap);
    // block.rs
    gen_json_schema!(cml_chain::block::Block);
    gen_json_schema!(cml_chain::block::Header);
    gen_json_schema!(cml_chain::block::HeaderBody);
    gen_json_schema!(cml_chain::block::OperationalCert);
    gen_json_schema!(cml_chain::block::ProtocolVersion);
    // byron.rs
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
    // certs.rs
    gen_json_schema!(cml_chain::certs::Certificate);
    gen_json_schema!(cml_chain::certs::DnsName);
    gen_json_schema!(cml_chain::certs::GenesisKeyDelegation);
    gen_json_schema!(cml_chain::certs::Ipv4);
    gen_json_schema!(cml_chain::certs::Ipv6);
    gen_json_schema!(cml_chain::certs::MIRAction);
    gen_json_schema!(cml_chain::certs::MIRPot);
    gen_json_schema!(cml_chain::certs::MoveInstantaneousReward);
    gen_json_schema!(cml_chain::certs::MoveInstantaneousRewardsCert);
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
    // crypto.rs
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
    // lib.rs
    gen_json_schema!(cml_chain::AssetName);
    gen_json_schema!(cml_chain::crypto::BootstrapWitness);
    //gen_json_schema!(cml_chain::BoundedBytes);
    gen_json_schema!(cml_chain::Int);
    gen_json_schema!(cml_chain::PositiveInterval);
    gen_json_schema!(cml_chain::ProtocolParamUpdate);
    gen_json_schema!(cml_chain::ProtocolVersionStruct);
    gen_json_schema!(cml_chain::Rational);
    gen_json_schema!(cml_chain::Script);
    gen_json_schema!(cml_chain::UnitInterval);
    gen_json_schema!(cml_chain::Update);
    gen_json_schema!(cml_chain::Value);
    gen_json_schema!(cml_chain::crypto::Vkeywitness);
    // plutus.rs
    gen_json_schema!(cml_chain::plutus::ConstrPlutusData);
    gen_json_schema!(cml_chain::plutus::CostModels);
    gen_json_schema!(cml_chain::plutus::ExUnitPrices);
    gen_json_schema!(cml_chain::plutus::ExUnits);
    gen_json_schema!(cml_chain::plutus::PlutusData);
    gen_json_schema!(cml_chain::plutus::PlutusMap);
    gen_json_schema!(cml_chain::plutus::PlutusV1Script);
    gen_json_schema!(cml_chain::plutus::PlutusV2Script);
    gen_json_schema!(cml_chain::plutus::Redeemer);
    gen_json_schema!(cml_chain::plutus::RedeemerTag);
    // transaction.rs
    gen_json_schema!(cml_chain::transaction::AlonzoTxOut);
    gen_json_schema!(cml_chain::transaction::BabbageTxOut);
    gen_json_schema!(cml_chain::transaction::DatumOption);
    gen_json_schema!(cml_chain::transaction::NativeScript);
    gen_json_schema!(cml_chain::transaction::RequiredSigners);
    gen_json_schema!(cml_chain::transaction::ScriptAll);
    gen_json_schema!(cml_chain::transaction::ScriptAny);
    gen_json_schema!(cml_chain::transaction::ScriptInvalidBefore);
    gen_json_schema!(cml_chain::transaction::ScriptInvalidHereafter);
    gen_json_schema!(cml_chain::transaction::ScriptNOfK);
    gen_json_schema!(cml_chain::transaction::ScriptPubkey);
    gen_json_schema!(cml_chain::transaction::ShelleyTxOut);
    gen_json_schema!(cml_chain::transaction::Transaction);
    gen_json_schema!(cml_chain::transaction::TransactionBody);
    gen_json_schema!(cml_chain::transaction::TransactionInput);
    gen_json_schema!(cml_chain::transaction::TransactionOutput);
    gen_json_schema!(cml_chain::transaction::TransactionWitnessSet);
    // utils.rs
    gen_json_schema!(cml_chain::utils::BigInt);
}
