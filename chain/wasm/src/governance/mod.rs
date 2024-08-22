// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use crate::address::RewardAccount;
use crate::assets::Coin;
use crate::block::ProtocolVersion;
use crate::certs::Url;
use crate::crypto::{AnchorDocHash, Ed25519KeyHash, ScriptHash, TransactionHash};
use crate::{
    MapCommitteeColdCredentialToEpoch, MapGovActionIdToVotingProcedure, MapRewardAccountToCoin,
    ProtocolParamUpdate, SetCommitteeColdCredential, UnitInterval, VoterList,
};
pub use cml_chain::governance::Vote;
use cml_core::ordered_hash_map::OrderedHashMap;
use cml_core_wasm::{impl_wasm_cbor_json_api, impl_wasm_conversions};
use wasm_bindgen::prelude::wasm_bindgen;

pub mod utils;

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct Anchor(cml_chain::governance::Anchor);

impl_wasm_cbor_json_api!(Anchor);

impl_wasm_conversions!(cml_chain::governance::Anchor, Anchor);

#[wasm_bindgen]
impl Anchor {
    pub fn anchor_url(&self) -> Url {
        self.0.anchor_url.clone().into()
    }

    pub fn anchor_doc_hash(&self) -> AnchorDocHash {
        self.0.anchor_doc_hash.into()
    }

