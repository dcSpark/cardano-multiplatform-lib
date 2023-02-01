#![allow(
    clippy::len_without_is_empty,
    clippy::too_many_arguments,
    clippy::new_without_default
)]

use wasm_bindgen::prelude::{wasm_bindgen, JsError, JsValue};

use wasm_crypto::impl_chain_crypto;

use core::{Deserialize, Serialize};

use core_crypto::RawBytesEncoding;

pub use wasm_chain::address::RewardAddress;

impl_chain_crypto!(VotingPubKey, PublicKey, wasm_crypto);
impl_chain_crypto!(StakingPubKey, PublicKey, wasm_crypto);
impl_chain_crypto!(StakeWitness, Ed25519Signature, wasm_crypto);

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Delegations(pub(crate) Vec<core::Delegation>);

#[wasm_bindgen]

impl Delegations {
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

impl From<Vec<core::Delegation>> for Delegations {
    fn from(native: Vec<core::Delegation>) -> Self {
        Self(native)
    }
}

impl From<Delegations> for Vec<core::Delegation> {
    fn from(wrapper: Delegations) -> Self {
        wrapper.0
    }
}

impl AsRef<Vec<core::Delegation>> for Delegations {
    fn as_ref(&self) -> &Vec<core::Delegation> {
        &self.0
    }
}

pub type LegacyKeyRegistration = VotingPubKey;

pub type Nonce = u64;

pub type StakeCredential = StakingPubKey;

pub type VotingPurpose = u64;

pub type Weight = u32;

#[wasm_bindgen]
pub enum DelegationDistributionKind {
    Weighted,
    LegacyKeyRegistration,
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct DelegationDistribution(pub(crate) core::DelegationDistribution);

#[wasm_bindgen]
impl DelegationDistribution {
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

    pub fn new_weighted(delegations: &Delegations) -> Self {
        Self(core::DelegationDistribution::new_weighted(
            delegations.clone().into(),
        ))
    }

    pub fn new_legacy_key_registration(legacy_key_registration: &LegacyKeyRegistration) -> Self {
        Self(core::DelegationDistribution::new_legacy_key_registration(
            legacy_key_registration.clone().into(),
        ))
    }

    pub fn kind(&self) -> DelegationDistributionKind {
        match &self.0 {
            core::DelegationDistribution::Weighted { .. } => DelegationDistributionKind::Weighted,
            core::DelegationDistribution::LegacyKeyRegistration(_) => {
                DelegationDistributionKind::LegacyKeyRegistration
            }
        }
    }

