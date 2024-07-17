// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

pub mod cbor_encodings;
pub mod serialization;
pub mod utils;

#[cfg(not(feature = "used_from_wasm"))]
use noop_proc_macro::wasm_bindgen;
#[cfg(feature = "used_from_wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

use crate::address::RewardAccount;
use crate::assets::Coin;
use crate::block::ProtocolVersion;
use crate::certs::{CommitteeColdCredential, Url};
use crate::crypto::{AnchorDocHash, Ed25519KeyHash, ScriptHash, TransactionHash};
use crate::{Epoch, ProtocolParamUpdate, SetCommitteeColdCredential, UnitInterval};
use cbor_encodings::{
    AnchorEncoding, ConstitutionEncoding, GovActionIdEncoding, HardForkInitiationActionEncoding,
    NewConstitutionEncoding, NoConfidenceEncoding, ParameterChangeActionEncoding,
    ProposalProcedureEncoding, TreasuryWithdrawalsActionEncoding, UpdateCommitteeEncoding,
    VotingProcedureEncoding,
};

use cml_core::ordered_hash_map::OrderedHashMap;
use cml_core::serialization::{LenEncoding, StringEncoding};
use std::collections::BTreeMap;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct Anchor {
    pub anchor_url: Url,
    pub anchor_doc_hash: AnchorDocHash,
    #[serde(skip)]
    pub encodings: Option<AnchorEncoding>,
}

impl Anchor {
    pub fn new(anchor_url: Url, anchor_doc_hash: AnchorDocHash) -> Self {
        Self {
            anchor_url,
            anchor_doc_hash,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct Constitution {
    pub anchor: Anchor,
    pub script_hash: Option<ScriptHash>,
    #[serde(skip)]
    pub encodings: Option<ConstitutionEncoding>,
}

impl Constitution {
    pub fn new(anchor: Anchor, script_hash: Option<ScriptHash>) -> Self {
        Self {
            anchor,
            script_hash,
            encodings: None,
        }
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub enum GovAction {
    ParameterChangeAction(ParameterChangeAction),
    HardForkInitiationAction(HardForkInitiationAction),
    TreasuryWithdrawalsAction(TreasuryWithdrawalsAction),
    NoConfidence(NoConfidence),
    UpdateCommittee(UpdateCommittee),
    NewConstitution(NewConstitution),
    InfoAction {
        #[serde(skip)]
        info_action_encoding: Option<cbor_event::Sz>,
        #[serde(skip)]
        len_encoding: LenEncoding,
    },
}

impl GovAction {
    pub fn new_parameter_change_action(
        action_id: Option<GovActionId>,
        update: ProtocolParamUpdate,
        policy_hash: Option<ScriptHash>,
    ) -> Self {
        Self::ParameterChangeAction(ParameterChangeAction::new(action_id, update, policy_hash))
    }

    pub fn new_hard_fork_initiation_action(
        action_id: Option<GovActionId>,
        version: ProtocolVersion,
    ) -> Self {
        Self::HardForkInitiationAction(HardForkInitiationAction::new(action_id, version))
    }

    pub fn new_treasury_withdrawals_action(
        withdrawal: OrderedHashMap<RewardAccount, Coin>,
        policy_hash: Option<ScriptHash>,
    ) -> Self {
        Self::TreasuryWithdrawalsAction(TreasuryWithdrawalsAction::new(withdrawal, policy_hash))
    }

    pub fn new_no_confidence(action_id: Option<GovActionId>) -> Self {
        Self::NoConfidence(NoConfidence::new(action_id))
    }

    pub fn new_update_committee(
        action_id: Option<GovActionId>,
        cold_credentials: SetCommitteeColdCredential,
        credentials: OrderedHashMap<CommitteeColdCredential, Epoch>,
        unit_interval: UnitInterval,
    ) -> Self {
        Self::UpdateCommittee(UpdateCommittee::new(
            action_id,
            cold_credentials,
            credentials,
            unit_interval,
        ))
    }

    pub fn new_new_constitution(
        action_id: Option<GovActionId>,
        constitution: Constitution,
    ) -> Self {
        Self::NewConstitution(NewConstitution::new(action_id, constitution))
    }

    pub fn new_info_action() -> Self {
        Self::InfoAction {
            info_action_encoding: None,
            len_encoding: LenEncoding::default(),
        }
    }
}

#[derive(
    Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema, derivative::Derivative,
)]
#[derivative(Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct GovActionId {
    pub transaction_id: TransactionHash,
    pub gov_action_index: u64,
    #[derivative(
        PartialEq = "ignore",
        Ord = "ignore",
        PartialOrd = "ignore",
        Hash = "ignore"
    )]
    #[serde(skip)]
    pub encodings: Option<GovActionIdEncoding>,
}

