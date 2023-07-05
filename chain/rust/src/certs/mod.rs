// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

pub mod cbor_encodings;
pub mod serialization;
pub mod utils;

use super::{Coin, DeltaCoin, Epoch, Port, UnitInterval};
use crate::address::RewardAccount;
use crate::crypto::{
    Ed25519KeyHash, GenesisDelegateHash, GenesisHash, PoolMetadataHash,
    ScriptHash, VRFKeyHash,
};
use cbor_encodings::{
    DnsNameEncoding, GenesisKeyDelegationEncoding, Ipv4Encoding, Ipv6Encoding,
    MoveInstantaneousRewardEncoding, MoveInstantaneousRewardsCertEncoding, MultiHostNameEncoding,
    PoolMetadataEncoding, PoolParamsEncoding, PoolRegistrationEncoding, PoolRetirementEncoding,
    SingleHostAddrEncoding, SingleHostNameEncoding, StakeDelegationEncoding,
    StakeDeregistrationEncoding, StakeRegistrationEncoding, UrlEncoding,
};
use cml_core::error::*;
use cml_core::ordered_hash_map::OrderedHashMap;
use cml_core::serialization::{LenEncoding, StringEncoding};
use std::convert::TryFrom;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub enum Certificate {
    StakeRegistration(StakeRegistration),
    StakeDeregistration(StakeDeregistration),
    StakeDelegation(StakeDelegation),
    PoolRegistration(PoolRegistration),
    PoolRetirement(PoolRetirement),
    GenesisKeyDelegation(GenesisKeyDelegation),
    MoveInstantaneousRewardsCert(MoveInstantaneousRewardsCert),
}

impl Certificate {
    pub fn new_stake_registration(stake_credential: StakeCredential) -> Self {
        Self::StakeRegistration(StakeRegistration::new(stake_credential))
    }

    pub fn new_stake_deregistration(stake_credential: StakeCredential) -> Self {
        Self::StakeDeregistration(StakeDeregistration::new(stake_credential))
    }

    pub fn new_stake_delegation(
        stake_credential: StakeCredential,
        ed25519_key_hash: Ed25519KeyHash,
    ) -> Self {
        Self::StakeDelegation(StakeDelegation::new(stake_credential, ed25519_key_hash))
    }

    pub fn new_pool_registration(pool_params: PoolParams) -> Self {
        Self::PoolRegistration(PoolRegistration::new(pool_params))
    }

    pub fn new_pool_retirement(ed25519_key_hash: Ed25519KeyHash, epoch: Epoch) -> Self {
        Self::PoolRetirement(PoolRetirement::new(ed25519_key_hash, epoch))
    }

    pub fn new_genesis_key_delegation(
        genesis_hash: GenesisHash,
        genesis_delegate_hash: GenesisDelegateHash,
        v_r_f_key_hash: VRFKeyHash,
    ) -> Self {
        Self::GenesisKeyDelegation(GenesisKeyDelegation::new(
            genesis_hash,
            genesis_delegate_hash,
            v_r_f_key_hash,
        ))
    }

