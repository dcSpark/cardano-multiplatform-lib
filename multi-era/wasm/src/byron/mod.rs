// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

pub mod block;
pub mod delegation;
pub mod mpc;
pub mod transaction;
pub mod update;
#[macro_use]
pub mod utils;

use crate::impl_wasm_cbor_json_api_byron;
use ::wasm_bindgen::prelude::{wasm_bindgen, JsValue};
use cml_chain_wasm::byron::ByronTxOut;
use cml_chain_wasm::utils::BigInt;
use cml_core_wasm::impl_wasm_conversions;
use std::collections::BTreeMap;
pub use utils::{Blake2b224, Blake2b256, ByronAny};

pub type SystemTag = String;

pub type ByronBlockId = Blake2b256;

pub type ByronPubKey = Vec<u8>;

pub type ByronSignature = Vec<u8>;

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronSlotId(cml_multi_era::byron::ByronSlotId);

impl_wasm_cbor_json_api_byron!(ByronSlotId);

impl_wasm_conversions!(cml_multi_era::byron::ByronSlotId, ByronSlotId);

#[wasm_bindgen]
impl ByronSlotId {
    pub fn epoch(&self) -> EpochId {
        self.0.epoch
    }

    pub fn slot(&self) -> u64 {
        self.0.slot
    }

    pub fn new(epoch: EpochId, slot: u64) -> Self {
        Self(cml_multi_era::byron::ByronSlotId::new(epoch, slot))
    }
}

pub type ByronTxId = Blake2b256;

pub type ByronUpdateId = Blake2b256;

pub type EpochId = u64;

// this is pasted from lib.rs due to the export structure being at cml-multi-era-wasm root:

use cml_chain_wasm::byron::{AddressId, StakeholderId};
use mpc::VssPubKey;

use crate::byron::{
    delegation::ByronDelegation,
    mpc::VssDecryptedShare,
    transaction::{ByronAttributes, ByronTxIn, ByronTxWitness},
    update::{
        ByronTxFeePolicy, ByronUpdateData, ByronUpdateProposal, ByronUpdateVote, SoftForkRule,
    },
};

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct AddressIdList(Vec<cml_chain::byron::AddressId>);

#[wasm_bindgen]
impl AddressIdList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> AddressId {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &AddressId) {
        self.0.push(elem.clone().into());
    }
}

impl From<Vec<cml_chain::byron::AddressId>> for AddressIdList {
    fn from(native: Vec<cml_chain::byron::AddressId>) -> Self {
        Self(native)
    }
}

impl From<AddressIdList> for Vec<cml_chain::byron::AddressId> {
    fn from(wasm: AddressIdList) -> Self {
        wasm.0
    }
}

