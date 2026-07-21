// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

#![allow(
    clippy::too_many_arguments,
    clippy::large_enum_variant,
    clippy::result_large_err
)]

extern crate derivative;
pub mod address;
pub mod assets;
pub mod auxdata;
pub mod block;
pub mod certs;
pub mod crypto;
mod extern_interface_check;
pub mod governance;
mod key_demand_assertions;
pub mod plutus;
pub mod transaction;
pub use cml_core::{Int, IntError};
pub mod cbor_encodings;
pub mod serialization;

use address::RewardAccount;
use assets::Coin;
use cbor_encodings::{
    DRepVotingThresholdsEncoding, NetworkIdEncoding, PoolVotingThresholdsEncoding,
    ProtocolParamUpdateEncoding, RationalEncoding, UnitIntervalEncoding,
};
use certs::{Certificate, CommitteeColdCredential};
use cml_core::non_empty::NonEmptyVec;
use cml_core::ordered_hash_map::OrderedHashMap;
use cml_core::serialization::LenEncoding;
use crypto::{BootstrapWitness, Ed25519KeyHash, ScriptHash, Vkeywitness};
use governance::ProposalProcedure;
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

pub type Epoch = u64;

#[derive(Clone, Debug)]
pub struct NetworkId {
    pub(crate) inner: u64,
    pub encodings: Option<NetworkIdEncoding>,
}

impl NetworkId {
    pub fn get(&self) -> u64 {
        self.inner
    }

    pub fn new(inner: u64) -> Self {
        Self {
            inner,
            encodings: None,
        }
    }
}

impl From<u64> for NetworkId {
    fn from(inner: u64) -> Self {
        NetworkId::new(inner)
    }
}

impl serde::Serialize for NetworkId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.inner.serialize(serializer)
    }
}

impl<'de> serde::de::Deserialize<'de> for NetworkId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let inner = <u64 as serde::de::Deserialize>::deserialize(deserializer)?;
        Ok(Self::new(inner))
    }
}

impl schemars::JsonSchema for NetworkId {
    fn schema_name() -> ::std::borrow::Cow<'static, str> {
        ::std::borrow::Cow::Borrowed("NetworkId")
    }

    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        <u64 as schemars::JsonSchema>::json_schema(generator)
    }

    fn inline_schema() -> bool {
        <u64 as schemars::JsonSchema>::inline_schema()
    }
}

/// Synthesized convenience alias for an anonymous generic-collection instance (not a CDDL rule name).
/// `[+ BootstrapWitness]`: at least one element, enforced at the `NonEmptyVec` `TryFrom<Vec<_>>` door (the CBOR decoder routes through the same door, so wire-side and API-side rejection are identical).
/// The tag-258 set idiom: the tag is an encoding detail — both the `#6.258(...)` and the bare-array wire forms are accepted (serialization defaults to tagged), so either round-trips byte-exactly.
/// Duplicate elements are preserved and re-emitted byte-exactly in wire order (the default for a set idiom; opt into rejection with `@duplicates reject`).
pub type NonemptySetBootstrapWitness = NonEmptyVec<BootstrapWitness>;

/// Synthesized convenience alias for an anonymous generic-collection instance (not a CDDL rule name).
/// `[+ Certificate]`: at least one element, enforced at the `NonEmptyVec` `TryFrom<Vec<_>>` door (the CBOR decoder routes through the same door, so wire-side and API-side rejection are identical).
/// The tag-258 set idiom: the tag is an encoding detail — both the `#6.258(...)` and the bare-array wire forms are accepted (serialization defaults to tagged), so either round-trips byte-exactly.
/// Duplicate elements are preserved and re-emitted byte-exactly in wire order (the default for a set idiom; opt into rejection with `@duplicates reject`).
pub type NonemptySetCertificate = NonEmptyVec<Certificate>;

