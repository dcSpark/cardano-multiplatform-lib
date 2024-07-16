// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

pub mod cbor_encodings;
pub mod serialization;
pub mod utils;

#[cfg(not(feature = "used_from_wasm"))]
use noop_proc_macro::wasm_bindgen;
#[cfg(feature = "used_from_wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

use crate::shelley::{
    GenesisKeyDelegation, ShelleyHeader, ShelleyPoolParams, ShelleyPoolRegistration,
    ShelleyTransactionOutput, ShelleyUpdate,
};
use cbor_encodings::{
    AllegraBlockEncoding, AllegraTransactionBodyEncoding, AllegraTransactionEncoding,
    AllegraTransactionWitnessSetEncoding,
};
use cml_chain::assets::Coin;
use cml_chain::auxdata::{ShelleyFormatAuxData, ShelleyMAFormatAuxData};
use cml_chain::certs::{
    PoolRetirement, StakeCredential, StakeDelegation, StakeDeregistration, StakeRegistration,
};
use cml_chain::crypto::{AuxiliaryDataHash, BootstrapWitness, Vkeywitness};
use cml_chain::transaction::{NativeScript, TransactionInput};
use cml_chain::Withdrawals;
use cml_chain::{DeltaCoin, LenEncoding, TransactionIndex};
use cml_core::ordered_hash_map::OrderedHashMap;
use cml_core::Epoch;
use cml_crypto::{Ed25519KeyHash, GenesisDelegateHash, GenesisHash, VRFKeyHash};
use std::collections::BTreeMap;

use self::cbor_encodings::{MoveInstantaneousRewardEncoding, MoveInstantaneousRewardsCertEncoding};

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub enum AllegraAuxiliaryData {
    Shelley(ShelleyFormatAuxData),
    ShelleyMA(ShelleyMAFormatAuxData),
}

impl AllegraAuxiliaryData {
    pub fn new_shelley(shelley: ShelleyFormatAuxData) -> Self {
        Self::Shelley(shelley)
    }

