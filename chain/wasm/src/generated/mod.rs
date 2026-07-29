// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

#![allow(
    clippy::len_without_is_empty,
    clippy::too_many_arguments,
    clippy::new_without_default
)]
pub mod address;
pub mod assets;
pub mod auxdata;
pub mod block;
pub mod certs;
pub mod collections;
pub mod crypto;
pub mod governance;
pub mod plutus;
pub mod requested_collections;
pub mod transaction;
pub use cml_core_wasm::Int;

use address::RewardAccount;
use assets::{AssetName, Coin, NonZeroInt64};
use auxdata::AuxiliaryData;
use certs::{Certificate, CommitteeColdCredential, Relay};
use cml_core::non_empty::NonEmptyVec;
use cml_core::non_empty_map::NonEmptyMap;
use cml_core::ordered_hash_map::OrderedHashMap;
use cml_core::ordered_set::{NonEmptyOrderedSet, OrderedSet};
use cml_core_wasm::{impl_wasm_cbor_json_api, impl_wasm_conversions, impl_wasm_list_needs_into};
use crypto::{BootstrapWitness, Ed25519KeyHash, ScriptHash, Vkeywitness};
use governance::{GovActionId, ProposalProcedure, Voter, VotingProcedure};
use plutus::{
    CostModels, ExUnitPrices, ExUnits, LegacyRedeemer, PlutusData, PlutusV1Script, PlutusV2Script,
    PlutusV3Script, RedeemerKey, RedeemerVal,
};
use transaction::{
    NativeScript, TransactionBody, TransactionInput, TransactionOutput, TransactionWitnessSet,
};
use wasm_bindgen::prelude::{JsError, wasm_bindgen};

impl_wasm_list_needs_into!(
    cml_chain::assets::AssetName,
    AssetName,
    AssetNameList,
    true,
    false
);

impl_wasm_list_needs_into!(
    cml_chain::crypto::BootstrapWitness,
    BootstrapWitness,
    BootstrapWitnessList,
    true,
    false
);

impl_wasm_list_needs_into!(
    cml_chain::certs::Certificate,
    Certificate,
    CertificateList,
    true,
    false
);

impl_wasm_list_needs_into!(
    cml_chain::certs::CommitteeColdCredential,
    CommitteeColdCredential,
    CommitteeColdCredentialList,
    true,
    false
);

/// `[* committee_cold_credential] @duplicates reject`: an insertion-ordered, duplicate-free set (order preserved for byte-exact round-trip). `add` is checked — an already-present element is refused; construct via `try_from` (the uniqueness door). `insert` is the std-set door (returns `false`, set unchanged, for an already-present element); `contains` tests membership.
#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct CommitteeColdCredentialOrderedSet(
    pub(crate) OrderedSet<cml_chain::certs::CommitteeColdCredential>,
);

impl_wasm_conversions!(
    OrderedSet<cml_chain::certs::CommitteeColdCredential>,
    CommitteeColdCredentialOrderedSet
);

