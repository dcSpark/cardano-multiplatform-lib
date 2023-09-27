#![allow(
    clippy::len_without_is_empty,
    clippy::too_many_arguments,
    clippy::new_without_default
)]
// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

use cml_core_wasm::{impl_wasm_cbor_json_api, impl_wasm_conversions, impl_wasm_json_api};

use cml_chain_wasm::address::Address;
pub mod utils;

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct Delegation(cml_cip36::Delegation);

impl_wasm_cbor_json_api!(Delegation);

impl_wasm_conversions!(cml_cip36::Delegation, Delegation);

#[wasm_bindgen]
impl Delegation {
    pub fn voting_pub_key(&self) -> VotingPubKey {
        self.0.voting_pub_key.clone().into()
    }

    pub fn weight(&self) -> Weight {
        self.0.weight
    }

    pub fn new(voting_pub_key: &VotingPubKey, weight: Weight) -> Self {
        Self(cml_cip36::Delegation::new(
            voting_pub_key.clone().into(),
            weight,
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct DelegationDistribution(cml_cip36::DelegationDistribution);

impl_wasm_cbor_json_api!(DelegationDistribution);

impl_wasm_conversions!(cml_cip36::DelegationDistribution, DelegationDistribution);

#[wasm_bindgen]
impl DelegationDistribution {
    pub fn new_weighted(delegations: &DelegationList) -> Self {
        Self(cml_cip36::DelegationDistribution::new_weighted(
            delegations.clone().into(),
        ))
    }

    pub fn new_legacy(legacy: &LegacyKeyRegistration) -> Self {
        Self(cml_cip36::DelegationDistribution::new_legacy(
            legacy.clone().into(),
        ))
    }

    pub fn kind(&self) -> DelegationDistributionKind {
        match &self.0 {
            cml_cip36::DelegationDistribution::Weighted { .. } => {
                DelegationDistributionKind::Weighted
            }
            cml_cip36::DelegationDistribution::Legacy { .. } => DelegationDistributionKind::Legacy,
        }
    }

    pub fn as_weighted(&self) -> Option<DelegationList> {
        match &self.0 {
            cml_cip36::DelegationDistribution::Weighted { delegations, .. } => {
                Some(delegations.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_legacy(&self) -> Option<LegacyKeyRegistration> {
        match &self.0 {
            cml_cip36::DelegationDistribution::Legacy { legacy, .. } => Some(legacy.clone().into()),
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
pub struct DelegationList(Vec<cml_cip36::Delegation>);

#[wasm_bindgen]
impl DelegationList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Delegation {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &Delegation) {
        self.0.push(elem.clone().into());
    }
}

impl From<Vec<cml_cip36::Delegation>> for DelegationList {
    fn from(native: Vec<cml_cip36::Delegation>) -> Self {
        Self(native)
    }
}

impl From<DelegationList> for Vec<cml_cip36::Delegation> {
    fn from(wasm: DelegationList) -> Self {
        wasm.0
    }
}

impl AsRef<Vec<cml_cip36::Delegation>> for DelegationList {
    fn as_ref(&self) -> &Vec<cml_cip36::Delegation> {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct DeregistrationCbor(cml_cip36::DeregistrationCbor);

// DeregistrationCbor does not implement Serialize as it may be a subset of metadata
impl_wasm_json_api!(DeregistrationCbor);

impl_wasm_conversions!(cml_cip36::DeregistrationCbor, DeregistrationCbor);

#[wasm_bindgen]
impl DeregistrationCbor {
    pub fn key_deregistration(&self) -> KeyDeregistration {
        self.0.key_deregistration.clone().into()
    }

    pub fn deregistration_witness(&self) -> DeregistrationWitness {
        self.0.deregistration_witness.clone().into()
    }

    pub fn new(
        key_deregistration: &KeyDeregistration,
        deregistration_witness: &DeregistrationWitness,
    ) -> Self {
        Self(cml_cip36::DeregistrationCbor::new(
            key_deregistration.clone().into(),
            deregistration_witness.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct DeregistrationWitness(cml_cip36::DeregistrationWitness);

impl_wasm_cbor_json_api!(DeregistrationWitness);

impl_wasm_conversions!(cml_cip36::DeregistrationWitness, DeregistrationWitness);

#[wasm_bindgen]
impl DeregistrationWitness {
    pub fn stake_witness(&self) -> StakeWitness {
        self.0.stake_witness.clone().into()
    }

    pub fn new(stake_witness: &StakeWitness) -> Self {
        Self(cml_cip36::DeregistrationWitness::new(
            stake_witness.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct KeyDeregistration(cml_cip36::KeyDeregistration);

impl_wasm_cbor_json_api!(KeyDeregistration);

impl_wasm_conversions!(cml_cip36::KeyDeregistration, KeyDeregistration);

#[wasm_bindgen]
impl KeyDeregistration {
    pub fn stake_credential(&self) -> StakeCredential {
        self.0.stake_credential.clone().into()
    }

    pub fn nonce(&self) -> Nonce {
        self.0.nonce
    }

    pub fn set_voting_purpose(&mut self, voting_purpose: VotingPurpose) {
        self.0.voting_purpose = voting_purpose
    }

    pub fn voting_purpose(&self) -> VotingPurpose {
        self.0.voting_purpose
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct KeyRegistration(cml_cip36::KeyRegistration);

impl_wasm_cbor_json_api!(KeyRegistration);

impl_wasm_conversions!(cml_cip36::KeyRegistration, KeyRegistration);

#[wasm_bindgen]
impl KeyRegistration {
    pub fn delegation(&self) -> DelegationDistribution {
        self.0.delegation.clone().into()
    }

    pub fn stake_credential(&self) -> StakeCredential {
        self.0.stake_credential.clone().into()
    }

    pub fn payment_address(&self) -> Address {
        self.0.payment_address.clone().into()
    }

    pub fn nonce(&self) -> Nonce {
        self.0.nonce
    }

    pub fn set_voting_purpose(&mut self, voting_purpose: VotingPurpose) {
        self.0.voting_purpose = voting_purpose
    }

    pub fn voting_purpose(&self) -> VotingPurpose {
        self.0.voting_purpose
    }
}

pub type LegacyKeyRegistration = cml_crypto_wasm::PublicKey;

pub type Nonce = u64;

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct RegistrationCbor(cml_cip36::RegistrationCbor);

// not implemented since RegistrationCbor doesn't implement Serialize as it's a subset of metadata
impl_wasm_json_api!(RegistrationCbor);

impl_wasm_conversions!(cml_cip36::RegistrationCbor, RegistrationCbor);

#[wasm_bindgen]
impl RegistrationCbor {
    pub fn key_registration(&self) -> KeyRegistration {
        self.0.key_registration.clone().into()
    }

    pub fn registration_witness(&self) -> RegistrationWitness {
        self.0.registration_witness.clone().into()
    }

    pub fn new(
        key_registration: &KeyRegistration,
        registration_witness: &RegistrationWitness,
    ) -> Self {
        Self(cml_cip36::RegistrationCbor::new(
            key_registration.clone().into(),
            registration_witness.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct RegistrationWitness(cml_cip36::RegistrationWitness);

impl_wasm_cbor_json_api!(RegistrationWitness);

impl_wasm_conversions!(cml_cip36::RegistrationWitness, RegistrationWitness);

#[wasm_bindgen]
impl RegistrationWitness {
    pub fn stake_witness(&self) -> StakeWitness {
        self.0.stake_witness.clone().into()
    }

    pub fn new(stake_witness: &StakeWitness) -> Self {
        Self(cml_cip36::RegistrationWitness::new(
            stake_witness.clone().into(),
        ))
    }
}

pub type StakeCredential = cml_crypto_wasm::PublicKey;

pub type StakeWitness = cml_crypto_wasm::Ed25519Signature;

pub type StakingPubKey = cml_crypto_wasm::PublicKey;

pub type VotingPubKey = cml_crypto_wasm::PublicKey;

pub type VotingPurpose = u64;

pub type Weight = u32;
