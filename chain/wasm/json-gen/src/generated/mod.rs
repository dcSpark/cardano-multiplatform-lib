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
    gen_json_schema!(cml_chain::certs::AuthCommitteeHotCert);
    gen_json_schema!(cml_chain::certs::Certificate);
    gen_json_schema!(cml_chain::certs::Credential);
    gen_json_schema!(cml_chain::certs::DNSName);
    gen_json_schema!(cml_chain::certs::DRep);
    gen_json_schema!(cml_chain::certs::Ipv4);
    gen_json_schema!(cml_chain::certs::Ipv6);
    gen_json_schema!(cml_chain::certs::MultiHostName);
    gen_json_schema!(cml_chain::certs::PoolMetadata);
    gen_json_schema!(cml_chain::certs::PoolParams);
    gen_json_schema!(cml_chain::certs::PoolRegistration);
    gen_json_schema!(cml_chain::certs::PoolRetirement);
    gen_json_schema!(cml_chain::certs::RegCert);
    gen_json_schema!(cml_chain::certs::RegDrepCert);
    gen_json_schema!(cml_chain::certs::Relay);
    gen_json_schema!(cml_chain::certs::ResignCommitteeColdCert);
    gen_json_schema!(cml_chain::certs::SingleHostAddr);
    gen_json_schema!(cml_chain::certs::SingleHostName);
    gen_json_schema!(cml_chain::certs::StakeDelegation);
    gen_json_schema!(cml_chain::certs::StakeDeregistration);
    gen_json_schema!(cml_chain::certs::StakeRegDelegCert);
    gen_json_schema!(cml_chain::certs::StakeRegistration);
    gen_json_schema!(cml_chain::certs::StakeVoteDelegCert);
    gen_json_schema!(cml_chain::certs::StakeVoteRegDelegCert);
    gen_json_schema!(cml_chain::certs::UnregCert);
    gen_json_schema!(cml_chain::certs::UnregDrepCert);
    gen_json_schema!(cml_chain::certs::UpdateDrepCert);
    gen_json_schema!(cml_chain::certs::Url);
    gen_json_schema!(cml_chain::certs::VoteDelegCert);
    gen_json_schema!(cml_chain::certs::VoteRegDelegCert);
    // crypto
    gen_json_schema!(cml_chain::crypto::AnchorDocHash);
    gen_json_schema!(cml_chain::crypto::AuxiliaryDataHash);
    gen_json_schema!(cml_chain::crypto::BlockBodyHash);
    gen_json_schema!(cml_chain::crypto::BlockHeaderHash);
    gen_json_schema!(cml_chain::crypto::BootstrapWitness);
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
    gen_json_schema!(cml_chain::crypto::Vkeywitness);
    // governance
    gen_json_schema!(cml_chain::governance::Anchor);
    gen_json_schema!(cml_chain::governance::Constitution);
    gen_json_schema!(cml_chain::governance::GovAction);
    gen_json_schema!(cml_chain::governance::GovActionId);
    gen_json_schema!(cml_chain::governance::HardForkInitiationAction);
    gen_json_schema!(cml_chain::governance::NewConstitution);
    gen_json_schema!(cml_chain::governance::NoConfidence);
    gen_json_schema!(cml_chain::governance::ParameterChangeAction);
    gen_json_schema!(cml_chain::governance::ProposalProcedure);
    gen_json_schema!(cml_chain::governance::TreasuryWithdrawalsAction);
    gen_json_schema!(cml_chain::governance::UpdateCommittee);
    gen_json_schema!(cml_chain::governance::Vote);
    gen_json_schema!(cml_chain::governance::Voter);
    gen_json_schema!(cml_chain::governance::VotingProcedure);
    // lib
    gen_json_schema!(cml_chain::DRepVotingThresholds);
    gen_json_schema!(cml_chain::Int);
    gen_json_schema!(cml_chain::NetworkId);
    gen_json_schema!(cml_chain::NonemptySetBootstrapWitness);
    gen_json_schema!(cml_chain::NonemptySetCertificate);
    gen_json_schema!(cml_chain::NonemptySetNativeScript);
    gen_json_schema!(cml_chain::NonemptySetPlutusData);
    gen_json_schema!(cml_chain::NonemptySetPlutusV1Script);
    gen_json_schema!(cml_chain::NonemptySetPlutusV2Script);
    gen_json_schema!(cml_chain::NonemptySetPlutusV3Script);
    gen_json_schema!(cml_chain::NonemptySetProposalProcedure);
    gen_json_schema!(cml_chain::NonemptySetTransactionInput);
    gen_json_schema!(cml_chain::NonemptySetVkeywitness);
    gen_json_schema!(cml_chain::PoolVotingThresholds);
    gen_json_schema!(cml_chain::ProtocolParamUpdate);
    gen_json_schema!(cml_chain::Rational);
    gen_json_schema!(cml_chain::Script);
    gen_json_schema!(cml_chain::SetEd25519KeyHash);
    gen_json_schema!(cml_chain::SetTransactionInput);
    gen_json_schema!(cml_chain::UnitInterval);
    gen_json_schema!(cml_chain::Value);
    gen_json_schema!(cml_chain::crypto::Vkeywitness);
    // plutus
    //gen_json_schema!(cml_chain::plutus::ConstrPlutusData);
    gen_json_schema!(cml_chain::plutus::CostModels);
    gen_json_schema!(cml_chain::plutus::ExUnitPrices);
    gen_json_schema!(cml_chain::plutus::ExUnits);
    gen_json_schema!(cml_chain::plutus::Language);
    gen_json_schema!(cml_chain::plutus::LegacyRedeemer);
    //gen_json_schema!(cml_chain::plutus::PlutusData);
    //gen_json_schema!(cml_chain::plutus::PlutusMap);
    gen_json_schema!(cml_chain::plutus::PlutusV1Script);
    gen_json_schema!(cml_chain::plutus::PlutusV2Script);
    gen_json_schema!(cml_chain::plutus::PlutusV3Script);
    gen_json_schema!(cml_chain::plutus::RedeemerKey);
    gen_json_schema!(cml_chain::plutus::RedeemerTag);
    gen_json_schema!(cml_chain::plutus::RedeemerVal);
    gen_json_schema!(cml_chain::plutus::Redeemers);
    // transaction
    gen_json_schema!(cml_chain::transaction::AlonzoFormatTxOut);
    gen_json_schema!(cml_chain::transaction::ConwayFormatTxOut);
    gen_json_schema!(cml_chain::transaction::DatumOption);
    gen_json_schema!(cml_chain::transaction::NativeScript);
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
