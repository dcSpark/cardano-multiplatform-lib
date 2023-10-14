#![allow(
    clippy::len_without_is_empty,
    clippy::too_many_arguments,
    clippy::new_without_default
)]

use ::wasm_bindgen::prelude::{wasm_bindgen, JsError, JsValue};
use cml_core_wasm::metadata::TransactionMetadatumList;
use cml_core_wasm::{impl_wasm_cbor_json_api, impl_wasm_conversions};

pub use cml_core_wasm::Int;

pub mod address;
pub mod assets;
pub mod auxdata;
pub mod block;
pub mod builders;
pub mod byron;
pub mod certs;
pub mod crypto;
pub mod fees;
pub mod governance;
pub mod plutus;
pub mod transaction;
pub mod utils;

use address::RewardAccount;
use assets::AssetName;
pub use assets::Value;
use auxdata::{AuxiliaryData, TransactionMetadatum};

use certs::{Certificate, Relay, StakeCredential};
use cml_chain::assets::NonZeroInt64;
pub use cml_chain::{assets::Coin, Epoch};
use cml_core::ordered_hash_map::OrderedHashMap;
use cml_crypto_wasm::{Ed25519KeyHash, ScriptHash};
use crypto::{BootstrapWitness, Vkeywitness};
use governance::{GovActionId, Voter};
use plutus::{
    CostModels, ExUnitPrices, ExUnits, PlutusData, PlutusV1Script, PlutusV2Script, PlutusV3Script,
    Redeemer,
};
use transaction::{
    NativeScript, TransactionBody, TransactionInput, TransactionOutput, TransactionWitnessSet,
};
pub use utils::NetworkId;

use crate::certs::CommitteeColdCredential;
use crate::governance::{ProposalProcedure, VotingProcedure};

//extern crate serde_wasm_bindgen;
// Code below here was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct AssetNameList(Vec<cml_chain::assets::AssetName>);

impl_wasm_conversions!(Vec<cml_chain::assets::AssetName>, AssetNameList);

#[wasm_bindgen]
impl AssetNameList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> AssetName {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &AssetName) {
        self.0.push(elem.clone().into());
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct BootstrapWitnessList(Vec<cml_chain::crypto::BootstrapWitness>);

impl_wasm_conversions!(
    Vec<cml_chain::crypto::BootstrapWitness>,
    BootstrapWitnessList
);

#[wasm_bindgen]
impl BootstrapWitnessList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> BootstrapWitness {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &BootstrapWitness) {
        self.0.push(elem.clone().into());
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct CertificateList(Vec<cml_chain::certs::Certificate>);

impl_wasm_conversions!(Vec<cml_chain::certs::Certificate>, CertificateList);

#[wasm_bindgen]
impl CertificateList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Certificate {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &Certificate) {
        self.0.push(elem.clone().into());
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct CommitteeColdCredentialList(Vec<cml_chain::certs::CommitteeColdCredential>);

impl_wasm_conversions!(
    Vec<cml_chain::certs::CommitteeColdCredential>,
    CommitteeColdCredentialList
);

#[wasm_bindgen]
impl CommitteeColdCredentialList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> CommitteeColdCredential {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &CommitteeColdCredential) {
        self.0.push(elem.clone().into());
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct DRepVotingThresholds(cml_chain::DRepVotingThresholds);

impl_wasm_cbor_json_api!(DRepVotingThresholds);

impl_wasm_conversions!(cml_chain::DRepVotingThresholds, DRepVotingThresholds);

#[wasm_bindgen]
impl DRepVotingThresholds {
    pub fn motion_no_confidence(&self) -> UnitInterval {
        self.0.motion_no_confidence.clone().into()
    }

    pub fn committee_normal(&self) -> UnitInterval {
        self.0.committee_normal.clone().into()
    }

    pub fn committee_no_confidence(&self) -> UnitInterval {
        self.0.committee_no_confidence.clone().into()
    }

    pub fn update_constitution(&self) -> UnitInterval {
        self.0.update_constitution.clone().into()
    }

    pub fn hard_fork_initiation(&self) -> UnitInterval {
        self.0.hard_fork_initiation.clone().into()
    }

    pub fn pp_network_group(&self) -> UnitInterval {
        self.0.pp_network_group.clone().into()
    }

    pub fn pp_economic_group(&self) -> UnitInterval {
        self.0.pp_economic_group.clone().into()
    }

