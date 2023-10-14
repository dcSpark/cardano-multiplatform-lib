// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use crate::byron::transaction::ByronAttributes;
use crate::byron::{
    BigIntList, ByronTxFeePolicyList, ByronUpdateProposalList, ByronUpdateVoteList,
    MapSystemTagToByronUpdateData, SoftForkRuleList,
};
use crate::byron::{Blake2b256, ByronPubKey, ByronSignature, ByronUpdateId, EpochId};
use cml_chain_wasm::utils::BigInt;
use cml_core_wasm::{impl_wasm_cbor_json_api_cbor_event_serialize, impl_wasm_conversions};
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct Bvermod(cml_multi_era::byron::update::Bvermod);

impl_wasm_cbor_json_api_cbor_event_serialize!(Bvermod);

impl_wasm_conversions!(cml_multi_era::byron::update::Bvermod, Bvermod);

#[wasm_bindgen]
impl Bvermod {
    pub fn script_version(&self) -> Vec<u16> {
        self.0.script_version.clone()
    }

    pub fn slot_duration(&self) -> BigIntList {
        self.0.slot_duration.clone().into()
    }

    pub fn max_block_size(&self) -> BigIntList {
        self.0.max_block_size.clone().into()
    }

    pub fn max_header_size(&self) -> BigIntList {
        self.0.max_header_size.clone().into()
    }

    pub fn max_tx_size(&self) -> BigIntList {
        self.0.max_tx_size.clone().into()
    }

    pub fn max_proposal_size(&self) -> BigIntList {
        self.0.max_proposal_size.clone().into()
    }

    pub fn mpc_thd(&self) -> Vec<u64> {
        self.0.mpc_thd.clone()
    }

    pub fn heavy_del_thd(&self) -> Vec<u64> {
        self.0.heavy_del_thd.clone()
    }

    pub fn update_vote_thd(&self) -> Vec<u64> {
        self.0.update_vote_thd.clone()
    }

    pub fn update_proposal_thd(&self) -> Vec<u64> {
        self.0.update_proposal_thd.clone()
    }

    pub fn update_implicit(&self) -> Vec<u64> {
        self.0.update_implicit.clone()
    }

    pub fn soft_fork_rule(&self) -> SoftForkRuleList {
        self.0.soft_fork_rule.clone().into()
    }

    pub fn tx_fee_policy(&self) -> ByronTxFeePolicyList {
        self.0.tx_fee_policy.clone().into()
    }

    pub fn unlock_stake_epoch(&self) -> Vec<EpochId> {
        self.0.unlock_stake_epoch.clone()
    }

