#![allow(clippy::len_without_is_empty, clippy::too_many_arguments, clippy::new_without_default)]

use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

use core::ordered_hash_map::OrderedHashMap;

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
}pub type LegacyKeyRegistration = VotingPubKey;

pub type Nonce = u64;

pub type RewardAddress = Vec<u8>;

pub type StakeCredential = StakingPubKey;

pub type StakeWitness = Ed25519Signature;

pub type VotingPurpose = u64;

pub type Weight = u32;

#[wasm_bindgen]

pub enum ArrDelegationOrLegacyKeyRegistrationKind {
    ArrDelegation,
    LegacyKeyRegistration,
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct ArrDelegationOrLegacyKeyRegistration(pub(crate) core::ArrDelegationOrLegacyKeyRegistration);

#[wasm_bindgen]

impl ArrDelegationOrLegacyKeyRegistration {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<ArrDelegationOrLegacyKeyRegistration, JsValue> {
        Deserialize::from_cbor_bytes(cbor_bytes).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<ArrDelegationOrLegacyKeyRegistration, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn new_arr_delegation(arr_delegation: &Delegations) -> Self {
        Self(core::ArrDelegationOrLegacyKeyRegistration::new_arr_delegation(arr_delegation.clone().into()))
    }

    pub fn new_legacy_key_registration(legacy_key_registration: &LegacyKeyRegistration) -> Self {
        Self(core::ArrDelegationOrLegacyKeyRegistration::new_legacy_key_registration(legacy_key_registration.clone().into()))
    }

    pub fn kind(&self) -> ArrDelegationOrLegacyKeyRegistrationKind {
        match &self.0 {
            core::ArrDelegationOrLegacyKeyRegistration::ArrDelegation{ .. } => ArrDelegationOrLegacyKeyRegistrationKind::ArrDelegation,
            core::ArrDelegationOrLegacyKeyRegistration::LegacyKeyRegistration(_) => ArrDelegationOrLegacyKeyRegistrationKind::LegacyKeyRegistration,
        }
    }

    pub fn as_arr_delegation(&self) -> Option<Delegations> {
        match &self.0 {
            core::ArrDelegationOrLegacyKeyRegistration::ArrDelegation{ arr_delegation, .. } => Some(arr_delegation.clone().into()),
            _ => None,
        }
    }

    pub fn as_legacy_key_registration(&self) -> Option<LegacyKeyRegistration> {
        match &self.0 {
            core::ArrDelegationOrLegacyKeyRegistration::LegacyKeyRegistration(legacy_key_registration) => Some(legacy_key_registration.clone().into()),
            _ => None,
        }
    }
}

impl From<core::ArrDelegationOrLegacyKeyRegistration> for ArrDelegationOrLegacyKeyRegistration {
    fn from(native: core::ArrDelegationOrLegacyKeyRegistration) -> Self {
        Self(native)
    }
}

impl From<ArrDelegationOrLegacyKeyRegistration> for core::ArrDelegationOrLegacyKeyRegistration {
    fn from(wasm: ArrDelegationOrLegacyKeyRegistration) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct Delegation(pub(crate) core::Delegation);

#[wasm_bindgen]

impl Delegation {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<Delegation, JsValue> {
        Deserialize::from_cbor_bytes(cbor_bytes).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<Delegation, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
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

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct DeregistrationCbor(pub(crate) core::DeregistrationCbor);

#[wasm_bindgen]

impl DeregistrationCbor {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<DeregistrationCbor, JsValue> {
        Deserialize::from_cbor_bytes(cbor_bytes).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<DeregistrationCbor, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn key_deregistration(&self) -> KeyDeregistration {
        self.0.key_deregistration.clone().into()
    }

    pub fn deregistration_witness(&self) -> DeregistrationWitness {
        self.0.deregistration_witness.clone().into()
    }