    pub fn new_move_instantaneous_rewards_cert(
        move_instantaneous_reward: MoveInstantaneousReward,
    ) -> Self {
        Self::MoveInstantaneousRewardsCert(MoveInstantaneousRewardsCert::new(
            move_instantaneous_reward,
        ))
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct DnsName {
    pub inner: String,
    #[serde(skip)]
    pub encodings: Option<DnsNameEncoding>,
}

impl DnsName {
    pub fn get(&self) -> &String {
        &self.inner
    }

    pub fn new(inner: String) -> Result<Self, DeserializeError> {
        if inner.len() > 64 {
            return Err(DeserializeError::new(
                "DnsName",
                DeserializeFailure::RangeCheck {
                    found: inner.len(),
                    min: Some(0),
                    max: Some(64),
                },
            ));
        }
        Ok(Self {
            inner,
            encodings: None,
        })
    }
}

impl TryFrom<String> for DnsName {
    type Error = DeserializeError;

    fn try_from(inner: String) -> Result<Self, Self::Error> {
        DnsName::new(inner)
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct GenesisKeyDelegation {
    pub genesis_hash: GenesisHash,
    pub genesis_delegate_hash: GenesisDelegateHash,
    pub v_r_f_key_hash: VRFKeyHash,
    #[serde(skip)]
    pub encodings: Option<GenesisKeyDelegationEncoding>,
}

impl GenesisKeyDelegation {
    pub fn new(
        genesis_hash: GenesisHash,
        genesis_delegate_hash: GenesisDelegateHash,
        v_r_f_key_hash: VRFKeyHash,
    ) -> Self {
        Self {
            genesis_hash,
            genesis_delegate_hash,
            v_r_f_key_hash,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct Ipv4 {
    pub inner: Vec<u8>,
    #[serde(skip)]
    pub encodings: Option<Ipv4Encoding>,
}

impl Ipv4 {
    pub fn get(&self) -> &Vec<u8> {
        &self.inner
    }

    pub fn new(inner: Vec<u8>) -> Result<Self, DeserializeError> {
        if inner.len() != 4 {
            return Err(DeserializeError::new(
                "Ipv4",
                DeserializeFailure::RangeCheck {
                    found: inner.len(),
                    min: Some(4),
                    max: Some(4),
                },
            ));
        }
        Ok(Self {
            inner,
            encodings: None,
        })
    }
}

impl TryFrom<Vec<u8>> for Ipv4 {
    type Error = DeserializeError;

    fn try_from(inner: Vec<u8>) -> Result<Self, Self::Error> {
        Ipv4::new(inner)
    }
}

impl From<Ipv4> for Vec<u8> {
    fn from(wrapper: Ipv4) -> Self {
        wrapper.inner
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct Ipv6 {
    pub inner: Vec<u8>,
    #[serde(skip)]
    pub encodings: Option<Ipv6Encoding>,
}

impl Ipv6 {
    pub fn get(&self) -> &Vec<u8> {
        &self.inner
    }

    pub fn new(inner: Vec<u8>) -> Result<Self, DeserializeError> {
        if inner.len() != 16 {
            return Err(DeserializeError::new(
                "Ipv6",
                DeserializeFailure::RangeCheck {
                    found: inner.len(),
                    min: Some(16),
                    max: Some(16),
                },
            ));
        }
        Ok(Self {
            inner,
            encodings: None,
        })
    }
}

impl TryFrom<Vec<u8>> for Ipv6 {
    type Error = DeserializeError;

    fn try_from(inner: Vec<u8>) -> Result<Self, Self::Error> {
        Ipv6::new(inner)
    }
}

impl From<Ipv6> for Vec<u8> {
    fn from(wrapper: Ipv6) -> Self {
        wrapper.inner
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub enum MIRAction {
    ToStakeCredentials {
        to_stake_credentials: OrderedHashMap<StakeCredential, DeltaCoin>,
        #[serde(skip)]
        to_stake_credentials_encoding: LenEncoding,
    },
    ToOtherPot {
        to_other_pot: Coin,
        #[serde(skip)]
        to_other_pot_encoding: Option<cbor_event::Sz>,
    },
}

impl MIRAction {
    pub fn new_to_stake_credentials(
        to_stake_credentials: OrderedHashMap<StakeCredential, DeltaCoin>,
    ) -> Self {
        Self::ToStakeCredentials {
            to_stake_credentials,
            to_stake_credentials_encoding: LenEncoding::default(),
        }
    }

    pub fn new_to_other_pot(to_other_pot: Coin) -> Self {
        Self::ToOtherPot {
            to_other_pot,
            to_other_pot_encoding: None,
        }
    }
}

#[derive(
    Copy,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Clone,
    Debug,
    serde::Deserialize,
    serde::Serialize,
    schemars::JsonSchema,
)]
#[wasm_bindgen::prelude::wasm_bindgen]
pub enum MIRPot {
    Reserve,
    Treasury,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct MoveInstantaneousReward {
    pub pot: MIRPot,
    pub action: MIRAction,
    #[serde(skip)]
    pub encodings: Option<MoveInstantaneousRewardEncoding>,
}

impl MoveInstantaneousReward {
    pub fn new(pot: MIRPot, action: MIRAction) -> Self {
        Self {
            pot,
            action,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct MoveInstantaneousRewardsCert {
    pub move_instantaneous_reward: MoveInstantaneousReward,
    #[serde(skip)]
    pub encodings: Option<MoveInstantaneousRewardsCertEncoding>,
}

impl MoveInstantaneousRewardsCert {
    pub fn new(move_instantaneous_reward: MoveInstantaneousReward) -> Self {
        Self {
            move_instantaneous_reward,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct MultiHostName {
    pub dns_name: DnsName,
    #[serde(skip)]
    pub encodings: Option<MultiHostNameEncoding>,
}

impl MultiHostName {
    pub fn new(dns_name: DnsName) -> Self {
        Self {
            dns_name,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct PoolMetadata {
    pub url: Url,
    pub pool_metadata_hash: PoolMetadataHash,
    #[serde(skip)]
    pub encodings: Option<PoolMetadataEncoding>,
}

impl PoolMetadata {
    pub fn new(url: Url, pool_metadata_hash: PoolMetadataHash) -> Self {
        Self {
            url,
            pool_metadata_hash,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct PoolParams {
    pub operator: Ed25519KeyHash,
    pub vrf_keyhash: VRFKeyHash,
    pub pledge: Coin,
    pub cost: Coin,
    pub margin: UnitInterval,
    pub reward_account: RewardAccount,
    pub pool_owners: Vec<Ed25519KeyHash>,
    pub relays: Vec<Relay>,
    pub pool_metadata: Option<PoolMetadata>,
    #[serde(skip)]
    pub encodings: Option<PoolParamsEncoding>,
}

impl PoolParams {
    pub fn new(
        operator: Ed25519KeyHash,
        vrf_keyhash: VRFKeyHash,
        pledge: Coin,
        cost: Coin,
        margin: UnitInterval,
        reward_account: RewardAccount,
        pool_owners: Vec<Ed25519KeyHash>,
        relays: Vec<Relay>,
        pool_metadata: Option<PoolMetadata>,
    ) -> Self {
        Self {
            operator,
            vrf_keyhash,
            pledge,
            cost,
            margin,
            reward_account,
            pool_owners,
            relays,
            pool_metadata,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct PoolRegistration {
    pub pool_params: PoolParams,
    #[serde(skip)]
    pub encodings: Option<PoolRegistrationEncoding>,
}

impl PoolRegistration {
    pub fn new(pool_params: PoolParams) -> Self {
        Self {
            pool_params,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct PoolRetirement {
    pub ed25519_key_hash: Ed25519KeyHash,
    pub epoch: Epoch,
    #[serde(skip)]
    pub encodings: Option<PoolRetirementEncoding>,
}

impl PoolRetirement {
    pub fn new(ed25519_key_hash: Ed25519KeyHash, epoch: Epoch) -> Self {
        Self {
            ed25519_key_hash,
            epoch,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub enum Relay {
    SingleHostAddr(SingleHostAddr),
    SingleHostName(SingleHostName),
    MultiHostName(MultiHostName),
}

impl Relay {
    pub fn new_single_host_addr(
        port: Option<Port>,
        ipv4: Option<Ipv4>,
        ipv6: Option<Ipv6>,
    ) -> Self {
        Self::SingleHostAddr(SingleHostAddr::new(port, ipv4, ipv6))
    }

    pub fn new_single_host_name(port: Option<Port>, dns_name: DnsName) -> Self {
        Self::SingleHostName(SingleHostName::new(port, dns_name))
    }

    pub fn new_multi_host_name(dns_name: DnsName) -> Self {
        Self::MultiHostName(MultiHostName::new(dns_name))
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct SingleHostAddr {
    pub port: Option<Port>,
    pub ipv4: Option<Ipv4>,
    pub ipv6: Option<Ipv6>,
    #[serde(skip)]
    pub encodings: Option<SingleHostAddrEncoding>,
}

impl SingleHostAddr {
    pub fn new(port: Option<Port>, ipv4: Option<Ipv4>, ipv6: Option<Ipv6>) -> Self {
        Self {
            port,
            ipv4,
            ipv6,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct SingleHostName {
    pub port: Option<Port>,
    pub dns_name: DnsName,
    #[serde(skip)]
    pub encodings: Option<SingleHostNameEncoding>,
}

impl SingleHostName {
    pub fn new(port: Option<Port>, dns_name: DnsName) -> Self {
        Self {
            port,
            dns_name,
            encodings: None,
        }
    }
}

#[derive(
    Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema, derivative::Derivative,
)]
#[derivative(
    Eq,
    PartialEq,
    Ord = "feature_allow_slow_enum",
    PartialOrd = "feature_allow_slow_enum",
    Hash
)]
pub enum StakeCredential {
    PubKey {
        hash: Ed25519KeyHash,
        #[derivative(
            PartialEq = "ignore",
            Ord = "ignore",
            PartialOrd = "ignore",
            Hash = "ignore"
        )]
        #[serde(skip)]
        len_encoding: LenEncoding,
        #[derivative(
            PartialEq = "ignore",
            Ord = "ignore",
            PartialOrd = "ignore",
            Hash = "ignore"
        )]
        #[serde(skip)]
        tag_encoding: Option<cbor_event::Sz>,
        #[derivative(
            PartialEq = "ignore",
            Ord = "ignore",
            PartialOrd = "ignore",
            Hash = "ignore"
        )]
        #[serde(skip)]
        hash_encoding: StringEncoding,
    },
    Script {
        hash: ScriptHash,
        #[derivative(
            PartialEq = "ignore",
            Ord = "ignore",
            PartialOrd = "ignore",
            Hash = "ignore"
        )]
        #[serde(skip)]
        len_encoding: LenEncoding,
        #[derivative(
            PartialEq = "ignore",
            Ord = "ignore",
            PartialOrd = "ignore",
            Hash = "ignore"
        )]
        #[serde(skip)]
        tag_encoding: Option<cbor_event::Sz>,
        #[derivative(
            PartialEq = "ignore",
            Ord = "ignore",
            PartialOrd = "ignore",
            Hash = "ignore"
        )]
        #[serde(skip)]
        hash_encoding: StringEncoding,
    },
}

impl StakeCredential {
    pub fn new_pub_key(hash: Ed25519KeyHash) -> Self {
        Self::PubKey {
            hash,
            len_encoding: LenEncoding::default(),
            tag_encoding: None,
            hash_encoding: StringEncoding::default(),
        }
    }

    pub fn new_script(hash: ScriptHash) -> Self {
        Self::Script {
            hash,
            len_encoding: LenEncoding::default(),
            tag_encoding: None,
            hash_encoding: StringEncoding::default(),
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct StakeDelegation {
    pub stake_credential: StakeCredential,
    pub ed25519_key_hash: Ed25519KeyHash,
    #[serde(skip)]
    pub encodings: Option<StakeDelegationEncoding>,
}

impl StakeDelegation {
    pub fn new(stake_credential: StakeCredential, ed25519_key_hash: Ed25519KeyHash) -> Self {
        Self {
            stake_credential,
            ed25519_key_hash,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct StakeDeregistration {
    pub stake_credential: StakeCredential,
    #[serde(skip)]
    pub encodings: Option<StakeDeregistrationEncoding>,
}

impl StakeDeregistration {
    pub fn new(stake_credential: StakeCredential) -> Self {
        Self {
            stake_credential,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct StakeRegistration {
    pub stake_credential: StakeCredential,
    #[serde(skip)]
    pub encodings: Option<StakeRegistrationEncoding>,
}

impl StakeRegistration {
    pub fn new(stake_credential: StakeCredential) -> Self {
        Self {
            stake_credential,
            encodings: None,
        }
    }
}

impl From<DnsName> for String {
    fn from(wrapper: DnsName) -> Self {
        wrapper.inner
    }
}

impl From<Url> for String {
    fn from(wrapper: Url) -> Self {
        wrapper.inner
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct Url {
    pub inner: String,
    #[serde(skip)]
    pub encodings: Option<UrlEncoding>,
}

impl Url {
    pub fn get(&self) -> &String {
        &self.inner
    }

    pub fn new(inner: String) -> Result<Self, DeserializeError> {
        if inner.len() > 64 {
            return Err(DeserializeError::new(
                "Url",
                DeserializeFailure::RangeCheck {
                    found: inner.len(),
                    min: Some(0),
                    max: Some(64),
                },
            ));
        }
        Ok(Self {
            inner,
            encodings: None,
        })
    }
}

impl TryFrom<String> for Url {
    type Error = DeserializeError;

    fn try_from(inner: String) -> Result<Self, Self::Error> {
        Url::new(inner)
    }
}