    pub fn new(
        script_version: Vec<u16>,
        slot_duration: &BigIntList,
        max_block_size: &BigIntList,
        max_header_size: &BigIntList,
        max_tx_size: &BigIntList,
        max_proposal_size: &BigIntList,
        mpc_thd: Vec<u64>,
        heavy_del_thd: Vec<u64>,
        update_vote_thd: Vec<u64>,
        update_proposal_thd: Vec<u64>,
        update_implicit: Vec<u64>,
        soft_fork_rule: &SoftForkRuleList,
        tx_fee_policy: &ByronTxFeePolicyList,
        unlock_stake_epoch: Vec<EpochId>,
    ) -> Self {
        Self(cml_multi_era::byron::update::Bvermod::new(
            script_version,
            slot_duration.clone().into(),
            max_block_size.clone().into(),
            max_header_size.clone().into(),
            max_tx_size.clone().into(),
            max_proposal_size.clone().into(),
            mpc_thd,
            heavy_del_thd,
            update_vote_thd,
            update_proposal_thd,
            update_implicit,
            soft_fork_rule.clone().into(),
            tx_fee_policy.clone().into(),
            unlock_stake_epoch,
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronBlockVersion(cml_multi_era::byron::update::ByronBlockVersion);

impl_wasm_cbor_json_api_cbor_event_serialize!(ByronBlockVersion);

impl_wasm_conversions!(
    cml_multi_era::byron::update::ByronBlockVersion,
    ByronBlockVersion
);

#[wasm_bindgen]
impl ByronBlockVersion {
    pub fn u16(&self) -> u16 {
        self.0.u16
    }

    pub fn u162(&self) -> u16 {
        self.0.u162
    }

    pub fn u8(&self) -> u8 {
        self.0.u8
    }

    pub fn new(u16: u16, u162: u16, u8: u8) -> Self {
        Self(cml_multi_era::byron::update::ByronBlockVersion::new(
            u16, u162, u8,
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronSoftwareVersion(cml_multi_era::byron::update::ByronSoftwareVersion);

impl_wasm_cbor_json_api_cbor_event_serialize!(ByronSoftwareVersion);

impl_wasm_conversions!(
    cml_multi_era::byron::update::ByronSoftwareVersion,
    ByronSoftwareVersion
);

#[wasm_bindgen]
impl ByronSoftwareVersion {
    pub fn application_name(&self) -> String {
        self.0.application_name.clone()
    }

    pub fn u32(&self) -> u32 {
        self.0.u32
    }

    pub fn new(application_name: String, u32: u32) -> Self {
        Self(cml_multi_era::byron::update::ByronSoftwareVersion::new(
            application_name,
            u32,
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronTxFeePolicy(cml_multi_era::byron::update::ByronTxFeePolicy);

impl_wasm_cbor_json_api_cbor_event_serialize!(ByronTxFeePolicy);

impl_wasm_conversions!(
    cml_multi_era::byron::update::ByronTxFeePolicy,
    ByronTxFeePolicy
);

#[wasm_bindgen]
impl ByronTxFeePolicy {
    pub fn index_1(&self) -> StdFeePolicy {
        self.0.index_1.clone().into()
    }

    pub fn new(index_1: &StdFeePolicy) -> Self {
        Self(cml_multi_era::byron::update::ByronTxFeePolicy::new(
            index_1.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronUpdate(cml_multi_era::byron::update::ByronUpdate);

impl_wasm_cbor_json_api_cbor_event_serialize!(ByronUpdate);

impl_wasm_conversions!(cml_multi_era::byron::update::ByronUpdate, ByronUpdate);

#[wasm_bindgen]
impl ByronUpdate {
    pub fn proposal(&self) -> ByronUpdateProposalList {
        self.0.proposal.clone().into()
    }

    pub fn votes(&self) -> ByronUpdateVoteList {
        self.0.votes.clone().into()
    }

    pub fn new(proposal: &ByronUpdateProposalList, votes: &ByronUpdateVoteList) -> Self {
        Self(cml_multi_era::byron::update::ByronUpdate::new(
            proposal.clone().into(),
            votes.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronUpdateData(cml_multi_era::byron::update::ByronUpdateData);

impl_wasm_cbor_json_api_cbor_event_serialize!(ByronUpdateData);

impl_wasm_conversions!(
    cml_multi_era::byron::update::ByronUpdateData,
    ByronUpdateData
);

#[wasm_bindgen]
impl ByronUpdateData {
    pub fn blake2b256(&self) -> Blake2b256 {
        self.0.blake2b256.into()
    }

    pub fn blake2b2562(&self) -> Blake2b256 {
        self.0.blake2b2562.into()
    }

    pub fn blake2b2563(&self) -> Blake2b256 {
        self.0.blake2b2563.into()
    }

    pub fn blake2b2564(&self) -> Blake2b256 {
        self.0.blake2b2564.into()
    }

    pub fn new(
        blake2b256: &Blake2b256,
        blake2b2562: &Blake2b256,
        blake2b2563: &Blake2b256,
        blake2b2564: &Blake2b256,
    ) -> Self {
        Self(cml_multi_era::byron::update::ByronUpdateData::new(
            blake2b256.clone().into(),
            blake2b2562.clone().into(),
            blake2b2563.clone().into(),
            blake2b2564.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronUpdateProposal(cml_multi_era::byron::update::ByronUpdateProposal);

impl_wasm_cbor_json_api_cbor_event_serialize!(ByronUpdateProposal);

impl_wasm_conversions!(
    cml_multi_era::byron::update::ByronUpdateProposal,
    ByronUpdateProposal
);

#[wasm_bindgen]
impl ByronUpdateProposal {
    pub fn block_version(&self) -> ByronBlockVersion {
        self.0.block_version.clone().into()
    }

    pub fn block_version_mod(&self) -> Bvermod {
        self.0.block_version_mod.clone().into()
    }

    pub fn software_version(&self) -> ByronSoftwareVersion {
        self.0.software_version.clone().into()
    }

    pub fn data(&self) -> MapSystemTagToByronUpdateData {
        self.0.data.clone().into()
    }

    pub fn byron_attributes(&self) -> ByronAttributes {
        self.0.byron_attributes.clone().into()
    }

    pub fn from(&self) -> ByronPubKey {
        self.0.from.clone()
    }

    pub fn signature(&self) -> ByronSignature {
        self.0.signature.clone()
    }

    pub fn new(
        block_version: &ByronBlockVersion,
        block_version_mod: &Bvermod,
        software_version: &ByronSoftwareVersion,
        data: &MapSystemTagToByronUpdateData,
        byron_attributes: &ByronAttributes,
        from: ByronPubKey,
        signature: ByronSignature,
    ) -> Self {
        Self(cml_multi_era::byron::update::ByronUpdateProposal::new(
            block_version.clone().into(),
            block_version_mod.clone().into(),
            software_version.clone().into(),
            data.clone().into(),
            byron_attributes.clone().into(),
            from,
            signature,
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronUpdateVote(cml_multi_era::byron::update::ByronUpdateVote);

impl_wasm_cbor_json_api_cbor_event_serialize!(ByronUpdateVote);

impl_wasm_conversions!(
    cml_multi_era::byron::update::ByronUpdateVote,
    ByronUpdateVote
);

#[wasm_bindgen]
impl ByronUpdateVote {
    pub fn voter(&self) -> ByronPubKey {
        self.0.voter.clone()
    }

    pub fn proposal_id(&self) -> ByronUpdateId {
        self.0.proposal_id.into()
    }

    pub fn vote(&self) -> bool {
        self.0.vote
    }

    pub fn signature(&self) -> ByronSignature {
        self.0.signature.clone()
    }

    pub fn new(
        voter: ByronPubKey,
        proposal_id: &ByronUpdateId,
        vote: bool,
        signature: ByronSignature,
    ) -> Self {
        Self(cml_multi_era::byron::update::ByronUpdateVote::new(
            voter,
            proposal_id.clone().into(),
            vote,
            signature,
        ))
    }
}

pub type CoinPortion = u64;

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct SoftForkRule(cml_multi_era::byron::update::SoftForkRule);

impl_wasm_cbor_json_api_cbor_event_serialize!(SoftForkRule);

impl_wasm_conversions!(cml_multi_era::byron::update::SoftForkRule, SoftForkRule);

#[wasm_bindgen]
impl SoftForkRule {
    pub fn coin_portion(&self) -> CoinPortion {
        self.0.coin_portion
    }

    pub fn coin_portion2(&self) -> CoinPortion {
        self.0.coin_portion2
    }

    pub fn coin_portion3(&self) -> CoinPortion {
        self.0.coin_portion3
    }

    pub fn new(
        coin_portion: CoinPortion,
        coin_portion2: CoinPortion,
        coin_portion3: CoinPortion,
    ) -> Self {
        Self(cml_multi_era::byron::update::SoftForkRule::new(
            coin_portion,
            coin_portion2,
            coin_portion3,
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct StdFeePolicy(cml_multi_era::byron::update::StdFeePolicy);

impl_wasm_cbor_json_api_cbor_event_serialize!(StdFeePolicy);

impl_wasm_conversions!(cml_multi_era::byron::update::StdFeePolicy, StdFeePolicy);

#[wasm_bindgen]
impl StdFeePolicy {
    pub fn big_int(&self) -> BigInt {
        self.0.big_int.clone().into()
    }

    pub fn big_int2(&self) -> BigInt {
        self.0.big_int2.clone().into()
    }

    pub fn new(big_int: &BigInt, big_int2: &BigInt) -> Self {
        Self(cml_multi_era::byron::update::StdFeePolicy::new(
            big_int.clone().into(),
            big_int2.clone().into(),
        ))
    }
}

pub type SystemTag = String;
