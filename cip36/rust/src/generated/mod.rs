#![allow(clippy::too_many_arguments)]

extern crate derivative;
// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

pub mod cbor_encodings;
pub mod serialization;

pub use cml_core::{
    error::{DeserializeError, DeserializeFailure},
    ordered_hash_map::OrderedHashMap,
    serialization::{Deserialize, LenEncoding, Serialize, StringEncoding},
};

pub use cml_chain::address::Address;

use cbor_encodings::*;

use std::convert::From;

/// To avoid linking voting keys directly with Cardano spending keys,
/// the voting key derivation path must start with a specific segment:
/// m / 1694' / 1815' / account' / chain / address_index
pub type CIP36VotingPubKey = cml_crypto::PublicKey;

pub type CIP36StakingPubKey = cml_crypto::PublicKey;

pub type CIP36LegacyKeyRegistration = CIP36VotingPubKey;

/// The nonce is an unsigned integer that should be monotonically rising across all transactions with the same staking key.
/// The advised way to construct a nonce is to use the current slot number.
/// This is a simple way to keep the nonce increasing without having to access the previous transaction data.
pub type CIP36Nonce = u64;

pub type CIP36StakeCredential = CIP36StakingPubKey;

pub type CIP36StakeWitness = cml_crypto::Ed25519Signature;

pub type CIP36VotingPurpose = u64;

pub type CIP36Weight = u32;

/// Weighted delegation input.
/// This is the proportion of weight to assign to this public key relative to the weights
/// of all other Delegations where this is used.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct CIP36Delegation {
    pub voting_pub_key: CIP36VotingPubKey,
    pub weight: CIP36Weight,
    #[serde(skip)]
    pub encodings: Option<CIP36DelegationEncoding>,
}

impl CIP36Delegation {
    pub fn new(voting_pub_key: CIP36VotingPubKey, weight: CIP36Weight) -> Self {
        Self {
            voting_pub_key,
            weight,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub enum CIP36DelegationDistribution {
    Weighted {
        delegations: Vec<CIP36Delegation>,
        #[serde(skip)]
        delegations_encoding: LenEncoding,
    },
    Legacy {
        legacy: CIP36LegacyKeyRegistration,
        #[serde(skip)]
        legacy_encoding: StringEncoding,
    },
}

impl CIP36DelegationDistribution {
    /// Create a new delegations delegation. Weights are relative to all others and will be rounded down.
    /// Leftover ADA will be delegated to the last item in the array.
    pub fn new_weighted(delegations: Vec<CIP36Delegation>) -> Self {
        Self::Weighted {
            delegations,
            delegations_encoding: LenEncoding::default(),
        }
    }

    /// Delegate to a single key i.e. CIP-15.
    pub fn new_legacy(legacy: CIP36LegacyKeyRegistration) -> Self {
        Self::Legacy {
            legacy,
            legacy_encoding: StringEncoding::default(),
        }
    }
}

/// This is the entire metadata schema for CIP-36 deregistration.
/// It can be parsed by passing in the CBOR bytes of the entire transaction metadatum
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct CIP36DeregistrationCbor {
    pub key_deregistration: CIP36KeyDeregistration,
    pub deregistration_witness: CIP36DeregistrationWitness,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct CIP36DeregistrationWitness {
    pub stake_witness: CIP36StakeWitness,
    #[serde(skip)]
    pub encodings: Option<CIP36DeregistrationWitnessEncoding>,
}

impl CIP36DeregistrationWitness {
    pub fn new(stake_witness: CIP36StakeWitness) -> Self {
        Self {
            stake_witness,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct CIP36KeyDeregistration {
    pub stake_credential: CIP36StakeCredential,
    pub nonce: CIP36Nonce,
    pub voting_purpose: CIP36VotingPurpose,
    #[serde(skip)]
    pub encodings: Option<CIP36KeyDeregistrationEncoding>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct CIP36KeyRegistration {
    pub delegation: CIP36DelegationDistribution,
    pub stake_credential: CIP36StakeCredential,
    pub payment_address: Address,
    pub nonce: CIP36Nonce,
    pub voting_purpose: CIP36VotingPurpose,
    #[serde(skip)]
    pub encodings: Option<CIP36KeyRegistrationEncoding>,
}

/// This is the entire metadata schema for CIP-36 registration.
/// It can be parsed by passing in the CBOR bytes of the entire transaction metadatum
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct CIP36RegistrationCbor {
    pub key_registration: CIP36KeyRegistration,
    pub registration_witness: CIP36RegistrationWitness,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct CIP36RegistrationWitness {
    pub stake_witness: CIP36StakeWitness,
    #[serde(skip)]
    pub encodings: Option<CIP36RegistrationWitnessEncoding>,
}

impl CIP36RegistrationWitness {
    pub fn new(stake_witness: CIP36StakeWitness) -> Self {
        Self {
            stake_witness,
            encodings: None,
        }
    }
}
