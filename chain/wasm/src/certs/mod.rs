// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use super::{
    Coin, Ed25519KeyHashList, Epoch, MapStakeCredentialToDeltaCoin, Port, RelayList, UnitInterval,
};
use crate::address::RewardAccount;
pub use cml_chain::certs::MIRPot;
use cml_core::ordered_hash_map::OrderedHashMap;
use cml_crypto_wasm::{
    Ed25519KeyHash, GenesisDelegateHash, GenesisHash, PoolMetadataHash, ScriptHash, VRFKeyHash,
};
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct Certificate(cml_chain::certs::Certificate);

#[wasm_bindgen]
impl Certificate {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<Certificate, JsValue> {
        cml_chain::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
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

    pub fn from_json(json: &str) -> Result<Certificate, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn new_stake_registration(stake_credential: &StakeCredential) -> Self {
        Self(cml_chain::certs::Certificate::new_stake_registration(
            stake_credential.clone().into(),
        ))
    }

    pub fn new_stake_deregistration(stake_credential: &StakeCredential) -> Self {
        Self(cml_chain::certs::Certificate::new_stake_deregistration(
            stake_credential.clone().into(),
        ))
    }

    pub fn new_stake_delegation(
        stake_credential: &StakeCredential,
        ed25519_key_hash: &Ed25519KeyHash,
    ) -> Self {
        Self(cml_chain::certs::Certificate::new_stake_delegation(
            stake_credential.clone().into(),
            ed25519_key_hash.clone().into(),
        ))
    }

    pub fn new_pool_registration(pool_params: &PoolParams) -> Self {
        Self(cml_chain::certs::Certificate::new_pool_registration(
            pool_params.clone().into(),
        ))
    }

    pub fn new_pool_retirement(ed25519_key_hash: &Ed25519KeyHash, epoch: Epoch) -> Self {
        Self(cml_chain::certs::Certificate::new_pool_retirement(
            ed25519_key_hash.clone().into(),
            epoch,
        ))
    }

    pub fn new_genesis_key_delegation(
        genesis_hash: &GenesisHash,
        genesis_delegate_hash: &GenesisDelegateHash,
        v_r_f_key_hash: &VRFKeyHash,
    ) -> Self {
        Self(cml_chain::certs::Certificate::new_genesis_key_delegation(
            genesis_hash.clone().into(),
            genesis_delegate_hash.clone().into(),
            v_r_f_key_hash.clone().into(),
        ))
    }

    pub fn new_move_instantaneous_rewards_cert(
        move_instantaneous_reward: &MoveInstantaneousReward,
    ) -> Self {
        Self(
            cml_chain::certs::Certificate::new_move_instantaneous_rewards_cert(
                move_instantaneous_reward.clone().into(),
            ),
        )
    }

    pub fn kind(&self) -> CertificateKind {
        match &self.0 {
            cml_chain::certs::Certificate::StakeRegistration(_) => {
                CertificateKind::StakeRegistration
            }
            cml_chain::certs::Certificate::StakeDeregistration(_) => {
                CertificateKind::StakeDeregistration
            }
            cml_chain::certs::Certificate::StakeDelegation(_) => CertificateKind::StakeDelegation,
            cml_chain::certs::Certificate::PoolRegistration(_) => CertificateKind::PoolRegistration,
            cml_chain::certs::Certificate::PoolRetirement(_) => CertificateKind::PoolRetirement,
            cml_chain::certs::Certificate::GenesisKeyDelegation(_) => {
                CertificateKind::GenesisKeyDelegation
            }
            cml_chain::certs::Certificate::MoveInstantaneousRewardsCert(_) => {
                CertificateKind::MoveInstantaneousRewardsCert
            }
        }
    }