    pub fn new(key_deregistration: &KeyDeregistration, deregistration_witness: &DeregistrationWitness) -> Self {
        Self(core::DeregistrationCbor::new(key_deregistration.clone().into(), deregistration_witness.clone().into()))
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

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct DeregistrationWitness(pub(crate) core::DeregistrationWitness);

#[wasm_bindgen]

impl DeregistrationWitness {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<DeregistrationWitness, JsValue> {
        Deserialize::from_cbor_bytes(cbor_bytes).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<DeregistrationWitness, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn stake_witness(&self) -> StakeWitness {
        self.0.stake_witness.clone().into()
    }

    pub fn new(stake_witness: &StakeWitness) -> Self {
        Self(core::DeregistrationWitness::new(stake_witness.clone().into()))
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

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct Ed25519Signature(pub(crate) core::Ed25519Signature);

#[wasm_bindgen]

impl Ed25519Signature {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<Ed25519Signature, JsValue> {
        Deserialize::from_cbor_bytes(cbor_bytes).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<Ed25519Signature, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn get(&self) -> Vec<u8> {
        self.0.get().clone()
    }
}

impl From<core::Ed25519Signature> for Ed25519Signature {
    fn from(native: core::Ed25519Signature) -> Self {
        Self(native)
    }
}

impl From<Ed25519Signature> for core::Ed25519Signature {
    fn from(wasm: Ed25519Signature) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct KeyDeregistration(pub(crate) core::KeyDeregistration);

#[wasm_bindgen]

impl KeyDeregistration {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<KeyDeregistration, JsValue> {
        Deserialize::from_cbor_bytes(cbor_bytes).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<KeyDeregistration, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
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
        Self(core::KeyDeregistration::new(stake_credential.clone().into(), nonce))
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

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct KeyRegistration(pub(crate) core::KeyRegistration);

#[wasm_bindgen]

impl KeyRegistration {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<KeyRegistration, JsValue> {
        Deserialize::from_cbor_bytes(cbor_bytes).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<KeyRegistration, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn delegation(&self) -> ArrDelegationOrLegacyKeyRegistration {
        self.0.delegation.clone().into()
    }

    pub fn stake_credential(&self) -> StakeCredential {
        self.0.stake_credential.clone().into()
    }

    pub fn reward_address(&self) -> RewardAddress {
        self.0.reward_address.clone()
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

    pub fn new(delegation: &ArrDelegationOrLegacyKeyRegistration, stake_credential: &StakeCredential, reward_address: RewardAddress, nonce: Nonce) -> Self {
        Self(core::KeyRegistration::new(delegation.clone().into(), stake_credential.clone().into(), reward_address, nonce))
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

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct RegistrationCbor(pub(crate) core::RegistrationCbor);

#[wasm_bindgen]

impl RegistrationCbor {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<RegistrationCbor, JsValue> {
        Deserialize::from_cbor_bytes(cbor_bytes).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<RegistrationCbor, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn key_registration(&self) -> KeyRegistration {
        self.0.key_registration.clone().into()
    }

    pub fn registration_witness(&self) -> RegistrationWitness {
        self.0.registration_witness.clone().into()
    }

    pub fn new(key_registration: &KeyRegistration, registration_witness: &RegistrationWitness) -> Self {
        Self(core::RegistrationCbor::new(key_registration.clone().into(), registration_witness.clone().into()))
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

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct RegistrationWitness(pub(crate) core::RegistrationWitness);

#[wasm_bindgen]

impl RegistrationWitness {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<RegistrationWitness, JsValue> {
        Deserialize::from_cbor_bytes(cbor_bytes).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<RegistrationWitness, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
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

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct StakingPubKey(pub(crate) core::StakingPubKey);

#[wasm_bindgen]

impl StakingPubKey {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<StakingPubKey, JsValue> {
        Deserialize::from_cbor_bytes(cbor_bytes).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<StakingPubKey, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn get(&self) -> Vec<u8> {
        self.0.get().clone()
    }
}

impl From<core::StakingPubKey> for StakingPubKey {
    fn from(native: core::StakingPubKey) -> Self {
        Self(native)
    }
}

impl From<StakingPubKey> for core::StakingPubKey {
    fn from(wasm: StakingPubKey) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct VotingPubKey(pub(crate) core::VotingPubKey);

#[wasm_bindgen]

impl VotingPubKey {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<VotingPubKey, JsValue> {
        Deserialize::from_cbor_bytes(cbor_bytes).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<VotingPubKey, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn get(&self) -> Vec<u8> {
        self.0.get().clone()
    }
}

impl From<core::VotingPubKey> for VotingPubKey {
    fn from(native: core::VotingPubKey) -> Self {
        Self(native)
    }
}

impl From<VotingPubKey> for core::VotingPubKey {
    fn from(wasm: VotingPubKey) -> Self {
        wasm.0
    }
}