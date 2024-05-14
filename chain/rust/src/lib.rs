// This recently introduced lint does not play well with the derivative crate.
// We have both Ord and PartialOrd derive automatically by derivative's proc macros
// but clippy sees these as hand implementations.
// Putting this allow locally where it's found did not seem to supress it,
// likely due to the structure of how the proc macro derives the code.
// Doing what is suggested by this lint would just result in us actually doing
// hand implementations of the PartialOrd (an maybe PartialEq) when there's no need,
// possibly impacting PartialOrd performance on top of being unnecessary and occuring in generated code.
// Possibly the derivative crate could get updated to suppress this lint
// from within their proc macros itself. Issue: https://github.com/mcarton/rust-derivative/issues/115
#![allow(clippy::non_canonical_partial_ord_impl)]

pub mod address;
pub mod assets;
pub mod auxdata;
pub mod block;
pub mod builders;
pub mod byron;
pub mod certs;
pub mod crypto;
pub mod deposit;
pub mod fees;
pub mod genesis;
pub mod governance;
pub mod json;
pub mod min_ada;
pub mod plutus;
pub mod serialization;
pub mod transaction;
pub mod utils;

pub use assets::{Coin, Value};
use certs::{Certificate, CommitteeColdCredential};
use cml_crypto::Ed25519KeyHash;
use crypto::{BootstrapWitness, Vkeywitness};
use utils::NonemptySetRawBytes;
pub use utils::{NetworkId, NonemptySet, Set};

//pub mod legacy_address;

pub use cml_core::{
    error::{DeserializeError, DeserializeFailure},
    ordered_hash_map::OrderedHashMap,
    serialization::{Deserialize, LenEncoding, Serialize, StringEncoding},
    CertificateIndex, Epoch, Int, Slot, TransactionIndex,
};

pub mod cbor_encodings;

extern crate derivative;

// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use address::RewardAccount;
use cbor_encodings::{
    DRepVotingThresholdsEncoding, PoolVotingThresholdsEncoding, ProtocolParamUpdateEncoding,
    RationalEncoding, UnitIntervalEncoding,
};
use governance::{ProposalProcedure, Voter};
use plutus::{
    CostModels, ExUnitPrices, ExUnits, PlutusData, PlutusV1Script, PlutusV2Script, PlutusV3Script,
};
use transaction::{NativeScript, TransactionInput};

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct DRepVotingThresholds {
    pub motion_no_confidence: UnitInterval,
    pub committee_normal: UnitInterval,
    pub committee_no_confidence: UnitInterval,
    pub update_constitution: UnitInterval,
    pub hard_fork_initiation: UnitInterval,
    pub pp_network_group: UnitInterval,
    pub pp_economic_group: UnitInterval,
    pub pp_technical_group: UnitInterval,
    pub pp_governance_group: UnitInterval,
    pub treasury_withdrawal: UnitInterval,
    #[serde(skip)]
    pub encodings: Option<DRepVotingThresholdsEncoding>,
}

impl DRepVotingThresholds {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        motion_no_confidence: UnitInterval,
        committee_normal: UnitInterval,
        committee_no_confidence: UnitInterval,
        update_constitution: UnitInterval,
        hard_fork_initiation: UnitInterval,
        pp_network_group: UnitInterval,
        pp_economic_group: UnitInterval,
        pp_technical_group: UnitInterval,
        pp_governance_group: UnitInterval,
        treasury_withdrawal: UnitInterval,
    ) -> Self {
        Self {
            motion_no_confidence,
            committee_normal,
            committee_no_confidence,
            update_constitution,
            hard_fork_initiation,
            pp_network_group,
            pp_economic_group,
            pp_technical_group,
            pp_governance_group,
            treasury_withdrawal,
            encodings: None,
        }
    }
}

pub type DeltaCoin = Int;

pub type NonemptySetBootstrapWitness = NonemptySet<BootstrapWitness>;

pub type NonemptySetCertificate = NonemptySet<Certificate>;

pub type NonemptySetNativeScript = NonemptySet<NativeScript>;

pub type NonemptySetPlutusData = NonemptySet<PlutusData>;