impl AsRef<Vec<cml_chain::byron::AddressId>> for AddressIdList {
    fn as_ref(&self) -> &Vec<cml_chain::byron::AddressId> {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct BigIntList(Vec<cml_chain::utils::BigInt>);

#[wasm_bindgen]
impl BigIntList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> BigInt {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &BigInt) {
        self.0.push(elem.clone().into());
    }
}

impl From<Vec<cml_chain::utils::BigInt>> for BigIntList {
    fn from(native: Vec<cml_chain::utils::BigInt>) -> Self {
        Self(native)
    }
}

impl From<BigIntList> for Vec<cml_chain::utils::BigInt> {
    fn from(wasm: BigIntList) -> Self {
        wasm.0
    }
}

impl AsRef<Vec<cml_chain::utils::BigInt>> for BigIntList {
    fn as_ref(&self) -> &Vec<cml_chain::utils::BigInt> {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronAnyList(Vec<cml_multi_era::byron::ByronAny>);

#[wasm_bindgen]
impl ByronAnyList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> ByronAny {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &ByronAny) {
        self.0.push(elem.clone().into());
    }
}

impl From<Vec<cml_multi_era::byron::ByronAny>> for ByronAnyList {
    fn from(native: Vec<cml_multi_era::byron::ByronAny>) -> Self {
        Self(native)
    }
}

impl From<ByronAnyList> for Vec<cml_multi_era::byron::ByronAny> {
    fn from(wasm: ByronAnyList) -> Self {
        wasm.0
    }
}

impl AsRef<Vec<cml_multi_era::byron::ByronAny>> for ByronAnyList {
    fn as_ref(&self) -> &Vec<cml_multi_era::byron::ByronAny> {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronAttributesList(Vec<cml_multi_era::byron::transaction::ByronAttributes>);

#[wasm_bindgen]
impl ByronAttributesList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> ByronAttributes {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &ByronAttributes) {
        self.0.push(elem.clone().into());
    }
}

impl From<Vec<cml_multi_era::byron::transaction::ByronAttributes>> for ByronAttributesList {
    fn from(native: Vec<cml_multi_era::byron::transaction::ByronAttributes>) -> Self {
        Self(native)
    }
}

impl From<ByronAttributesList> for Vec<cml_multi_era::byron::transaction::ByronAttributes> {
    fn from(wasm: ByronAttributesList) -> Self {
        wasm.0
    }
}

impl AsRef<Vec<cml_multi_era::byron::transaction::ByronAttributes>> for ByronAttributesList {
    fn as_ref(&self) -> &Vec<cml_multi_era::byron::transaction::ByronAttributes> {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronDelegationList(Vec<cml_multi_era::byron::delegation::ByronDelegation>);

#[wasm_bindgen]
impl ByronDelegationList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> ByronDelegation {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &ByronDelegation) {
        self.0.push(elem.clone().into());
    }
}

impl From<Vec<cml_multi_era::byron::delegation::ByronDelegation>> for ByronDelegationList {
    fn from(native: Vec<cml_multi_era::byron::delegation::ByronDelegation>) -> Self {
        Self(native)
    }
}

impl From<ByronDelegationList> for Vec<cml_multi_era::byron::delegation::ByronDelegation> {
    fn from(wasm: ByronDelegationList) -> Self {
        wasm.0
    }
}

impl AsRef<Vec<cml_multi_era::byron::delegation::ByronDelegation>> for ByronDelegationList {
    fn as_ref(&self) -> &Vec<cml_multi_era::byron::delegation::ByronDelegation> {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronTxFeePolicyList(Vec<cml_multi_era::byron::update::ByronTxFeePolicy>);

#[wasm_bindgen]
impl ByronTxFeePolicyList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> ByronTxFeePolicy {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &ByronTxFeePolicy) {
        self.0.push(elem.clone().into());
    }
}

impl From<Vec<cml_multi_era::byron::update::ByronTxFeePolicy>> for ByronTxFeePolicyList {
    fn from(native: Vec<cml_multi_era::byron::update::ByronTxFeePolicy>) -> Self {
        Self(native)
    }
}

impl From<ByronTxFeePolicyList> for Vec<cml_multi_era::byron::update::ByronTxFeePolicy> {
    fn from(wasm: ByronTxFeePolicyList) -> Self {
        wasm.0
    }
}

impl AsRef<Vec<cml_multi_era::byron::update::ByronTxFeePolicy>> for ByronTxFeePolicyList {
    fn as_ref(&self) -> &Vec<cml_multi_era::byron::update::ByronTxFeePolicy> {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronTxInList(Vec<cml_multi_era::byron::transaction::ByronTxIn>);

#[wasm_bindgen]
impl ByronTxInList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> ByronTxIn {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &ByronTxIn) {
        self.0.push(elem.clone().into());
    }
}

impl From<Vec<cml_multi_era::byron::transaction::ByronTxIn>> for ByronTxInList {
    fn from(native: Vec<cml_multi_era::byron::transaction::ByronTxIn>) -> Self {
        Self(native)
    }
}

impl From<ByronTxInList> for Vec<cml_multi_era::byron::transaction::ByronTxIn> {
    fn from(wasm: ByronTxInList) -> Self {
        wasm.0
    }
}

impl AsRef<Vec<cml_multi_era::byron::transaction::ByronTxIn>> for ByronTxInList {
    fn as_ref(&self) -> &Vec<cml_multi_era::byron::transaction::ByronTxIn> {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronTxOutList(Vec<cml_chain::byron::ByronTxOut>);

#[wasm_bindgen]
impl ByronTxOutList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> ByronTxOut {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &ByronTxOut) {
        self.0.push(elem.clone().into());
    }
}

impl From<Vec<cml_chain::byron::ByronTxOut>> for ByronTxOutList {
    fn from(native: Vec<cml_chain::byron::ByronTxOut>) -> Self {
        Self(native)
    }
}

impl From<ByronTxOutList> for Vec<cml_chain::byron::ByronTxOut> {
    fn from(wasm: ByronTxOutList) -> Self {
        wasm.0
    }
}

impl AsRef<Vec<cml_chain::byron::ByronTxOut>> for ByronTxOutList {
    fn as_ref(&self) -> &Vec<cml_chain::byron::ByronTxOut> {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronTxWitnessList(Vec<cml_multi_era::byron::transaction::ByronTxWitness>);

#[wasm_bindgen]
impl ByronTxWitnessList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> ByronTxWitness {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &ByronTxWitness) {
        self.0.push(elem.clone().into());
    }
}

impl From<Vec<cml_multi_era::byron::transaction::ByronTxWitness>> for ByronTxWitnessList {
    fn from(native: Vec<cml_multi_era::byron::transaction::ByronTxWitness>) -> Self {
        Self(native)
    }
}

impl From<ByronTxWitnessList> for Vec<cml_multi_era::byron::transaction::ByronTxWitness> {
    fn from(wasm: ByronTxWitnessList) -> Self {
        wasm.0
    }
}

impl AsRef<Vec<cml_multi_era::byron::transaction::ByronTxWitness>> for ByronTxWitnessList {
    fn as_ref(&self) -> &Vec<cml_multi_era::byron::transaction::ByronTxWitness> {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronUpdateProposalList(Vec<cml_multi_era::byron::update::ByronUpdateProposal>);

#[wasm_bindgen]
impl ByronUpdateProposalList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> ByronUpdateProposal {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &ByronUpdateProposal) {
        self.0.push(elem.clone().into());
    }
}

impl From<Vec<cml_multi_era::byron::update::ByronUpdateProposal>> for ByronUpdateProposalList {
    fn from(native: Vec<cml_multi_era::byron::update::ByronUpdateProposal>) -> Self {
        Self(native)
    }
}

impl From<ByronUpdateProposalList> for Vec<cml_multi_era::byron::update::ByronUpdateProposal> {
    fn from(wasm: ByronUpdateProposalList) -> Self {
        wasm.0
    }
}

impl AsRef<Vec<cml_multi_era::byron::update::ByronUpdateProposal>> for ByronUpdateProposalList {
    fn as_ref(&self) -> &Vec<cml_multi_era::byron::update::ByronUpdateProposal> {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronUpdateVoteList(Vec<cml_multi_era::byron::update::ByronUpdateVote>);

#[wasm_bindgen]
impl ByronUpdateVoteList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> ByronUpdateVote {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &ByronUpdateVote) {
        self.0.push(elem.clone().into());
    }
}

impl From<Vec<cml_multi_era::byron::update::ByronUpdateVote>> for ByronUpdateVoteList {
    fn from(native: Vec<cml_multi_era::byron::update::ByronUpdateVote>) -> Self {
        Self(native)
    }
}

impl From<ByronUpdateVoteList> for Vec<cml_multi_era::byron::update::ByronUpdateVote> {
    fn from(wasm: ByronUpdateVoteList) -> Self {
        wasm.0
    }
}

impl AsRef<Vec<cml_multi_era::byron::update::ByronUpdateVote>> for ByronUpdateVoteList {
    fn as_ref(&self) -> &Vec<cml_multi_era::byron::update::ByronUpdateVote> {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct BytesList(Vec<Vec<u8>>);

#[wasm_bindgen]
impl BytesList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Vec<u8> {
        self.0[index].clone()
    }

    pub fn add(&mut self, elem: Vec<u8>) {
        self.0.push(elem);
    }
}

impl From<Vec<Vec<u8>>> for BytesList {
    fn from(native: Vec<Vec<u8>>) -> Self {
        Self(native)
    }
}

impl From<BytesList> for Vec<Vec<u8>> {
    fn from(wasm: BytesList) -> Self {
        wasm.0
    }
}

impl AsRef<Vec<Vec<u8>>> for BytesList {
    fn as_ref(&self) -> &Vec<Vec<u8>> {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MapSystemTagToByronUpdateData(
    BTreeMap<
        cml_multi_era::byron::update::SystemTag,
        cml_multi_era::byron::update::ByronUpdateData,
    >,
);

#[wasm_bindgen]
impl MapSystemTagToByronUpdateData {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, key: SystemTag, value: &ByronUpdateData) -> Option<ByronUpdateData> {
        self.0.insert(key, value.clone().into()).map(Into::into)
    }

    pub fn get(&self, key: SystemTag) -> Option<ByronUpdateData> {
        self.0.get(&key).map(|v| v.clone().into())
    }

    pub fn keys(&self) -> SystemTagList {
        SystemTagList(self.0.iter().map(|(k, _v)| k.clone()).collect::<Vec<_>>())
    }
}

impl
    From<
        BTreeMap<
            cml_multi_era::byron::update::SystemTag,
            cml_multi_era::byron::update::ByronUpdateData,
        >,
    > for MapSystemTagToByronUpdateData
{
    fn from(
        native: BTreeMap<
            cml_multi_era::byron::update::SystemTag,
            cml_multi_era::byron::update::ByronUpdateData,
        >,
    ) -> Self {
        Self(native)
    }
}

impl From<MapSystemTagToByronUpdateData>
    for BTreeMap<
        cml_multi_era::byron::update::SystemTag,
        cml_multi_era::byron::update::ByronUpdateData,
    >
{
    fn from(wasm: MapSystemTagToByronUpdateData) -> Self {
        wasm.0
    }
}

impl
    AsRef<
        BTreeMap<
            cml_multi_era::byron::update::SystemTag,
            cml_multi_era::byron::update::ByronUpdateData,
        >,
    > for MapSystemTagToByronUpdateData
{
    fn as_ref(
        &self,
    ) -> &BTreeMap<
        cml_multi_era::byron::update::SystemTag,
        cml_multi_era::byron::update::ByronUpdateData,
    > {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct SoftForkRuleList(Vec<cml_multi_era::byron::update::SoftForkRule>);

#[wasm_bindgen]
impl SoftForkRuleList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> SoftForkRule {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &SoftForkRule) {
        self.0.push(elem.clone().into());
    }
}

impl From<Vec<cml_multi_era::byron::update::SoftForkRule>> for SoftForkRuleList {
    fn from(native: Vec<cml_multi_era::byron::update::SoftForkRule>) -> Self {
        Self(native)
    }
}

impl From<SoftForkRuleList> for Vec<cml_multi_era::byron::update::SoftForkRule> {
    fn from(wasm: SoftForkRuleList) -> Self {
        wasm.0
    }
}

impl AsRef<Vec<cml_multi_era::byron::update::SoftForkRule>> for SoftForkRuleList {
    fn as_ref(&self) -> &Vec<cml_multi_era::byron::update::SoftForkRule> {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct StakeholderIdList(Vec<cml_chain::byron::StakeholderId>);

#[wasm_bindgen]
impl StakeholderIdList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> StakeholderId {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &StakeholderId) {
        self.0.push(elem.clone().into());
    }
}

impl From<Vec<cml_chain::byron::StakeholderId>> for StakeholderIdList {
    fn from(native: Vec<cml_chain::byron::StakeholderId>) -> Self {
        Self(native)
    }
}

impl From<StakeholderIdList> for Vec<cml_chain::byron::StakeholderId> {
    fn from(wasm: StakeholderIdList) -> Self {
        wasm.0
    }
}

impl AsRef<Vec<cml_chain::byron::StakeholderId>> for StakeholderIdList {
    fn as_ref(&self) -> &Vec<cml_chain::byron::StakeholderId> {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct SystemTagList(Vec<cml_multi_era::byron::update::SystemTag>);

#[wasm_bindgen]
impl SystemTagList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> SystemTag {
        self.0[index].clone()
    }

    pub fn add(&mut self, elem: SystemTag) {
        self.0.push(elem);
    }
}

impl From<Vec<cml_multi_era::byron::update::SystemTag>> for SystemTagList {
    fn from(native: Vec<cml_multi_era::byron::update::SystemTag>) -> Self {
        Self(native)
    }
}

impl From<SystemTagList> for Vec<cml_multi_era::byron::update::SystemTag> {
    fn from(wasm: SystemTagList) -> Self {
        wasm.0
    }
}

impl AsRef<Vec<cml_multi_era::byron::update::SystemTag>> for SystemTagList {
    fn as_ref(&self) -> &Vec<cml_multi_era::byron::update::SystemTag> {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct VssDecryptedShareList(Vec<cml_multi_era::byron::mpc::VssDecryptedShare>);

#[wasm_bindgen]
impl VssDecryptedShareList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> VssDecryptedShare {
        self.0[index].clone()
    }

    pub fn add(&mut self, elem: VssDecryptedShare) {
        self.0.push(elem);
    }
}

impl From<Vec<cml_multi_era::byron::mpc::VssDecryptedShare>> for VssDecryptedShareList {
    fn from(native: Vec<cml_multi_era::byron::mpc::VssDecryptedShare>) -> Self {
        Self(native)
    }
}

impl From<VssDecryptedShareList> for Vec<cml_multi_era::byron::mpc::VssDecryptedShare> {
    fn from(wasm: VssDecryptedShareList) -> Self {
        wasm.0
    }
}

impl AsRef<Vec<cml_multi_era::byron::mpc::VssDecryptedShare>> for VssDecryptedShareList {
    fn as_ref(&self) -> &Vec<cml_multi_era::byron::mpc::VssDecryptedShare> {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct VssPubKeyList(Vec<cml_multi_era::byron::mpc::VssPubKey>);

#[wasm_bindgen]
impl VssPubKeyList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> VssPubKey {
        self.0[index].clone()
    }

    pub fn add(&mut self, elem: VssPubKey) {
        self.0.push(elem);
    }
}

impl From<Vec<cml_multi_era::byron::mpc::VssPubKey>> for VssPubKeyList {
    fn from(native: Vec<cml_multi_era::byron::mpc::VssPubKey>) -> Self {
        Self(native)
    }
}

impl From<VssPubKeyList> for Vec<cml_multi_era::byron::mpc::VssPubKey> {
    fn from(wasm: VssPubKeyList) -> Self {
        wasm.0
    }
}

impl AsRef<Vec<cml_multi_era::byron::mpc::VssPubKey>> for VssPubKeyList {
    fn as_ref(&self) -> &Vec<cml_multi_era::byron::mpc::VssPubKey> {
        &self.0
    }
}