    pub fn as_stake_registration(&self) -> Option<StakeRegistration> {
        match &self.0 {
            cml_chain::certs::Certificate::StakeRegistration(stake_registration) => {
                Some(stake_registration.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_stake_deregistration(&self) -> Option<StakeDeregistration> {
        match &self.0 {
            cml_chain::certs::Certificate::StakeDeregistration(stake_deregistration) => {
                Some(stake_deregistration.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_stake_delegation(&self) -> Option<StakeDelegation> {
        match &self.0 {
            cml_chain::certs::Certificate::StakeDelegation(stake_delegation) => {
                Some(stake_delegation.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_pool_registration(&self) -> Option<PoolRegistration> {
        match &self.0 {
            cml_chain::certs::Certificate::PoolRegistration(pool_registration) => {
                Some(pool_registration.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_pool_retirement(&self) -> Option<PoolRetirement> {
        match &self.0 {
            cml_chain::certs::Certificate::PoolRetirement(pool_retirement) => {
                Some(pool_retirement.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_genesis_key_delegation(&self) -> Option<GenesisKeyDelegation> {
        match &self.0 {
            cml_chain::certs::Certificate::GenesisKeyDelegation(genesis_key_delegation) => {
                Some(genesis_key_delegation.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_move_instantaneous_rewards_cert(&self) -> Option<MoveInstantaneousRewardsCert> {
        match &self.0 {
            cml_chain::certs::Certificate::MoveInstantaneousRewardsCert(
                move_instantaneous_rewards_cert,
            ) => Some(move_instantaneous_rewards_cert.clone().into()),
            _ => None,
        }
    }
}

impl From<cml_chain::certs::Certificate> for Certificate {
    fn from(native: cml_chain::certs::Certificate) -> Self {
        Self(native)
    }
}

impl From<Certificate> for cml_chain::certs::Certificate {
    fn from(wasm: Certificate) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::certs::Certificate> for Certificate {
    fn as_ref(&self) -> &cml_chain::certs::Certificate {
        &self.0
    }
}

#[wasm_bindgen]
pub enum CertificateKind {
    StakeRegistration,
    StakeDeregistration,
    StakeDelegation,
    PoolRegistration,
    PoolRetirement,
    GenesisKeyDelegation,
    MoveInstantaneousRewardsCert,
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct DnsName(cml_chain::certs::DnsName);

#[wasm_bindgen]
impl DnsName {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<DnsName, JsValue> {
        cml_chain::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
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

    pub fn from_json(json: &str) -> Result<DnsName, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn get(&self) -> String {
        self.0.get().clone()
    }
}

impl From<cml_chain::certs::DnsName> for DnsName {
    fn from(native: cml_chain::certs::DnsName) -> Self {
        Self(native)
    }
}

impl From<DnsName> for cml_chain::certs::DnsName {
    fn from(wasm: DnsName) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::certs::DnsName> for DnsName {
    fn as_ref(&self) -> &cml_chain::certs::DnsName {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct GenesisKeyDelegation(cml_chain::certs::GenesisKeyDelegation);

#[wasm_bindgen]
impl GenesisKeyDelegation {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<GenesisKeyDelegation, JsValue> {
        cml_chain::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
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

    pub fn from_json(json: &str) -> Result<GenesisKeyDelegation, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn genesis_hash(&self) -> GenesisHash {
        self.0.genesis_hash.clone().into()
    }

    pub fn genesis_delegate_hash(&self) -> GenesisDelegateHash {
        self.0.genesis_delegate_hash.clone().into()
    }

    pub fn v_r_f_key_hash(&self) -> VRFKeyHash {
        self.0.v_r_f_key_hash.clone().into()
    }

    pub fn new(
        genesis_hash: &GenesisHash,
        genesis_delegate_hash: &GenesisDelegateHash,
        v_r_f_key_hash: &VRFKeyHash,
    ) -> Self {
        Self(cml_chain::certs::GenesisKeyDelegation::new(
            genesis_hash.clone().into(),
            genesis_delegate_hash.clone().into(),
            v_r_f_key_hash.clone().into(),
        ))
    }
}

impl From<cml_chain::certs::GenesisKeyDelegation> for GenesisKeyDelegation {
    fn from(native: cml_chain::certs::GenesisKeyDelegation) -> Self {
        Self(native)
    }
}

impl From<GenesisKeyDelegation> for cml_chain::certs::GenesisKeyDelegation {
    fn from(wasm: GenesisKeyDelegation) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::certs::GenesisKeyDelegation> for GenesisKeyDelegation {
    fn as_ref(&self) -> &cml_chain::certs::GenesisKeyDelegation {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct Ipv4(cml_chain::certs::Ipv4);

#[wasm_bindgen]
impl Ipv4 {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<Ipv4, JsValue> {
        cml_chain::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
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

    pub fn from_json(json: &str) -> Result<Ipv4, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn get(&self) -> Vec<u8> {
        self.0.get().clone()
    }
}

impl From<cml_chain::certs::Ipv4> for Ipv4 {
    fn from(native: cml_chain::certs::Ipv4) -> Self {
        Self(native)
    }
}

impl From<Ipv4> for cml_chain::certs::Ipv4 {
    fn from(wasm: Ipv4) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::certs::Ipv4> for Ipv4 {
    fn as_ref(&self) -> &cml_chain::certs::Ipv4 {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct Ipv6(cml_chain::certs::Ipv6);

#[wasm_bindgen]
impl Ipv6 {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<Ipv6, JsValue> {
        cml_chain::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
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

    pub fn from_json(json: &str) -> Result<Ipv6, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn get(&self) -> Vec<u8> {
        self.0.get().clone()
    }
}

impl From<cml_chain::certs::Ipv6> for Ipv6 {
    fn from(native: cml_chain::certs::Ipv6) -> Self {
        Self(native)
    }
}

impl From<Ipv6> for cml_chain::certs::Ipv6 {
    fn from(wasm: Ipv6) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::certs::Ipv6> for Ipv6 {
    fn as_ref(&self) -> &cml_chain::certs::Ipv6 {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MIRAction(cml_chain::certs::MIRAction);

#[wasm_bindgen]
impl MIRAction {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<MIRAction, JsValue> {
        cml_chain::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
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

    pub fn from_json(json: &str) -> Result<MIRAction, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn new_to_stake_credentials(to_stake_credentials: &MapStakeCredentialToDeltaCoin) -> Self {
        Self(cml_chain::certs::MIRAction::new_to_stake_credentials(
            to_stake_credentials.clone().into(),
        ))
    }

    pub fn new_to_other_pot(to_other_pot: Coin) -> Self {
        Self(cml_chain::certs::MIRAction::new_to_other_pot(to_other_pot))
    }

    pub fn kind(&self) -> MIRActionKind {
        match &self.0 {
            cml_chain::certs::MIRAction::ToStakeCredentials { .. } => {
                MIRActionKind::ToStakeCredentials
            }
            cml_chain::certs::MIRAction::ToOtherPot { .. } => MIRActionKind::ToOtherPot,
        }
    }

    pub fn as_to_stake_credentials(&self) -> Option<MapStakeCredentialToDeltaCoin> {
        match &self.0 {
            cml_chain::certs::MIRAction::ToStakeCredentials {
                to_stake_credentials,
                ..
            } => Some(to_stake_credentials.clone().into()),
            _ => None,
        }
    }

    pub fn as_to_other_pot(&self) -> Option<Coin> {
        match &self.0 {
            cml_chain::certs::MIRAction::ToOtherPot { to_other_pot, .. } => Some(*to_other_pot),
            _ => None,
        }
    }
}

impl From<cml_chain::certs::MIRAction> for MIRAction {
    fn from(native: cml_chain::certs::MIRAction) -> Self {
        Self(native)
    }
}

impl From<MIRAction> for cml_chain::certs::MIRAction {
    fn from(wasm: MIRAction) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::certs::MIRAction> for MIRAction {
    fn as_ref(&self) -> &cml_chain::certs::MIRAction {
        &self.0
    }
}

#[wasm_bindgen]
pub enum MIRActionKind {
    ToStakeCredentials,
    ToOtherPot,
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MoveInstantaneousReward(cml_chain::certs::MoveInstantaneousReward);

#[wasm_bindgen]
impl MoveInstantaneousReward {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<MoveInstantaneousReward, JsValue> {
        cml_chain::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
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

    pub fn from_json(json: &str) -> Result<MoveInstantaneousReward, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn pot(&self) -> MIRPot {
        self.0.pot
    }

    pub fn action(&self) -> MIRAction {
        self.0.action.clone().into()
    }

    pub fn new(pot: MIRPot, action: &MIRAction) -> Self {
        Self(cml_chain::certs::MoveInstantaneousReward::new(
            pot.into(),
            action.clone().into(),
        ))
    }
}

impl From<cml_chain::certs::MoveInstantaneousReward> for MoveInstantaneousReward {
    fn from(native: cml_chain::certs::MoveInstantaneousReward) -> Self {
        Self(native)
    }
}

impl From<MoveInstantaneousReward> for cml_chain::certs::MoveInstantaneousReward {
    fn from(wasm: MoveInstantaneousReward) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::certs::MoveInstantaneousReward> for MoveInstantaneousReward {
    fn as_ref(&self) -> &cml_chain::certs::MoveInstantaneousReward {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MoveInstantaneousRewardsCert(cml_chain::certs::MoveInstantaneousRewardsCert);

#[wasm_bindgen]
impl MoveInstantaneousRewardsCert {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<MoveInstantaneousRewardsCert, JsValue> {
        cml_chain::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
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

    pub fn from_json(json: &str) -> Result<MoveInstantaneousRewardsCert, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn move_instantaneous_reward(&self) -> MoveInstantaneousReward {
        self.0.move_instantaneous_reward.clone().into()
    }

    pub fn new(move_instantaneous_reward: &MoveInstantaneousReward) -> Self {
        Self(cml_chain::certs::MoveInstantaneousRewardsCert::new(
            move_instantaneous_reward.clone().into(),
        ))
    }
}

impl From<cml_chain::certs::MoveInstantaneousRewardsCert> for MoveInstantaneousRewardsCert {
    fn from(native: cml_chain::certs::MoveInstantaneousRewardsCert) -> Self {
        Self(native)
    }
}

impl From<MoveInstantaneousRewardsCert> for cml_chain::certs::MoveInstantaneousRewardsCert {
    fn from(wasm: MoveInstantaneousRewardsCert) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::certs::MoveInstantaneousRewardsCert> for MoveInstantaneousRewardsCert {
    fn as_ref(&self) -> &cml_chain::certs::MoveInstantaneousRewardsCert {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MultiHostName(cml_chain::certs::MultiHostName);

#[wasm_bindgen]
impl MultiHostName {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<MultiHostName, JsValue> {
        cml_chain::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
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

    pub fn from_json(json: &str) -> Result<MultiHostName, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn dns_name(&self) -> DnsName {
        self.0.dns_name.clone().into()
    }

    pub fn new(dns_name: &DnsName) -> Self {
        Self(cml_chain::certs::MultiHostName::new(
            dns_name.clone().into(),
        ))
    }
}

impl From<cml_chain::certs::MultiHostName> for MultiHostName {
    fn from(native: cml_chain::certs::MultiHostName) -> Self {
        Self(native)
    }
}

impl From<MultiHostName> for cml_chain::certs::MultiHostName {
    fn from(wasm: MultiHostName) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::certs::MultiHostName> for MultiHostName {
    fn as_ref(&self) -> &cml_chain::certs::MultiHostName {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct PoolMetadata(cml_chain::certs::PoolMetadata);

#[wasm_bindgen]
impl PoolMetadata {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<PoolMetadata, JsValue> {
        cml_chain::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
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

    pub fn from_json(json: &str) -> Result<PoolMetadata, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn url(&self) -> Url {
        self.0.url.clone().into()
    }

    pub fn pool_metadata_hash(&self) -> PoolMetadataHash {
        self.0.pool_metadata_hash.clone().into()
    }

    pub fn new(url: &Url, pool_metadata_hash: &PoolMetadataHash) -> Self {
        Self(cml_chain::certs::PoolMetadata::new(
            url.clone().into(),
            pool_metadata_hash.clone().into(),
        ))
    }
}

impl From<cml_chain::certs::PoolMetadata> for PoolMetadata {
    fn from(native: cml_chain::certs::PoolMetadata) -> Self {
        Self(native)
    }
}

impl From<PoolMetadata> for cml_chain::certs::PoolMetadata {
    fn from(wasm: PoolMetadata) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::certs::PoolMetadata> for PoolMetadata {
    fn as_ref(&self) -> &cml_chain::certs::PoolMetadata {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct PoolParams(cml_chain::certs::PoolParams);

#[wasm_bindgen]
impl PoolParams {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<PoolParams, JsValue> {
        cml_chain::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
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

    pub fn from_json(json: &str) -> Result<PoolParams, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn operator(&self) -> Ed25519KeyHash {
        self.0.operator.clone().into()
    }

    pub fn vrf_keyhash(&self) -> VRFKeyHash {
        self.0.vrf_keyhash.clone().into()
    }

    pub fn pledge(&self) -> Coin {
        self.0.pledge
    }

    pub fn cost(&self) -> Coin {
        self.0.cost
    }

    pub fn margin(&self) -> UnitInterval {
        self.0.margin.clone().into()
    }

    pub fn reward_account(&self) -> RewardAccount {
        self.0.reward_account.clone().into()
    }

    pub fn pool_owners(&self) -> Ed25519KeyHashList {
        self.0.pool_owners.clone().into()
    }

    pub fn relays(&self) -> RelayList {
        self.0.relays.clone().into()
    }

    pub fn pool_metadata(&self) -> Option<PoolMetadata> {
        self.0.pool_metadata.clone().map(std::convert::Into::into)
    }

    pub fn new(
        operator: &Ed25519KeyHash,
        vrf_keyhash: &VRFKeyHash,
        pledge: Coin,
        cost: Coin,
        margin: &UnitInterval,
        reward_account: &RewardAccount,
        pool_owners: &Ed25519KeyHashList,
        relays: &RelayList,
        pool_metadata: Option<PoolMetadata>,
    ) -> Self {
        Self(cml_chain::certs::PoolParams::new(
            operator.clone().into(),
            vrf_keyhash.clone().into(),
            pledge,
            cost,
            margin.clone().into(),
            reward_account.clone().into(),
            pool_owners.clone().into(),
            relays.clone().into(),
            pool_metadata.map(Into::into),
        ))
    }
}

impl From<cml_chain::certs::PoolParams> for PoolParams {
    fn from(native: cml_chain::certs::PoolParams) -> Self {
        Self(native)
    }
}

impl From<PoolParams> for cml_chain::certs::PoolParams {
    fn from(wasm: PoolParams) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::certs::PoolParams> for PoolParams {
    fn as_ref(&self) -> &cml_chain::certs::PoolParams {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct PoolRegistration(cml_chain::certs::PoolRegistration);

#[wasm_bindgen]
impl PoolRegistration {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<PoolRegistration, JsValue> {
        cml_chain::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
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

    pub fn from_json(json: &str) -> Result<PoolRegistration, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn pool_params(&self) -> PoolParams {
        self.0.pool_params.clone().into()
    }

    pub fn new(pool_params: &PoolParams) -> Self {
        Self(cml_chain::certs::PoolRegistration::new(
            pool_params.clone().into(),
        ))
    }
}

impl From<cml_chain::certs::PoolRegistration> for PoolRegistration {
    fn from(native: cml_chain::certs::PoolRegistration) -> Self {
        Self(native)
    }
}

impl From<PoolRegistration> for cml_chain::certs::PoolRegistration {
    fn from(wasm: PoolRegistration) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::certs::PoolRegistration> for PoolRegistration {
    fn as_ref(&self) -> &cml_chain::certs::PoolRegistration {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct PoolRetirement(cml_chain::certs::PoolRetirement);

#[wasm_bindgen]
impl PoolRetirement {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<PoolRetirement, JsValue> {
        cml_chain::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
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

    pub fn from_json(json: &str) -> Result<PoolRetirement, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn ed25519_key_hash(&self) -> Ed25519KeyHash {
        self.0.ed25519_key_hash.clone().into()
    }

    pub fn epoch(&self) -> Epoch {
        self.0.epoch
    }

    pub fn new(ed25519_key_hash: &Ed25519KeyHash, epoch: Epoch) -> Self {
        Self(cml_chain::certs::PoolRetirement::new(
            ed25519_key_hash.clone().into(),
            epoch,
        ))
    }
}

impl From<cml_chain::certs::PoolRetirement> for PoolRetirement {
    fn from(native: cml_chain::certs::PoolRetirement) -> Self {
        Self(native)
    }
}

impl From<PoolRetirement> for cml_chain::certs::PoolRetirement {
    fn from(wasm: PoolRetirement) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::certs::PoolRetirement> for PoolRetirement {
    fn as_ref(&self) -> &cml_chain::certs::PoolRetirement {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct Relay(cml_chain::certs::Relay);

#[wasm_bindgen]
impl Relay {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<Relay, JsValue> {
        cml_chain::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
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

    pub fn from_json(json: &str) -> Result<Relay, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn new_single_host_addr(
        port: Option<Port>,
        ipv4: Option<Ipv4>,
        ipv6: Option<Ipv6>,
    ) -> Self {
        Self(cml_chain::certs::Relay::new_single_host_addr(
            port,
            ipv4.map(Into::into),
            ipv6.map(Into::into),
        ))
    }

    pub fn new_single_host_name(port: Option<Port>, dns_name: &DnsName) -> Self {
        Self(cml_chain::certs::Relay::new_single_host_name(
            port,
            dns_name.clone().into(),
        ))
    }

    pub fn new_multi_host_name(dns_name: &DnsName) -> Self {
        Self(cml_chain::certs::Relay::new_multi_host_name(
            dns_name.clone().into(),
        ))
    }

    pub fn kind(&self) -> RelayKind {
        match &self.0 {
            cml_chain::certs::Relay::SingleHostAddr(_) => RelayKind::SingleHostAddr,
            cml_chain::certs::Relay::SingleHostName(_) => RelayKind::SingleHostName,
            cml_chain::certs::Relay::MultiHostName(_) => RelayKind::MultiHostName,
        }
    }

    pub fn as_single_host_addr(&self) -> Option<SingleHostAddr> {
        match &self.0 {
            cml_chain::certs::Relay::SingleHostAddr(single_host_addr) => {
                Some(single_host_addr.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_single_host_name(&self) -> Option<SingleHostName> {
        match &self.0 {
            cml_chain::certs::Relay::SingleHostName(single_host_name) => {
                Some(single_host_name.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_multi_host_name(&self) -> Option<MultiHostName> {
        match &self.0 {
            cml_chain::certs::Relay::MultiHostName(multi_host_name) => {
                Some(multi_host_name.clone().into())
            }
            _ => None,
        }
    }
}

impl From<cml_chain::certs::Relay> for Relay {
    fn from(native: cml_chain::certs::Relay) -> Self {
        Self(native)
    }
}

impl From<Relay> for cml_chain::certs::Relay {
    fn from(wasm: Relay) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::certs::Relay> for Relay {
    fn as_ref(&self) -> &cml_chain::certs::Relay {
        &self.0
    }
}

#[wasm_bindgen]
pub enum RelayKind {
    SingleHostAddr,
    SingleHostName,
    MultiHostName,
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct SingleHostAddr(cml_chain::certs::SingleHostAddr);

#[wasm_bindgen]
impl SingleHostAddr {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<SingleHostAddr, JsValue> {
        cml_chain::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
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

    pub fn from_json(json: &str) -> Result<SingleHostAddr, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn port(&self) -> Option<Port> {
        self.0.port
    }

    pub fn ipv4(&self) -> Option<Ipv4> {
        self.0.ipv4.clone().map(std::convert::Into::into)
    }

    pub fn ipv6(&self) -> Option<Ipv6> {
        self.0.ipv6.clone().map(std::convert::Into::into)
    }

    pub fn new(port: Option<Port>, ipv4: Option<Ipv4>, ipv6: Option<Ipv6>) -> Self {
        Self(cml_chain::certs::SingleHostAddr::new(
            port,
            ipv4.map(Into::into),
            ipv6.map(Into::into),
        ))
    }
}

impl From<cml_chain::certs::SingleHostAddr> for SingleHostAddr {
    fn from(native: cml_chain::certs::SingleHostAddr) -> Self {
        Self(native)
    }
}

impl From<SingleHostAddr> for cml_chain::certs::SingleHostAddr {
    fn from(wasm: SingleHostAddr) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::certs::SingleHostAddr> for SingleHostAddr {
    fn as_ref(&self) -> &cml_chain::certs::SingleHostAddr {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct SingleHostName(cml_chain::certs::SingleHostName);

#[wasm_bindgen]
impl SingleHostName {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<SingleHostName, JsValue> {
        cml_chain::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
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

    pub fn from_json(json: &str) -> Result<SingleHostName, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn port(&self) -> Option<Port> {
        self.0.port
    }

    pub fn dns_name(&self) -> DnsName {
        self.0.dns_name.clone().into()
    }

    pub fn new(port: Option<Port>, dns_name: &DnsName) -> Self {
        Self(cml_chain::certs::SingleHostName::new(
            port,
            dns_name.clone().into(),
        ))
    }
}

impl From<cml_chain::certs::SingleHostName> for SingleHostName {
    fn from(native: cml_chain::certs::SingleHostName) -> Self {
        Self(native)
    }
}

impl From<SingleHostName> for cml_chain::certs::SingleHostName {
    fn from(wasm: SingleHostName) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::certs::SingleHostName> for SingleHostName {
    fn as_ref(&self) -> &cml_chain::certs::SingleHostName {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct StakeCredential(cml_chain::certs::StakeCredential);

#[wasm_bindgen]
impl StakeCredential {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<StakeCredential, JsValue> {
        cml_chain::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
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

    pub fn from_json(json: &str) -> Result<StakeCredential, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn new_pub_key(hash: &Ed25519KeyHash) -> Self {
        Self(cml_chain::certs::StakeCredential::new_pub_key(
            hash.clone().into(),
        ))
    }

    pub fn new_script(hash: &ScriptHash) -> Self {
        Self(cml_chain::certs::StakeCredential::new_script(
            hash.clone().into(),
        ))
    }

    pub fn kind(&self) -> StakeCredentialKind {
        match &self.0 {
            cml_chain::certs::StakeCredential::PubKey { .. } => StakeCredentialKind::PubKey,
            cml_chain::certs::StakeCredential::Script { .. } => StakeCredentialKind::Script,
        }
    }

    pub fn as_pub_key(&self) -> Option<Ed25519KeyHash> {
        match &self.0 {
            cml_chain::certs::StakeCredential::PubKey { hash, .. } => Some(hash.clone().into()),
            _ => None,
        }
    }

    pub fn as_script(&self) -> Option<ScriptHash> {
        match &self.0 {
            cml_chain::certs::StakeCredential::Script { hash, .. } => Some(hash.clone().into()),
            _ => None,
        }
    }
}

impl From<cml_chain::certs::StakeCredential> for StakeCredential {
    fn from(native: cml_chain::certs::StakeCredential) -> Self {
        Self(native)
    }
}

impl From<StakeCredential> for cml_chain::certs::StakeCredential {
    fn from(wasm: StakeCredential) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::certs::StakeCredential> for StakeCredential {
    fn as_ref(&self) -> &cml_chain::certs::StakeCredential {
        &self.0
    }
}

#[wasm_bindgen]
pub enum StakeCredentialKind {
    PubKey,
    Script,
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct StakeDelegation(cml_chain::certs::StakeDelegation);

#[wasm_bindgen]
impl StakeDelegation {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<StakeDelegation, JsValue> {
        cml_chain::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
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

    pub fn from_json(json: &str) -> Result<StakeDelegation, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn stake_credential(&self) -> StakeCredential {
        self.0.stake_credential.clone().into()
    }

    pub fn ed25519_key_hash(&self) -> Ed25519KeyHash {
        self.0.ed25519_key_hash.clone().into()
    }

    pub fn new(stake_credential: &StakeCredential, ed25519_key_hash: &Ed25519KeyHash) -> Self {
        Self(cml_chain::certs::StakeDelegation::new(
            stake_credential.clone().into(),
            ed25519_key_hash.clone().into(),
        ))
    }
}

impl From<cml_chain::certs::StakeDelegation> for StakeDelegation {
    fn from(native: cml_chain::certs::StakeDelegation) -> Self {
        Self(native)
    }
}

impl From<StakeDelegation> for cml_chain::certs::StakeDelegation {
    fn from(wasm: StakeDelegation) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::certs::StakeDelegation> for StakeDelegation {
    fn as_ref(&self) -> &cml_chain::certs::StakeDelegation {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct StakeDeregistration(cml_chain::certs::StakeDeregistration);

#[wasm_bindgen]
impl StakeDeregistration {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<StakeDeregistration, JsValue> {
        cml_chain::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
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

    pub fn from_json(json: &str) -> Result<StakeDeregistration, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn stake_credential(&self) -> StakeCredential {
        self.0.stake_credential.clone().into()
    }

    pub fn new(stake_credential: &StakeCredential) -> Self {
        Self(cml_chain::certs::StakeDeregistration::new(
            stake_credential.clone().into(),
        ))
    }
}

impl From<cml_chain::certs::StakeDeregistration> for StakeDeregistration {
    fn from(native: cml_chain::certs::StakeDeregistration) -> Self {
        Self(native)
    }
}

impl From<StakeDeregistration> for cml_chain::certs::StakeDeregistration {
    fn from(wasm: StakeDeregistration) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::certs::StakeDeregistration> for StakeDeregistration {
    fn as_ref(&self) -> &cml_chain::certs::StakeDeregistration {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct StakeRegistration(cml_chain::certs::StakeRegistration);

#[wasm_bindgen]
impl StakeRegistration {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<StakeRegistration, JsValue> {
        cml_chain::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
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

    pub fn from_json(json: &str) -> Result<StakeRegistration, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn stake_credential(&self) -> StakeCredential {
        self.0.stake_credential.clone().into()
    }

    pub fn new(stake_credential: &StakeCredential) -> Self {
        Self(cml_chain::certs::StakeRegistration::new(
            stake_credential.clone().into(),
        ))
    }
}

impl From<cml_chain::certs::StakeRegistration> for StakeRegistration {
    fn from(native: cml_chain::certs::StakeRegistration) -> Self {
        Self(native)
    }
}

impl From<StakeRegistration> for cml_chain::certs::StakeRegistration {
    fn from(wasm: StakeRegistration) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::certs::StakeRegistration> for StakeRegistration {
    fn as_ref(&self) -> &cml_chain::certs::StakeRegistration {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct Url(cml_chain::certs::Url);

#[wasm_bindgen]
impl Url {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<Url, JsValue> {
        cml_chain::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
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

    pub fn from_json(json: &str) -> Result<Url, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn get(&self) -> String {
        self.0.get().clone()
    }
}

impl From<cml_chain::certs::Url> for Url {
    fn from(native: cml_chain::certs::Url) -> Self {
        Self(native)
    }
}

impl From<Url> for cml_chain::certs::Url {
    fn from(wasm: Url) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::certs::Url> for Url {
    fn as_ref(&self) -> &cml_chain::certs::Url {
        &self.0
    }
}