/// Synthesized convenience alias for an anonymous generic-collection instance (not a CDDL rule name).
/// `[+ NativeScript]`: at least one element, enforced at the `NonEmptyVec` `TryFrom<Vec<_>>` door (the CBOR decoder routes through the same door, so wire-side and API-side rejection are identical).
/// The tag-258 set idiom: the tag is an encoding detail — both the `#6.258(...)` and the bare-array wire forms are accepted (serialization defaults to tagged), so either round-trips byte-exactly.
/// Duplicate elements are preserved and re-emitted byte-exactly in wire order (the default for a set idiom; opt into rejection with `@duplicates reject`).
pub type NonemptySetNativeScript = NonEmptyVec<NativeScript>;

/// Synthesized convenience alias for an anonymous generic-collection instance (not a CDDL rule name).
/// `[+ PlutusData]`: at least one element, enforced at the `NonEmptyVec` `TryFrom<Vec<_>>` door (the CBOR decoder routes through the same door, so wire-side and API-side rejection are identical).
/// The tag-258 set idiom: the tag is an encoding detail — both the `#6.258(...)` and the bare-array wire forms are accepted (serialization defaults to tagged), so either round-trips byte-exactly.
/// Duplicate elements are preserved and re-emitted byte-exactly in wire order (the default for a set idiom; opt into rejection with `@duplicates reject`).
pub type NonemptySetPlutusData = NonEmptyVec<PlutusData>;

/// Synthesized convenience alias for an anonymous generic-collection instance (not a CDDL rule name).
/// `[+ PlutusV1Script]`: at least one element, enforced at the `NonEmptyVec` `TryFrom<Vec<_>>` door (the CBOR decoder routes through the same door, so wire-side and API-side rejection are identical).
/// The tag-258 set idiom: the tag is an encoding detail — both the `#6.258(...)` and the bare-array wire forms are accepted (serialization defaults to tagged), so either round-trips byte-exactly.
/// Duplicate elements are preserved and re-emitted byte-exactly in wire order (the default for a set idiom; opt into rejection with `@duplicates reject`).
pub type NonemptySetPlutusV1Script = NonEmptyVec<PlutusV1Script>;

/// Synthesized convenience alias for an anonymous generic-collection instance (not a CDDL rule name).
/// `[+ PlutusV2Script]`: at least one element, enforced at the `NonEmptyVec` `TryFrom<Vec<_>>` door (the CBOR decoder routes through the same door, so wire-side and API-side rejection are identical).
/// The tag-258 set idiom: the tag is an encoding detail — both the `#6.258(...)` and the bare-array wire forms are accepted (serialization defaults to tagged), so either round-trips byte-exactly.
/// Duplicate elements are preserved and re-emitted byte-exactly in wire order (the default for a set idiom; opt into rejection with `@duplicates reject`).
pub type NonemptySetPlutusV2Script = NonEmptyVec<PlutusV2Script>;

/// Synthesized convenience alias for an anonymous generic-collection instance (not a CDDL rule name).
/// `[+ PlutusV3Script]`: at least one element, enforced at the `NonEmptyVec` `TryFrom<Vec<_>>` door (the CBOR decoder routes through the same door, so wire-side and API-side rejection are identical).
/// The tag-258 set idiom: the tag is an encoding detail — both the `#6.258(...)` and the bare-array wire forms are accepted (serialization defaults to tagged), so either round-trips byte-exactly.
/// Duplicate elements are preserved and re-emitted byte-exactly in wire order (the default for a set idiom; opt into rejection with `@duplicates reject`).
pub type NonemptySetPlutusV3Script = NonEmptyVec<PlutusV3Script>;

/// Synthesized convenience alias for an anonymous generic-collection instance (not a CDDL rule name).
/// `[+ ProposalProcedure]`: at least one element, enforced at the `NonEmptyVec` `TryFrom<Vec<_>>` door (the CBOR decoder routes through the same door, so wire-side and API-side rejection are identical).
/// The tag-258 set idiom: the tag is an encoding detail — both the `#6.258(...)` and the bare-array wire forms are accepted (serialization defaults to tagged), so either round-trips byte-exactly.
/// Duplicate elements are preserved and re-emitted byte-exactly in wire order (the default for a set idiom; opt into rejection with `@duplicates reject`).
pub type NonemptySetProposalProcedure = NonEmptyVec<ProposalProcedure>;

