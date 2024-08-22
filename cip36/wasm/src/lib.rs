#![allow(
    clippy::len_without_is_empty,
    clippy::too_many_arguments,
    clippy::new_without_default
)]
// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use wasm_bindgen::prelude::wasm_bindgen;

use cml_core_wasm::{impl_wasm_cbor_json_api, impl_wasm_conversions, impl_wasm_json_api};

use cml_chain_wasm::address::Address;
pub mod utils;

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct CIP36Delegation(cml_cip36::CIP36Delegation);

impl_wasm_cbor_json_api!(CIP36Delegation);

impl_wasm_conversions!(cml_cip36::CIP36Delegation, CIP36Delegation);

#[wasm_bindgen]
impl CIP36Delegation {
    pub fn voting_pub_key(&self) -> CIP36VotingPubKey {
        self.0.voting_pub_key.clone().into()
    }

    pub fn weight(&self) -> CIP36Weight {
        self.0.weight
    }

    pub fn new(voting_pub_key: &CIP36VotingPubKey, weight: CIP36Weight) -> Self {
        Self(cml_cip36::CIP36Delegation::new(
            voting_pub_key.clone().into(),
            weight,
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct CIP36DelegationDistribution(cml_cip36::CIP36DelegationDistribution);

impl_wasm_cbor_json_api!(CIP36DelegationDistribution);

impl_wasm_conversions!(
    cml_cip36::CIP36DelegationDistribution,
    CIP36DelegationDistribution
);

#[wasm_bindgen]
impl CIP36DelegationDistribution {
    pub fn new_weighted(delegations: &CIP36DelegationList) -> Self {
        Self(cml_cip36::CIP36DelegationDistribution::new_weighted(
            delegations.clone().into(),
        ))
    }

    pub fn new_legacy(legacy: &LegacyKeyRegistration) -> Self {
        Self(cml_cip36::CIP36DelegationDistribution::new_legacy(
            legacy.clone().into(),
        ))
    }

    pub fn kind(&self) -> DelegationDistributionKind {
        match &self.0 {
            cml_cip36::CIP36DelegationDistribution::Weighted { .. } => {
                DelegationDistributionKind::Weighted
            }
            cml_cip36::CIP36DelegationDistribution::Legacy { .. } => {
                DelegationDistributionKind::Legacy
            }
        }
    }

    pub fn as_weighted(&self) -> Option<CIP36DelegationList> {
        match &self.0 {
            cml_cip36::CIP36DelegationDistribution::Weighted { delegations, .. } => {
                Some(delegations.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_legacy(&self) -> Option<LegacyKeyRegistration> {
        match &self.0 {
            cml_cip36::CIP36DelegationDistribution::Legacy { legacy, .. } => {
                Some(legacy.clone().into())
            }
            _ => None,
        }
    }
}

#[wasm_bindgen]
pub enum DelegationDistributionKind {
    Weighted,
    Legacy,
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct CIP36DelegationList(Vec<cml_cip36::CIP36Delegation>);

#[wasm_bindgen]
impl CIP36DelegationList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> CIP36Delegation {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &CIP36Delegation) {
        self.0.push(elem.clone().into());
    }
}

impl From<Vec<cml_cip36::CIP36Delegation>> for CIP36DelegationList {
    fn from(native: Vec<cml_cip36::CIP36Delegation>) -> Self {
        Self(native)
    }
}

impl From<CIP36DelegationList> for Vec<cml_cip36::CIP36Delegation> {
    fn from(wasm: CIP36DelegationList) -> Self {
        wasm.0
    }
}

impl AsRef<Vec<cml_cip36::CIP36Delegation>> for CIP36DelegationList {
    fn as_ref(&self) -> &Vec<cml_cip36::CIP36Delegation> {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct CIP36DeregistrationCbor(cml_cip36::CIP36DeregistrationCbor);

// CIP36DeregistrationCbor does not implement Serialize as it may be a subset of metadata
impl_wasm_json_api!(CIP36DeregistrationCbor);

impl_wasm_conversions!(cml_cip36::CIP36DeregistrationCbor, CIP36DeregistrationCbor);

#[wasm_bindgen]
impl CIP36DeregistrationCbor {
    pub fn key_deregistration(&self) -> CIP36KeyDeregistration {
        self.0.key_deregistration.clone().into()
    }

    pub fn deregistration_witness(&self) -> CIP36DeregistrationWitness {
        self.0.deregistration_witness.clone().into()
    }

    pub fn new(
        key_deregistration: &CIP36KeyDeregistration,
        deregistration_witness: &CIP36DeregistrationWitness,
    ) -> Self {
        Self(cml_cip36::CIP36DeregistrationCbor::new(
            key_deregistration.clone().into(),
            deregistration_witness.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct CIP36DeregistrationWitness(cml_cip36::CIP36DeregistrationWitness);

impl_wasm_cbor_json_api!(CIP36DeregistrationWitness);

impl_wasm_conversions!(
    cml_cip36::CIP36DeregistrationWitness,
    CIP36DeregistrationWitness
);

#[wasm_bindgen]
impl CIP36DeregistrationWitness {
    pub fn stake_witness(&self) -> CIP36StakeWitness {
        self.0.stake_witness.clone().into()
    }

    pub fn new(stake_witness: &CIP36StakeWitness) -> Self {
        Self(cml_cip36::CIP36DeregistrationWitness::new(
            stake_witness.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct CIP36KeyDeregistration(cml_cip36::CIP36KeyDeregistration);

impl_wasm_cbor_json_api!(CIP36KeyDeregistration);

impl_wasm_conversions!(cml_cip36::CIP36KeyDeregistration, CIP36KeyDeregistration);

#[wasm_bindgen]
impl CIP36KeyDeregistration {
    pub fn stake_credential(&self) -> CIP36StakeCredential {
        self.0.stake_credential.clone().into()
    }

    pub fn nonce(&self) -> CIP36Nonce {
        self.0.nonce
    }

    pub fn set_voting_purpose(&mut self, voting_purpose: CIP36VotingPurpose) {
        self.0.voting_purpose = voting_purpose
    }

    pub fn voting_purpose(&self) -> CIP36VotingPurpose {
        self.0.voting_purpose
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct CIP36KeyRegistration(cml_cip36::CIP36KeyRegistration);

impl_wasm_cbor_json_api!(CIP36KeyRegistration);

impl_wasm_conversions!(cml_cip36::CIP36KeyRegistration, CIP36KeyRegistration);

#[wasm_bindgen]
impl CIP36KeyRegistration {
    pub fn delegation(&self) -> CIP36DelegationDistribution {
        self.0.delegation.clone().into()
    }

    pub fn stake_credential(&self) -> CIP36StakeCredential {
        self.0.stake_credential.clone().into()
    }

    pub fn payment_address(&self) -> Address {
        self.0.payment_address.clone().into()
    }

    pub fn nonce(&self) -> CIP36Nonce {
        self.0.nonce
    }

    pub fn set_voting_purpose(&mut self, voting_purpose: CIP36VotingPurpose) {
        self.0.voting_purpose = voting_purpose
    }

    pub fn voting_purpose(&self) -> CIP36VotingPurpose {
        self.0.voting_purpose
    }
}

pub type LegacyKeyRegistration = cml_crypto_wasm::PublicKey;

pub type CIP36Nonce = u64;

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct CIP36RegistrationCbor(cml_cip36::CIP36RegistrationCbor);

// not implemented since CIP36RegistrationCbor doesn't implement Serialize as it's a subset of metadata
impl_wasm_json_api!(CIP36RegistrationCbor);

impl_wasm_conversions!(cml_cip36::CIP36RegistrationCbor, CIP36RegistrationCbor);

#[wasm_bindgen]
impl CIP36RegistrationCbor {
    pub fn key_registration(&self) -> CIP36KeyRegistration {
        self.0.key_registration.clone().into()
    }

    pub fn registration_witness(&self) -> CIP36RegistrationWitness {
        self.0.registration_witness.clone().into()
    }

    pub fn new(
        key_registration: &CIP36KeyRegistration,
        registration_witness: &CIP36RegistrationWitness,
    ) -> Self {
        Self(cml_cip36::CIP36RegistrationCbor::new(
            key_registration.clone().into(),
            registration_witness.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct CIP36RegistrationWitness(cml_cip36::CIP36RegistrationWitness);

impl_wasm_cbor_json_api!(CIP36RegistrationWitness);

impl_wasm_conversions!(
    cml_cip36::CIP36RegistrationWitness,
    CIP36RegistrationWitness
);

#[wasm_bindgen]
impl CIP36RegistrationWitness {
    pub fn stake_witness(&self) -> CIP36StakeWitness {
        self.0.stake_witness.clone().into()
    }

    pub fn new(stake_witness: &CIP36StakeWitness) -> Self {
        Self(cml_cip36::CIP36RegistrationWitness::new(
            stake_witness.clone().into(),
        ))
    }
}

pub type CIP36StakeCredential = cml_crypto_wasm::PublicKey;

pub type CIP36StakeWitness = cml_crypto_wasm::Ed25519Signature;

pub type CIP36StakingPubKey = cml_crypto_wasm::PublicKey;

pub type CIP36VotingPubKey = cml_crypto_wasm::PublicKey;

pub type CIP36VotingPurpose = u64;

pub type CIP36Weight = u32;