#[wasm_bindgen]
impl CommitteeColdCredentialOrderedSet {
    pub fn new() -> Self {
        Self(OrderedSet::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> CommitteeColdCredential {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &CommitteeColdCredential) -> Result<(), JsError> {
        self.0
            .push(elem.clone().into())
            .map_err(|e| JsError::new(&e.to_string()))
    }

    pub fn insert(&mut self, elem: &CommitteeColdCredential) -> bool {
        self.0.insert(elem.clone().into())
    }

    pub fn contains(&self, elem: &CommitteeColdCredential) -> bool {
        self.0.contains(&elem.clone().into())
    }

    pub fn try_from(
        list: &CommitteeColdCredentialList,
    ) -> Result<CommitteeColdCredentialOrderedSet, JsError> {
        let inner: Vec<cml_chain::certs::CommitteeColdCredential> = list.clone().into();
        OrderedSet::try_from(inner)
            .map(Self)
            .map_err(|e| JsError::new(&e.to_string()))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct DRepVotingThresholds(pub(crate) cml_chain::DRepVotingThresholds);

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

impl_wasm_list_needs_into!(
    cml_chain::crypto::Ed25519KeyHash,
    Ed25519KeyHash,
    Ed25519KeyHashList,
    true,
    true
);

/// `[* ed25519_key_hash] @duplicates reject`: an insertion-ordered, duplicate-free set (order preserved for byte-exact round-trip). `add` is checked — an already-present element is refused; construct via `try_from` (the uniqueness door). `insert` is the std-set door (returns `false`, set unchanged, for an already-present element); `contains` tests membership.
#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct Ed25519KeyHashOrderedSet(pub(crate) OrderedSet<cml_chain::crypto::Ed25519KeyHash>);

impl_wasm_conversions!(
    OrderedSet<cml_chain::crypto::Ed25519KeyHash>,
    Ed25519KeyHashOrderedSet
);

#[wasm_bindgen]
impl Ed25519KeyHashOrderedSet {
    pub fn new() -> Self {
        Self(OrderedSet::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Ed25519KeyHash {
        self.0[index].into()
    }

    pub fn add(&mut self, elem: &Ed25519KeyHash) -> Result<(), JsError> {
        self.0
            .push(elem.clone().into())
            .map_err(|e| JsError::new(&e.to_string()))
    }

    pub fn insert(&mut self, elem: &Ed25519KeyHash) -> bool {
        self.0.insert(elem.clone().into())
    }

    pub fn contains(&self, elem: &Ed25519KeyHash) -> bool {
        self.0.contains(&elem.clone().into())
    }

    pub fn try_from(list: &Ed25519KeyHashList) -> Result<Ed25519KeyHashOrderedSet, JsError> {
        let inner: Vec<cml_chain::crypto::Ed25519KeyHash> = list.clone().into();
        OrderedSet::try_from(inner)
            .map(Self)
            .map_err(|e| JsError::new(&e.to_string()))
    }
}

pub type Epoch = u64;

impl_wasm_list_needs_into!(
    cml_chain::governance::GovActionId,
    GovActionId,
    GovActionIdList,
    true,
    false
);

pub type IntError = JsError;

impl_wasm_list_needs_into!(
    cml_chain::plutus::LegacyRedeemer,
    LegacyRedeemer,
    LegacyRedeemerList,
    true,
    false
);

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MapAssetNameToNonZeroInt64(
    pub(crate) OrderedHashMap<cml_chain::assets::AssetName, cml_chain::assets::NonZeroInt64>,
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
        AssetNameList(self.0.keys().cloned().collect::<Vec<_>>())
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MapAssetNameToU64(pub(crate) OrderedHashMap<cml_chain::assets::AssetName, u64>);

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
        AssetNameList(self.0.keys().cloned().collect::<Vec<_>>())
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MapCommitteeColdCredentialToEpoch(
    pub(crate) OrderedHashMap<cml_chain::certs::CommitteeColdCredential, cml_chain::Epoch>,
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
        CommitteeColdCredentialList(self.0.keys().cloned().collect::<Vec<_>>())
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
// rustfmt::skip: rustfmt breaks after the field vis leaving trailing whitespace and errors
// (rust-lang/rustfmt#5703, fix PR #5708 unmerged). Remove when #5708 ships.
#[rustfmt::skip]
pub struct MapGovActionIdToVotingProcedure(
    pub(crate) OrderedHashMap<cml_chain::governance::GovActionId, cml_chain::governance::VotingProcedure>,
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
        GovActionIdList(self.0.keys().cloned().collect::<Vec<_>>())
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MapRedeemerKeyToRedeemerVal(
    pub(crate) OrderedHashMap<cml_chain::plutus::RedeemerKey, cml_chain::plutus::RedeemerVal>,
);

impl_wasm_conversions!(OrderedHashMap<cml_chain::plutus::RedeemerKey, cml_chain::plutus::RedeemerVal>, MapRedeemerKeyToRedeemerVal);

#[wasm_bindgen]
impl MapRedeemerKeyToRedeemerVal {
    pub fn new() -> Self {
        Self(OrderedHashMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, key: &RedeemerKey, value: &RedeemerVal) -> Option<RedeemerVal> {
        self.0
            .insert(key.clone().into(), value.clone().into())
            .map(Into::into)
    }

    pub fn get(&self, key: &RedeemerKey) -> Option<RedeemerVal> {
        self.0.get(key.as_ref()).map(|v| v.clone().into())
    }

    pub fn keys(&self) -> RedeemerKeyList {
        RedeemerKeyList(self.0.keys().cloned().collect::<Vec<_>>())
    }
}

pub type MapRewardAccountToCoin = Withdrawals;

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MapTransactionIndexToAuxiliaryData(
    pub(crate) OrderedHashMap<cml_chain::TransactionIndex, cml_chain::auxdata::AuxiliaryData>,
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
pub struct MapU64ToArrI64(pub(crate) OrderedHashMap<u64, Vec<i64>>);

impl_wasm_conversions!(OrderedHashMap<u64, Vec<i64>>, MapU64ToArrI64);

#[wasm_bindgen]
impl MapU64ToArrI64 {
    pub fn new() -> Self {
        Self(OrderedHashMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, key: u64, value: Vec<i64>) -> Option<Vec<i64>> {
        self.0.insert(key, value)
    }

    pub fn get(&self, key: u64) -> Option<Vec<i64>> {
        self.0.get(&key).cloned()
    }

    pub fn keys(&self) -> Vec<u64> {
        self.0.keys().copied().collect::<Vec<_>>()
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
// rustfmt::skip: rustfmt breaks after the field vis leaving trailing whitespace and errors
// (rust-lang/rustfmt#5703, fix PR #5708 unmerged). Remove when #5708 ships.
#[rustfmt::skip]
pub struct MapVoterToMapGovActionIdToVotingProcedure(
    pub(crate) OrderedHashMap<cml_chain::governance::Voter, NonEmptyMap<cml_chain::governance::GovActionId, cml_chain::governance::VotingProcedure>>,
);

impl_wasm_conversions!(OrderedHashMap<cml_chain::governance::Voter, NonEmptyMap<cml_chain::governance::GovActionId, cml_chain::governance::VotingProcedure>>, MapVoterToMapGovActionIdToVotingProcedure);

#[wasm_bindgen]
impl MapVoterToMapGovActionIdToVotingProcedure {
    pub fn new() -> Self {
        Self(OrderedHashMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(
        &mut self,
        key: &Voter,
        value: &NonEmptyMapGovActionIdToVotingProcedure,
    ) -> Option<NonEmptyMapGovActionIdToVotingProcedure> {
        self.0
            .insert(key.clone().into(), value.clone().into())
            .map(Into::into)
    }

    pub fn get(&self, key: &Voter) -> Option<NonEmptyMapGovActionIdToVotingProcedure> {
        self.0.get(key.as_ref()).map(|v| v.clone().into())
    }

    pub fn keys(&self) -> VoterList {
        VoterList(self.0.keys().cloned().collect::<Vec<_>>())
    }
}

impl_wasm_list_needs_into!(
    cml_chain::transaction::NativeScript,
    NativeScript,
    NativeScriptList,
    true,
    false
);

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct NetworkId(pub(crate) cml_chain::NetworkId);

impl_wasm_cbor_json_api!(NetworkId);

impl_wasm_conversions!(cml_chain::NetworkId, NetworkId);

#[wasm_bindgen]
impl NetworkId {
    pub fn new(inner: u64) -> Self {
        Self(cml_chain::NetworkId::new(inner))
    }

    pub fn get(&self) -> u64 {
        self.0.get()
    }
}

/// `[+ bootstrap_witness] @duplicates reject`: an insertion-ordered, duplicate-free set (order preserved for byte-exact round-trip). `add` is checked — an already-present element is refused; construct via `try_from` (the uniqueness door). `insert` is the std-set door (returns `false`, set unchanged, for an already-present element); `contains` tests membership.
#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct NonEmptyBootstrapWitnessOrderedSet(
    pub(crate) NonEmptyOrderedSet<cml_chain::crypto::BootstrapWitness>,
);

impl_wasm_conversions!(
    NonEmptyOrderedSet<cml_chain::crypto::BootstrapWitness>,
    NonEmptyBootstrapWitnessOrderedSet
);

#[wasm_bindgen]
impl NonEmptyBootstrapWitnessOrderedSet {
    pub fn new(first: &BootstrapWitness) -> Self {
        Self(NonEmptyOrderedSet::new(first.clone().into()))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> BootstrapWitness {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &BootstrapWitness) -> Result<(), JsError> {
        self.0
            .push(elem.clone().into())
            .map_err(|e| JsError::new(&e.to_string()))
    }

    pub fn insert(&mut self, elem: &BootstrapWitness) -> bool {
        self.0.insert(elem.clone().into())
    }

    pub fn contains(&self, elem: &BootstrapWitness) -> bool {
        self.0.contains(&elem.clone().into())
    }

    pub fn try_from(
        list: &BootstrapWitnessList,
    ) -> Result<NonEmptyBootstrapWitnessOrderedSet, JsError> {
        let inner: Vec<cml_chain::crypto::BootstrapWitness> = list.clone().into();
        NonEmptyOrderedSet::try_from(inner)
            .map(Self)
            .map_err(|e| JsError::new(&e.to_string()))
    }
}

/// `[+ certificate] @duplicates reject`: an insertion-ordered, duplicate-free set (order preserved for byte-exact round-trip). `add` is checked — an already-present element is refused; construct via `try_from` (the uniqueness door). `insert` is the std-set door (returns `false`, set unchanged, for an already-present element); `contains` tests membership.
#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct NonEmptyCertificateOrderedSet(
    pub(crate) NonEmptyOrderedSet<cml_chain::certs::Certificate>,
);

impl_wasm_conversions!(
    NonEmptyOrderedSet<cml_chain::certs::Certificate>,
    NonEmptyCertificateOrderedSet
);

#[wasm_bindgen]
impl NonEmptyCertificateOrderedSet {
    pub fn new(first: &Certificate) -> Self {
        Self(NonEmptyOrderedSet::new(first.clone().into()))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Certificate {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &Certificate) -> Result<(), JsError> {
        self.0
            .push(elem.clone().into())
            .map_err(|e| JsError::new(&e.to_string()))
    }

    pub fn insert(&mut self, elem: &Certificate) -> bool {
        self.0.insert(elem.clone().into())
    }

    pub fn contains(&self, elem: &Certificate) -> bool {
        self.0.contains(&elem.clone().into())
    }

    pub fn try_from(list: &CertificateList) -> Result<NonEmptyCertificateOrderedSet, JsError> {
        let inner: Vec<cml_chain::certs::Certificate> = list.clone().into();
        NonEmptyOrderedSet::try_from(inner)
            .map(Self)
            .map_err(|e| JsError::new(&e.to_string()))
    }
}

/// `[+ ed25519_key_hash] @duplicates reject`: an insertion-ordered, duplicate-free set (order preserved for byte-exact round-trip). `add` is checked — an already-present element is refused; construct via `try_from` (the uniqueness door). `insert` is the std-set door (returns `false`, set unchanged, for an already-present element); `contains` tests membership.
#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct NonEmptyEd25519KeyHashOrderedSet(
    pub(crate) NonEmptyOrderedSet<cml_chain::crypto::Ed25519KeyHash>,
);

impl_wasm_conversions!(
    NonEmptyOrderedSet<cml_chain::crypto::Ed25519KeyHash>,
    NonEmptyEd25519KeyHashOrderedSet
);

#[wasm_bindgen]
impl NonEmptyEd25519KeyHashOrderedSet {
    pub fn new(first: &Ed25519KeyHash) -> Self {
        Self(NonEmptyOrderedSet::new(first.clone().into()))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Ed25519KeyHash {
        self.0[index].into()
    }

    pub fn add(&mut self, elem: &Ed25519KeyHash) -> Result<(), JsError> {
        self.0
            .push(elem.clone().into())
            .map_err(|e| JsError::new(&e.to_string()))
    }

    pub fn insert(&mut self, elem: &Ed25519KeyHash) -> bool {
        self.0.insert(elem.clone().into())
    }

    pub fn contains(&self, elem: &Ed25519KeyHash) -> bool {
        self.0.contains(&elem.clone().into())
    }

    pub fn try_from(
        list: &Ed25519KeyHashList,
    ) -> Result<NonEmptyEd25519KeyHashOrderedSet, JsError> {
        let inner: Vec<cml_chain::crypto::Ed25519KeyHash> = list.clone().into();
        NonEmptyOrderedSet::try_from(inner)
            .map(Self)
            .map_err(|e| JsError::new(&e.to_string()))
    }
}

/// `[+ LegacyRedeemer]`: at least one element, enforced by the `NonEmptyVec` representation.
/// Enter via `try_from` or `new(first)`.
/// `add` can never violate the bound; removal is checked in the core type.
#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct NonEmptyLegacyRedeemerList(pub(crate) NonEmptyVec<cml_chain::plutus::LegacyRedeemer>);

impl_wasm_conversions!(
    NonEmptyVec<cml_chain::plutus::LegacyRedeemer>,
    NonEmptyLegacyRedeemerList
);

#[wasm_bindgen]
impl NonEmptyLegacyRedeemerList {
    pub fn new(first: &LegacyRedeemer) -> Self {
        Self(NonEmptyVec::new(first.clone().into()))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> LegacyRedeemer {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &LegacyRedeemer) {
        self.0.push(elem.clone().into());
    }

    pub fn try_from(list: &LegacyRedeemerList) -> Result<NonEmptyLegacyRedeemerList, JsError> {
        let inner: Vec<cml_chain::plutus::LegacyRedeemer> = list.clone().into();
        NonEmptyVec::try_from(inner)
            .map(Self)
            .map_err(|e| JsError::new(&e.to_string()))
    }
}

/// `{+ k => v}` (`MapGovActionIdToVotingProcedure`): at least one entry, enforced by the `NonEmptyMap` representation.
/// Enter via `try_from` or `new(first_key, first_value)`.
/// `insert` can never violate the bound; removal is checked in the core type.
#[derive(Clone, Debug)]
#[wasm_bindgen]
// rustfmt::skip: rustfmt breaks after the field vis leaving trailing whitespace and errors
// (rust-lang/rustfmt#5703, fix PR #5708 unmerged). Remove when #5708 ships.
#[rustfmt::skip]
pub struct NonEmptyMapGovActionIdToVotingProcedure(
    pub(crate) NonEmptyMap<cml_chain::governance::GovActionId, cml_chain::governance::VotingProcedure>,
);

impl_wasm_conversions!(NonEmptyMap<cml_chain::governance::GovActionId, cml_chain::governance::VotingProcedure>, NonEmptyMapGovActionIdToVotingProcedure);

#[wasm_bindgen]
impl NonEmptyMapGovActionIdToVotingProcedure {
    pub fn new(first_key: &GovActionId, first_value: &VotingProcedure) -> Self {
        Self(NonEmptyMap::new(
            first_key.clone().into(),
            first_value.clone().into(),
        ))
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
        GovActionIdList(self.0.keys().cloned().collect::<Vec<_>>())
    }

    pub fn try_from(
        map: &MapGovActionIdToVotingProcedure,
    ) -> Result<NonEmptyMapGovActionIdToVotingProcedure, JsError> {
        let inner: OrderedHashMap<
            cml_chain::governance::GovActionId,
            cml_chain::governance::VotingProcedure,
        > = map.clone().into();
        NonEmptyMap::try_from(inner)
            .map(Self)
            .map_err(|e| JsError::new(&e.to_string()))
    }
}

/// `{+ k => v}` (`MapRedeemerKeyToRedeemerVal`): at least one entry, enforced by the `NonEmptyMap` representation.
/// Enter via `try_from` or `new(first_key, first_value)`.
/// `insert` can never violate the bound; removal is checked in the core type.
#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct NonEmptyMapRedeemerKeyToRedeemerVal(
    pub(crate) NonEmptyMap<cml_chain::plutus::RedeemerKey, cml_chain::plutus::RedeemerVal>,
);

impl_wasm_conversions!(NonEmptyMap<cml_chain::plutus::RedeemerKey, cml_chain::plutus::RedeemerVal>, NonEmptyMapRedeemerKeyToRedeemerVal);

#[wasm_bindgen]
impl NonEmptyMapRedeemerKeyToRedeemerVal {
    pub fn new(first_key: &RedeemerKey, first_value: &RedeemerVal) -> Self {
        Self(NonEmptyMap::new(
            first_key.clone().into(),
            first_value.clone().into(),
        ))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, key: &RedeemerKey, value: &RedeemerVal) -> Option<RedeemerVal> {
        self.0
            .insert(key.clone().into(), value.clone().into())
            .map(Into::into)
    }

    pub fn get(&self, key: &RedeemerKey) -> Option<RedeemerVal> {
        self.0.get(key.as_ref()).map(|v| v.clone().into())
    }

    pub fn keys(&self) -> RedeemerKeyList {
        RedeemerKeyList(self.0.keys().cloned().collect::<Vec<_>>())
    }

    pub fn try_from(
        map: &MapRedeemerKeyToRedeemerVal,
    ) -> Result<NonEmptyMapRedeemerKeyToRedeemerVal, JsError> {
        let inner: OrderedHashMap<cml_chain::plutus::RedeemerKey, cml_chain::plutus::RedeemerVal> =
            map.clone().into();
        NonEmptyMap::try_from(inner)
            .map(Self)
            .map_err(|e| JsError::new(&e.to_string()))
    }
}

/// `[+ native_script] @duplicates reject`: an insertion-ordered, duplicate-free set (order preserved for byte-exact round-trip). `add` is checked — an already-present element is refused; construct via `try_from` (the uniqueness door). `insert` is the std-set door (returns `false`, set unchanged, for an already-present element); `contains` tests membership.
#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct NonEmptyNativeScriptOrderedSet(
    pub(crate) NonEmptyOrderedSet<cml_chain::transaction::NativeScript>,
);

impl_wasm_conversions!(
    NonEmptyOrderedSet<cml_chain::transaction::NativeScript>,
    NonEmptyNativeScriptOrderedSet
);

#[wasm_bindgen]
impl NonEmptyNativeScriptOrderedSet {
    pub fn new(first: &NativeScript) -> Self {
        Self(NonEmptyOrderedSet::new(first.clone().into()))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> NativeScript {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &NativeScript) -> Result<(), JsError> {
        self.0
            .push(elem.clone().into())
            .map_err(|e| JsError::new(&e.to_string()))
    }

    pub fn insert(&mut self, elem: &NativeScript) -> bool {
        self.0.insert(elem.clone().into())
    }

    pub fn contains(&self, elem: &NativeScript) -> bool {
        self.0.contains(&elem.clone().into())
    }

    pub fn try_from(list: &NativeScriptList) -> Result<NonEmptyNativeScriptOrderedSet, JsError> {
        let inner: Vec<cml_chain::transaction::NativeScript> = list.clone().into();
        NonEmptyOrderedSet::try_from(inner)
            .map(Self)
            .map_err(|e| JsError::new(&e.to_string()))
    }
}

/// `[+ plutus_data] @duplicates reject`: an insertion-ordered, duplicate-free set (order preserved for byte-exact round-trip). `add` is checked — an already-present element is refused; construct via `try_from` (the uniqueness door). `insert` is the std-set door (returns `false`, set unchanged, for an already-present element); `contains` tests membership.
#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct NonEmptyPlutusDataOrderedSet(
    pub(crate) NonEmptyOrderedSet<cml_chain::plutus::PlutusData>,
);

impl_wasm_conversions!(
    NonEmptyOrderedSet<cml_chain::plutus::PlutusData>,
    NonEmptyPlutusDataOrderedSet
);

#[wasm_bindgen]
impl NonEmptyPlutusDataOrderedSet {
    pub fn new(first: &PlutusData) -> Self {
        Self(NonEmptyOrderedSet::new(first.clone().into()))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> PlutusData {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &PlutusData) -> Result<(), JsError> {
        self.0
            .push(elem.clone().into())
            .map_err(|e| JsError::new(&e.to_string()))
    }

    pub fn insert(&mut self, elem: &PlutusData) -> bool {
        self.0.insert(elem.clone().into())
    }

    pub fn contains(&self, elem: &PlutusData) -> bool {
        self.0.contains(&elem.clone().into())
    }

    pub fn try_from(list: &PlutusDataList) -> Result<NonEmptyPlutusDataOrderedSet, JsError> {
        let inner: Vec<cml_chain::plutus::PlutusData> = list.clone().into();
        NonEmptyOrderedSet::try_from(inner)
            .map(Self)
            .map_err(|e| JsError::new(&e.to_string()))
    }
}

/// `[+ plutus_v1_script] @duplicates reject`: an insertion-ordered, duplicate-free set (order preserved for byte-exact round-trip). `add` is checked — an already-present element is refused; construct via `try_from` (the uniqueness door). `insert` is the std-set door (returns `false`, set unchanged, for an already-present element); `contains` tests membership.
#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct NonEmptyPlutusV1ScriptOrderedSet(
    pub(crate) NonEmptyOrderedSet<cml_chain::plutus::PlutusV1Script>,
);

impl_wasm_conversions!(
    NonEmptyOrderedSet<cml_chain::plutus::PlutusV1Script>,
    NonEmptyPlutusV1ScriptOrderedSet
);

#[wasm_bindgen]
impl NonEmptyPlutusV1ScriptOrderedSet {
    pub fn new(first: &PlutusV1Script) -> Self {
        Self(NonEmptyOrderedSet::new(first.clone().into()))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> PlutusV1Script {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &PlutusV1Script) -> Result<(), JsError> {
        self.0
            .push(elem.clone().into())
            .map_err(|e| JsError::new(&e.to_string()))
    }

    pub fn insert(&mut self, elem: &PlutusV1Script) -> bool {
        self.0.insert(elem.clone().into())
    }

    pub fn contains(&self, elem: &PlutusV1Script) -> bool {
        self.0.contains(&elem.clone().into())
    }

    pub fn try_from(
        list: &PlutusV1ScriptList,
    ) -> Result<NonEmptyPlutusV1ScriptOrderedSet, JsError> {
        let inner: Vec<cml_chain::plutus::PlutusV1Script> = list.clone().into();
        NonEmptyOrderedSet::try_from(inner)
            .map(Self)
            .map_err(|e| JsError::new(&e.to_string()))
    }
}

/// `[+ plutus_v2_script] @duplicates reject`: an insertion-ordered, duplicate-free set (order preserved for byte-exact round-trip). `add` is checked — an already-present element is refused; construct via `try_from` (the uniqueness door). `insert` is the std-set door (returns `false`, set unchanged, for an already-present element); `contains` tests membership.
#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct NonEmptyPlutusV2ScriptOrderedSet(
    pub(crate) NonEmptyOrderedSet<cml_chain::plutus::PlutusV2Script>,
);

impl_wasm_conversions!(
    NonEmptyOrderedSet<cml_chain::plutus::PlutusV2Script>,
    NonEmptyPlutusV2ScriptOrderedSet
);

#[wasm_bindgen]
impl NonEmptyPlutusV2ScriptOrderedSet {
    pub fn new(first: &PlutusV2Script) -> Self {
        Self(NonEmptyOrderedSet::new(first.clone().into()))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> PlutusV2Script {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &PlutusV2Script) -> Result<(), JsError> {
        self.0
            .push(elem.clone().into())
            .map_err(|e| JsError::new(&e.to_string()))
    }

    pub fn insert(&mut self, elem: &PlutusV2Script) -> bool {
        self.0.insert(elem.clone().into())
    }

    pub fn contains(&self, elem: &PlutusV2Script) -> bool {
        self.0.contains(&elem.clone().into())
    }

    pub fn try_from(
        list: &PlutusV2ScriptList,
    ) -> Result<NonEmptyPlutusV2ScriptOrderedSet, JsError> {
        let inner: Vec<cml_chain::plutus::PlutusV2Script> = list.clone().into();
        NonEmptyOrderedSet::try_from(inner)
            .map(Self)
            .map_err(|e| JsError::new(&e.to_string()))
    }
}

/// `[+ plutus_v3_script] @duplicates reject`: an insertion-ordered, duplicate-free set (order preserved for byte-exact round-trip). `add` is checked — an already-present element is refused; construct via `try_from` (the uniqueness door). `insert` is the std-set door (returns `false`, set unchanged, for an already-present element); `contains` tests membership.
#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct NonEmptyPlutusV3ScriptOrderedSet(
    pub(crate) NonEmptyOrderedSet<cml_chain::plutus::PlutusV3Script>,
);

impl_wasm_conversions!(
    NonEmptyOrderedSet<cml_chain::plutus::PlutusV3Script>,
    NonEmptyPlutusV3ScriptOrderedSet
);

#[wasm_bindgen]
impl NonEmptyPlutusV3ScriptOrderedSet {
    pub fn new(first: &PlutusV3Script) -> Self {
        Self(NonEmptyOrderedSet::new(first.clone().into()))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> PlutusV3Script {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &PlutusV3Script) -> Result<(), JsError> {
        self.0
            .push(elem.clone().into())
            .map_err(|e| JsError::new(&e.to_string()))
    }

    pub fn insert(&mut self, elem: &PlutusV3Script) -> bool {
        self.0.insert(elem.clone().into())
    }

    pub fn contains(&self, elem: &PlutusV3Script) -> bool {
        self.0.contains(&elem.clone().into())
    }

    pub fn try_from(
        list: &PlutusV3ScriptList,
    ) -> Result<NonEmptyPlutusV3ScriptOrderedSet, JsError> {
        let inner: Vec<cml_chain::plutus::PlutusV3Script> = list.clone().into();
        NonEmptyOrderedSet::try_from(inner)
            .map(Self)
            .map_err(|e| JsError::new(&e.to_string()))
    }
}

/// `[+ proposal_procedure] @duplicates reject`: an insertion-ordered, duplicate-free set (order preserved for byte-exact round-trip). `add` is checked — an already-present element is refused; construct via `try_from` (the uniqueness door). `insert` is the std-set door (returns `false`, set unchanged, for an already-present element); `contains` tests membership.
#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct NonEmptyProposalProcedureOrderedSet(
    pub(crate) NonEmptyOrderedSet<cml_chain::governance::ProposalProcedure>,
);

impl_wasm_conversions!(
    NonEmptyOrderedSet<cml_chain::governance::ProposalProcedure>,
    NonEmptyProposalProcedureOrderedSet
);

#[wasm_bindgen]
impl NonEmptyProposalProcedureOrderedSet {
    pub fn new(first: &ProposalProcedure) -> Self {
        Self(NonEmptyOrderedSet::new(first.clone().into()))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> ProposalProcedure {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &ProposalProcedure) -> Result<(), JsError> {
        self.0
            .push(elem.clone().into())
            .map_err(|e| JsError::new(&e.to_string()))
    }

    pub fn insert(&mut self, elem: &ProposalProcedure) -> bool {
        self.0.insert(elem.clone().into())
    }

    pub fn contains(&self, elem: &ProposalProcedure) -> bool {
        self.0.contains(&elem.clone().into())
    }

    pub fn try_from(
        list: &ProposalProcedureList,
    ) -> Result<NonEmptyProposalProcedureOrderedSet, JsError> {
        let inner: Vec<cml_chain::governance::ProposalProcedure> = list.clone().into();
        NonEmptyOrderedSet::try_from(inner)
            .map(Self)
            .map_err(|e| JsError::new(&e.to_string()))
    }
}

/// `[+ transaction_input] @duplicates reject`: an insertion-ordered, duplicate-free set (order preserved for byte-exact round-trip). `add` is checked — an already-present element is refused; construct via `try_from` (the uniqueness door). `insert` is the std-set door (returns `false`, set unchanged, for an already-present element); `contains` tests membership.
#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct NonEmptyTransactionInputOrderedSet(
    pub(crate) NonEmptyOrderedSet<cml_chain::transaction::TransactionInput>,
);

impl_wasm_conversions!(
    NonEmptyOrderedSet<cml_chain::transaction::TransactionInput>,
    NonEmptyTransactionInputOrderedSet
);

#[wasm_bindgen]
impl NonEmptyTransactionInputOrderedSet {
    pub fn new(first: &TransactionInput) -> Self {
        Self(NonEmptyOrderedSet::new(first.clone().into()))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> TransactionInput {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &TransactionInput) -> Result<(), JsError> {
        self.0
            .push(elem.clone().into())
            .map_err(|e| JsError::new(&e.to_string()))
    }

    pub fn insert(&mut self, elem: &TransactionInput) -> bool {
        self.0.insert(elem.clone().into())
    }

    pub fn contains(&self, elem: &TransactionInput) -> bool {
        self.0.contains(&elem.clone().into())
    }

    pub fn try_from(
        list: &TransactionInputList,
    ) -> Result<NonEmptyTransactionInputOrderedSet, JsError> {
        let inner: Vec<cml_chain::transaction::TransactionInput> = list.clone().into();
        NonEmptyOrderedSet::try_from(inner)
            .map(Self)
            .map_err(|e| JsError::new(&e.to_string()))
    }
}

/// `[+ vkeywitness] @duplicates reject`: an insertion-ordered, duplicate-free set (order preserved for byte-exact round-trip). `add` is checked — an already-present element is refused; construct via `try_from` (the uniqueness door). `insert` is the std-set door (returns `false`, set unchanged, for an already-present element); `contains` tests membership.
#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct NonEmptyVkeywitnessOrderedSet(
    pub(crate) NonEmptyOrderedSet<cml_chain::crypto::Vkeywitness>,
);

impl_wasm_conversions!(
    NonEmptyOrderedSet<cml_chain::crypto::Vkeywitness>,
    NonEmptyVkeywitnessOrderedSet
);

#[wasm_bindgen]
impl NonEmptyVkeywitnessOrderedSet {
    pub fn new(first: &Vkeywitness) -> Self {
        Self(NonEmptyOrderedSet::new(first.clone().into()))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Vkeywitness {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &Vkeywitness) -> Result<(), JsError> {
        self.0
            .push(elem.clone().into())
            .map_err(|e| JsError::new(&e.to_string()))
    }

    pub fn insert(&mut self, elem: &Vkeywitness) -> bool {
        self.0.insert(elem.clone().into())
    }

    pub fn contains(&self, elem: &Vkeywitness) -> bool {
        self.0.contains(&elem.clone().into())
    }

    pub fn try_from(list: &VkeywitnessList) -> Result<NonEmptyVkeywitnessOrderedSet, JsError> {
        let inner: Vec<cml_chain::crypto::Vkeywitness> = list.clone().into();
        NonEmptyOrderedSet::try_from(inner)
            .map(Self)
            .map_err(|e| JsError::new(&e.to_string()))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct NonemptySetBootstrapWitness(pub(crate) cml_chain::NonemptySetBootstrapWitness);

impl_wasm_cbor_json_api!(NonemptySetBootstrapWitness);

impl_wasm_conversions!(
    cml_chain::NonemptySetBootstrapWitness,
    NonemptySetBootstrapWitness
);

#[wasm_bindgen]
impl NonemptySetBootstrapWitness {
    pub fn new(inner: &NonEmptyBootstrapWitnessOrderedSet) -> Self {
        Self(cml_chain::NonemptySetBootstrapWitness::new(
            inner.clone().into(),
        ))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> BootstrapWitness {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &BootstrapWitness) -> Result<(), JsError> {
        self.0
            .push(elem.clone().into())
            .map_err(|e| JsError::new(&e.to_string()))
    }

    pub fn insert(&mut self, elem: &BootstrapWitness) -> bool {
        self.0.insert(elem.clone().into())
    }

    pub fn contains(&self, elem: &BootstrapWitness) -> bool {
        self.0.contains(&elem.clone().into())
    }

    pub fn try_from(list: &BootstrapWitnessList) -> Result<NonemptySetBootstrapWitness, JsError> {
        let list: Vec<cml_chain::crypto::BootstrapWitness> = list.clone().into();
        cml_chain::NonemptySetBootstrapWitness::try_from(list)
            .map(Self)
            .map_err(|e| JsError::new(&e.to_string()))
    }

    pub fn try_opt_from(
        list: &BootstrapWitnessList,
    ) -> Result<Option<NonemptySetBootstrapWitness>, JsError> {
        let list: Vec<cml_chain::crypto::BootstrapWitness> = list.clone().into();
        cml_chain::NonemptySetBootstrapWitness::try_opt_from(list)
            .map(|opt| opt.map(Self))
            .map_err(|e| JsError::new(&e.to_string()))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct NonemptySetCertificate(pub(crate) cml_chain::NonemptySetCertificate);

impl_wasm_cbor_json_api!(NonemptySetCertificate);

impl_wasm_conversions!(cml_chain::NonemptySetCertificate, NonemptySetCertificate);

#[wasm_bindgen]
impl NonemptySetCertificate {
    pub fn new(inner: &NonEmptyCertificateOrderedSet) -> Self {
        Self(cml_chain::NonemptySetCertificate::new(inner.clone().into()))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Certificate {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &Certificate) -> Result<(), JsError> {
        self.0
            .push(elem.clone().into())
            .map_err(|e| JsError::new(&e.to_string()))
    }

    pub fn insert(&mut self, elem: &Certificate) -> bool {
        self.0.insert(elem.clone().into())
    }

    pub fn contains(&self, elem: &Certificate) -> bool {
        self.0.contains(&elem.clone().into())
    }

    pub fn try_from(list: &CertificateList) -> Result<NonemptySetCertificate, JsError> {
        let list: Vec<cml_chain::certs::Certificate> = list.clone().into();
        cml_chain::NonemptySetCertificate::try_from(list)
            .map(Self)
            .map_err(|e| JsError::new(&e.to_string()))
    }

    pub fn try_opt_from(list: &CertificateList) -> Result<Option<NonemptySetCertificate>, JsError> {
        let list: Vec<cml_chain::certs::Certificate> = list.clone().into();
        cml_chain::NonemptySetCertificate::try_opt_from(list)
            .map(|opt| opt.map(Self))
            .map_err(|e| JsError::new(&e.to_string()))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct NonemptySetEd25519KeyHash(pub(crate) cml_chain::NonemptySetEd25519KeyHash);

impl_wasm_cbor_json_api!(NonemptySetEd25519KeyHash);

impl_wasm_conversions!(
    cml_chain::NonemptySetEd25519KeyHash,
    NonemptySetEd25519KeyHash
);

#[wasm_bindgen]
impl NonemptySetEd25519KeyHash {
    pub fn new(inner: &NonEmptyEd25519KeyHashOrderedSet) -> Self {
        Self(cml_chain::NonemptySetEd25519KeyHash::new(
            inner.clone().into(),
        ))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Ed25519KeyHash {
        self.0[index].into()
    }

    pub fn add(&mut self, elem: &Ed25519KeyHash) -> Result<(), JsError> {
        self.0
            .push(elem.clone().into())
            .map_err(|e| JsError::new(&e.to_string()))
    }

    pub fn insert(&mut self, elem: &Ed25519KeyHash) -> bool {
        self.0.insert(elem.clone().into())
    }

    pub fn contains(&self, elem: &Ed25519KeyHash) -> bool {
        self.0.contains(&elem.clone().into())
    }

    pub fn try_from(list: &Ed25519KeyHashList) -> Result<NonemptySetEd25519KeyHash, JsError> {
        let list: Vec<cml_chain::crypto::Ed25519KeyHash> = list.clone().into();
        cml_chain::NonemptySetEd25519KeyHash::try_from(list)
            .map(Self)
            .map_err(|e| JsError::new(&e.to_string()))
    }

    pub fn try_opt_from(
        list: &Ed25519KeyHashList,
    ) -> Result<Option<NonemptySetEd25519KeyHash>, JsError> {
        let list: Vec<cml_chain::crypto::Ed25519KeyHash> = list.clone().into();
        cml_chain::NonemptySetEd25519KeyHash::try_opt_from(list)
            .map(|opt| opt.map(Self))
            .map_err(|e| JsError::new(&e.to_string()))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct NonemptySetNativeScript(pub(crate) cml_chain::NonemptySetNativeScript);

impl_wasm_cbor_json_api!(NonemptySetNativeScript);

impl_wasm_conversions!(cml_chain::NonemptySetNativeScript, NonemptySetNativeScript);

#[wasm_bindgen]
impl NonemptySetNativeScript {
    pub fn new(inner: &NonEmptyNativeScriptOrderedSet) -> Self {
        Self(cml_chain::NonemptySetNativeScript::new(
            inner.clone().into(),
        ))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> NativeScript {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &NativeScript) -> Result<(), JsError> {
        self.0
            .push(elem.clone().into())
            .map_err(|e| JsError::new(&e.to_string()))
    }

    pub fn insert(&mut self, elem: &NativeScript) -> bool {
        self.0.insert(elem.clone().into())
    }

    pub fn contains(&self, elem: &NativeScript) -> bool {
        self.0.contains(&elem.clone().into())
    }

    pub fn try_from(list: &NativeScriptList) -> Result<NonemptySetNativeScript, JsError> {
        let list: Vec<cml_chain::transaction::NativeScript> = list.clone().into();
        cml_chain::NonemptySetNativeScript::try_from(list)
            .map(Self)
            .map_err(|e| JsError::new(&e.to_string()))
    }

    pub fn try_opt_from(
        list: &NativeScriptList,
    ) -> Result<Option<NonemptySetNativeScript>, JsError> {
        let list: Vec<cml_chain::transaction::NativeScript> = list.clone().into();
        cml_chain::NonemptySetNativeScript::try_opt_from(list)
            .map(|opt| opt.map(Self))
            .map_err(|e| JsError::new(&e.to_string()))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct NonemptySetPlutusData(pub(crate) cml_chain::NonemptySetPlutusData);

impl_wasm_cbor_json_api!(NonemptySetPlutusData);

impl_wasm_conversions!(cml_chain::NonemptySetPlutusData, NonemptySetPlutusData);

#[wasm_bindgen]
impl NonemptySetPlutusData {
    pub fn new(inner: &NonEmptyPlutusDataOrderedSet) -> Self {
        Self(cml_chain::NonemptySetPlutusData::new(inner.clone().into()))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> PlutusData {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &PlutusData) -> Result<(), JsError> {
        self.0
            .push(elem.clone().into())
            .map_err(|e| JsError::new(&e.to_string()))
    }

    pub fn insert(&mut self, elem: &PlutusData) -> bool {
        self.0.insert(elem.clone().into())
    }

    pub fn contains(&self, elem: &PlutusData) -> bool {
        self.0.contains(&elem.clone().into())
    }

    pub fn try_from(list: &PlutusDataList) -> Result<NonemptySetPlutusData, JsError> {
        let list: Vec<cml_chain::plutus::PlutusData> = list.clone().into();
        cml_chain::NonemptySetPlutusData::try_from(list)
            .map(Self)
            .map_err(|e| JsError::new(&e.to_string()))
    }

    pub fn try_opt_from(list: &PlutusDataList) -> Result<Option<NonemptySetPlutusData>, JsError> {
        let list: Vec<cml_chain::plutus::PlutusData> = list.clone().into();
        cml_chain::NonemptySetPlutusData::try_opt_from(list)
            .map(|opt| opt.map(Self))
            .map_err(|e| JsError::new(&e.to_string()))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct NonemptySetPlutusV1Script(pub(crate) cml_chain::NonemptySetPlutusV1Script);

impl_wasm_cbor_json_api!(NonemptySetPlutusV1Script);

impl_wasm_conversions!(
    cml_chain::NonemptySetPlutusV1Script,
    NonemptySetPlutusV1Script
);

#[wasm_bindgen]
impl NonemptySetPlutusV1Script {
    pub fn new(inner: &NonEmptyPlutusV1ScriptOrderedSet) -> Self {
        Self(cml_chain::NonemptySetPlutusV1Script::new(
            inner.clone().into(),
        ))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> PlutusV1Script {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &PlutusV1Script) -> Result<(), JsError> {
        self.0
            .push(elem.clone().into())
            .map_err(|e| JsError::new(&e.to_string()))
    }

    pub fn insert(&mut self, elem: &PlutusV1Script) -> bool {
        self.0.insert(elem.clone().into())
    }

    pub fn contains(&self, elem: &PlutusV1Script) -> bool {
        self.0.contains(&elem.clone().into())
    }

    pub fn try_from(list: &PlutusV1ScriptList) -> Result<NonemptySetPlutusV1Script, JsError> {
        let list: Vec<cml_chain::plutus::PlutusV1Script> = list.clone().into();
        cml_chain::NonemptySetPlutusV1Script::try_from(list)
            .map(Self)
            .map_err(|e| JsError::new(&e.to_string()))
    }

    pub fn try_opt_from(
        list: &PlutusV1ScriptList,
    ) -> Result<Option<NonemptySetPlutusV1Script>, JsError> {
        let list: Vec<cml_chain::plutus::PlutusV1Script> = list.clone().into();
        cml_chain::NonemptySetPlutusV1Script::try_opt_from(list)
            .map(|opt| opt.map(Self))
            .map_err(|e| JsError::new(&e.to_string()))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct NonemptySetPlutusV2Script(pub(crate) cml_chain::NonemptySetPlutusV2Script);

impl_wasm_cbor_json_api!(NonemptySetPlutusV2Script);

impl_wasm_conversions!(
    cml_chain::NonemptySetPlutusV2Script,
    NonemptySetPlutusV2Script
);

#[wasm_bindgen]
impl NonemptySetPlutusV2Script {
    pub fn new(inner: &NonEmptyPlutusV2ScriptOrderedSet) -> Self {
        Self(cml_chain::NonemptySetPlutusV2Script::new(
            inner.clone().into(),
        ))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> PlutusV2Script {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &PlutusV2Script) -> Result<(), JsError> {
        self.0
            .push(elem.clone().into())
            .map_err(|e| JsError::new(&e.to_string()))
    }

    pub fn insert(&mut self, elem: &PlutusV2Script) -> bool {
        self.0.insert(elem.clone().into())
    }

    pub fn contains(&self, elem: &PlutusV2Script) -> bool {
        self.0.contains(&elem.clone().into())
    }

    pub fn try_from(list: &PlutusV2ScriptList) -> Result<NonemptySetPlutusV2Script, JsError> {
        let list: Vec<cml_chain::plutus::PlutusV2Script> = list.clone().into();
        cml_chain::NonemptySetPlutusV2Script::try_from(list)
            .map(Self)
            .map_err(|e| JsError::new(&e.to_string()))
    }

    pub fn try_opt_from(
        list: &PlutusV2ScriptList,
    ) -> Result<Option<NonemptySetPlutusV2Script>, JsError> {
        let list: Vec<cml_chain::plutus::PlutusV2Script> = list.clone().into();
        cml_chain::NonemptySetPlutusV2Script::try_opt_from(list)
            .map(|opt| opt.map(Self))
            .map_err(|e| JsError::new(&e.to_string()))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct NonemptySetPlutusV3Script(pub(crate) cml_chain::NonemptySetPlutusV3Script);

impl_wasm_cbor_json_api!(NonemptySetPlutusV3Script);

impl_wasm_conversions!(
    cml_chain::NonemptySetPlutusV3Script,
    NonemptySetPlutusV3Script
);

#[wasm_bindgen]
impl NonemptySetPlutusV3Script {
    pub fn new(inner: &NonEmptyPlutusV3ScriptOrderedSet) -> Self {
        Self(cml_chain::NonemptySetPlutusV3Script::new(
            inner.clone().into(),
        ))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> PlutusV3Script {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &PlutusV3Script) -> Result<(), JsError> {
        self.0
            .push(elem.clone().into())
            .map_err(|e| JsError::new(&e.to_string()))
    }

    pub fn insert(&mut self, elem: &PlutusV3Script) -> bool {
        self.0.insert(elem.clone().into())
    }

    pub fn contains(&self, elem: &PlutusV3Script) -> bool {
        self.0.contains(&elem.clone().into())
    }

    pub fn try_from(list: &PlutusV3ScriptList) -> Result<NonemptySetPlutusV3Script, JsError> {
        let list: Vec<cml_chain::plutus::PlutusV3Script> = list.clone().into();
        cml_chain::NonemptySetPlutusV3Script::try_from(list)
            .map(Self)
            .map_err(|e| JsError::new(&e.to_string()))
    }

    pub fn try_opt_from(
        list: &PlutusV3ScriptList,
    ) -> Result<Option<NonemptySetPlutusV3Script>, JsError> {
        let list: Vec<cml_chain::plutus::PlutusV3Script> = list.clone().into();
        cml_chain::NonemptySetPlutusV3Script::try_opt_from(list)
            .map(|opt| opt.map(Self))
            .map_err(|e| JsError::new(&e.to_string()))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct NonemptySetProposalProcedure(pub(crate) cml_chain::NonemptySetProposalProcedure);

impl_wasm_cbor_json_api!(NonemptySetProposalProcedure);

impl_wasm_conversions!(
    cml_chain::NonemptySetProposalProcedure,
    NonemptySetProposalProcedure
);

#[wasm_bindgen]
impl NonemptySetProposalProcedure {
    pub fn new(inner: &NonEmptyProposalProcedureOrderedSet) -> Self {
        Self(cml_chain::NonemptySetProposalProcedure::new(
            inner.clone().into(),
        ))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> ProposalProcedure {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &ProposalProcedure) -> Result<(), JsError> {
        self.0
            .push(elem.clone().into())
            .map_err(|e| JsError::new(&e.to_string()))
    }

    pub fn insert(&mut self, elem: &ProposalProcedure) -> bool {
        self.0.insert(elem.clone().into())
    }

    pub fn contains(&self, elem: &ProposalProcedure) -> bool {
        self.0.contains(&elem.clone().into())
    }

    pub fn try_from(list: &ProposalProcedureList) -> Result<NonemptySetProposalProcedure, JsError> {
        let list: Vec<cml_chain::governance::ProposalProcedure> = list.clone().into();
        cml_chain::NonemptySetProposalProcedure::try_from(list)
            .map(Self)
            .map_err(|e| JsError::new(&e.to_string()))
    }

    pub fn try_opt_from(
        list: &ProposalProcedureList,
    ) -> Result<Option<NonemptySetProposalProcedure>, JsError> {
        let list: Vec<cml_chain::governance::ProposalProcedure> = list.clone().into();
        cml_chain::NonemptySetProposalProcedure::try_opt_from(list)
            .map(|opt| opt.map(Self))
            .map_err(|e| JsError::new(&e.to_string()))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct NonemptySetTransactionInput(pub(crate) cml_chain::NonemptySetTransactionInput);

impl_wasm_cbor_json_api!(NonemptySetTransactionInput);

impl_wasm_conversions!(
    cml_chain::NonemptySetTransactionInput,
    NonemptySetTransactionInput
);

#[wasm_bindgen]
impl NonemptySetTransactionInput {
    pub fn new(inner: &NonEmptyTransactionInputOrderedSet) -> Self {
        Self(cml_chain::NonemptySetTransactionInput::new(
            inner.clone().into(),
        ))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> TransactionInput {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &TransactionInput) -> Result<(), JsError> {
        self.0
            .push(elem.clone().into())
            .map_err(|e| JsError::new(&e.to_string()))
    }

    pub fn insert(&mut self, elem: &TransactionInput) -> bool {
        self.0.insert(elem.clone().into())
    }

    pub fn contains(&self, elem: &TransactionInput) -> bool {
        self.0.contains(&elem.clone().into())
    }

    pub fn try_from(list: &TransactionInputList) -> Result<NonemptySetTransactionInput, JsError> {
        let list: Vec<cml_chain::transaction::TransactionInput> = list.clone().into();
        cml_chain::NonemptySetTransactionInput::try_from(list)
            .map(Self)
            .map_err(|e| JsError::new(&e.to_string()))
    }

    pub fn try_opt_from(
        list: &TransactionInputList,
    ) -> Result<Option<NonemptySetTransactionInput>, JsError> {
        let list: Vec<cml_chain::transaction::TransactionInput> = list.clone().into();
        cml_chain::NonemptySetTransactionInput::try_opt_from(list)
            .map(|opt| opt.map(Self))
            .map_err(|e| JsError::new(&e.to_string()))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct NonemptySetVkeywitness(pub(crate) cml_chain::NonemptySetVkeywitness);

impl_wasm_cbor_json_api!(NonemptySetVkeywitness);

impl_wasm_conversions!(cml_chain::NonemptySetVkeywitness, NonemptySetVkeywitness);

#[wasm_bindgen]
impl NonemptySetVkeywitness {
    pub fn new(inner: &NonEmptyVkeywitnessOrderedSet) -> Self {
        Self(cml_chain::NonemptySetVkeywitness::new(inner.clone().into()))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Vkeywitness {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &Vkeywitness) -> Result<(), JsError> {
        self.0
            .push(elem.clone().into())
            .map_err(|e| JsError::new(&e.to_string()))
    }

    pub fn insert(&mut self, elem: &Vkeywitness) -> bool {
        self.0.insert(elem.clone().into())
    }

    pub fn contains(&self, elem: &Vkeywitness) -> bool {
        self.0.contains(&elem.clone().into())
    }

    pub fn try_from(list: &VkeywitnessList) -> Result<NonemptySetVkeywitness, JsError> {
        let list: Vec<cml_chain::crypto::Vkeywitness> = list.clone().into();
        cml_chain::NonemptySetVkeywitness::try_from(list)
            .map(Self)
            .map_err(|e| JsError::new(&e.to_string()))
    }

    pub fn try_opt_from(list: &VkeywitnessList) -> Result<Option<NonemptySetVkeywitness>, JsError> {
        let list: Vec<cml_chain::crypto::Vkeywitness> = list.clone().into();
        cml_chain::NonemptySetVkeywitness::try_opt_from(list)
            .map(|opt| opt.map(Self))
            .map_err(|e| JsError::new(&e.to_string()))
    }
}

impl_wasm_list_needs_into!(
    cml_chain::plutus::PlutusData,
    PlutusData,
    PlutusDataList,
    true,
    false
);

impl_wasm_list_needs_into!(
    cml_chain::plutus::PlutusV1Script,
    PlutusV1Script,
    PlutusV1ScriptList,
    true,
    false
);

impl_wasm_list_needs_into!(
    cml_chain::plutus::PlutusV2Script,
    PlutusV2Script,
    PlutusV2ScriptList,
    true,
    false
);

impl_wasm_list_needs_into!(
    cml_chain::plutus::PlutusV3Script,
    PlutusV3Script,
    PlutusV3ScriptList,
    true,
    false
);

pub type PolicyId = ScriptHash;

impl_wasm_list_needs_into!(cml_chain::PolicyId, PolicyId, PolicyIdList, true, true);

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct PoolVotingThresholds(pub(crate) cml_chain::PoolVotingThresholds);

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

    pub fn security_relevant_parameter_voting_threshold(&self) -> UnitInterval {
        self.0
            .security_relevant_parameter_voting_threshold
            .clone()
            .into()
    }

    pub fn new(
        motion_no_confidence: &UnitInterval,
        committee_normal: &UnitInterval,
        committee_no_confidence: &UnitInterval,
        hard_fork_initiation: &UnitInterval,
        security_relevant_parameter_voting_threshold: &UnitInterval,
    ) -> Self {
        Self(cml_chain::PoolVotingThresholds::new(
            motion_no_confidence.clone().into(),
            committee_normal.clone().into(),
            committee_no_confidence.clone().into(),
            hard_fork_initiation.clone().into(),
            security_relevant_parameter_voting_threshold.clone().into(),
        ))
    }
}

pub type Port = u16;

impl_wasm_list_needs_into!(
    cml_chain::governance::ProposalProcedure,
    ProposalProcedure,
    ProposalProcedureList,
    true,
    false
);

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ProtocolParamUpdate(pub(crate) cml_chain::ProtocolParamUpdate);

impl_wasm_cbor_json_api!(ProtocolParamUpdate);

impl_wasm_conversions!(cml_chain::ProtocolParamUpdate, ProtocolParamUpdate);

#[wasm_bindgen]
impl ProtocolParamUpdate {
    pub fn set_minfee_a(&mut self, minfee_a: Coin) {
        self.0.minfee_a = Some(minfee_a)
    }

    pub fn minfee_a(&self) -> Option<Coin> {
        self.0.minfee_a
    }

    pub fn set_minfee_b(&mut self, minfee_b: Coin) {
        self.0.minfee_b = Some(minfee_b)
    }

    pub fn minfee_b(&self) -> Option<Coin> {
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

    pub fn set_committee_term_limit(&mut self, committee_term_limit: Epoch) {
        self.0.committee_term_limit = Some(committee_term_limit)
    }

    pub fn committee_term_limit(&self) -> Option<Epoch> {
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

    pub fn set_min_fee_ref_script_cost_per_byte(
        &mut self,
        min_fee_ref_script_cost_per_byte: &Rational,
    ) {
        self.0.min_fee_ref_script_cost_per_byte =
            Some(min_fee_ref_script_cost_per_byte.clone().into())
    }

    pub fn min_fee_ref_script_cost_per_byte(&self) -> Option<Rational> {
        self.0
            .min_fee_ref_script_cost_per_byte
            .clone()
            .map(std::convert::Into::into)
    }

    pub fn new() -> Self {
        Self(cml_chain::ProtocolParamUpdate::new())
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct Rational(pub(crate) cml_chain::Rational);

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

impl_wasm_list_needs_into!(
    cml_chain::plutus::RedeemerKey,
    RedeemerKey,
    RedeemerKeyList,
    true,
    false
);

impl_wasm_list_needs_into!(cml_chain::certs::Relay, Relay, RelayList, true, false);

impl_wasm_list_needs_into!(
    cml_chain::address::RewardAccount,
    RewardAccount,
    RewardAccountList,
    true,
    false
);

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct SetCommitteeColdCredential(pub(crate) cml_chain::SetCommitteeColdCredential);

impl_wasm_cbor_json_api!(SetCommitteeColdCredential);

impl_wasm_conversions!(
    cml_chain::SetCommitteeColdCredential,
    SetCommitteeColdCredential
);

#[wasm_bindgen]
impl SetCommitteeColdCredential {
    pub fn new(inner: &CommitteeColdCredentialOrderedSet) -> Self {
        Self(cml_chain::SetCommitteeColdCredential::new(
            inner.clone().into(),
        ))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> CommitteeColdCredential {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &CommitteeColdCredential) -> Result<(), JsError> {
        self.0
            .push(elem.clone().into())
            .map_err(|e| JsError::new(&e.to_string()))
    }

    pub fn insert(&mut self, elem: &CommitteeColdCredential) -> bool {
        self.0.insert(elem.clone().into())
    }

    pub fn contains(&self, elem: &CommitteeColdCredential) -> bool {
        self.0.contains(&elem.clone().into())
    }

    pub fn try_from(
        list: &CommitteeColdCredentialList,
    ) -> Result<SetCommitteeColdCredential, JsError> {
        let list: Vec<cml_chain::certs::CommitteeColdCredential> = list.clone().into();
        cml_chain::SetCommitteeColdCredential::try_from(list)
            .map(Self)
            .map_err(|e| JsError::new(&e.to_string()))
    }

    pub fn try_opt_from(
        list: &CommitteeColdCredentialList,
    ) -> Result<Option<SetCommitteeColdCredential>, JsError> {
        let list: Vec<cml_chain::certs::CommitteeColdCredential> = list.clone().into();
        cml_chain::SetCommitteeColdCredential::try_opt_from(list)
            .map(|opt| opt.map(Self))
            .map_err(|e| JsError::new(&e.to_string()))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct SetEd25519KeyHash(pub(crate) cml_chain::SetEd25519KeyHash);

impl_wasm_cbor_json_api!(SetEd25519KeyHash);

impl_wasm_conversions!(cml_chain::SetEd25519KeyHash, SetEd25519KeyHash);

#[wasm_bindgen]
impl SetEd25519KeyHash {
    pub fn new(inner: &Ed25519KeyHashOrderedSet) -> Self {
        Self(cml_chain::SetEd25519KeyHash::new(inner.clone().into()))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Ed25519KeyHash {
        self.0[index].into()
    }

    pub fn add(&mut self, elem: &Ed25519KeyHash) -> Result<(), JsError> {
        self.0
            .push(elem.clone().into())
            .map_err(|e| JsError::new(&e.to_string()))
    }

    pub fn insert(&mut self, elem: &Ed25519KeyHash) -> bool {
        self.0.insert(elem.clone().into())
    }

    pub fn contains(&self, elem: &Ed25519KeyHash) -> bool {
        self.0.contains(&elem.clone().into())
    }

    pub fn try_from(list: &Ed25519KeyHashList) -> Result<SetEd25519KeyHash, JsError> {
        let list: Vec<cml_chain::crypto::Ed25519KeyHash> = list.clone().into();
        cml_chain::SetEd25519KeyHash::try_from(list)
            .map(Self)
            .map_err(|e| JsError::new(&e.to_string()))
    }

    pub fn try_opt_from(list: &Ed25519KeyHashList) -> Result<Option<SetEd25519KeyHash>, JsError> {
        let list: Vec<cml_chain::crypto::Ed25519KeyHash> = list.clone().into();
        cml_chain::SetEd25519KeyHash::try_opt_from(list)
            .map(|opt| opt.map(Self))
            .map_err(|e| JsError::new(&e.to_string()))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct SetTransactionInput(pub(crate) cml_chain::SetTransactionInput);

impl_wasm_cbor_json_api!(SetTransactionInput);

impl_wasm_conversions!(cml_chain::SetTransactionInput, SetTransactionInput);

#[wasm_bindgen]
impl SetTransactionInput {
    pub fn new(inner: &TransactionInputOrderedSet) -> Self {
        Self(cml_chain::SetTransactionInput::new(inner.clone().into()))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> TransactionInput {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &TransactionInput) -> Result<(), JsError> {
        self.0
            .push(elem.clone().into())
            .map_err(|e| JsError::new(&e.to_string()))
    }

    pub fn insert(&mut self, elem: &TransactionInput) -> bool {
        self.0.insert(elem.clone().into())
    }

    pub fn contains(&self, elem: &TransactionInput) -> bool {
        self.0.contains(&elem.clone().into())
    }

    pub fn try_from(list: &TransactionInputList) -> Result<SetTransactionInput, JsError> {
        let list: Vec<cml_chain::transaction::TransactionInput> = list.clone().into();
        cml_chain::SetTransactionInput::try_from(list)
            .map(Self)
            .map_err(|e| JsError::new(&e.to_string()))
    }

    pub fn try_opt_from(
        list: &TransactionInputList,
    ) -> Result<Option<SetTransactionInput>, JsError> {
        let list: Vec<cml_chain::transaction::TransactionInput> = list.clone().into();
        cml_chain::SetTransactionInput::try_opt_from(list)
            .map(|opt| opt.map(Self))
            .map_err(|e| JsError::new(&e.to_string()))
    }
}

pub type Slot = u64;

pub type SubCoin = Rational;

impl_wasm_list_needs_into!(
    cml_chain::transaction::TransactionBody,
    TransactionBody,
    TransactionBodyList,
    true,
    false
);

pub type TransactionIndex = u16;

impl_wasm_list_needs_into!(
    cml_chain::transaction::TransactionInput,
    TransactionInput,
    TransactionInputList,
    true,
    false
);

/// `[* transaction_input] @duplicates reject`: an insertion-ordered, duplicate-free set (order preserved for byte-exact round-trip). `add` is checked — an already-present element is refused; construct via `try_from` (the uniqueness door). `insert` is the std-set door (returns `false`, set unchanged, for an already-present element); `contains` tests membership.
#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct TransactionInputOrderedSet(
    pub(crate) OrderedSet<cml_chain::transaction::TransactionInput>,
);

impl_wasm_conversions!(
    OrderedSet<cml_chain::transaction::TransactionInput>,
    TransactionInputOrderedSet
);

#[wasm_bindgen]
impl TransactionInputOrderedSet {
    pub fn new() -> Self {
        Self(OrderedSet::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> TransactionInput {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &TransactionInput) -> Result<(), JsError> {
        self.0
            .push(elem.clone().into())
            .map_err(|e| JsError::new(&e.to_string()))
    }

    pub fn insert(&mut self, elem: &TransactionInput) -> bool {
        self.0.insert(elem.clone().into())
    }

    pub fn contains(&self, elem: &TransactionInput) -> bool {
        self.0.contains(&elem.clone().into())
    }

    pub fn try_from(list: &TransactionInputList) -> Result<TransactionInputOrderedSet, JsError> {
        let inner: Vec<cml_chain::transaction::TransactionInput> = list.clone().into();
        OrderedSet::try_from(inner)
            .map(Self)
            .map_err(|e| JsError::new(&e.to_string()))
    }
}

pub type TransactionMetadatumLabel = u64;

impl_wasm_list_needs_into!(
    cml_chain::transaction::TransactionOutput,
    TransactionOutput,
    TransactionOutputList,
    true,
    false
);

impl_wasm_list_needs_into!(
    cml_chain::transaction::TransactionWitnessSet,
    TransactionWitnessSet,
    TransactionWitnessSetList,
    true,
    false
);

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct UnitInterval(pub(crate) cml_chain::UnitInterval);

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

impl_wasm_list_needs_into!(
    cml_chain::crypto::Vkeywitness,
    Vkeywitness,
    VkeywitnessList,
    true,
    false
);

impl_wasm_list_needs_into!(cml_chain::governance::Voter, Voter, VoterList, true, false);

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct Withdrawals(pub(crate) cml_chain::Withdrawals);

impl_wasm_conversions!(cml_chain::Withdrawals, Withdrawals);

#[wasm_bindgen]
impl Withdrawals {
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
        RewardAccountList(self.0.keys().cloned().collect::<Vec<_>>())
    }
}