/// Synthesized convenience alias for an anonymous generic-collection instance (not a CDDL rule name).
/// `[+ TransactionInput]`: at least one element, enforced at the `NonEmptyVec` `TryFrom<Vec<_>>` door (the CBOR decoder routes through the same door, so wire-side and API-side rejection are identical).
/// The tag-258 set idiom: the tag is an encoding detail — both the `#6.258(...)` and the bare-array wire forms are accepted (serialization defaults to tagged), so either round-trips byte-exactly.
/// Duplicate elements are preserved and re-emitted byte-exactly in wire order (the default for a set idiom; opt into rejection with `@duplicates reject`).
pub type NonemptySetTransactionInput = NonEmptyVec<TransactionInput>;

/// Synthesized convenience alias for an anonymous generic-collection instance (not a CDDL rule name).
/// `[+ Vkeywitness]`: at least one element, enforced at the `NonEmptyVec` `TryFrom<Vec<_>>` door (the CBOR decoder routes through the same door, so wire-side and API-side rejection are identical).
/// The tag-258 set idiom: the tag is an encoding detail — both the `#6.258(...)` and the bare-array wire forms are accepted (serialization defaults to tagged), so either round-trips byte-exactly.
/// Duplicate elements are preserved and re-emitted byte-exactly in wire order (the default for a set idiom; opt into rejection with `@duplicates reject`).
pub type NonemptySetVkeywitness = NonEmptyVec<Vkeywitness>;

pub type PolicyId = ScriptHash;

pub type PolicyIdList = Vec<PolicyId>;

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
pub enum Script {
    Native {
        script: NativeScript,
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
        tag_encoding: Option<cbor_event::Sz>,
    },
    PlutusV1 {
        script: PlutusV1Script,
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
        tag_encoding: Option<cbor_event::Sz>,
    },
    PlutusV2 {
        script: PlutusV2Script,
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
        tag_encoding: Option<cbor_event::Sz>,
    },
    PlutusV3 {
        script: PlutusV3Script,
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

/// Synthesized convenience alias for an anonymous generic-collection instance (not a CDDL rule name).
/// The tag-258 set idiom: the tag is an encoding detail — both the `#6.258(...)` and the bare-array wire forms are accepted (serialization defaults to tagged), so either round-trips byte-exactly.
/// Duplicate elements are preserved and re-emitted byte-exactly in wire order (the default for a set idiom; opt into rejection with `@duplicates reject`).
pub type SetCommitteeColdCredential = Vec<CommitteeColdCredential>;

/// Synthesized convenience alias for an anonymous generic-collection instance (not a CDDL rule name).
/// The tag-258 set idiom: the tag is an encoding detail — both the `#6.258(...)` and the bare-array wire forms are accepted (serialization defaults to tagged), so either round-trips byte-exactly.
/// Duplicate elements are preserved and re-emitted byte-exactly in wire order (the default for a set idiom; opt into rejection with `@duplicates reject`).
pub type SetEd25519KeyHash = Vec<Ed25519KeyHash>;

/// Synthesized convenience alias for an anonymous generic-collection instance (not a CDDL rule name).
/// The tag-258 set idiom: the tag is an encoding detail — both the `#6.258(...)` and the bare-array wire forms are accepted (serialization defaults to tagged), so either round-trips byte-exactly.
/// Duplicate elements are preserved and re-emitted byte-exactly in wire order (the default for a set idiom; opt into rejection with `@duplicates reject`).
pub type SetTransactionInput = Vec<TransactionInput>;

pub type Slot = u64;

pub type SubCoin = Rational;

pub type TransactionIndex = u16;

pub type TransactionMetadatumLabel = u64;

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

pub type Withdrawals = OrderedHashMap<RewardAccount, Coin>;

impl From<NetworkId> for u64 {
    fn from(wrapper: NetworkId) -> Self {
        wrapper.inner
    }
}