    pub fn pp_technical_group(&self) -> UnitInterval {
        self.0.pp_technical_group.clone().into()
    }

    pub fn pp_governance_group(&self) -> UnitInterval {
        self.0.pp_governance_group.clone().into()
    }

    pub fn treasury_withdrawal(&self) -> UnitInterval {
        self.0.treasury_withdrawal.clone().into()
    }

    pub fn new(
        motion_no_confidence: &UnitInterval,
        committee_normal: &UnitInterval,
        committee_no_confidence: &UnitInterval,
        update_constitution: &UnitInterval,
        hard_fork_initiation: &UnitInterval,
        pp_network_group: &UnitInterval,
        pp_economic_group: &UnitInterval,
        pp_technical_group: &UnitInterval,
        pp_governance_group: &UnitInterval,
        treasury_withdrawal: &UnitInterval,
    ) -> Self {
        Self(cml_chain::DRepVotingThresholds::new(
            motion_no_confidence.clone().into(),
            committee_normal.clone().into(),
            committee_no_confidence.clone().into(),
            update_constitution.clone().into(),
            hard_fork_initiation.clone().into(),
            pp_network_group.clone().into(),
            pp_economic_group.clone().into(),
            pp_technical_group.clone().into(),
            pp_governance_group.clone().into(),
            treasury_withdrawal.clone().into(),
        ))
    }
}

pub type DeltaCoin = Int;

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct Ed25519KeyHashList(Vec<cml_chain::crypto::Ed25519KeyHash>);

impl_wasm_conversions!(Vec<cml_chain::crypto::Ed25519KeyHash>, Ed25519KeyHashList);

#[wasm_bindgen]
impl Ed25519KeyHashList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Ed25519KeyHash {
        self.0[index].into()
    }

