// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use super::{
    Coin, Ed25519KeyHashList, Epoch, MapStakeCredentialToDeltaCoin, Port, RelayList, UnitInterval,
};
use crate::address::RewardAccount;
pub use cml_chain::certs::MIRPot;
use cml_crypto_wasm::{
    Ed25519KeyHash, GenesisDelegateHash, GenesisHash, PoolMetadataHash, ScriptHash, VRFKeyHash,
};
use cml_core_wasm::{impl_wasm_cbor_json_api, impl_wasm_conversions};
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct Certificate(cml_chain::certs::Certificate);

impl_wasm_cbor_json_api!(Certificate);

impl_wasm_conversions!(cml_chain::certs::Certificate, Certificate);

#[wasm_bindgen]
impl Certificate {
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

impl_wasm_cbor_json_api!(DnsName);

impl_wasm_conversions!(cml_chain::certs::DnsName, DnsName);

#[wasm_bindgen]
impl DnsName {
    pub fn get(&self) -> String {
        self.0.get().clone()
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct GenesisKeyDelegation(cml_chain::certs::GenesisKeyDelegation);

impl_wasm_cbor_json_api!(GenesisKeyDelegation);

impl_wasm_conversions!(cml_chain::certs::GenesisKeyDelegation, GenesisKeyDelegation);

#[wasm_bindgen]
impl GenesisKeyDelegation {
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

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct Ipv4(cml_chain::certs::Ipv4);

impl_wasm_cbor_json_api!(Ipv4);

impl_wasm_conversions!(cml_chain::certs::Ipv4, Ipv4);

#[wasm_bindgen]
impl Ipv4 {
    pub fn get(&self) -> Vec<u8> {
        self.0.get().clone()
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct Ipv6(cml_chain::certs::Ipv6);

impl_wasm_cbor_json_api!(Ipv6);

impl_wasm_conversions!(cml_chain::certs::Ipv6, Ipv6);

#[wasm_bindgen]
impl Ipv6 {
    pub fn get(&self) -> Vec<u8> {
        self.0.get().clone()
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MIRAction(cml_chain::certs::MIRAction);

impl_wasm_cbor_json_api!(MIRAction);

impl_wasm_conversions!(cml_chain::certs::MIRAction, MIRAction);

#[wasm_bindgen]
impl MIRAction {
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

#[wasm_bindgen]
pub enum MIRActionKind {
    ToStakeCredentials,
    ToOtherPot,
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MoveInstantaneousReward(cml_chain::certs::MoveInstantaneousReward);

impl_wasm_cbor_json_api!(MoveInstantaneousReward);

impl_wasm_conversions!(
    cml_chain::certs::MoveInstantaneousReward,
    MoveInstantaneousReward
);

#[wasm_bindgen]
impl MoveInstantaneousReward {
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

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MoveInstantaneousRewardsCert(cml_chain::certs::MoveInstantaneousRewardsCert);

impl_wasm_cbor_json_api!(MoveInstantaneousRewardsCert);

impl_wasm_conversions!(
    cml_chain::certs::MoveInstantaneousRewardsCert,
    MoveInstantaneousRewardsCert
);

#[wasm_bindgen]
impl MoveInstantaneousRewardsCert {
    pub fn move_instantaneous_reward(&self) -> MoveInstantaneousReward {
        self.0.move_instantaneous_reward.clone().into()
    }

    pub fn new(move_instantaneous_reward: &MoveInstantaneousReward) -> Self {
        Self(cml_chain::certs::MoveInstantaneousRewardsCert::new(
            move_instantaneous_reward.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MultiHostName(cml_chain::certs::MultiHostName);

impl_wasm_cbor_json_api!(MultiHostName);

impl_wasm_conversions!(cml_chain::certs::MultiHostName, MultiHostName);

#[wasm_bindgen]
impl MultiHostName {
    pub fn dns_name(&self) -> DnsName {
        self.0.dns_name.clone().into()
    }

    pub fn new(dns_name: &DnsName) -> Self {
        Self(cml_chain::certs::MultiHostName::new(
            dns_name.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct PoolMetadata(cml_chain::certs::PoolMetadata);

impl_wasm_cbor_json_api!(PoolMetadata);

impl_wasm_conversions!(cml_chain::certs::PoolMetadata, PoolMetadata);

#[wasm_bindgen]
impl PoolMetadata {
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

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct PoolParams(cml_chain::certs::PoolParams);

impl_wasm_cbor_json_api!(PoolParams);

impl_wasm_conversions!(cml_chain::certs::PoolParams, PoolParams);

#[wasm_bindgen]
impl PoolParams {
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

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct PoolRegistration(cml_chain::certs::PoolRegistration);

impl_wasm_cbor_json_api!(PoolRegistration);

impl_wasm_conversions!(cml_chain::certs::PoolRegistration, PoolRegistration);

#[wasm_bindgen]
impl PoolRegistration {
    pub fn pool_params(&self) -> PoolParams {
        self.0.pool_params.clone().into()
    }

    pub fn new(pool_params: &PoolParams) -> Self {
        Self(cml_chain::certs::PoolRegistration::new(
            pool_params.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct PoolRetirement(cml_chain::certs::PoolRetirement);

impl_wasm_cbor_json_api!(PoolRetirement);

impl_wasm_conversions!(cml_chain::certs::PoolRetirement, PoolRetirement);

#[wasm_bindgen]
impl PoolRetirement {
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

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct Relay(cml_chain::certs::Relay);

impl_wasm_cbor_json_api!(Relay);

impl_wasm_conversions!(cml_chain::certs::Relay, Relay);

#[wasm_bindgen]
impl Relay {
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

#[wasm_bindgen]
pub enum RelayKind {
    SingleHostAddr,
    SingleHostName,
    MultiHostName,
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct SingleHostAddr(cml_chain::certs::SingleHostAddr);

impl_wasm_cbor_json_api!(SingleHostAddr);

impl_wasm_conversions!(cml_chain::certs::SingleHostAddr, SingleHostAddr);

#[wasm_bindgen]
impl SingleHostAddr {
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

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct SingleHostName(cml_chain::certs::SingleHostName);

impl_wasm_cbor_json_api!(SingleHostName);

impl_wasm_conversions!(cml_chain::certs::SingleHostName, SingleHostName);

#[wasm_bindgen]
impl SingleHostName {
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

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct StakeCredential(cml_chain::certs::StakeCredential);

impl_wasm_cbor_json_api!(StakeCredential);

impl_wasm_conversions!(cml_chain::certs::StakeCredential, StakeCredential);

#[wasm_bindgen]
impl StakeCredential {
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

#[wasm_bindgen]
pub enum StakeCredentialKind {
    PubKey,
    Script,
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct StakeDelegation(cml_chain::certs::StakeDelegation);

impl_wasm_cbor_json_api!(StakeDelegation);

impl_wasm_conversions!(cml_chain::certs::StakeDelegation, StakeDelegation);

#[wasm_bindgen]
impl StakeDelegation {
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

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct StakeDeregistration(cml_chain::certs::StakeDeregistration);

impl_wasm_cbor_json_api!(StakeDeregistration);

impl_wasm_conversions!(cml_chain::certs::StakeDeregistration, StakeDeregistration);

#[wasm_bindgen]
impl StakeDeregistration {
    pub fn stake_credential(&self) -> StakeCredential {
        self.0.stake_credential.clone().into()
    }

    pub fn new(stake_credential: &StakeCredential) -> Self {
        Self(cml_chain::certs::StakeDeregistration::new(
            stake_credential.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct StakeRegistration(cml_chain::certs::StakeRegistration);

impl_wasm_cbor_json_api!(StakeRegistration);

impl_wasm_conversions!(cml_chain::certs::StakeRegistration, StakeRegistration);

#[wasm_bindgen]
impl StakeRegistration {
    pub fn stake_credential(&self) -> StakeCredential {
        self.0.stake_credential.clone().into()
    }

    pub fn new(stake_credential: &StakeCredential) -> Self {
        Self(cml_chain::certs::StakeRegistration::new(
            stake_credential.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct Url(cml_chain::certs::Url);

impl_wasm_cbor_json_api!(Url);

impl_wasm_conversions!(cml_chain::certs::Url, Url);

#[wasm_bindgen]
impl Url {
    pub fn get(&self) -> String {
        self.0.get().clone()
    }
}
