#![allow(clippy::too_many_arguments)]

// This library was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use cardano_multiplatform_lib_core as cml_core;
use cardano_multiplatform_lib_crypto as cml_crypto;

pub use cml_core::{
    ordered_hash_map::OrderedHashMap,
    error::{DeserializeError, DeserializeFailure},
    serialization::{Serialize, Deserialize, StringEncoding, LenEncoding},
};

use cbor_event::{self, de::Deserializer, se::Serializer};

use std::io::{BufRead, Write};

use cbor_event::Type as CBORType;

use cbor_event::Special as CBORSpecial;

use serialization::*;

use std::collections::BTreeMap;

use std::convert::{From, TryFrom};

pub mod serialization;

use cbor_event::Sz;

pub mod cbor_encodings;

use cbor_encodings::*;

extern crate derivative;

use derivative::Derivative;

pub type VotingPubKey = cml_crypto::chain::ChainCrypto<cml_crypto::PublicKey>;

pub type StakingPubKey = cml_crypto::chain::ChainCrypto<cml_crypto::PublicKey>;

type Ed25519Signature = cml_crypto::chain::ChainCrypto<cml_crypto::Ed25519Signature>;

pub type LegacyKeyRegistration = VotingPubKey;

pub type Nonce = u64;

pub type RewardAddress = Vec<u8>;

pub type StakeCredential = StakingPubKey;

pub type StakeWitness = Ed25519Signature;

pub type VotingPurpose = u64;

pub type Weight = u32;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub enum ArrDelegationOrLegacyKeyRegistration {
    ArrDelegation {
        arr_delegation: Vec<Delegation>,
        #[serde(skip)]
        arr_delegation_encoding: LenEncoding,
    }
    ,
    LegacyKeyRegistration(LegacyKeyRegistration),
}

impl ArrDelegationOrLegacyKeyRegistration {
    pub fn new_arr_delegation(arr_delegation: Vec<Delegation>) -> Self {
        Self::ArrDelegation {
            arr_delegation,
            arr_delegation_encoding: LenEncoding::default(),
        }
    }

    pub fn new_legacy_key_registration(legacy_key_registration: LegacyKeyRegistration) -> Self {
        Self::LegacyKeyRegistration(legacy_key_registration)
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct Delegation {
    pub voting_pub_key: VotingPubKey,
    pub weight: Weight,
    #[serde(skip)]
    pub encodings: Option<DelegationEncoding>,
}

impl Delegation {
    pub fn new(voting_pub_key: VotingPubKey, weight: Weight) -> Self {
        Self {
            voting_pub_key,
            weight,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct DeregistrationCbor {
    pub key_deregistration: KeyDeregistration,
    pub deregistration_witness: DeregistrationWitness,
    #[serde(skip)]
    pub encodings: Option<DeregistrationCborEncoding>,
}

impl DeregistrationCbor {
    pub fn new(key_deregistration: KeyDeregistration, deregistration_witness: DeregistrationWitness) -> Self {
        Self {
            key_deregistration,
            deregistration_witness,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct DeregistrationWitness {
    pub stake_witness: StakeWitness,
    #[serde(skip)]
    pub encodings: Option<DeregistrationWitnessEncoding>,
}

impl DeregistrationWitness {
    pub fn new(stake_witness: StakeWitness) -> Self {
        Self {
            stake_witness,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct KeyDeregistration {
    pub stake_credential: StakeCredential,
    pub nonce: Nonce,
    pub voting_purpose: VotingPurpose,
    #[serde(skip)]
    pub encodings: Option<KeyDeregistrationEncoding>,
}

impl KeyDeregistration {
    pub fn new(stake_credential: StakeCredential, nonce: Nonce) -> Self {
        Self {
            stake_credential,
            nonce,
            voting_purpose: 0,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct KeyRegistration {
    pub delegation: ArrDelegationOrLegacyKeyRegistration,
    pub stake_credential: StakeCredential,
    pub reward_address: RewardAddress,
    pub nonce: Nonce,
    pub voting_purpose: VotingPurpose,
    #[serde(skip)]
    pub encodings: Option<KeyRegistrationEncoding>,
}

impl KeyRegistration {
    pub fn new(delegation: ArrDelegationOrLegacyKeyRegistration, stake_credential: StakeCredential, reward_address: RewardAddress, nonce: Nonce) -> Self {
        Self {
            delegation,
            stake_credential,
            reward_address,
            nonce,
            voting_purpose: 0,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct RegistrationCbor {
    pub key_registration: KeyRegistration,
    pub registration_witness: RegistrationWitness,
    #[serde(skip)]
    pub encodings: Option<RegistrationCborEncoding>,
}

impl RegistrationCbor {
    pub fn new(key_registration: KeyRegistration, registration_witness: RegistrationWitness) -> Self {
        Self {
            key_registration,
            registration_witness,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct RegistrationWitness {
    pub stake_witness: StakeWitness,
    #[serde(skip)]
    pub encodings: Option<RegistrationWitnessEncoding>,
}

impl RegistrationWitness {
    pub fn new(stake_witness: StakeWitness) -> Self {
        Self {
            stake_witness,
            encodings: None,
        }
    }
}