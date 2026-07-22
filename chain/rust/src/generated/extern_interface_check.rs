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
fn _assert_raw_bytes<T: cml_core::serialization::RawBytesEncoding>() {}
#[allow(dead_code)]
fn _assert_copy<T: Copy>() {}
#[allow(dead_code)]
fn _assert_serialize_embedded_group<T: cml_core::serialization::SerializeEmbeddedGroup>() {}
#[allow(dead_code)]
fn _assert_deserialize_embedded_group<T: cml_core::serialization::DeserializeEmbeddedGroup>() {}
#[allow(unused_imports)]
use crate::generated::DeltaCoin as _;
#[allow(unused_imports)]
use crate::generated::Epoch as _;
#[allow(unused_imports)]
use crate::generated::PolicyId as _;
#[allow(unused_imports)]
use crate::generated::Port as _;
#[allow(unused_imports)]
use crate::generated::Slot as _;
#[allow(unused_imports)]
use crate::generated::SubCoin as _;
#[allow(unused_imports)]
use crate::generated::TransactionIndex as _;
#[allow(unused_imports)]
use crate::generated::TransactionMetadatumLabel as _;
#[allow(unused_imports)]
use crate::generated::Withdrawals as _;
#[allow(unused_imports)]
use crate::generated::assets::Coin as _;
#[allow(unused_imports)]
use crate::generated::assets::Mint as _;
#[allow(unused_imports)]
use crate::generated::assets::MultiAsset as _;
#[allow(unused_imports)]
use crate::generated::assets::NonZeroInt64 as _;
#[allow(unused_imports)]
use crate::generated::assets::PositiveCoin as _;
#[allow(unused_imports)]
use crate::generated::auxdata::ShelleyFormatAuxData as _;
#[allow(unused_imports)]
use crate::generated::certs::CommitteeColdCredential as _;
#[allow(unused_imports)]
use crate::generated::certs::CommitteeHotCredential as _;
#[allow(unused_imports)]
use crate::generated::certs::DrepCredential as _;
#[allow(unused_imports)]
use crate::generated::certs::StakeCredential as _;
#[allow(unused_imports)]
use crate::generated::governance::Vote as _;
#[allow(unused_imports)]
use crate::generated::governance::VotingProcedures as _;
#[allow(unused_imports)]
use crate::generated::plutus::Language as _;
#[allow(unused_imports)]
use crate::generated::plutus::RedeemerTag as _;
#[allow(dead_code)]
fn _extern_interface_self_check() {
    _assert_serialize::<crate::generated::crypto::AddrAttributes>();
    _assert_deserialize::<crate::generated::crypto::AddrAttributes>();
    _assert_serialize::<crate::generated::address::Address>();
    _assert_deserialize::<crate::generated::address::Address>();
    _assert_serialize::<crate::generated::transaction::AlonzoFormatTxOut>();
    _assert_deserialize::<crate::generated::transaction::AlonzoFormatTxOut>();
    _assert_serialize::<crate::generated::governance::Anchor>();
    _assert_deserialize::<crate::generated::governance::Anchor>();
    _assert_raw_bytes::<crate::generated::crypto::AnchorDocHash>();
    _assert_copy::<crate::generated::crypto::AnchorDocHash>();
    _assert_serialize::<crate::generated::assets::AssetName>();
    _assert_deserialize::<crate::generated::assets::AssetName>();
    _assert_serialize::<crate::generated::certs::AuthCommitteeHotCert>();
    _assert_serialize_embedded_group::<crate::generated::certs::AuthCommitteeHotCert>();
    _assert_deserialize::<crate::generated::certs::AuthCommitteeHotCert>();
    _assert_deserialize_embedded_group::<crate::generated::certs::AuthCommitteeHotCert>();
    _assert_serialize::<crate::generated::auxdata::AuxiliaryData>();
    _assert_deserialize::<crate::generated::auxdata::AuxiliaryData>();
    _assert_raw_bytes::<crate::generated::crypto::AuxiliaryDataHash>();
    _assert_copy::<crate::generated::crypto::AuxiliaryDataHash>();
    _assert_serialize::<crate::generated::plutus::BigInteger>();
    _assert_deserialize::<crate::generated::plutus::BigInteger>();
    _assert_serialize::<crate::generated::block::Block>();
    _assert_deserialize::<crate::generated::block::Block>();
    _assert_raw_bytes::<crate::generated::crypto::BlockBodyHash>();
    _assert_copy::<crate::generated::crypto::BlockBodyHash>();
    _assert_raw_bytes::<crate::generated::crypto::BlockHeaderHash>();
    _assert_copy::<crate::generated::crypto::BlockHeaderHash>();
    _assert_serialize::<crate::generated::crypto::BootstrapWitness>();
    _assert_deserialize::<crate::generated::crypto::BootstrapWitness>();
    _assert_serialize::<crate::generated::certs::Certificate>();
    _assert_deserialize::<crate::generated::certs::Certificate>();
    _assert_serialize::<crate::generated::governance::Constitution>();
    _assert_deserialize::<crate::generated::governance::Constitution>();
    _assert_serialize::<crate::generated::plutus::ConstrPlutusData>();
    _assert_deserialize::<crate::generated::plutus::ConstrPlutusData>();
    _assert_serialize::<crate::generated::auxdata::ConwayFormatAuxData>();
    _assert_deserialize::<crate::generated::auxdata::ConwayFormatAuxData>();
    _assert_serialize::<crate::generated::transaction::ConwayFormatTxOut>();
    _assert_deserialize::<crate::generated::transaction::ConwayFormatTxOut>();
    _assert_serialize::<crate::generated::plutus::CostModels>();
    _assert_deserialize::<crate::generated::plutus::CostModels>();
    _assert_serialize::<crate::generated::certs::Credential>();
    _assert_deserialize::<crate::generated::certs::Credential>();
    _assert_serialize::<crate::generated::certs::DNSName>();
    _assert_deserialize::<crate::generated::certs::DNSName>();
    _assert_serialize::<crate::generated::certs::DRep>();
    _assert_deserialize::<crate::generated::certs::DRep>();
    _assert_serialize::<crate::generated::DRepVotingThresholds>();
    _assert_deserialize::<crate::generated::DRepVotingThresholds>();
    _assert_raw_bytes::<crate::generated::crypto::DatumHash>();
    _assert_copy::<crate::generated::crypto::DatumHash>();
    _assert_serialize::<crate::generated::transaction::DatumOption>();
    _assert_deserialize::<crate::generated::transaction::DatumOption>();
    _assert_raw_bytes::<crate::generated::crypto::Ed25519KeyHash>();
    _assert_copy::<crate::generated::crypto::Ed25519KeyHash>();
    _assert_raw_bytes::<crate::generated::crypto::Ed25519Signature>();
    _assert_serialize::<crate::generated::plutus::ExUnitPrices>();
    _assert_deserialize::<crate::generated::plutus::ExUnitPrices>();
    _assert_serialize::<crate::generated::plutus::ExUnits>();
    _assert_deserialize::<crate::generated::plutus::ExUnits>();
    _assert_raw_bytes::<crate::generated::crypto::GenesisDelegateHash>();
    _assert_copy::<crate::generated::crypto::GenesisDelegateHash>();
    _assert_raw_bytes::<crate::generated::crypto::GenesisHash>();
    _assert_copy::<crate::generated::crypto::GenesisHash>();
    _assert_serialize::<crate::generated::governance::GovAction>();
    _assert_deserialize::<crate::generated::governance::GovAction>();
    _assert_serialize::<crate::generated::governance::GovActionId>();
    _assert_deserialize::<crate::generated::governance::GovActionId>();
    _assert_serialize::<crate::generated::governance::HardForkInitiationAction>();
    _assert_serialize_embedded_group::<crate::generated::governance::HardForkInitiationAction>();
    _assert_deserialize::<crate::generated::governance::HardForkInitiationAction>();
    _assert_deserialize_embedded_group::<crate::generated::governance::HardForkInitiationAction>();
    _assert_serialize::<crate::generated::block::Header>();
    _assert_deserialize::<crate::generated::block::Header>();
    _assert_serialize::<crate::generated::block::HeaderBody>();
    _assert_deserialize::<crate::generated::block::HeaderBody>();
    _assert_serialize::<crate::generated::certs::Ipv4>();
    _assert_deserialize::<crate::generated::certs::Ipv4>();
    _assert_serialize::<crate::generated::certs::Ipv6>();
    _assert_deserialize::<crate::generated::certs::Ipv6>();
    _assert_serialize::<crate::generated::crypto::KESSignature>();
    _assert_deserialize::<crate::generated::crypto::KESSignature>();
    _assert_raw_bytes::<crate::generated::crypto::KESVkey>();
    _assert_copy::<crate::generated::crypto::KESVkey>();
    _assert_serialize::<crate::generated::plutus::LegacyRedeemer>();
    _assert_deserialize::<crate::generated::plutus::LegacyRedeemer>();
    _assert_serialize::<crate::generated::auxdata::Metadata>();
    _assert_deserialize::<crate::generated::auxdata::Metadata>();
    _assert_serialize::<crate::generated::certs::MultiHostName>();
    _assert_serialize_embedded_group::<crate::generated::certs::MultiHostName>();
    _assert_deserialize::<crate::generated::certs::MultiHostName>();
    _assert_deserialize_embedded_group::<crate::generated::certs::MultiHostName>();
    _assert_serialize::<crate::generated::transaction::NativeScript>();
    _assert_deserialize::<crate::generated::transaction::NativeScript>();
    _assert_serialize::<crate::generated::NetworkId>();
    _assert_deserialize::<crate::generated::NetworkId>();
    _assert_serialize::<crate::generated::governance::NewConstitution>();
    _assert_serialize_embedded_group::<crate::generated::governance::NewConstitution>();
    _assert_deserialize::<crate::generated::governance::NewConstitution>();
    _assert_deserialize_embedded_group::<crate::generated::governance::NewConstitution>();
    _assert_serialize::<crate::generated::governance::NoConfidence>();
    _assert_serialize_embedded_group::<crate::generated::governance::NoConfidence>();
    _assert_deserialize::<crate::generated::governance::NoConfidence>();
    _assert_deserialize_embedded_group::<crate::generated::governance::NoConfidence>();
    _assert_serialize::<crate::generated::crypto::Nonce>();
    _assert_deserialize::<crate::generated::crypto::Nonce>();
    _assert_raw_bytes::<crate::generated::crypto::NonceHash>();
    _assert_copy::<crate::generated::crypto::NonceHash>();
    _assert_serialize::<crate::generated::block::OperationalCert>();
    _assert_serialize_embedded_group::<crate::generated::block::OperationalCert>();
    _assert_deserialize::<crate::generated::block::OperationalCert>();
    _assert_deserialize_embedded_group::<crate::generated::block::OperationalCert>();
    _assert_serialize::<crate::generated::governance::ParameterChangeAction>();
    _assert_serialize_embedded_group::<crate::generated::governance::ParameterChangeAction>();
    _assert_deserialize::<crate::generated::governance::ParameterChangeAction>();
    _assert_deserialize_embedded_group::<crate::generated::governance::ParameterChangeAction>();
    _assert_serialize::<crate::generated::plutus::PlutusData>();
    _assert_deserialize::<crate::generated::plutus::PlutusData>();
    _assert_serialize::<crate::generated::plutus::PlutusMap>();
    _assert_deserialize::<crate::generated::plutus::PlutusMap>();
    _assert_serialize::<crate::generated::plutus::PlutusV1Script>();
    _assert_deserialize::<crate::generated::plutus::PlutusV1Script>();
    _assert_serialize::<crate::generated::plutus::PlutusV2Script>();
    _assert_deserialize::<crate::generated::plutus::PlutusV2Script>();
    _assert_serialize::<crate::generated::plutus::PlutusV3Script>();
    _assert_deserialize::<crate::generated::plutus::PlutusV3Script>();
    _assert_serialize::<crate::generated::certs::PoolMetadata>();
    _assert_deserialize::<crate::generated::certs::PoolMetadata>();
    _assert_raw_bytes::<crate::generated::crypto::PoolMetadataHash>();
    _assert_copy::<crate::generated::crypto::PoolMetadataHash>();
    _assert_serialize::<crate::generated::certs::PoolRetirement>();
    _assert_serialize_embedded_group::<crate::generated::certs::PoolRetirement>();
    _assert_deserialize::<crate::generated::certs::PoolRetirement>();
    _assert_deserialize_embedded_group::<crate::generated::certs::PoolRetirement>();
    _assert_serialize::<crate::generated::PoolVotingThresholds>();
    _assert_deserialize::<crate::generated::PoolVotingThresholds>();
    _assert_serialize::<crate::generated::governance::ProposalProcedure>();
    _assert_deserialize::<crate::generated::governance::ProposalProcedure>();
    _assert_serialize::<crate::generated::ProtocolParamUpdate>();
    _assert_deserialize::<crate::generated::ProtocolParamUpdate>();
    _assert_serialize::<crate::generated::block::ProtocolVersion>();
    _assert_serialize_embedded_group::<crate::generated::block::ProtocolVersion>();
    _assert_deserialize::<crate::generated::block::ProtocolVersion>();
    _assert_deserialize_embedded_group::<crate::generated::block::ProtocolVersion>();
    _assert_serialize::<crate::generated::Rational>();
    _assert_deserialize::<crate::generated::Rational>();
    _assert_serialize::<crate::generated::plutus::RedeemerKey>();
    _assert_deserialize::<crate::generated::plutus::RedeemerKey>();
    _assert_serialize::<crate::generated::plutus::RedeemerVal>();
    _assert_deserialize::<crate::generated::plutus::RedeemerVal>();
    _assert_serialize::<crate::generated::plutus::Redeemers>();
    _assert_deserialize::<crate::generated::plutus::Redeemers>();
    _assert_serialize::<crate::generated::certs::RegCert>();
    _assert_serialize_embedded_group::<crate::generated::certs::RegCert>();
    _assert_deserialize::<crate::generated::certs::RegCert>();
    _assert_deserialize_embedded_group::<crate::generated::certs::RegCert>();
    _assert_serialize::<crate::generated::certs::RegDrepCert>();
    _assert_serialize_embedded_group::<crate::generated::certs::RegDrepCert>();
    _assert_deserialize::<crate::generated::certs::RegDrepCert>();
    _assert_deserialize_embedded_group::<crate::generated::certs::RegDrepCert>();
    _assert_serialize::<crate::generated::certs::Relay>();
    _assert_deserialize::<crate::generated::certs::Relay>();
    _assert_serialize::<crate::generated::transaction::RequiredSigners>();
    _assert_deserialize::<crate::generated::transaction::RequiredSigners>();
    _assert_serialize::<crate::generated::certs::ResignCommitteeColdCert>();
    _assert_serialize_embedded_group::<crate::generated::certs::ResignCommitteeColdCert>();
    _assert_deserialize::<crate::generated::certs::ResignCommitteeColdCert>();
    _assert_deserialize_embedded_group::<crate::generated::certs::ResignCommitteeColdCert>();
    _assert_serialize::<crate::generated::address::RewardAccount>();
    _assert_deserialize::<crate::generated::address::RewardAccount>();
    _assert_serialize::<crate::generated::transaction::ScriptAll>();
    _assert_serialize_embedded_group::<crate::generated::transaction::ScriptAll>();
    _assert_deserialize::<crate::generated::transaction::ScriptAll>();
    _assert_deserialize_embedded_group::<crate::generated::transaction::ScriptAll>();
    _assert_serialize::<crate::generated::transaction::ScriptAny>();
    _assert_serialize_embedded_group::<crate::generated::transaction::ScriptAny>();
    _assert_deserialize::<crate::generated::transaction::ScriptAny>();
    _assert_deserialize_embedded_group::<crate::generated::transaction::ScriptAny>();
    _assert_raw_bytes::<crate::generated::crypto::ScriptDataHash>();
    _assert_copy::<crate::generated::crypto::ScriptDataHash>();
    _assert_raw_bytes::<crate::generated::crypto::ScriptHash>();
    _assert_copy::<crate::generated::crypto::ScriptHash>();
    _assert_serialize::<crate::generated::transaction::ScriptInvalidBefore>();
    _assert_serialize_embedded_group::<crate::generated::transaction::ScriptInvalidBefore>();
    _assert_deserialize::<crate::generated::transaction::ScriptInvalidBefore>();
    _assert_deserialize_embedded_group::<crate::generated::transaction::ScriptInvalidBefore>();
    _assert_serialize::<crate::generated::transaction::ScriptInvalidHereafter>();
    _assert_serialize_embedded_group::<crate::generated::transaction::ScriptInvalidHereafter>();
    _assert_deserialize::<crate::generated::transaction::ScriptInvalidHereafter>();
    _assert_deserialize_embedded_group::<crate::generated::transaction::ScriptInvalidHereafter>();
    _assert_serialize::<crate::generated::transaction::ScriptNOfK>();
    _assert_serialize_embedded_group::<crate::generated::transaction::ScriptNOfK>();
    _assert_deserialize::<crate::generated::transaction::ScriptNOfK>();
    _assert_deserialize_embedded_group::<crate::generated::transaction::ScriptNOfK>();
    _assert_serialize::<crate::generated::transaction::ScriptPubkey>();
    _assert_serialize_embedded_group::<crate::generated::transaction::ScriptPubkey>();
    _assert_deserialize::<crate::generated::transaction::ScriptPubkey>();
    _assert_deserialize_embedded_group::<crate::generated::transaction::ScriptPubkey>();
    _assert_serialize::<crate::generated::transaction::ScriptRef>();
    _assert_deserialize::<crate::generated::transaction::ScriptRef>();
    _assert_serialize::<crate::generated::auxdata::ShelleyMAFormatAuxData>();
    _assert_deserialize::<crate::generated::auxdata::ShelleyMAFormatAuxData>();
    _assert_serialize::<crate::generated::certs::SingleHostAddr>();
    _assert_serialize_embedded_group::<crate::generated::certs::SingleHostAddr>();
    _assert_deserialize::<crate::generated::certs::SingleHostAddr>();
    _assert_deserialize_embedded_group::<crate::generated::certs::SingleHostAddr>();
    _assert_serialize::<crate::generated::certs::SingleHostName>();
    _assert_serialize_embedded_group::<crate::generated::certs::SingleHostName>();
    _assert_deserialize::<crate::generated::certs::SingleHostName>();
    _assert_deserialize_embedded_group::<crate::generated::certs::SingleHostName>();
    _assert_serialize::<crate::generated::certs::StakeDelegation>();
    _assert_serialize_embedded_group::<crate::generated::certs::StakeDelegation>();
    _assert_deserialize::<crate::generated::certs::StakeDelegation>();
    _assert_deserialize_embedded_group::<crate::generated::certs::StakeDelegation>();
    _assert_serialize::<crate::generated::certs::StakeDeregistration>();
    _assert_serialize_embedded_group::<crate::generated::certs::StakeDeregistration>();
    _assert_deserialize::<crate::generated::certs::StakeDeregistration>();
    _assert_deserialize_embedded_group::<crate::generated::certs::StakeDeregistration>();
    _assert_serialize::<crate::generated::certs::StakeRegDelegCert>();
    _assert_serialize_embedded_group::<crate::generated::certs::StakeRegDelegCert>();
    _assert_deserialize::<crate::generated::certs::StakeRegDelegCert>();
    _assert_deserialize_embedded_group::<crate::generated::certs::StakeRegDelegCert>();
    _assert_serialize::<crate::generated::certs::StakeRegistration>();
    _assert_serialize_embedded_group::<crate::generated::certs::StakeRegistration>();
    _assert_deserialize::<crate::generated::certs::StakeRegistration>();
    _assert_deserialize_embedded_group::<crate::generated::certs::StakeRegistration>();
    _assert_serialize::<crate::generated::certs::StakeVoteDelegCert>();
    _assert_serialize_embedded_group::<crate::generated::certs::StakeVoteDelegCert>();
    _assert_deserialize::<crate::generated::certs::StakeVoteDelegCert>();
    _assert_deserialize_embedded_group::<crate::generated::certs::StakeVoteDelegCert>();
    _assert_serialize::<crate::generated::certs::StakeVoteRegDelegCert>();
    _assert_serialize_embedded_group::<crate::generated::certs::StakeVoteRegDelegCert>();
    _assert_deserialize::<crate::generated::certs::StakeVoteRegDelegCert>();
    _assert_deserialize_embedded_group::<crate::generated::certs::StakeVoteRegDelegCert>();
    _assert_serialize::<crate::generated::transaction::Transaction>();
    _assert_deserialize::<crate::generated::transaction::Transaction>();
    _assert_serialize::<crate::generated::transaction::TransactionBody>();
    _assert_deserialize::<crate::generated::transaction::TransactionBody>();
    _assert_raw_bytes::<crate::generated::crypto::TransactionHash>();
    _assert_copy::<crate::generated::crypto::TransactionHash>();
    _assert_serialize::<crate::generated::transaction::TransactionInput>();
    _assert_deserialize::<crate::generated::transaction::TransactionInput>();
    _assert_serialize::<crate::generated::transaction::TransactionOutput>();
    _assert_deserialize::<crate::generated::transaction::TransactionOutput>();
    _assert_serialize::<crate::generated::transaction::TransactionWitnessSet>();
    _assert_deserialize::<crate::generated::transaction::TransactionWitnessSet>();
    _assert_serialize::<crate::generated::governance::TreasuryWithdrawalsAction>();
    _assert_serialize_embedded_group::<crate::generated::governance::TreasuryWithdrawalsAction>();
    _assert_deserialize::<crate::generated::governance::TreasuryWithdrawalsAction>();
    _assert_deserialize_embedded_group::<crate::generated::governance::TreasuryWithdrawalsAction>();
    _assert_serialize::<crate::generated::UnitInterval>();
    _assert_deserialize::<crate::generated::UnitInterval>();
    _assert_serialize::<crate::generated::certs::UnregCert>();
    _assert_serialize_embedded_group::<crate::generated::certs::UnregCert>();
    _assert_deserialize::<crate::generated::certs::UnregCert>();
    _assert_deserialize_embedded_group::<crate::generated::certs::UnregCert>();
    _assert_serialize::<crate::generated::certs::UnregDrepCert>();
    _assert_serialize_embedded_group::<crate::generated::certs::UnregDrepCert>();
    _assert_deserialize::<crate::generated::certs::UnregDrepCert>();
    _assert_deserialize_embedded_group::<crate::generated::certs::UnregDrepCert>();
    _assert_serialize::<crate::generated::certs::UpdateDrepCert>();
    _assert_serialize_embedded_group::<crate::generated::certs::UpdateDrepCert>();
    _assert_deserialize::<crate::generated::certs::UpdateDrepCert>();
    _assert_deserialize_embedded_group::<crate::generated::certs::UpdateDrepCert>();
    _assert_serialize::<crate::generated::certs::Url>();
    _assert_deserialize::<crate::generated::certs::Url>();
    _assert_serialize::<crate::generated::crypto::VRFCert>();
    _assert_deserialize::<crate::generated::crypto::VRFCert>();
    _assert_raw_bytes::<crate::generated::crypto::VRFKeyHash>();
    _assert_copy::<crate::generated::crypto::VRFKeyHash>();
    _assert_raw_bytes::<crate::generated::crypto::VRFVkey>();
    _assert_copy::<crate::generated::crypto::VRFVkey>();
    _assert_serialize::<crate::generated::assets::Value>();
    _assert_deserialize::<crate::generated::assets::Value>();
    _assert_raw_bytes::<crate::generated::crypto::Vkey>();
    _assert_serialize::<crate::generated::crypto::Vkeywitness>();
    _assert_deserialize::<crate::generated::crypto::Vkeywitness>();
    _assert_serialize::<crate::generated::certs::VoteDelegCert>();
    _assert_serialize_embedded_group::<crate::generated::certs::VoteDelegCert>();
    _assert_deserialize::<crate::generated::certs::VoteDelegCert>();
    _assert_deserialize_embedded_group::<crate::generated::certs::VoteDelegCert>();
    _assert_serialize::<crate::generated::certs::VoteRegDelegCert>();
    _assert_serialize_embedded_group::<crate::generated::certs::VoteRegDelegCert>();
    _assert_deserialize::<crate::generated::certs::VoteRegDelegCert>();
    _assert_deserialize_embedded_group::<crate::generated::certs::VoteRegDelegCert>();
    _assert_serialize::<crate::generated::governance::Voter>();
    _assert_deserialize::<crate::generated::governance::Voter>();
    _assert_serialize::<crate::generated::governance::VotingProcedure>();
    _assert_deserialize::<crate::generated::governance::VotingProcedure>();
}
