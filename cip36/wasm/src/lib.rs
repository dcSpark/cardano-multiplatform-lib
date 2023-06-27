#![allow(
    clippy::len_without_is_empty,
    clippy::too_many_arguments,
    clippy::new_without_default
)]
// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

use cml_chain_wasm::address::RewardAddress;
pub mod utils;

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct Delegation(cml_cip36::Delegation);

#[wasm_bindgen]
impl Delegation {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_core::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<Delegation, JsValue> {
        cml_core::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<Delegation, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

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

impl From<cml_cip36::Delegation> for Delegation {
    fn from(native: cml_cip36::Delegation) -> Self {
        Self(native)
    }
}

impl From<Delegation> for cml_cip36::Delegation {
    fn from(wasm: Delegation) -> Self {
        wasm.0
    }
}

impl AsRef<cml_cip36::Delegation> for Delegation {
    fn as_ref(&self) -> &cml_cip36::Delegation {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct DelegationDistribution(cml_cip36::DelegationDistribution);

#[wasm_bindgen]
impl DelegationDistribution {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_core::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<DelegationDistribution, JsValue> {
        cml_core::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<DelegationDistribution, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

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

impl From<cml_cip36::DelegationDistribution> for DelegationDistribution {
    fn from(native: cml_cip36::DelegationDistribution) -> Self {
        Self(native)
    }
}

impl From<DelegationDistribution> for cml_cip36::DelegationDistribution {
    fn from(wasm: DelegationDistribution) -> Self {
        wasm.0
    }
}

impl AsRef<cml_cip36::DelegationDistribution> for DelegationDistribution {
    fn as_ref(&self) -> &cml_cip36::DelegationDistribution {
        &self.0
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

#[wasm_bindgen]
impl DeregistrationCbor {
    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<DeregistrationCbor, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

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

impl From<cml_cip36::DeregistrationCbor> for DeregistrationCbor {
    fn from(native: cml_cip36::DeregistrationCbor) -> Self {
        Self(native)
    }
}

impl From<DeregistrationCbor> for cml_cip36::DeregistrationCbor {
    fn from(wasm: DeregistrationCbor) -> Self {
        wasm.0
    }
}

impl AsRef<cml_cip36::DeregistrationCbor> for DeregistrationCbor {
    fn as_ref(&self) -> &cml_cip36::DeregistrationCbor {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct DeregistrationWitness(cml_cip36::DeregistrationWitness);

#[wasm_bindgen]
impl DeregistrationWitness {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_core::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<DeregistrationWitness, JsValue> {
        cml_core::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<DeregistrationWitness, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn stake_witness(&self) -> StakeWitness {
        self.0.stake_witness.clone().into()
    }

    pub fn new(stake_witness: &StakeWitness) -> Self {
        Self(cml_cip36::DeregistrationWitness::new(
            stake_witness.clone().into(),
        ))
    }
}

impl From<cml_cip36::DeregistrationWitness> for DeregistrationWitness {
    fn from(native: cml_cip36::DeregistrationWitness) -> Self {
        Self(native)
    }
}

impl From<DeregistrationWitness> for cml_cip36::DeregistrationWitness {
    fn from(wasm: DeregistrationWitness) -> Self {
        wasm.0
    }
}

impl AsRef<cml_cip36::DeregistrationWitness> for DeregistrationWitness {
    fn as_ref(&self) -> &cml_cip36::DeregistrationWitness {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct KeyDeregistration(cml_cip36::KeyDeregistration);

#[wasm_bindgen]
impl KeyDeregistration {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_core::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<KeyDeregistration, JsValue> {
        cml_core::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<KeyDeregistration, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

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

    pub fn new(stake_credential: &StakeCredential, nonce: Nonce) -> Self {
        Self(cml_cip36::KeyDeregistration::new(
            stake_credential.clone().into(),
            nonce,
        ))
    }
}

impl From<cml_cip36::KeyDeregistration> for KeyDeregistration {
    fn from(native: cml_cip36::KeyDeregistration) -> Self {
        Self(native)
    }
}

impl From<KeyDeregistration> for cml_cip36::KeyDeregistration {
    fn from(wasm: KeyDeregistration) -> Self {
        wasm.0
    }
}

impl AsRef<cml_cip36::KeyDeregistration> for KeyDeregistration {
    fn as_ref(&self) -> &cml_cip36::KeyDeregistration {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct KeyRegistration(cml_cip36::KeyRegistration);

#[wasm_bindgen]
impl KeyRegistration {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_core::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<KeyRegistration, JsValue> {
        cml_core::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<KeyRegistration, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn delegation(&self) -> DelegationDistribution {
        self.0.delegation.clone().into()
    }

    pub fn stake_credential(&self) -> StakeCredential {
        self.0.stake_credential.clone().into()
    }

    pub fn reward_address(&self) -> RewardAddress {
        self.0.reward_address.clone().into()
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

    pub fn new(
        delegation: &DelegationDistribution,
        stake_credential: &StakeCredential,
        reward_address: &RewardAddress,
        nonce: Nonce,
    ) -> Self {
        Self(cml_cip36::KeyRegistration::new(
            delegation.clone().into(),
            stake_credential.clone().into(),
            reward_address.clone().into(),
            nonce,
        ))
    }
}

impl From<cml_cip36::KeyRegistration> for KeyRegistration {
    fn from(native: cml_cip36::KeyRegistration) -> Self {
        Self(native)
    }
}

impl From<KeyRegistration> for cml_cip36::KeyRegistration {
    fn from(wasm: KeyRegistration) -> Self {
        wasm.0
    }
}

impl AsRef<cml_cip36::KeyRegistration> for KeyRegistration {
    fn as_ref(&self) -> &cml_cip36::KeyRegistration {
        &self.0
    }
}

pub type LegacyKeyRegistration = cml_crypto_wasm::PublicKey;

pub type Nonce = u64;

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct RegistrationCbor(cml_cip36::RegistrationCbor);

#[wasm_bindgen]
impl RegistrationCbor {
    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<RegistrationCbor, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

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

impl From<cml_cip36::RegistrationCbor> for RegistrationCbor {
    fn from(native: cml_cip36::RegistrationCbor) -> Self {
        Self(native)
    }
}

impl From<RegistrationCbor> for cml_cip36::RegistrationCbor {
    fn from(wasm: RegistrationCbor) -> Self {
        wasm.0
    }
}

impl AsRef<cml_cip36::RegistrationCbor> for RegistrationCbor {
    fn as_ref(&self) -> &cml_cip36::RegistrationCbor {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct RegistrationWitness(cml_cip36::RegistrationWitness);

#[wasm_bindgen]
impl RegistrationWitness {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_core::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<RegistrationWitness, JsValue> {
        cml_core::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<RegistrationWitness, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn stake_witness(&self) -> StakeWitness {
        self.0.stake_witness.clone().into()
    }

    pub fn new(stake_witness: &StakeWitness) -> Self {
        Self(cml_cip36::RegistrationWitness::new(
            stake_witness.clone().into(),
        ))
    }
}

impl From<cml_cip36::RegistrationWitness> for RegistrationWitness {
    fn from(native: cml_cip36::RegistrationWitness) -> Self {
        Self(native)
    }
}

impl From<RegistrationWitness> for cml_cip36::RegistrationWitness {
    fn from(wasm: RegistrationWitness) -> Self {
        wasm.0
    }
}

impl AsRef<cml_cip36::RegistrationWitness> for RegistrationWitness {
    fn as_ref(&self) -> &cml_cip36::RegistrationWitness {
        &self.0
    }
}

pub type StakeCredential = cml_crypto_wasm::PublicKey;

pub type StakeWitness = cml_crypto_wasm::Ed25519Signature;

pub type StakingPubKey = cml_crypto_wasm::PublicKey;

pub type VotingPubKey = cml_crypto_wasm::PublicKey;

pub type VotingPurpose = u64;

pub type Weight = u32;