    pub fn new(anchor_url: &Url, anchor_doc_hash: &AnchorDocHash) -> Self {
        Self(cml_chain::governance::Anchor::new(
            anchor_url.clone().into(),
            anchor_doc_hash.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct Constitution(cml_chain::governance::Constitution);

impl_wasm_cbor_json_api!(Constitution);

impl_wasm_conversions!(cml_chain::governance::Constitution, Constitution);

#[wasm_bindgen]
impl Constitution {
    pub fn anchor(&self) -> Anchor {
        self.0.anchor.clone().into()
    }

    pub fn script_hash(&self) -> Option<ScriptHash> {
        self.0.script_hash.map(std::convert::Into::into)
    }

    pub fn new(anchor: &Anchor, script_hash: Option<ScriptHash>) -> Self {
        Self(cml_chain::governance::Constitution::new(
            anchor.clone().into(),
            script_hash.map(Into::into),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct GovAction(cml_chain::governance::GovAction);

impl_wasm_cbor_json_api!(GovAction);

impl_wasm_conversions!(cml_chain::governance::GovAction, GovAction);

#[wasm_bindgen]
impl GovAction {
    pub fn new_parameter_change_action(
        action_id: Option<GovActionId>,
        update: &ProtocolParamUpdate,
        policy_hash: Option<ScriptHash>,
    ) -> Self {
        Self(
            cml_chain::governance::GovAction::new_parameter_change_action(
                action_id.map(Into::into),
                update.clone().into(),
                policy_hash.map(Into::into),
            ),
        )
    }

    pub fn new_hard_fork_initiation_action(
        action_id: Option<GovActionId>,
        version: &ProtocolVersion,
    ) -> Self {
        Self(
            cml_chain::governance::GovAction::new_hard_fork_initiation_action(
                action_id.map(Into::into),
                version.clone().into(),
            ),
        )
    }

    pub fn new_treasury_withdrawals_action(
        withdrawal: &MapRewardAccountToCoin,
        policy_hash: Option<ScriptHash>,
    ) -> Self {
        Self(
            cml_chain::governance::GovAction::new_treasury_withdrawals_action(
                withdrawal.clone().into(),
                policy_hash.map(Into::into),
            ),
        )
    }

    pub fn new_no_confidence(action_id: Option<GovActionId>) -> Self {
        Self(cml_chain::governance::GovAction::new_no_confidence(
            action_id.map(Into::into),
        ))
    }

    pub fn new_update_committee(
        action_id: Option<GovActionId>,
        cold_credentials: &SetCommitteeColdCredential,
        credentials: &MapCommitteeColdCredentialToEpoch,
        unit_interval: &UnitInterval,
    ) -> Self {
        Self(cml_chain::governance::GovAction::new_update_committee(
            action_id.map(Into::into),
            cold_credentials.clone().into(),
            credentials.clone().into(),
            unit_interval.clone().into(),
        ))
    }

    pub fn new_new_constitution(
        action_id: Option<GovActionId>,
        constitution: &Constitution,
    ) -> Self {
        Self(cml_chain::governance::GovAction::new_new_constitution(
            action_id.map(Into::into),
            constitution.clone().into(),
        ))
    }

    pub fn new_info_action() -> Self {
        Self(cml_chain::governance::GovAction::new_info_action())
    }

    pub fn kind(&self) -> GovActionKind {
        match &self.0 {
            cml_chain::governance::GovAction::ParameterChangeAction(_) => {
                GovActionKind::ParameterChangeAction
            }
            cml_chain::governance::GovAction::HardForkInitiationAction(_) => {
                GovActionKind::HardForkInitiationAction
            }
            cml_chain::governance::GovAction::TreasuryWithdrawalsAction(_) => {
                GovActionKind::TreasuryWithdrawalsAction
            }
            cml_chain::governance::GovAction::NoConfidence(_) => GovActionKind::NoConfidence,
            cml_chain::governance::GovAction::UpdateCommittee(_) => GovActionKind::UpdateCommittee,
            cml_chain::governance::GovAction::NewConstitution(_) => GovActionKind::NewConstitution,
            cml_chain::governance::GovAction::InfoAction { .. } => GovActionKind::InfoAction,
        }
    }

    pub fn as_parameter_change_action(&self) -> Option<ParameterChangeAction> {
        match &self.0 {
            cml_chain::governance::GovAction::ParameterChangeAction(parameter_change_action) => {
                Some(parameter_change_action.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_hard_fork_initiation_action(&self) -> Option<HardForkInitiationAction> {
        match &self.0 {
            cml_chain::governance::GovAction::HardForkInitiationAction(
                hard_fork_initiation_action,
            ) => Some(hard_fork_initiation_action.clone().into()),
            _ => None,
        }
    }

    pub fn as_treasury_withdrawals_action(&self) -> Option<TreasuryWithdrawalsAction> {
        match &self.0 {
            cml_chain::governance::GovAction::TreasuryWithdrawalsAction(
                treasury_withdrawals_action,
            ) => Some(treasury_withdrawals_action.clone().into()),
            _ => None,
        }
    }

    pub fn as_no_confidence(&self) -> Option<NoConfidence> {
        match &self.0 {
            cml_chain::governance::GovAction::NoConfidence(no_confidence) => {
                Some(no_confidence.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_update_committee(&self) -> Option<UpdateCommittee> {
        match &self.0 {
            cml_chain::governance::GovAction::UpdateCommittee(update_committee) => {
                Some(update_committee.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_new_constitution(&self) -> Option<NewConstitution> {
        match &self.0 {
            cml_chain::governance::GovAction::NewConstitution(new_constitution) => {
                Some(new_constitution.clone().into())
            }
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct GovActionId(cml_chain::governance::GovActionId);

impl_wasm_cbor_json_api!(GovActionId);

impl_wasm_conversions!(cml_chain::governance::GovActionId, GovActionId);

#[wasm_bindgen]
impl GovActionId {
    pub fn transaction_id(&self) -> TransactionHash {
        self.0.transaction_id.into()
    }

    pub fn gov_action_index(&self) -> u64 {
        self.0.gov_action_index
    }

    pub fn new(transaction_id: &TransactionHash, gov_action_index: u64) -> Self {
        Self(cml_chain::governance::GovActionId::new(
            transaction_id.clone().into(),
            gov_action_index,
        ))
    }
}

#[wasm_bindgen]
pub enum GovActionKind {
    ParameterChangeAction,
    HardForkInitiationAction,
    TreasuryWithdrawalsAction,
    NoConfidence,
    UpdateCommittee,
    NewConstitution,
    InfoAction,
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct HardForkInitiationAction(cml_chain::governance::HardForkInitiationAction);

impl_wasm_cbor_json_api!(HardForkInitiationAction);

impl_wasm_conversions!(
    cml_chain::governance::HardForkInitiationAction,
    HardForkInitiationAction
);

#[wasm_bindgen]
impl HardForkInitiationAction {
    pub fn action_id(&self) -> Option<GovActionId> {
        self.0.action_id.clone().map(std::convert::Into::into)
    }

    pub fn version(&self) -> ProtocolVersion {
        self.0.version.clone().into()
    }

    pub fn new(action_id: Option<GovActionId>, version: &ProtocolVersion) -> Self {
        Self(cml_chain::governance::HardForkInitiationAction::new(
            action_id.map(Into::into),
            version.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct NewConstitution(cml_chain::governance::NewConstitution);

impl_wasm_cbor_json_api!(NewConstitution);

impl_wasm_conversions!(cml_chain::governance::NewConstitution, NewConstitution);

#[wasm_bindgen]
impl NewConstitution {
    pub fn action_id(&self) -> Option<GovActionId> {
        self.0.action_id.clone().map(std::convert::Into::into)
    }

    pub fn constitution(&self) -> Constitution {
        self.0.constitution.clone().into()
    }

    pub fn new(action_id: Option<GovActionId>, constitution: &Constitution) -> Self {
        Self(cml_chain::governance::NewConstitution::new(
            action_id.map(Into::into),
            constitution.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct NoConfidence(cml_chain::governance::NoConfidence);

impl_wasm_cbor_json_api!(NoConfidence);

impl_wasm_conversions!(cml_chain::governance::NoConfidence, NoConfidence);

#[wasm_bindgen]
impl NoConfidence {
    pub fn action_id(&self) -> Option<GovActionId> {
        self.0.action_id.clone().map(std::convert::Into::into)
    }

    pub fn new(action_id: Option<GovActionId>) -> Self {
        Self(cml_chain::governance::NoConfidence::new(
            action_id.map(Into::into),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ParameterChangeAction(cml_chain::governance::ParameterChangeAction);

impl_wasm_cbor_json_api!(ParameterChangeAction);

impl_wasm_conversions!(
    cml_chain::governance::ParameterChangeAction,
    ParameterChangeAction
);

#[wasm_bindgen]
impl ParameterChangeAction {
    pub fn action_id(&self) -> Option<GovActionId> {
        self.0.action_id.clone().map(std::convert::Into::into)
    }

    pub fn update(&self) -> ProtocolParamUpdate {
        self.0.update.clone().into()
    }

    pub fn policy_hash(&self) -> Option<ScriptHash> {
        self.0.policy_hash.map(std::convert::Into::into)
    }

    pub fn new(
        action_id: Option<GovActionId>,
        update: &ProtocolParamUpdate,
        policy_hash: Option<ScriptHash>,
    ) -> Self {
        Self(cml_chain::governance::ParameterChangeAction::new(
            action_id.map(Into::into),
            update.clone().into(),
            policy_hash.map(Into::into),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ProposalProcedure(cml_chain::governance::ProposalProcedure);

impl_wasm_cbor_json_api!(ProposalProcedure);

impl_wasm_conversions!(cml_chain::governance::ProposalProcedure, ProposalProcedure);

#[wasm_bindgen]
impl ProposalProcedure {
    pub fn deposit(&self) -> Coin {
        self.0.deposit
    }

    pub fn reward_account(&self) -> RewardAccount {
        self.0.reward_account.clone().into()
    }

    pub fn gov_action(&self) -> GovAction {
        self.0.gov_action.clone().into()
    }

    pub fn anchor(&self) -> Anchor {
        self.0.anchor.clone().into()
    }

    pub fn new(
        deposit: Coin,
        reward_account: &RewardAccount,
        gov_action: &GovAction,
        anchor: &Anchor,
    ) -> Self {
        Self(cml_chain::governance::ProposalProcedure::new(
            deposit,
            reward_account.clone().into(),
            gov_action.clone().into(),
            anchor.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct TreasuryWithdrawalsAction(cml_chain::governance::TreasuryWithdrawalsAction);

impl_wasm_cbor_json_api!(TreasuryWithdrawalsAction);

impl_wasm_conversions!(
    cml_chain::governance::TreasuryWithdrawalsAction,
    TreasuryWithdrawalsAction
);

#[wasm_bindgen]
impl TreasuryWithdrawalsAction {
    pub fn withdrawal(&self) -> MapRewardAccountToCoin {
        self.0.withdrawal.clone().into()
    }

    pub fn policy_hash(&self) -> Option<ScriptHash> {
        self.0.policy_hash.map(std::convert::Into::into)
    }

    pub fn new(withdrawal: &MapRewardAccountToCoin, policy_hash: Option<ScriptHash>) -> Self {
        Self(cml_chain::governance::TreasuryWithdrawalsAction::new(
            withdrawal.clone().into(),
            policy_hash.map(Into::into),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct UpdateCommittee(cml_chain::governance::UpdateCommittee);

impl_wasm_cbor_json_api!(UpdateCommittee);

impl_wasm_conversions!(cml_chain::governance::UpdateCommittee, UpdateCommittee);

#[wasm_bindgen]
impl UpdateCommittee {
    pub fn action_id(&self) -> Option<GovActionId> {
        self.0.action_id.clone().map(std::convert::Into::into)
    }

    pub fn cold_credentials(&self) -> SetCommitteeColdCredential {
        self.0.cold_credentials.clone().into()
    }

    pub fn credentials(&self) -> MapCommitteeColdCredentialToEpoch {
        self.0.credentials.clone().into()
    }

    pub fn unit_interval(&self) -> UnitInterval {
        self.0.unit_interval.clone().into()
    }

    pub fn new(
        action_id: Option<GovActionId>,
        cold_credentials: &SetCommitteeColdCredential,
        credentials: &MapCommitteeColdCredentialToEpoch,
        unit_interval: &UnitInterval,
    ) -> Self {
        Self(cml_chain::governance::UpdateCommittee::new(
            action_id.map(Into::into),
            cold_credentials.clone().into(),
            credentials.clone().into(),
            unit_interval.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct Voter(cml_chain::governance::Voter);

impl_wasm_cbor_json_api!(Voter);

impl_wasm_conversions!(cml_chain::governance::Voter, Voter);

#[wasm_bindgen]
impl Voter {
    pub fn new_constitutional_committee_hot_key_hash(ed25519_key_hash: &Ed25519KeyHash) -> Self {
        Self(
            cml_chain::governance::Voter::new_constitutional_committee_hot_key_hash(
                ed25519_key_hash.clone().into(),
            ),
        )
    }

    pub fn new_constitutional_committee_hot_script_hash(script_hash: &ScriptHash) -> Self {
        Self(
            cml_chain::governance::Voter::new_constitutional_committee_hot_script_hash(
                script_hash.clone().into(),
            ),
        )
    }

    pub fn new_d_rep_key_hash(ed25519_key_hash: &Ed25519KeyHash) -> Self {
        Self(cml_chain::governance::Voter::new_d_rep_key_hash(
            ed25519_key_hash.clone().into(),
        ))
    }

    pub fn new_d_rep_script_hash(script_hash: &ScriptHash) -> Self {
        Self(cml_chain::governance::Voter::new_d_rep_script_hash(
            script_hash.clone().into(),
        ))
    }

    pub fn new_staking_pool_key_hash(ed25519_key_hash: &Ed25519KeyHash) -> Self {
        Self(cml_chain::governance::Voter::new_staking_pool_key_hash(
            ed25519_key_hash.clone().into(),
        ))
    }

    pub fn kind(&self) -> VoterKind {
        match &self.0 {
            cml_chain::governance::Voter::ConstitutionalCommitteeHotKeyHash { .. } => {
                VoterKind::ConstitutionalCommitteeHotKeyHash
            }
            cml_chain::governance::Voter::ConstitutionalCommitteeHotScriptHash { .. } => {
                VoterKind::ConstitutionalCommitteeHotScriptHash
            }
            cml_chain::governance::Voter::DRepKeyHash { .. } => VoterKind::DRepKeyHash,
            cml_chain::governance::Voter::DRepScriptHash { .. } => VoterKind::DRepScriptHash,
            cml_chain::governance::Voter::StakingPoolKeyHash { .. } => {
                VoterKind::StakingPoolKeyHash
            }
        }
    }

    pub fn as_constitutional_committee_hot_key_hash(&self) -> Option<Ed25519KeyHash> {
        match &self.0 {
            cml_chain::governance::Voter::ConstitutionalCommitteeHotKeyHash {
                ed25519_key_hash,
                ..
            } => Some((*ed25519_key_hash).into()),
            _ => None,
        }
    }

    pub fn as_constitutional_committee_hot_script_hash(&self) -> Option<ScriptHash> {
        match &self.0 {
            cml_chain::governance::Voter::ConstitutionalCommitteeHotScriptHash {
                script_hash,
                ..
            } => Some((*script_hash).into()),
            _ => None,
        }
    }

    pub fn as_d_rep_key_hash(&self) -> Option<Ed25519KeyHash> {
        match &self.0 {
            cml_chain::governance::Voter::DRepKeyHash {
                ed25519_key_hash, ..
            } => Some((*ed25519_key_hash).into()),
            _ => None,
        }
    }

    pub fn as_d_rep_script_hash(&self) -> Option<ScriptHash> {
        match &self.0 {
            cml_chain::governance::Voter::DRepScriptHash { script_hash, .. } => {
                Some((*script_hash).into())
            }
            _ => None,
        }
    }

    pub fn as_staking_pool_key_hash(&self) -> Option<Ed25519KeyHash> {
        match &self.0 {
            cml_chain::governance::Voter::StakingPoolKeyHash {
                ed25519_key_hash, ..
            } => Some((*ed25519_key_hash).into()),
            _ => None,
        }
    }
}

#[wasm_bindgen]
pub enum VoterKind {
    ConstitutionalCommitteeHotKeyHash,
    ConstitutionalCommitteeHotScriptHash,
    DRepKeyHash,
    DRepScriptHash,
    StakingPoolKeyHash,
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct VotingProcedure(cml_chain::governance::VotingProcedure);

impl_wasm_cbor_json_api!(VotingProcedure);

impl_wasm_conversions!(cml_chain::governance::VotingProcedure, VotingProcedure);

#[wasm_bindgen]
impl VotingProcedure {
    pub fn vote(&self) -> Vote {
        self.0.vote
    }

    pub fn anchor(&self) -> Option<Anchor> {
        self.0.anchor.clone().map(std::convert::Into::into)
    }

    pub fn new(vote: Vote, anchor: Option<Anchor>) -> Self {
        Self(cml_chain::governance::VotingProcedure::new(
            vote,
            anchor.map(Into::into),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct VotingProcedures(cml_chain::governance::VotingProcedures);

impl_wasm_conversions!(cml_chain::governance::VotingProcedures, VotingProcedures);

#[wasm_bindgen]
impl VotingProcedures {
    pub fn new() -> Self {
        Self(OrderedHashMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(
        &mut self,
        key: &Voter,
        value: &MapGovActionIdToVotingProcedure,
    ) -> Option<MapGovActionIdToVotingProcedure> {
        self.0
            .insert(key.clone().into(), value.clone().into())
            .map(Into::into)
    }

    pub fn get(&self, key: &Voter) -> Option<MapGovActionIdToVotingProcedure> {
        self.0.get(key.as_ref()).map(|v| v.clone().into())
    }

    pub fn keys(&self) -> VoterList {
        self.0
            .iter()
            .map(|(k, _v)| k.clone())
            .collect::<Vec<_>>()
            .into()
    }
}