    pub fn as_arr_delegation(&self) -> Option<Delegations> {
        match &self.0 {
            core::DelegationDistribution::Weighted { delegations, .. } => {
                Some(delegations.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_legacy_key_registration(&self) -> Option<LegacyKeyRegistration> {
        match &self.0 {
            core::DelegationDistribution::LegacyKeyRegistration(legacy_key_registration) => {
                Some(legacy_key_registration.clone().into())
            }
            _ => None,
        }
    }
}

impl From<core::DelegationDistribution> for DelegationDistribution {
    fn from(native: core::DelegationDistribution) -> Self {
        Self(native)
    }
}

impl From<DelegationDistribution> for core::DelegationDistribution {
    fn from(wasm: DelegationDistribution) -> Self {
        wasm.0
    }
}

impl AsRef<core::DelegationDistribution> for DelegationDistribution {
    fn as_ref(&self) -> &core::DelegationDistribution {
        &self.0
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Delegation(pub(crate) core::Delegation);

#[wasm_bindgen]
impl Delegation {
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
        Self(core::Delegation::new(voting_pub_key.clone().into(), weight))
    }
}

impl From<core::Delegation> for Delegation {
    fn from(native: core::Delegation) -> Self {
        Self(native)
    }
}

impl From<Delegation> for core::Delegation {
    fn from(wasm: Delegation) -> Self {
        wasm.0
    }
}

impl AsRef<core::Delegation> for Delegation {
    fn as_ref(&self) -> &core::Delegation {
        &self.0
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct DeregistrationCbor(pub(crate) core::DeregistrationCbor);

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
        Self(core::DeregistrationCbor::new(
            key_deregistration.clone().into(),
            deregistration_witness.clone().into(),
        ))
    }
}

impl From<core::DeregistrationCbor> for DeregistrationCbor {
    fn from(native: core::DeregistrationCbor) -> Self {
        Self(native)
    }
}

impl From<DeregistrationCbor> for core::DeregistrationCbor {
    fn from(wasm: DeregistrationCbor) -> Self {
        wasm.0
    }
}

impl AsRef<core::DeregistrationCbor> for DeregistrationCbor {
    fn as_ref(&self) -> &core::DeregistrationCbor {
        &self.0
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct DeregistrationWitness(pub(crate) core::DeregistrationWitness);

#[wasm_bindgen]
impl DeregistrationWitness {
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
        Self(core::DeregistrationWitness::new(
            stake_witness.clone().into(),
        ))
    }
}

impl From<core::DeregistrationWitness> for DeregistrationWitness {
    fn from(native: core::DeregistrationWitness) -> Self {
        Self(native)
    }
}

impl From<DeregistrationWitness> for core::DeregistrationWitness {
    fn from(wasm: DeregistrationWitness) -> Self {
        wasm.0
    }
}

impl AsRef<core::DeregistrationWitness> for DeregistrationWitness {
    fn as_ref(&self) -> &core::DeregistrationWitness {
        &self.0
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct KeyDeregistration(pub(crate) core::KeyDeregistration);

#[wasm_bindgen]
impl KeyDeregistration {
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
        Self(core::KeyDeregistration::new(
            stake_credential.clone().into(),
            nonce,
        ))
    }
}

impl From<core::KeyDeregistration> for KeyDeregistration {
    fn from(native: core::KeyDeregistration) -> Self {
        Self(native)
    }
}

impl From<KeyDeregistration> for core::KeyDeregistration {
    fn from(wasm: KeyDeregistration) -> Self {
        wasm.0
    }
}

impl AsRef<core::KeyDeregistration> for KeyDeregistration {
    fn as_ref(&self) -> &core::KeyDeregistration {
        &self.0
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct KeyRegistration(pub(crate) core::KeyRegistration);

#[wasm_bindgen]
impl KeyRegistration {
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
        Self(core::KeyRegistration::new(
            delegation.clone().into(),
            stake_credential.clone().into(),
            reward_address.clone().into(),
            nonce,
        ))
    }
}

impl From<core::KeyRegistration> for KeyRegistration {
    fn from(native: core::KeyRegistration) -> Self {
        Self(native)
    }
}

impl From<KeyRegistration> for core::KeyRegistration {
    fn from(wasm: KeyRegistration) -> Self {
        wasm.0
    }
}

impl AsRef<core::KeyRegistration> for KeyRegistration {
    fn as_ref(&self) -> &core::KeyRegistration {
        &self.0
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct RegistrationCbor(pub(crate) core::RegistrationCbor);

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
        Self(core::RegistrationCbor::new(
            key_registration.clone().into(),
            registration_witness.clone().into(),
        ))
    }
}

impl From<core::RegistrationCbor> for RegistrationCbor {
    fn from(native: core::RegistrationCbor) -> Self {
        Self(native)
    }
}

impl From<RegistrationCbor> for core::RegistrationCbor {
    fn from(wasm: RegistrationCbor) -> Self {
        wasm.0
    }
}

impl AsRef<core::RegistrationCbor> for RegistrationCbor {
    fn as_ref(&self) -> &core::RegistrationCbor {
        &self.0
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct RegistrationWitness(pub(crate) core::RegistrationWitness);

#[wasm_bindgen]
impl RegistrationWitness {
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
        Self(core::RegistrationWitness::new(stake_witness.clone().into()))
    }
}

impl From<core::RegistrationWitness> for RegistrationWitness {
    fn from(native: core::RegistrationWitness) -> Self {
        Self(native)
    }
}

impl From<RegistrationWitness> for core::RegistrationWitness {
    fn from(wasm: RegistrationWitness) -> Self {
        wasm.0
    }
}

impl AsRef<core::RegistrationWitness> for RegistrationWitness {
    fn as_ref(&self) -> &core::RegistrationWitness {
        &self.0
    }
}