    pub fn add(&mut self, elem: &Ed25519KeyHash) {
        self.0.push(elem.clone().into());
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct GovActionIdList(Vec<cml_chain::governance::GovActionId>);

impl_wasm_conversions!(Vec<cml_chain::governance::GovActionId>, GovActionIdList);

#[wasm_bindgen]
impl GovActionIdList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> GovActionId {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &GovActionId) {
        self.0.push(elem.clone().into());
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct IntList(Vec<cml_chain::Int>);

impl_wasm_conversions!(Vec<cml_chain::Int>, IntList);

#[wasm_bindgen]
impl IntList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Int {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &Int) {
        self.0.push(elem.clone().into());
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MapAssetNameToNonZeroInt64(
    OrderedHashMap<cml_chain::assets::AssetName, cml_chain::assets::NonZeroInt64>,
);

impl_wasm_conversions!(OrderedHashMap<cml_chain::assets::AssetName, cml_chain::assets::NonZeroInt64>, MapAssetNameToNonZeroInt64);

#[wasm_bindgen]
impl MapAssetNameToNonZeroInt64 {
    pub fn new() -> Self {
        Self(OrderedHashMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, key: &AssetName, value: NonZeroInt64) -> Option<NonZeroInt64> {
        self.0.insert(key.clone().into(), value)
    }

    pub fn get(&self, key: &AssetName) -> Option<NonZeroInt64> {
        self.0.get(key.as_ref()).copied()
    }

    pub fn keys(&self) -> AssetNameList {
        AssetNameList(self.0.iter().map(|(k, _v)| k.clone()).collect::<Vec<_>>())
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MapAssetNameToU64(OrderedHashMap<cml_chain::assets::AssetName, u64>);

impl_wasm_conversions!(OrderedHashMap<cml_chain::assets::AssetName, u64>, MapAssetNameToU64);

#[wasm_bindgen]
impl MapAssetNameToU64 {
    pub fn new() -> Self {
        Self(OrderedHashMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, key: &AssetName, value: u64) -> Option<u64> {
        self.0.insert(key.clone().into(), value)
    }

    pub fn get(&self, key: &AssetName) -> Option<u64> {
        self.0.get(key.as_ref()).copied()
    }

    pub fn keys(&self) -> AssetNameList {
        AssetNameList(self.0.iter().map(|(k, _v)| k.clone()).collect::<Vec<_>>())
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MapCommitteeColdCredentialToEpoch(
    OrderedHashMap<cml_chain::certs::CommitteeColdCredential, cml_chain::Epoch>,
);

impl_wasm_conversions!(OrderedHashMap<cml_chain::certs::CommitteeColdCredential, cml_chain::Epoch>, MapCommitteeColdCredentialToEpoch);

#[wasm_bindgen]
impl MapCommitteeColdCredentialToEpoch {
    pub fn new() -> Self {
        Self(OrderedHashMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, key: &CommitteeColdCredential, value: Epoch) -> Option<Epoch> {
        self.0.insert(key.clone().into(), value)
    }

    pub fn get(&self, key: &CommitteeColdCredential) -> Option<Epoch> {
        self.0.get(key.as_ref()).copied()
    }

    pub fn keys(&self) -> CommitteeColdCredentialList {
        CommitteeColdCredentialList(self.0.iter().map(|(k, _v)| k.clone()).collect::<Vec<_>>())
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MapGovActionIdToVotingProcedure(
    OrderedHashMap<cml_chain::governance::GovActionId, cml_chain::governance::VotingProcedure>,
);

impl_wasm_conversions!(OrderedHashMap<cml_chain::governance::GovActionId, cml_chain::governance::VotingProcedure>, MapGovActionIdToVotingProcedure);

#[wasm_bindgen]
impl MapGovActionIdToVotingProcedure {
    pub fn new() -> Self {
        Self(OrderedHashMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(
        &mut self,
        key: &GovActionId,
        value: &VotingProcedure,
    ) -> Option<VotingProcedure> {
        self.0
            .insert(key.clone().into(), value.clone().into())
            .map(Into::into)
    }

    pub fn get(&self, key: &GovActionId) -> Option<VotingProcedure> {
        self.0.get(key.as_ref()).map(|v| v.clone().into())
    }

    pub fn keys(&self) -> GovActionIdList {
        GovActionIdList(self.0.iter().map(|(k, _v)| k.clone()).collect::<Vec<_>>())
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MapPlutusDataToPlutusData(
    OrderedHashMap<cml_chain::plutus::PlutusData, cml_chain::plutus::PlutusData>,
);

impl_wasm_conversions!(OrderedHashMap<cml_chain::plutus::PlutusData, cml_chain::plutus::PlutusData>, MapPlutusDataToPlutusData);

#[wasm_bindgen]
impl MapPlutusDataToPlutusData {
    pub fn new() -> Self {
        Self(OrderedHashMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, key: &PlutusData, value: &PlutusData) -> Option<PlutusData> {
        self.0
            .insert(key.clone().into(), value.clone().into())
            .map(Into::into)
    }

    pub fn get(&self, key: &PlutusData) -> Option<PlutusData> {
        self.0.get(key.as_ref()).map(|v| v.clone().into())
    }

    pub fn keys(&self) -> PlutusDataList {
        PlutusDataList(self.0.iter().map(|(k, _v)| k.clone()).collect::<Vec<_>>())
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MapRewardAccountToCoin(
    OrderedHashMap<cml_chain::address::RewardAccount, cml_chain::assets::Coin>,
);

impl_wasm_conversions!(OrderedHashMap<cml_chain::address::RewardAccount, cml_chain::assets::Coin>, MapRewardAccountToCoin);

#[wasm_bindgen]
impl MapRewardAccountToCoin {
    pub fn new() -> Self {
        Self(OrderedHashMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, key: &RewardAccount, value: Coin) -> Option<Coin> {
        self.0.insert(key.clone().into(), value)
    }

    pub fn get(&self, key: &RewardAccount) -> Option<Coin> {
        self.0.get(key.as_ref()).copied()
    }

    pub fn keys(&self) -> RewardAccountList {
        RewardAccountList(self.0.iter().map(|(k, _v)| k.clone()).collect::<Vec<_>>())
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MapStakeCredentialToDeltaCoin(
    OrderedHashMap<cml_chain::certs::StakeCredential, cml_chain::DeltaCoin>,
);

impl_wasm_conversions!(OrderedHashMap<cml_chain::certs::StakeCredential, cml_chain::DeltaCoin>, MapStakeCredentialToDeltaCoin);

#[wasm_bindgen]
impl MapStakeCredentialToDeltaCoin {
    pub fn new() -> Self {
        Self(OrderedHashMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, key: &StakeCredential, value: &DeltaCoin) -> Option<DeltaCoin> {
        self.0
            .insert(key.clone().into(), value.clone().into())
            .map(Into::into)
    }

    pub fn get(&self, key: &StakeCredential) -> Option<DeltaCoin> {
        self.0.get(key.as_ref()).map(|v| v.clone().into())
    }

    pub fn keys(&self) -> StakeCredentialList {
        StakeCredentialList(self.0.iter().map(|(k, _v)| k.clone()).collect::<Vec<_>>())
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MapTransactionIndexToAuxiliaryData(
    OrderedHashMap<cml_chain::TransactionIndex, cml_chain::auxdata::AuxiliaryData>,
);

impl_wasm_conversions!(OrderedHashMap<cml_chain::TransactionIndex, cml_chain::auxdata::AuxiliaryData>, MapTransactionIndexToAuxiliaryData);

#[wasm_bindgen]
impl MapTransactionIndexToAuxiliaryData {
    pub fn new() -> Self {
        Self(OrderedHashMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(
        &mut self,
        key: TransactionIndex,
        value: &AuxiliaryData,
    ) -> Option<AuxiliaryData> {
        self.0.insert(key, value.clone().into()).map(Into::into)
    }

    pub fn get(&self, key: TransactionIndex) -> Option<AuxiliaryData> {
        self.0.get(&key).map(|v| v.clone().into())
    }

    pub fn keys(&self) -> Vec<TransactionIndex> {
        self.0.keys().copied().collect::<Vec<_>>()
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MapTransactionMetadatumToTransactionMetadatum(
    OrderedHashMap<
        cml_chain::auxdata::TransactionMetadatum,
        cml_chain::auxdata::TransactionMetadatum,
    >,
);

impl_wasm_conversions!(OrderedHashMap<cml_chain::auxdata::TransactionMetadatum, cml_chain::auxdata::TransactionMetadatum>, MapTransactionMetadatumToTransactionMetadatum);

#[wasm_bindgen]
impl MapTransactionMetadatumToTransactionMetadatum {
    pub fn new() -> Self {
        Self(OrderedHashMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(
        &mut self,
        key: &TransactionMetadatum,
        value: &TransactionMetadatum,
    ) -> Option<TransactionMetadatum> {
        self.0
            .insert(key.clone().into(), value.clone().into())
            .map(Into::into)
    }

    pub fn get(&self, key: &TransactionMetadatum) -> Option<TransactionMetadatum> {
        self.0.get(key.as_ref()).map(|v| v.clone().into())
    }

    pub fn keys(&self) -> TransactionMetadatumList {
        self.0
            .iter()
            .map(|(k, _v)| k.clone())
            .collect::<Vec<_>>()
            .into()
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct NativeScriptList(Vec<cml_chain::transaction::NativeScript>);

impl_wasm_conversions!(Vec<cml_chain::transaction::NativeScript>, NativeScriptList);

#[wasm_bindgen]
impl NativeScriptList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> NativeScript {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &NativeScript) {
        self.0.push(elem.clone().into());
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct PlutusDataList(Vec<cml_chain::plutus::PlutusData>);

impl_wasm_conversions!(Vec<cml_chain::plutus::PlutusData>, PlutusDataList);

#[wasm_bindgen]
impl PlutusDataList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> PlutusData {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &PlutusData) {
        self.0.push(elem.clone().into());
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct PlutusV1ScriptList(Vec<cml_chain::plutus::PlutusV1Script>);

impl_wasm_conversions!(Vec<cml_chain::plutus::PlutusV1Script>, PlutusV1ScriptList);

#[wasm_bindgen]
impl PlutusV1ScriptList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> PlutusV1Script {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &PlutusV1Script) {
        self.0.push(elem.clone().into());
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct PlutusV2ScriptList(Vec<cml_chain::plutus::PlutusV2Script>);

impl_wasm_conversions!(Vec<cml_chain::plutus::PlutusV2Script>, PlutusV2ScriptList);

#[wasm_bindgen]
impl PlutusV2ScriptList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> PlutusV2Script {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &PlutusV2Script) {
        self.0.push(elem.clone().into());
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct PlutusV3ScriptList(Vec<cml_chain::plutus::PlutusV3Script>);

impl_wasm_conversions!(Vec<cml_chain::plutus::PlutusV3Script>, PlutusV3ScriptList);

#[wasm_bindgen]
impl PlutusV3ScriptList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> PlutusV3Script {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &PlutusV3Script) {
        self.0.push(elem.clone().into());
    }
}

pub type PolicyId = ScriptHash;

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct PolicyIdList(Vec<cml_chain::PolicyId>);

impl_wasm_conversions!(Vec<cml_chain::PolicyId>, PolicyIdList);

#[wasm_bindgen]
impl PolicyIdList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> PolicyId {
        self.0[index].into()
    }

    pub fn add(&mut self, elem: &PolicyId) {
        self.0.push(elem.clone().into());
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct PoolVotingThresholds(cml_chain::PoolVotingThresholds);

impl_wasm_cbor_json_api!(PoolVotingThresholds);

impl_wasm_conversions!(cml_chain::PoolVotingThresholds, PoolVotingThresholds);

#[wasm_bindgen]
impl PoolVotingThresholds {
    pub fn motion_no_confidence(&self) -> UnitInterval {
        self.0.motion_no_confidence.clone().into()
    }

    pub fn committee_normal(&self) -> UnitInterval {
        self.0.committee_normal.clone().into()
    }

    pub fn committee_no_confidence(&self) -> UnitInterval {
        self.0.committee_no_confidence.clone().into()
    }

    pub fn hard_fork_initiation(&self) -> UnitInterval {
        self.0.hard_fork_initiation.clone().into()
    }

    pub fn new(
        motion_no_confidence: &UnitInterval,
        committee_normal: &UnitInterval,
        committee_no_confidence: &UnitInterval,
        hard_fork_initiation: &UnitInterval,
    ) -> Self {
        Self(cml_chain::PoolVotingThresholds::new(
            motion_no_confidence.clone().into(),
            committee_normal.clone().into(),
            committee_no_confidence.clone().into(),
            hard_fork_initiation.clone().into(),
        ))
    }
}

pub type Port = u16;

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ProposalProcedureList(Vec<cml_chain::governance::ProposalProcedure>);

impl_wasm_conversions!(
    Vec<cml_chain::governance::ProposalProcedure>,
    ProposalProcedureList
);

#[wasm_bindgen]
impl ProposalProcedureList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> ProposalProcedure {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &ProposalProcedure) {
        self.0.push(elem.clone().into());
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ProtocolParamUpdate(cml_chain::ProtocolParamUpdate);

impl_wasm_cbor_json_api!(ProtocolParamUpdate);

impl_wasm_conversions!(cml_chain::ProtocolParamUpdate, ProtocolParamUpdate);

#[wasm_bindgen]
impl ProtocolParamUpdate {
    pub fn set_minfee_a(&mut self, minfee_a: u64) {
        self.0.minfee_a = Some(minfee_a)
    }

    pub fn minfee_a(&self) -> Option<u64> {
        self.0.minfee_a
    }

    pub fn set_minfee_b(&mut self, minfee_b: u64) {
        self.0.minfee_b = Some(minfee_b)
    }

    pub fn minfee_b(&self) -> Option<u64> {
        self.0.minfee_b
    }

    pub fn set_max_block_body_size(&mut self, max_block_body_size: u64) {
        self.0.max_block_body_size = Some(max_block_body_size)
    }

    pub fn max_block_body_size(&self) -> Option<u64> {
        self.0.max_block_body_size
    }

    pub fn set_max_transaction_size(&mut self, max_transaction_size: u64) {
        self.0.max_transaction_size = Some(max_transaction_size)
    }

    pub fn max_transaction_size(&self) -> Option<u64> {
        self.0.max_transaction_size
    }

    pub fn set_max_block_header_size(&mut self, max_block_header_size: u64) {
        self.0.max_block_header_size = Some(max_block_header_size)
    }

    pub fn max_block_header_size(&self) -> Option<u64> {
        self.0.max_block_header_size
    }

    pub fn set_key_deposit(&mut self, key_deposit: Coin) {
        self.0.key_deposit = Some(key_deposit)
    }

    pub fn key_deposit(&self) -> Option<Coin> {
        self.0.key_deposit
    }

    pub fn set_pool_deposit(&mut self, pool_deposit: Coin) {
        self.0.pool_deposit = Some(pool_deposit)
    }

    pub fn pool_deposit(&self) -> Option<Coin> {
        self.0.pool_deposit
    }

    pub fn set_maximum_epoch(&mut self, maximum_epoch: Epoch) {
        self.0.maximum_epoch = Some(maximum_epoch)
    }

    pub fn maximum_epoch(&self) -> Option<Epoch> {
        self.0.maximum_epoch
    }

    pub fn set_n_opt(&mut self, n_opt: u64) {
        self.0.n_opt = Some(n_opt)
    }

    pub fn n_opt(&self) -> Option<u64> {
        self.0.n_opt
    }

    pub fn set_pool_pledge_influence(&mut self, pool_pledge_influence: &Rational) {
        self.0.pool_pledge_influence = Some(pool_pledge_influence.clone().into())
    }

    pub fn pool_pledge_influence(&self) -> Option<Rational> {
        self.0
            .pool_pledge_influence
            .clone()
            .map(std::convert::Into::into)
    }

    pub fn set_expansion_rate(&mut self, expansion_rate: &UnitInterval) {
        self.0.expansion_rate = Some(expansion_rate.clone().into())
    }

    pub fn expansion_rate(&self) -> Option<UnitInterval> {
        self.0.expansion_rate.clone().map(std::convert::Into::into)
    }

    pub fn set_treasury_growth_rate(&mut self, treasury_growth_rate: &UnitInterval) {
        self.0.treasury_growth_rate = Some(treasury_growth_rate.clone().into())
    }

    pub fn treasury_growth_rate(&self) -> Option<UnitInterval> {
        self.0
            .treasury_growth_rate
            .clone()
            .map(std::convert::Into::into)
    }

    pub fn set_min_pool_cost(&mut self, min_pool_cost: Coin) {
        self.0.min_pool_cost = Some(min_pool_cost)
    }

    pub fn min_pool_cost(&self) -> Option<Coin> {
        self.0.min_pool_cost
    }

    pub fn set_ada_per_utxo_byte(&mut self, ada_per_utxo_byte: Coin) {
        self.0.ada_per_utxo_byte = Some(ada_per_utxo_byte)
    }

    pub fn ada_per_utxo_byte(&self) -> Option<Coin> {
        self.0.ada_per_utxo_byte
    }

    pub fn set_cost_models_for_script_languages(
        &mut self,
        cost_models_for_script_languages: &CostModels,
    ) {
        self.0.cost_models_for_script_languages =
            Some(cost_models_for_script_languages.clone().into())
    }

    pub fn cost_models_for_script_languages(&self) -> Option<CostModels> {
        self.0
            .cost_models_for_script_languages
            .clone()
            .map(std::convert::Into::into)
    }

    pub fn set_execution_costs(&mut self, execution_costs: &ExUnitPrices) {
        self.0.execution_costs = Some(execution_costs.clone().into())
    }

    pub fn execution_costs(&self) -> Option<ExUnitPrices> {
        self.0.execution_costs.clone().map(std::convert::Into::into)
    }

    pub fn set_max_tx_ex_units(&mut self, max_tx_ex_units: &ExUnits) {
        self.0.max_tx_ex_units = Some(max_tx_ex_units.clone().into())
    }

    pub fn max_tx_ex_units(&self) -> Option<ExUnits> {
        self.0.max_tx_ex_units.clone().map(std::convert::Into::into)
    }

    pub fn set_max_block_ex_units(&mut self, max_block_ex_units: &ExUnits) {
        self.0.max_block_ex_units = Some(max_block_ex_units.clone().into())
    }

    pub fn max_block_ex_units(&self) -> Option<ExUnits> {
        self.0
            .max_block_ex_units
            .clone()
            .map(std::convert::Into::into)
    }

    pub fn set_max_value_size(&mut self, max_value_size: u64) {
        self.0.max_value_size = Some(max_value_size)
    }

    pub fn max_value_size(&self) -> Option<u64> {
        self.0.max_value_size
    }

    pub fn set_collateral_percentage(&mut self, collateral_percentage: u64) {
        self.0.collateral_percentage = Some(collateral_percentage)
    }

    pub fn collateral_percentage(&self) -> Option<u64> {
        self.0.collateral_percentage
    }

    pub fn set_max_collateral_inputs(&mut self, max_collateral_inputs: u64) {
        self.0.max_collateral_inputs = Some(max_collateral_inputs)
    }

    pub fn max_collateral_inputs(&self) -> Option<u64> {
        self.0.max_collateral_inputs
    }

    pub fn set_pool_voting_thresholds(&mut self, pool_voting_thresholds: &PoolVotingThresholds) {
        self.0.pool_voting_thresholds = Some(pool_voting_thresholds.clone().into())
    }

    pub fn pool_voting_thresholds(&self) -> Option<PoolVotingThresholds> {
        self.0
            .pool_voting_thresholds
            .clone()
            .map(std::convert::Into::into)
    }

    pub fn set_d_rep_voting_thresholds(&mut self, d_rep_voting_thresholds: &DRepVotingThresholds) {
        self.0.d_rep_voting_thresholds = Some(d_rep_voting_thresholds.clone().into())
    }

    pub fn d_rep_voting_thresholds(&self) -> Option<DRepVotingThresholds> {
        self.0
            .d_rep_voting_thresholds
            .clone()
            .map(std::convert::Into::into)
    }

    pub fn set_min_committee_size(&mut self, min_committee_size: u64) {
        self.0.min_committee_size = Some(min_committee_size)
    }

    pub fn min_committee_size(&self) -> Option<u64> {
        self.0.min_committee_size
    }

    pub fn set_committee_term_limit(&mut self, committee_term_limit: u64) {
        self.0.committee_term_limit = Some(committee_term_limit)
    }

    pub fn committee_term_limit(&self) -> Option<u64> {
        self.0.committee_term_limit
    }

    pub fn set_governance_action_validity_period(
        &mut self,
        governance_action_validity_period: Epoch,
    ) {
        self.0.governance_action_validity_period = Some(governance_action_validity_period)
    }

    pub fn governance_action_validity_period(&self) -> Option<Epoch> {
        self.0.governance_action_validity_period
    }

    pub fn set_governance_action_deposit(&mut self, governance_action_deposit: Coin) {
        self.0.governance_action_deposit = Some(governance_action_deposit)
    }

    pub fn governance_action_deposit(&self) -> Option<Coin> {
        self.0.governance_action_deposit
    }

    pub fn set_d_rep_deposit(&mut self, d_rep_deposit: Coin) {
        self.0.d_rep_deposit = Some(d_rep_deposit)
    }

    pub fn d_rep_deposit(&self) -> Option<Coin> {
        self.0.d_rep_deposit
    }

    pub fn set_d_rep_inactivity_period(&mut self, d_rep_inactivity_period: Epoch) {
        self.0.d_rep_inactivity_period = Some(d_rep_inactivity_period)
    }

    pub fn d_rep_inactivity_period(&self) -> Option<Epoch> {
        self.0.d_rep_inactivity_period
    }

    pub fn new() -> Self {
        Self(cml_chain::ProtocolParamUpdate::new())
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct Rational(cml_chain::Rational);

impl_wasm_cbor_json_api!(Rational);

impl_wasm_conversions!(cml_chain::Rational, Rational);

#[wasm_bindgen]
impl Rational {
    pub fn numerator(&self) -> u64 {
        self.0.numerator
    }

    pub fn denominator(&self) -> u64 {
        self.0.denominator
    }

    pub fn new(numerator: u64, denominator: u64) -> Self {
        Self(cml_chain::Rational::new(numerator, denominator))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct RedeemerList(Vec<cml_chain::plutus::Redeemer>);

impl_wasm_conversions!(Vec<cml_chain::plutus::Redeemer>, RedeemerList);

#[wasm_bindgen]
impl RedeemerList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Redeemer {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &Redeemer) {
        self.0.push(elem.clone().into());
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct RelayList(Vec<cml_chain::certs::Relay>);

impl_wasm_conversions!(Vec<cml_chain::certs::Relay>, RelayList);

#[wasm_bindgen]
impl RelayList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Relay {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &Relay) {
        self.0.push(elem.clone().into());
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct RewardAccountList(Vec<cml_chain::address::RewardAccount>);

impl_wasm_conversions!(Vec<cml_chain::address::RewardAccount>, RewardAccountList);

#[wasm_bindgen]
impl RewardAccountList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> RewardAccount {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &RewardAccount) {
        self.0.push(elem.clone().into());
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct Script(cml_chain::Script);

impl_wasm_cbor_json_api!(Script);

impl_wasm_conversions!(cml_chain::Script, Script);

#[wasm_bindgen]
impl Script {
    pub fn new_native(script: &NativeScript) -> Self {
        Self(cml_chain::Script::new_native(script.clone().into()))
    }

    pub fn new_plutus_v1(script: &PlutusV1Script) -> Self {
        Self(cml_chain::Script::new_plutus_v1(script.clone().into()))
    }

    pub fn new_plutus_v2(script: &PlutusV2Script) -> Self {
        Self(cml_chain::Script::new_plutus_v2(script.clone().into()))
    }

    pub fn new_plutus_v3(script: &PlutusV3Script) -> Self {
        Self(cml_chain::Script::new_plutus_v3(script.clone().into()))
    }

    pub fn kind(&self) -> ScriptKind {
        match &self.0 {
            cml_chain::Script::Native { .. } => ScriptKind::Native,
            cml_chain::Script::PlutusV1 { .. } => ScriptKind::PlutusV1,
            cml_chain::Script::PlutusV2 { .. } => ScriptKind::PlutusV2,
            cml_chain::Script::PlutusV3 { .. } => ScriptKind::PlutusV3,
        }
    }

    pub fn as_native(&self) -> Option<NativeScript> {
        match &self.0 {
            cml_chain::Script::Native { script, .. } => Some(script.clone().into()),
            _ => None,
        }
    }

    pub fn as_plutus_v1(&self) -> Option<PlutusV1Script> {
        match &self.0 {
            cml_chain::Script::PlutusV1 { script, .. } => Some(script.clone().into()),
            _ => None,
        }
    }

    pub fn as_plutus_v2(&self) -> Option<PlutusV2Script> {
        match &self.0 {
            cml_chain::Script::PlutusV2 { script, .. } => Some(script.clone().into()),
            _ => None,
        }
    }

    pub fn as_plutus_v3(&self) -> Option<PlutusV3Script> {
        match &self.0 {
            cml_chain::Script::PlutusV3 { script, .. } => Some(script.clone().into()),
            _ => None,
        }
    }
}

#[wasm_bindgen]
pub enum ScriptKind {
    Native,
    PlutusV1,
    PlutusV2,
    PlutusV3,
}

pub type Slot = u64;

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct StakeCredentialList(Vec<cml_chain::certs::StakeCredential>);

impl_wasm_conversions!(Vec<cml_chain::certs::StakeCredential>, StakeCredentialList);

#[wasm_bindgen]
impl StakeCredentialList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> StakeCredential {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &StakeCredential) {
        self.0.push(elem.clone().into());
    }
}

pub type SubCoin = Rational;

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct TransactionBodyList(Vec<cml_chain::transaction::TransactionBody>);

impl_wasm_conversions!(
    Vec<cml_chain::transaction::TransactionBody>,
    TransactionBodyList
);

#[wasm_bindgen]
impl TransactionBodyList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> TransactionBody {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &TransactionBody) {
        self.0.push(elem.clone().into());
    }
}

pub type TransactionIndex = u16;

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct TransactionInputList(Vec<cml_chain::transaction::TransactionInput>);

impl_wasm_conversions!(
    Vec<cml_chain::transaction::TransactionInput>,
    TransactionInputList
);

#[wasm_bindgen]
impl TransactionInputList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> TransactionInput {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &TransactionInput) {
        self.0.push(elem.clone().into());
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct TransactionOutputList(Vec<cml_chain::transaction::TransactionOutput>);

impl_wasm_conversions!(
    Vec<cml_chain::transaction::TransactionOutput>,
    TransactionOutputList
);

#[wasm_bindgen]
impl TransactionOutputList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> TransactionOutput {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &TransactionOutput) {
        self.0.push(elem.clone().into());
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct TransactionWitnessSetList(Vec<cml_chain::transaction::TransactionWitnessSet>);

impl_wasm_conversions!(
    Vec<cml_chain::transaction::TransactionWitnessSet>,
    TransactionWitnessSetList
);

#[wasm_bindgen]
impl TransactionWitnessSetList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> TransactionWitnessSet {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &TransactionWitnessSet) {
        self.0.push(elem.clone().into());
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct UnitInterval(cml_chain::UnitInterval);

impl_wasm_cbor_json_api!(UnitInterval);

impl_wasm_conversions!(cml_chain::UnitInterval, UnitInterval);

#[wasm_bindgen]
impl UnitInterval {
    pub fn start(&self) -> u64 {
        self.0.start
    }

    pub fn end(&self) -> u64 {
        self.0.end
    }

    pub fn new(start: u64, end: u64) -> Self {
        Self(cml_chain::UnitInterval::new(start, end))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct VkeywitnessList(Vec<cml_chain::crypto::Vkeywitness>);

impl_wasm_conversions!(Vec<cml_chain::crypto::Vkeywitness>, VkeywitnessList);

#[wasm_bindgen]
impl VkeywitnessList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Vkeywitness {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &Vkeywitness) {
        self.0.push(elem.clone().into());
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct VoterList(Vec<cml_chain::governance::Voter>);

impl_wasm_conversions!(Vec<cml_chain::governance::Voter>, VoterList);

#[wasm_bindgen]
impl VoterList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Voter {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &Voter) {
        self.0.push(elem.clone().into());
    }
}

pub type Withdrawals = MapRewardAccountToCoin;