    pub fn new_shelley_ma(shelley_ma: ShelleyMAFormatAuxData) -> Self {
        Self::ShelleyMA(shelley_ma)
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct AllegraBlock {
    pub header: ShelleyHeader,
    pub transaction_bodies: Vec<AllegraTransactionBody>,
    pub transaction_witness_sets: Vec<AllegraTransactionWitnessSet>,
    pub auxiliary_data_set: OrderedHashMap<TransactionIndex, AllegraAuxiliaryData>,
    #[serde(skip)]
    pub encodings: Option<AllegraBlockEncoding>,
}

impl AllegraBlock {
    pub fn new(
        header: ShelleyHeader,
        transaction_bodies: Vec<AllegraTransactionBody>,
        transaction_witness_sets: Vec<AllegraTransactionWitnessSet>,
        auxiliary_data_set: OrderedHashMap<TransactionIndex, AllegraAuxiliaryData>,
    ) -> Self {
        Self {
            header,
            transaction_bodies,
            transaction_witness_sets,
            auxiliary_data_set,
            encodings: None,
        }
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub enum AllegraCertificate {
    StakeRegistration(StakeRegistration),
    StakeDeregistration(StakeDeregistration),
    StakeDelegation(StakeDelegation),
    ShelleyPoolRegistration(ShelleyPoolRegistration),
    PoolRetirement(PoolRetirement),
    GenesisKeyDelegation(GenesisKeyDelegation),
    MoveInstantaneousRewardsCert(MoveInstantaneousRewardsCert),
}

impl AllegraCertificate {
    pub fn new_stake_registration(stake_credential: StakeCredential) -> Self {
        Self::StakeRegistration(StakeRegistration::new(stake_credential))
    }

    pub fn new_stake_deregistration(stake_credential: StakeCredential) -> Self {
        Self::StakeDeregistration(StakeDeregistration::new(stake_credential))
    }

    pub fn new_stake_delegation(
        stake_credential: StakeCredential,
        ed25519_key_hash: Ed25519KeyHash,
    ) -> Self {
        Self::StakeDelegation(StakeDelegation::new(stake_credential, ed25519_key_hash))
    }

    pub fn new_shelley_pool_registration(pool_params: ShelleyPoolParams) -> Self {
        Self::ShelleyPoolRegistration(ShelleyPoolRegistration::new(pool_params))
    }

    pub fn new_pool_retirement(ed25519_key_hash: Ed25519KeyHash, epoch: Epoch) -> Self {
        Self::PoolRetirement(PoolRetirement::new(ed25519_key_hash, epoch))
    }

    pub fn new_genesis_key_delegation(
        genesis_hash: GenesisHash,
        genesis_delegate_hash: GenesisDelegateHash,
        vrf_key_hash: VRFKeyHash,
    ) -> Self {
        Self::GenesisKeyDelegation(GenesisKeyDelegation::new(
            genesis_hash,
            genesis_delegate_hash,
            vrf_key_hash,
        ))
    }

    pub fn new_move_instantaneous_rewards_cert(
        move_instantaneous_reward: MoveInstantaneousReward,
    ) -> Self {
        Self::MoveInstantaneousRewardsCert(MoveInstantaneousRewardsCert::new(
            move_instantaneous_reward,
        ))
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct AllegraTransaction {
    pub body: AllegraTransactionBody,
    pub witness_set: AllegraTransactionWitnessSet,
    pub auxiliary_data: Option<AllegraAuxiliaryData>,
    #[serde(skip)]
    pub encodings: Option<AllegraTransactionEncoding>,
}

impl AllegraTransaction {
    pub fn new(
        body: AllegraTransactionBody,
        witness_set: AllegraTransactionWitnessSet,
        auxiliary_data: Option<AllegraAuxiliaryData>,
    ) -> Self {
        Self {
            body,
            witness_set,
            auxiliary_data,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct AllegraTransactionBody {
    pub inputs: Vec<TransactionInput>,
    pub outputs: Vec<ShelleyTransactionOutput>,
    pub fee: Coin,
    pub ttl: Option<u64>,
    pub certs: Option<Vec<AllegraCertificate>>,
    pub withdrawals: Option<Withdrawals>,
    pub update: Option<ShelleyUpdate>,
    pub auxiliary_data_hash: Option<AuxiliaryDataHash>,
    pub validity_interval_start: Option<u64>,
    #[serde(skip)]
    pub encodings: Option<AllegraTransactionBodyEncoding>,
}

impl AllegraTransactionBody {
    pub fn new(
        inputs: Vec<TransactionInput>,
        outputs: Vec<ShelleyTransactionOutput>,
        fee: Coin,
    ) -> Self {
        Self {
            inputs,
            outputs,
            fee,
            ttl: None,
            certs: None,
            withdrawals: None,
            update: None,
            auxiliary_data_hash: None,
            validity_interval_start: None,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct AllegraTransactionWitnessSet {
    pub vkeywitnesses: Option<Vec<Vkeywitness>>,
    pub native_scripts: Option<Vec<NativeScript>>,
    pub bootstrap_witnesses: Option<Vec<BootstrapWitness>>,
    #[serde(skip)]
    pub encodings: Option<AllegraTransactionWitnessSetEncoding>,
}

impl AllegraTransactionWitnessSet {
    pub fn new() -> Self {
        Self {
            vkeywitnesses: None,
            native_scripts: None,
            bootstrap_witnesses: None,
            encodings: None,
        }
    }
}

impl Default for AllegraTransactionWitnessSet {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub enum MIRAction {
    ToStakeCredentials {
        to_stake_credentials: OrderedHashMap<StakeCredential, DeltaCoin>,
        #[serde(skip)]
        to_stake_credentials_encoding: LenEncoding,
    },
    ToOtherPot {
        to_other_pot: Coin,
        #[serde(skip)]
        to_other_pot_encoding: Option<cbor_event::Sz>,
    },
}

impl MIRAction {
    pub fn new_to_stake_credentials(
        to_stake_credentials: OrderedHashMap<StakeCredential, DeltaCoin>,
    ) -> Self {
        Self::ToStakeCredentials {
            to_stake_credentials,
            to_stake_credentials_encoding: LenEncoding::default(),
        }
    }

    pub fn new_to_other_pot(to_other_pot: Coin) -> Self {
        Self::ToOtherPot {
            to_other_pot,
            to_other_pot_encoding: None,
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
pub enum MIRPot {
    Reserve,
    Treasury,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct MoveInstantaneousReward {
    pub pot: MIRPot,
    pub action: MIRAction,
    #[serde(skip)]
    pub encodings: Option<MoveInstantaneousRewardEncoding>,
}

impl MoveInstantaneousReward {
    pub fn new(pot: MIRPot, action: MIRAction) -> Self {
        Self {
            pot,
            action,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct MoveInstantaneousRewardsCert {
    pub move_instantaneous_reward: MoveInstantaneousReward,
    #[serde(skip)]
    pub encodings: Option<MoveInstantaneousRewardsCertEncoding>,
}

impl MoveInstantaneousRewardsCert {
    pub fn new(move_instantaneous_reward: MoveInstantaneousReward) -> Self {
        Self {
            move_instantaneous_reward,
            encodings: None,
        }
    }
}