pub type NonemptySetPlutusV1Script = NonemptySet<PlutusV1Script>;

pub type NonemptySetPlutusV2Script = NonemptySet<PlutusV2Script>;

pub type NonemptySetPlutusV3Script = NonemptySet<PlutusV3Script>;

pub type NonemptySetProposalProcedure = NonemptySet<ProposalProcedure>;

pub type NonemptySetTransactionInput = NonemptySet<TransactionInput>;

pub type NonemptySetVkeywitness = NonemptySet<Vkeywitness>;

pub type PolicyId = cml_crypto::ScriptHash;

pub type PolicyIdList = Vec<PolicyId>;

pub type RequiredSigners = NonemptySetRawBytes<Ed25519KeyHash>;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct PoolVotingThresholds {
    pub motion_no_confidence: UnitInterval,
    pub committee_normal: UnitInterval,
    pub committee_no_confidence: UnitInterval,
    pub hard_fork_initiation: UnitInterval,
    pub security_relevant_parameter_voting_threshold: UnitInterval,
    #[serde(skip)]
    pub encodings: Option<PoolVotingThresholdsEncoding>,
}

impl PoolVotingThresholds {
    pub fn new(
        motion_no_confidence: UnitInterval,
        committee_normal: UnitInterval,
        committee_no_confidence: UnitInterval,
        hard_fork_initiation: UnitInterval,
        security_relevant_parameter_voting_threshold: UnitInterval,
    ) -> Self {
        Self {
            motion_no_confidence,
            committee_normal,
            committee_no_confidence,
            hard_fork_initiation,
            security_relevant_parameter_voting_threshold,
            encodings: None,
        }
    }
}

pub type Port = u16;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ProtocolParamUpdate {
    pub minfee_a: Option<Coin>,
    pub minfee_b: Option<Coin>,
    pub max_block_body_size: Option<u64>,
    pub max_transaction_size: Option<u64>,
    pub max_block_header_size: Option<u64>,
    pub key_deposit: Option<Coin>,
    pub pool_deposit: Option<Coin>,
    pub maximum_epoch: Option<Epoch>,
    pub n_opt: Option<u64>,
    pub pool_pledge_influence: Option<Rational>,
    pub expansion_rate: Option<UnitInterval>,
    pub treasury_growth_rate: Option<UnitInterval>,
    pub min_pool_cost: Option<Coin>,
    pub ada_per_utxo_byte: Option<Coin>,
    pub cost_models_for_script_languages: Option<CostModels>,
    pub execution_costs: Option<ExUnitPrices>,
    pub max_tx_ex_units: Option<ExUnits>,
    pub max_block_ex_units: Option<ExUnits>,
    pub max_value_size: Option<u64>,
    pub collateral_percentage: Option<u64>,
    pub max_collateral_inputs: Option<u64>,
    pub pool_voting_thresholds: Option<PoolVotingThresholds>,
    pub d_rep_voting_thresholds: Option<DRepVotingThresholds>,
    pub min_committee_size: Option<u64>,
    pub committee_term_limit: Option<Epoch>,
    pub governance_action_validity_period: Option<Epoch>,
    pub governance_action_deposit: Option<Coin>,
    pub d_rep_deposit: Option<Coin>,
    pub d_rep_inactivity_period: Option<Epoch>,
    pub min_fee_ref_script_cost_per_byte: Option<Rational>,
    #[serde(skip)]
    pub encodings: Option<ProtocolParamUpdateEncoding>,
}

impl ProtocolParamUpdate {
    pub fn new() -> Self {
        Self {
            minfee_a: None,
            minfee_b: None,
            max_block_body_size: None,
            max_transaction_size: None,
            max_block_header_size: None,
            key_deposit: None,
            pool_deposit: None,
            maximum_epoch: None,
            n_opt: None,
            pool_pledge_influence: None,
            expansion_rate: None,
            treasury_growth_rate: None,
            min_pool_cost: None,
            ada_per_utxo_byte: None,
            cost_models_for_script_languages: None,
            execution_costs: None,
            max_tx_ex_units: None,
            max_block_ex_units: None,
            max_value_size: None,
            collateral_percentage: None,
            max_collateral_inputs: None,
            pool_voting_thresholds: None,
            d_rep_voting_thresholds: None,
            min_committee_size: None,
            committee_term_limit: None,
            governance_action_validity_period: None,
            governance_action_deposit: None,
            d_rep_deposit: None,
            d_rep_inactivity_period: None,
            min_fee_ref_script_cost_per_byte: None,
            encodings: None,
        }
    }
}

