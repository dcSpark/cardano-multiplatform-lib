use cml_core::json_schema_gen::Registrar;
use cml_core::json_schema_gen::check_schema_ref_closure;

pub fn add_schemas(generator: &mut schemars::SchemaGenerator) {
    let mut reg = Registrar::new(generator);
    // address
    reg.add::<cml_chain::address::Address>();
    reg.add::<cml_chain::address::RewardAccount>();
    // assets
    reg.add::<cml_chain::assets::AssetName>();
    reg.add::<cml_chain::assets::Value>();
    // auxdata
    reg.add::<cml_chain::auxdata::AuxiliaryData>();
    reg.add::<cml_chain::auxdata::ConwayFormatAuxData>();
    reg.add::<cml_chain::auxdata::Metadata>();
    reg.add::<cml_chain::auxdata::ShelleyMAFormatAuxData>();
    // block
    reg.add::<cml_chain::block::Block>();
    reg.add::<cml_chain::block::Header>();
    reg.add::<cml_chain::block::HeaderBody>();
    reg.add::<cml_chain::block::OperationalCert>();
    reg.add::<cml_chain::block::ProtocolVersion>();
    // certs
    reg.add::<cml_chain::certs::AuthCommitteeHotCert>();
    reg.add::<cml_chain::certs::Certificate>();
    reg.add::<cml_chain::certs::Credential>();
    reg.add::<cml_chain::certs::DNSName>();
    reg.add::<cml_chain::certs::DRep>();
    reg.add::<cml_chain::certs::Ipv4>();
    reg.add::<cml_chain::certs::Ipv6>();
    reg.add::<cml_chain::certs::MultiHostName>();
    reg.add::<cml_chain::certs::PoolMetadata>();
    reg.add::<cml_chain::certs::PoolParams>();
    reg.add::<cml_chain::certs::PoolRegistration>();
    reg.add::<cml_chain::certs::PoolRetirement>();
    reg.add::<cml_chain::certs::RegCert>();
    reg.add::<cml_chain::certs::RegDrepCert>();
    reg.add::<cml_chain::certs::Relay>();
    reg.add::<cml_chain::certs::ResignCommitteeColdCert>();
    reg.add::<cml_chain::certs::SingleHostAddr>();
    reg.add::<cml_chain::certs::SingleHostName>();
    reg.add::<cml_chain::certs::StakeDelegation>();
    reg.add::<cml_chain::certs::StakeDeregistration>();
    reg.add::<cml_chain::certs::StakeRegDelegCert>();
    reg.add::<cml_chain::certs::StakeRegistration>();
    reg.add::<cml_chain::certs::StakeVoteDelegCert>();
    reg.add::<cml_chain::certs::StakeVoteRegDelegCert>();
    reg.add::<cml_chain::certs::UnregCert>();
    reg.add::<cml_chain::certs::UnregDrepCert>();
    reg.add::<cml_chain::certs::UpdateDrepCert>();
    reg.add::<cml_chain::certs::Url>();
    reg.add::<cml_chain::certs::VoteDelegCert>();
    reg.add::<cml_chain::certs::VoteRegDelegCert>();
    // crypto
    reg.add::<cml_chain::crypto::AddrAttributes>();
    reg.add::<cml_chain::crypto::AnchorDocHash>();
    reg.add::<cml_chain::crypto::AuxiliaryDataHash>();
    reg.add::<cml_chain::crypto::BlockBodyHash>();
    reg.add::<cml_chain::crypto::BlockHeaderHash>();
    reg.add::<cml_chain::crypto::BootstrapWitness>();
    reg.add::<cml_chain::crypto::DatumHash>();
    reg.add::<cml_chain::crypto::Ed25519KeyHash>();
    reg.add::<cml_chain::crypto::Ed25519Signature>();
    reg.add::<cml_chain::crypto::GenesisDelegateHash>();
    reg.add::<cml_chain::crypto::GenesisHash>();
    reg.add::<cml_chain::crypto::KESSignature>();
    reg.add::<cml_chain::crypto::KESVkey>();
    reg.add::<cml_chain::crypto::Nonce>();
    reg.add::<cml_chain::crypto::NonceHash>();
    reg.add::<cml_chain::crypto::PoolMetadataHash>();
    reg.add::<cml_chain::crypto::ScriptDataHash>();
    reg.add::<cml_chain::crypto::ScriptHash>();
    reg.add::<cml_chain::crypto::TransactionHash>();
    reg.add::<cml_chain::crypto::VRFCert>();
    reg.add::<cml_chain::crypto::VRFKeyHash>();
    reg.add::<cml_chain::crypto::VRFVkey>();
    reg.add::<cml_chain::crypto::Vkey>();
    reg.add::<cml_chain::crypto::Vkeywitness>();
    // governance
    reg.add::<cml_chain::governance::Anchor>();
    reg.add::<cml_chain::governance::Constitution>();
    reg.add::<cml_chain::governance::GovAction>();
    reg.add::<cml_chain::governance::GovActionId>();
    reg.add::<cml_chain::governance::HardForkInitiationAction>();
    reg.add::<cml_chain::governance::NewConstitution>();
    reg.add::<cml_chain::governance::NoConfidence>();
    reg.add::<cml_chain::governance::ParameterChangeAction>();
    reg.add::<cml_chain::governance::ProposalProcedure>();
    reg.add::<cml_chain::governance::TreasuryWithdrawalsAction>();
    reg.add::<cml_chain::governance::UpdateCommittee>();
    reg.add::<cml_chain::governance::Vote>();
    reg.add::<cml_chain::governance::Voter>();
    reg.add::<cml_chain::governance::VotingProcedure>();
    // lib
    reg.add::<cml_chain::DRepVotingThresholds>();
    reg.add::<cml_chain::Int>();
    reg.add::<cml_chain::NetworkId>();
    reg.add::<cml_chain::NonemptySetBootstrapWitness>();
    reg.add::<cml_chain::NonemptySetCertificate>();
    reg.add::<cml_chain::NonemptySetEd25519KeyHash>();
    reg.add::<cml_chain::NonemptySetNativeScript>();
    reg.add::<cml_chain::NonemptySetPlutusData>();
    reg.add::<cml_chain::NonemptySetPlutusV1Script>();
    reg.add::<cml_chain::NonemptySetPlutusV2Script>();
    reg.add::<cml_chain::NonemptySetPlutusV3Script>();
    reg.add::<cml_chain::NonemptySetProposalProcedure>();
    reg.add::<cml_chain::NonemptySetTransactionInput>();
    reg.add::<cml_chain::NonemptySetVkeywitness>();
    reg.add::<cml_chain::PoolVotingThresholds>();
    reg.add::<cml_chain::ProtocolParamUpdate>();
    reg.add::<cml_chain::Rational>();
    reg.add::<cml_chain::Script>();
    reg.add::<cml_chain::SetCommitteeColdCredential>();
    reg.add::<cml_chain::SetEd25519KeyHash>();
    reg.add::<cml_chain::SetTransactionInput>();
    reg.add::<cml_chain::UnitInterval>();
    // plutus
    reg.add::<cml_chain::plutus::BigInteger>();
    reg.add::<cml_chain::plutus::ConstrPlutusData>();
    reg.add::<cml_chain::plutus::CostModels>();
    reg.add::<cml_chain::plutus::ExUnitPrices>();
    reg.add::<cml_chain::plutus::ExUnits>();
    reg.add::<cml_chain::plutus::Language>();
    reg.add::<cml_chain::plutus::LegacyRedeemer>();
    reg.add::<cml_chain::plutus::PlutusData>();
    reg.add::<cml_chain::plutus::PlutusV1Script>();
    reg.add::<cml_chain::plutus::PlutusV2Script>();
    reg.add::<cml_chain::plutus::PlutusV3Script>();
    reg.add::<cml_chain::plutus::RedeemerKey>();
    reg.add::<cml_chain::plutus::RedeemerTag>();
    reg.add::<cml_chain::plutus::RedeemerVal>();
    reg.add::<cml_chain::plutus::Redeemers>();
    // transaction
    reg.add::<cml_chain::transaction::AlonzoFormatTxOut>();
    reg.add::<cml_chain::transaction::ConwayFormatTxOut>();
    reg.add::<cml_chain::transaction::DatumOption>();
    reg.add::<cml_chain::transaction::NativeScript>();
    reg.add::<cml_chain::transaction::ScriptAll>();
    reg.add::<cml_chain::transaction::ScriptAny>();
    reg.add::<cml_chain::transaction::ScriptInvalidBefore>();
    reg.add::<cml_chain::transaction::ScriptInvalidHereafter>();
    reg.add::<cml_chain::transaction::ScriptNOfK>();
    reg.add::<cml_chain::transaction::ScriptPubkey>();
    reg.add::<cml_chain::transaction::ScriptRef>();
    reg.add::<cml_chain::transaction::Transaction>();
    reg.add::<cml_chain::transaction::TransactionBody>();
    reg.add::<cml_chain::transaction::TransactionInput>();
    reg.add::<cml_chain::transaction::TransactionOutput>();
    reg.add::<cml_chain::transaction::TransactionWitnessSet>();
    reg.add::<cml_chain::byron::AddressContent>();
    reg.add::<cml_chain::byron::ByronAddress>();
    reg.add::<cml_chain::byron::ByronAddrType>();
    reg.add::<cml_chain::byron::ByronTxOut>();
    reg.add::<cml_chain::byron::Crc32>();
    reg.add::<cml_chain::byron::SpendingData>();
    reg.add::<cml_chain::byron::StakeholderId>();
    reg.add::<cml_crypto::Bip32PublicKey>();
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
    document.insert("title".to_owned(), "cml_chain".into());
    document.insert("$defs".to_owned(), generator.take_definitions(true).into());
    let document = serde_json::Value::Object(document);
    check_schema_ref_closure(&document, &definitions_path);
    std::fs::write(
        schema_path.join("cml_chain.schema.json"),
        serde_json::to_string_pretty(&document).unwrap(),
    )
    .unwrap();
}