impl GovActionId {
    pub fn new(transaction_id: TransactionHash, gov_action_index: u64) -> Self {
        Self {
            transaction_id,
            gov_action_index,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct HardForkInitiationAction {
    pub action_id: Option<GovActionId>,
    pub version: ProtocolVersion,
    #[serde(skip)]
    pub encodings: Option<HardForkInitiationActionEncoding>,
}

impl HardForkInitiationAction {
    pub fn new(action_id: Option<GovActionId>, version: ProtocolVersion) -> Self {
        Self {
            action_id,
            version,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct NewConstitution {
    pub action_id: Option<GovActionId>,
    pub constitution: Constitution,
    #[serde(skip)]
    pub encodings: Option<NewConstitutionEncoding>,
}

impl NewConstitution {
    pub fn new(action_id: Option<GovActionId>, constitution: Constitution) -> Self {
        Self {
            action_id,
            constitution,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct NoConfidence {
    pub action_id: Option<GovActionId>,
    #[serde(skip)]
    pub encodings: Option<NoConfidenceEncoding>,
}

impl NoConfidence {
    pub fn new(action_id: Option<GovActionId>) -> Self {
        Self {
            action_id,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ParameterChangeAction {
    pub action_id: Option<GovActionId>,
    pub update: ProtocolParamUpdate,
    pub policy_hash: Option<ScriptHash>,
    #[serde(skip)]
    pub encodings: Option<ParameterChangeActionEncoding>,
}

impl ParameterChangeAction {
    pub fn new(
        action_id: Option<GovActionId>,
        update: ProtocolParamUpdate,
        policy_hash: Option<ScriptHash>,
    ) -> Self {
        Self {
            action_id,
            update,
            policy_hash,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ProposalProcedure {
    pub deposit: Coin,
    pub reward_account: RewardAccount,
    pub gov_action: GovAction,
    pub anchor: Anchor,
    #[serde(skip)]
    pub encodings: Option<ProposalProcedureEncoding>,
}

impl ProposalProcedure {
    pub fn new(
        deposit: Coin,
        reward_account: RewardAccount,
        gov_action: GovAction,
        anchor: Anchor,
    ) -> Self {
        Self {
            deposit,
            reward_account,
            gov_action,
            anchor,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct TreasuryWithdrawalsAction {
    pub withdrawal: OrderedHashMap<RewardAccount, Coin>,
    pub policy_hash: Option<ScriptHash>,
    #[serde(skip)]
    pub encodings: Option<TreasuryWithdrawalsActionEncoding>,
}

impl TreasuryWithdrawalsAction {
    pub fn new(
        withdrawal: OrderedHashMap<RewardAccount, Coin>,
        policy_hash: Option<ScriptHash>,
    ) -> Self {
        Self {
            withdrawal,
            policy_hash,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct UpdateCommittee {
    pub action_id: Option<GovActionId>,
    pub cold_credentials: SetCommitteeColdCredential,
    pub credentials: OrderedHashMap<CommitteeColdCredential, Epoch>,
    pub unit_interval: UnitInterval,
    #[serde(skip)]
    pub encodings: Option<UpdateCommitteeEncoding>,
}

impl UpdateCommittee {
    pub fn new(
        action_id: Option<GovActionId>,
        cold_credentials: SetCommitteeColdCredential,
        credentials: OrderedHashMap<CommitteeColdCredential, Epoch>,
        unit_interval: UnitInterval,
    ) -> Self {
        Self {
            action_id,
            cold_credentials,
            credentials,
            unit_interval,
            encodings: None,
        }
    }
}

#[derive(
    Copy,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Clone,
    Debug,
    serde::Deserialize,
    serde::Serialize,
    schemars::JsonSchema,
)]
#[wasm_bindgen]
pub enum Vote {
    No,
    Yes,
    Abstain,
}

#[derive(
    Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema, derivative::Derivative,
)]
#[derivative(
    Eq,
    PartialEq,
    Ord = "feature_allow_slow_enum",
    PartialOrd = "feature_allow_slow_enum",
    Hash
)]
pub enum Voter {
    ConstitutionalCommitteeHotKeyHash {
        ed25519_key_hash: Ed25519KeyHash,
        #[derivative(
            PartialEq = "ignore",
            Ord = "ignore",
            PartialOrd = "ignore",
            Hash = "ignore"
        )]
        #[serde(skip)]
        len_encoding: LenEncoding,
        #[derivative(
            PartialEq = "ignore",
            Ord = "ignore",
            PartialOrd = "ignore",
            Hash = "ignore"
        )]
        #[serde(skip)]
        index_0_encoding: Option<cbor_event::Sz>,
        #[derivative(
            PartialEq = "ignore",
            Ord = "ignore",
            PartialOrd = "ignore",
            Hash = "ignore"
        )]
        #[serde(skip)]
        ed25519_key_hash_encoding: StringEncoding,
    },
    ConstitutionalCommitteeHotScriptHash {
        script_hash: ScriptHash,
        #[derivative(
            PartialEq = "ignore",
            Ord = "ignore",
            PartialOrd = "ignore",
            Hash = "ignore"
        )]
        #[serde(skip)]
        len_encoding: LenEncoding,
        #[derivative(
            PartialEq = "ignore",
            Ord = "ignore",
            PartialOrd = "ignore",
            Hash = "ignore"
        )]
        #[serde(skip)]
        index_0_encoding: Option<cbor_event::Sz>,
        #[derivative(
            PartialEq = "ignore",
            Ord = "ignore",
            PartialOrd = "ignore",
            Hash = "ignore"
        )]
        #[serde(skip)]
        script_hash_encoding: StringEncoding,
    },
    DRepKeyHash {
        ed25519_key_hash: Ed25519KeyHash,
        #[derivative(
            PartialEq = "ignore",
            Ord = "ignore",
            PartialOrd = "ignore",
            Hash = "ignore"
        )]
        #[serde(skip)]
        len_encoding: LenEncoding,
        #[derivative(
            PartialEq = "ignore",
            Ord = "ignore",
            PartialOrd = "ignore",
            Hash = "ignore"
        )]
        #[serde(skip)]
        index_0_encoding: Option<cbor_event::Sz>,
        #[derivative(
            PartialEq = "ignore",
            Ord = "ignore",
            PartialOrd = "ignore",
            Hash = "ignore"
        )]
        #[serde(skip)]
        ed25519_key_hash_encoding: StringEncoding,
    },
    DRepScriptHash {
        script_hash: ScriptHash,
        #[derivative(
            PartialEq = "ignore",
            Ord = "ignore",
            PartialOrd = "ignore",
            Hash = "ignore"
        )]
        #[serde(skip)]
        len_encoding: LenEncoding,
        #[derivative(
            PartialEq = "ignore",
            Ord = "ignore",
            PartialOrd = "ignore",
            Hash = "ignore"
        )]
        #[serde(skip)]
        index_0_encoding: Option<cbor_event::Sz>,
        #[derivative(
            PartialEq = "ignore",
            Ord = "ignore",
            PartialOrd = "ignore",
            Hash = "ignore"
        )]
        #[serde(skip)]
        script_hash_encoding: StringEncoding,
    },
    StakingPoolKeyHash {
        ed25519_key_hash: Ed25519KeyHash,
        #[derivative(
            PartialEq = "ignore",
            Ord = "ignore",
            PartialOrd = "ignore",
            Hash = "ignore"
        )]
        #[serde(skip)]
        len_encoding: LenEncoding,
        #[derivative(
            PartialEq = "ignore",
            Ord = "ignore",
            PartialOrd = "ignore",
            Hash = "ignore"
        )]
        #[serde(skip)]
        index_0_encoding: Option<cbor_event::Sz>,
        #[derivative(
            PartialEq = "ignore",
            Ord = "ignore",
            PartialOrd = "ignore",
            Hash = "ignore"
        )]
        #[serde(skip)]
        ed25519_key_hash_encoding: StringEncoding,
    },
}

