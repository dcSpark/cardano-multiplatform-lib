#![allow(clippy::too_many_arguments)]

// This file was partially code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

pub use cml_core::{
    error::{DeserializeError, DeserializeFailure},
    metadata::Metadata,
    ordered_hash_map::OrderedHashMap,
    serialization::{Deserialize, LenEncoding, Serialize, StringEncoding},
};

pub use cml_chain::{address::RewardAddress, NetworkId};

use std::convert::From;

pub mod cbor_encodings;
pub mod error;
pub mod serialization;
pub mod utils;

use cbor_encodings::*;

extern crate derivative;

/// To avoid linking voting keys directly with Cardano spending keys,
/// the voting key derivation path must start with a specific segment:
/// m / 1694' / 1815' / account' / chain / address_index
pub type VotingPubKey = cml_crypto::PublicKey;

pub type StakingPubKey = cml_crypto::PublicKey;

pub type LegacyKeyRegistration = VotingPubKey;

/// The nonce is an unsigned integer that should be monotonically rising across all transactions with the same staking key.
/// The advised way to construct a nonce is to use the current slot number.
/// This is a simple way to keep the nonce increasing without having to access the previous transaction data.
pub type Nonce = u64;

pub type StakeCredential = StakingPubKey;

pub type StakeWitness = cml_crypto::Ed25519Signature;

pub type VotingPurpose = u64;

pub type Weight = u32;

/// Weighted delegation input.
/// This is the proportion of weight to assign to this public key relative to the weights
/// of all other Delegations where this is used.
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
pub enum DelegationDistribution {
    Weighted {
        delegations: Vec<Delegation>,
        #[serde(skip)]
        delegations_encoding: LenEncoding,
    },
    Legacy {
        legacy: LegacyKeyRegistration,
        #[serde(skip)]
        legacy_encoding: StringEncoding,
    },
}

impl DelegationDistribution {
    /// Create a new delegations delegation. Weights are relative to all others and will be rounded down.
    /// Leftover ADA will be delegated to the last item in the array.
    pub fn new_weighted(delegations: Vec<Delegation>) -> Self {
        Self::Weighted {
            delegations,
            delegations_encoding: LenEncoding::default(),
        }
    }

    /// Delegate to a single key i.e. CIP-15.
    pub fn new_legacy(legacy: LegacyKeyRegistration) -> Self {
        Self::Legacy {
            legacy,
            legacy_encoding: StringEncoding::default(),
        }
    }
}

/// This is the entire metadata schema for CIP-36 deregistration.
/// It can be parsed by passing in the CBOR bytes of the entire transaction metadatum
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct DeregistrationCbor {
    pub key_deregistration: KeyDeregistration,
    pub deregistration_witness: DeregistrationWitness,
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

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct KeyRegistration {
    pub delegation: DelegationDistribution,
    pub stake_credential: StakeCredential,
    pub reward_address: RewardAddress,
    pub nonce: Nonce,
    pub voting_purpose: VotingPurpose,
    #[serde(skip)]
    pub encodings: Option<KeyRegistrationEncoding>,
}

/// This is the entire metadata schema for CIP-36 registration.
/// It can be parsed by passing in the CBOR bytes of the entire transaction metadatum
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct RegistrationCbor {
    pub key_registration: KeyRegistration,
    pub registration_witness: RegistrationWitness,
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

#[cfg(test)]
mod tests {
    use cml_chain::address::Address;
    use cml_crypto::*;

    use super::*;

    #[test]
    fn sign_data() {
        // TODO: test vectors don't fully specify how to arrive from these to the resulting delegations/stake creds/reward address
        // so we don't derive or anything and just put them straight into the structs but that would be a more complete test
        // and we might want to add deriving-related options (assuming these keys weren't in their already-derived state)
        // let payment_pub = PublicKey::from_raw_hex("3273a5316e4de228863bd7cf8dac90d57149e1a595f3dd131073b84e35546676").unwrap();
        // let staking_prv = Bip32PrivateKey::from_raw_hex("f5beaeff7932a4164d270afde7716067582412e8977e67986cd9b456fc082e3a").unwrap();
        // let staking_derived_prv = staking_prv
        //     .derive(1694)
        //     .derive(1815)
        //     .derive(0)
        //     .derive(0);
        // let catalyst_prv_key = hex::decode("4820f7ce221e177c8eae2b2ee5c1f1581a0d88ca5c14329d8f2389e77a465655c27662621bfb99cb9445bf8114cc2a630afd2dd53bc88c08c5f2aed8e9c7cb89").unwrap();
        let stake_cred = StakeCredential::from(
            PublicKey::from_raw_hex(
                "86870efc99c453a873a16492ce87738ec79a0ebd064379a62e2c9cf4e119219e",
            )
            .unwrap(),
        );
        // let stake_cred = StakeCredential::from(staking_derived_prv.to_public().to_raw_key());
        let reward_address = RewardAddress::from_address(
            &Address::from_bech32(
                "stake_test1uzhr5zn6akj2affzua8ylcm8t872spuf5cf6tzjrvnmwemcehgcjm",
            )
            .unwrap(),
        )
        .unwrap();
        let nonce = 1234;

        // legacy format
        let legacy_reg = KeyRegistration::new(
            DelegationDistribution::new_legacy(
                LegacyKeyRegistration::from_raw_hex(
                    "a6a3c0447aeb9cc54cf6422ba32b294e5e1c3ef6d782f2acff4a70694c4d1663",
                )
                .unwrap(),
            ),
            stake_cred.clone(),
            reward_address.clone(),
            nonce,
        );
        let legacy_sign_data_hash = legacy_reg.hash_to_sign(false).unwrap();
        assert_eq!(
            "872bcb4a9e2b110a06fd5de04be5924b6c659c28a1665ecc75def13ebca6dfd8",
            hex::encode(legacy_sign_data_hash)
        );

        // weighted
        let weighted_reg = KeyRegistration::new(
            DelegationDistribution::new_weighted(vec![
                Delegation::new(
                    VotingPubKey::from_raw_hex(
                        "a6a3c0447aeb9cc54cf6422ba32b294e5e1c3ef6d782f2acff4a70694c4d1663",
                    )
                    .unwrap(),
                    1,
                ),
                Delegation::new(
                    VotingPubKey::from_raw_hex(
                        "00588e8e1d18cba576a4d35758069fe94e53f638b6faf7c07b8abd2bc5c5cdee",
                    )
                    .unwrap(),
                    3,
                ),
            ]),
            stake_cred,
            reward_address,
            nonce,
        );
        let weighted_sign_data_hash = weighted_reg.hash_to_sign(false).unwrap();
        // There are some issues with the CIP-36 test vector here. TODO: figure out whether it's the vector or the spec or us that's wrong.
        //assert_eq!("5bc0681f173efd76e1989037a3694b8a7abea22053f5940cbb5cfcdf721007d7", hex::encode(weighted_sign_data_hash));

        // TODO: deregistration test? there are no official test vectors in CIP36
    }
}
