// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

#![allow(
    clippy::len_without_is_empty,
    clippy::too_many_arguments,
    clippy::new_without_default
)]
pub mod collections;

use cml_chain_wasm::address::Address as PaymentAddress;
use cml_core::non_empty::NonEmptyVec;
use cml_core_wasm::{
    impl_wasm_cbor_json_api, impl_wasm_conversions, impl_wasm_json_api, impl_wasm_list_needs_into,
};
use cml_crypto_wasm::{Ed25519Signature, PublicKey};
use wasm_bindgen::prelude::{JsError, wasm_bindgen};

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct CIP36Delegation(pub(crate) cml_cip36::CIP36Delegation);

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
pub struct CIP36DelegationDistribution(pub(crate) cml_cip36::CIP36DelegationDistribution);

impl_wasm_cbor_json_api!(CIP36DelegationDistribution);

impl_wasm_conversions!(
    cml_cip36::CIP36DelegationDistribution,
    CIP36DelegationDistribution
);

#[wasm_bindgen]
impl CIP36DelegationDistribution {
    pub fn new_weighted(weighted: &NonEmptyCIP36DelegationList) -> Self {
        Self(cml_cip36::CIP36DelegationDistribution::new_weighted(
            weighted.clone().into(),
        ))
    }

    pub fn new_legacy(legacy: &CIP36LegacyKeyRegistration) -> Self {
        Self(cml_cip36::CIP36DelegationDistribution::new_legacy(
            legacy.clone().into(),
        ))
    }

    pub fn kind(&self) -> CIP36DelegationDistributionKind {
        match &self.0 {
            cml_cip36::CIP36DelegationDistribution::Weighted { .. } => {
                CIP36DelegationDistributionKind::Weighted
            }
            cml_cip36::CIP36DelegationDistribution::Legacy { .. } => {
                CIP36DelegationDistributionKind::Legacy
            }
        }
    }

    pub fn as_weighted(&self) -> Option<NonEmptyCIP36DelegationList> {
        match &self.0 {
            cml_cip36::CIP36DelegationDistribution::Weighted { weighted, .. } => {
                Some(weighted.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_legacy(&self) -> Option<CIP36LegacyKeyRegistration> {
        match &self.0 {
            cml_cip36::CIP36DelegationDistribution::Legacy { legacy, .. } => {
                Some(legacy.clone().into())
            }
            _ => None,
        }
    }
}

#[wasm_bindgen]
pub enum CIP36DelegationDistributionKind {
    Weighted,
    Legacy,
}

impl_wasm_list_needs_into!(
    cml_cip36::CIP36Delegation,
    CIP36Delegation,
    CIP36DelegationList,
    true,
    false
);

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct CIP36DeregistrationCbor(pub(crate) cml_cip36::CIP36DeregistrationCbor);

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
pub struct CIP36DeregistrationWitness(pub(crate) cml_cip36::CIP36DeregistrationWitness);

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
pub struct CIP36KeyDeregistration(pub(crate) cml_cip36::CIP36KeyDeregistration);

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

    pub fn new(stake_credential: &CIP36StakeCredential, nonce: CIP36Nonce) -> Self {
        Self(cml_cip36::CIP36KeyDeregistration::new(
            stake_credential.clone().into(),
            nonce,
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct CIP36KeyRegistration(pub(crate) cml_cip36::CIP36KeyRegistration);

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

    pub fn payment_address(&self) -> PaymentAddress {
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

    pub fn new(
        delegation: &CIP36DelegationDistribution,
        stake_credential: &CIP36StakeCredential,
        payment_address: &PaymentAddress,
        nonce: CIP36Nonce,
    ) -> Self {
        Self(cml_cip36::CIP36KeyRegistration::new(
            delegation.clone().into(),
            stake_credential.clone().into(),
            payment_address.clone().into(),
            nonce,
        ))
    }
}

pub type CIP36LegacyKeyRegistration = PublicKey;

pub type CIP36Nonce = u64;

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct CIP36RegistrationCbor(pub(crate) cml_cip36::CIP36RegistrationCbor);

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
pub struct CIP36RegistrationWitness(pub(crate) cml_cip36::CIP36RegistrationWitness);

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

pub type CIP36StakeCredential = PublicKey;

pub type CIP36StakeWitness = Ed25519Signature;

pub type CIP36StakingPubKey = PublicKey;

pub type CIP36VotingPubKey = PublicKey;

pub type CIP36VotingPurpose = u64;

pub type CIP36Weight = u32;

/// `[+ CIP36Delegation]`: at least one element, enforced by the `NonEmptyVec` representation.
/// Enter via `try_from` or `new(first)`.
/// `add` can never violate the bound; removal is checked in the core type.
#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct NonEmptyCIP36DelegationList(pub(crate) NonEmptyVec<cml_cip36::CIP36Delegation>);

impl_wasm_conversions!(
    NonEmptyVec<cml_cip36::CIP36Delegation>,
    NonEmptyCIP36DelegationList
);

#[wasm_bindgen]
impl NonEmptyCIP36DelegationList {
    pub fn new(first: &CIP36Delegation) -> Self {
        Self(NonEmptyVec::new(first.clone().into()))
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

    pub fn try_from(list: &CIP36DelegationList) -> Result<NonEmptyCIP36DelegationList, JsError> {
        let inner: Vec<cml_cip36::CIP36Delegation> = list.clone().into();
        NonEmptyVec::try_from(inner)
            .map(Self)
            .map_err(|e| JsError::new(&e.to_string()))
    }
}