impl Voter {
    pub fn new_constitutional_committee_hot_key_hash(ed25519_key_hash: Ed25519KeyHash) -> Self {
        Self::ConstitutionalCommitteeHotKeyHash {
            ed25519_key_hash,
            len_encoding: LenEncoding::default(),
            index_0_encoding: None,
            ed25519_key_hash_encoding: StringEncoding::default(),
        }
    }

    pub fn new_constitutional_committee_hot_script_hash(script_hash: ScriptHash) -> Self {
        Self::ConstitutionalCommitteeHotScriptHash {
            script_hash,
            len_encoding: LenEncoding::default(),
            index_0_encoding: None,
            script_hash_encoding: StringEncoding::default(),
        }
    }

    pub fn new_d_rep_key_hash(ed25519_key_hash: Ed25519KeyHash) -> Self {
        Self::DRepKeyHash {
            ed25519_key_hash,
            len_encoding: LenEncoding::default(),
            index_0_encoding: None,
            ed25519_key_hash_encoding: StringEncoding::default(),
        }
    }

    pub fn new_d_rep_script_hash(script_hash: ScriptHash) -> Self {
        Self::DRepScriptHash {
            script_hash,
            len_encoding: LenEncoding::default(),
            index_0_encoding: None,
            script_hash_encoding: StringEncoding::default(),
        }
    }

    pub fn new_staking_pool_key_hash(ed25519_key_hash: Ed25519KeyHash) -> Self {
        Self::StakingPoolKeyHash {
            ed25519_key_hash,
            len_encoding: LenEncoding::default(),
            index_0_encoding: None,
            ed25519_key_hash_encoding: StringEncoding::default(),
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct VotingProcedure {
    pub vote: Vote,
    pub anchor: Option<Anchor>,
    #[serde(skip)]
    pub encodings: Option<VotingProcedureEncoding>,
}

impl VotingProcedure {
    pub fn new(vote: Vote, anchor: Option<Anchor>) -> Self {
        Self {
            vote,
            anchor,
            encodings: None,
        }
    }
}

pub type VotingProcedures = OrderedHashMap<Voter, OrderedHashMap<GovActionId, VotingProcedure>>;