impl Default for ProtocolParamUpdate {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct Rational {
    pub numerator: u64,
    pub denominator: u64,
    #[serde(skip)]
    pub encodings: Option<RationalEncoding>,
}

impl Rational {
    pub fn new(numerator: u64, denominator: u64) -> Self {
        Self {
            numerator,
            denominator,
            encodings: None,
        }
    }
}

pub type RewardAccountList = Vec<RewardAccount>;

#[derive(
    Clone, Debug, derivative::Derivative, serde::Deserialize, serde::Serialize, schemars::JsonSchema,
)]
#[derivative(Hash, PartialEq)]
pub enum Script {
    Native {
        script: NativeScript,
        #[serde(skip)]
        #[derivative(PartialEq = "ignore", Hash = "ignore")]
        len_encoding: LenEncoding,
        #[serde(skip)]
        #[derivative(PartialEq = "ignore", Hash = "ignore")]
        tag_encoding: Option<cbor_event::Sz>,
    },
    PlutusV1 {
        script: PlutusV1Script,
        #[serde(skip)]
        #[derivative(PartialEq = "ignore", Hash = "ignore")]
        len_encoding: LenEncoding,
        #[serde(skip)]
        #[derivative(PartialEq = "ignore", Hash = "ignore")]
        tag_encoding: Option<cbor_event::Sz>,
    },
    PlutusV2 {
        script: PlutusV2Script,
        #[serde(skip)]
        #[derivative(PartialEq = "ignore", Hash = "ignore")]
        len_encoding: LenEncoding,
        #[serde(skip)]
        #[derivative(PartialEq = "ignore", Hash = "ignore")]
        tag_encoding: Option<cbor_event::Sz>,
    },
    PlutusV3 {
        script: PlutusV3Script,
        #[serde(skip)]
        #[derivative(PartialEq = "ignore", Hash = "ignore")]
        len_encoding: LenEncoding,
        #[serde(skip)]
        #[derivative(PartialEq = "ignore", Hash = "ignore")]
        tag_encoding: Option<cbor_event::Sz>,
    },
}

impl Script {
    pub fn new_native(script: NativeScript) -> Self {
        Self::Native {
            script,
            len_encoding: LenEncoding::default(),
            tag_encoding: None,
        }
    }

    pub fn new_plutus_v1(script: PlutusV1Script) -> Self {
        Self::PlutusV1 {
            script,
            len_encoding: LenEncoding::default(),
            tag_encoding: None,
        }
    }

    pub fn new_plutus_v2(script: PlutusV2Script) -> Self {
        Self::PlutusV2 {
            script,
            len_encoding: LenEncoding::default(),
            tag_encoding: None,
        }
    }

    pub fn new_plutus_v3(script: PlutusV3Script) -> Self {
        Self::PlutusV3 {
            script,
            len_encoding: LenEncoding::default(),
            tag_encoding: None,
        }
    }
}

pub type SetCommitteeColdCredential = Set<CommitteeColdCredential>;
pub type SetEd25519KeyHash = NonemptySetRawBytes<Ed25519KeyHash>;

pub type SetTransactionInput = Set<TransactionInput>;

pub type SubCoin = Rational;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct UnitInterval {
    pub start: u64,
    pub end: u64,
    #[serde(skip)]
    pub encodings: Option<UnitIntervalEncoding>,
}

impl UnitInterval {
    pub fn new(start: u64, end: u64) -> Self {
        Self {
            start,
            end,
            encodings: None,
        }
    }
}

pub type VoterList = Vec<Voter>;

pub type Withdrawals = OrderedHashMap<RewardAccount, Coin>;
