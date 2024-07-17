// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

pub mod cbor_encodings;
pub mod serialization;
pub mod utils;

use super::{Coin, Epoch, Port, SetEd25519KeyHash, UnitInterval};
use crate::address::RewardAccount;
use crate::crypto::{Ed25519KeyHash, PoolMetadataHash, ScriptHash, VRFKeyHash};
use crate::governance::Anchor;
use cbor_encodings::{
    AuthCommitteeHotCertEncoding, DNSNameEncoding, Ipv4Encoding, Ipv6Encoding,
    MultiHostNameEncoding, PoolMetadataEncoding, PoolParamsEncoding, PoolRegistrationEncoding,
    PoolRetirementEncoding, RegCertEncoding, RegDrepCertEncoding, ResignCommitteeColdCertEncoding,
    SingleHostAddrEncoding, SingleHostNameEncoding, StakeDelegationEncoding,
    StakeDeregistrationEncoding, StakeRegDelegCertEncoding, StakeRegistrationEncoding,
    StakeVoteDelegCertEncoding, StakeVoteRegDelegCertEncoding, UnregCertEncoding,
    UnregDrepCertEncoding, UpdateDrepCertEncoding, UrlEncoding, VoteDelegCertEncoding,
    VoteRegDelegCertEncoding,
};
use cml_core::error::*;

use cml_core::serialization::{LenEncoding, StringEncoding};

use std::convert::TryFrom;

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct AuthCommitteeHotCert {
    pub committee_cold_credential: CommitteeColdCredential,
    pub committee_hot_credential: CommitteeHotCredential,
    #[serde(skip)]
    pub encodings: Option<AuthCommitteeHotCertEncoding>,
}

impl AuthCommitteeHotCert {
    pub fn new(
        committee_cold_credential: CommitteeColdCredential,
        committee_hot_credential: CommitteeHotCredential,
    ) -> Self {
        Self {
            committee_cold_credential,
            committee_hot_credential,
            encodings: None,
        }
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub enum Certificate {
    /// Will be deprecated in the next era. Use RegCert instead which takes an explicit deposit amount, as that can change.
    StakeRegistration(StakeRegistration),
    /// Will be deprecated in the next era. Use UnregCert instead which takes an explicit deposit amount, as that can change.
    StakeDeregistration(StakeDeregistration),
    /// Delegate to a take pool only
    StakeDelegation(StakeDelegation),
    PoolRegistration(PoolRegistration),
    PoolRetirement(PoolRetirement),
    /// Registers a stake credential.
    RegCert(RegCert),
    /// Unregisters a stake credential.
    UnregCert(UnregCert),
    /// Delegate to a DRep for voting only
    VoteDelegCert(VoteDelegCert),
    /// Delegate to a stake pool and a DRep
    StakeVoteDelegCert(StakeVoteDelegCert),
    /// Register a stake credential and delegate to a pool in a single cert
    StakeRegDelegCert(StakeRegDelegCert),
    /// Register a stake credential and delegate to a DRep in a single cert
    VoteRegDelegCert(VoteRegDelegCert),
    /// Register a stake credential and delegate to a pool and a DRep in a single cert
    StakeVoteRegDelegCert(StakeVoteRegDelegCert),
    AuthCommitteeHotCert(AuthCommitteeHotCert),
    ResignCommitteeColdCert(ResignCommitteeColdCert),
    RegDrepCert(RegDrepCert),
    UnregDrepCert(UnregDrepCert),
    UpdateDrepCert(UpdateDrepCert),
}

impl Certificate {
    /// Will be deprecated in the next era. Use RegCert instead which takes an explicit deposit amount, as that can change.
    pub fn new_stake_registration(stake_credential: StakeCredential) -> Self {
        Self::StakeRegistration(StakeRegistration::new(stake_credential))
    }

    /// Will be deprecated in the next era. Use UnregCert instead which takes an explicit deposit amount, as that can change.
    pub fn new_stake_deregistration(stake_credential: StakeCredential) -> Self {
        Self::StakeDeregistration(StakeDeregistration::new(stake_credential))
    }

    /// Delegate to a take pool only
    pub fn new_stake_delegation(stake_credential: StakeCredential, pool: Ed25519KeyHash) -> Self {
        Self::StakeDelegation(StakeDelegation::new(stake_credential, pool))
    }

    pub fn new_pool_registration(pool_params: PoolParams) -> Self {
        Self::PoolRegistration(PoolRegistration::new(pool_params))
    }

    pub fn new_pool_retirement(pool: Ed25519KeyHash, epoch: Epoch) -> Self {
        Self::PoolRetirement(PoolRetirement::new(pool, epoch))
    }

    /// Registers a stake credential.
    pub fn new_reg_cert(stake_credential: StakeCredential, deposit: Coin) -> Self {
        Self::RegCert(RegCert::new(stake_credential, deposit))
    }

    /// Unregisters a stake credential.
    pub fn new_unreg_cert(stake_credential: StakeCredential, deposit: Coin) -> Self {
        Self::UnregCert(UnregCert::new(stake_credential, deposit))
    }

    /// Delegate to a DRep for voting only
    pub fn new_vote_deleg_cert(stake_credential: StakeCredential, d_rep: DRep) -> Self {
        Self::VoteDelegCert(VoteDelegCert::new(stake_credential, d_rep))
    }

    /// Delegate to a stake pool and a DRep
    pub fn new_stake_vote_deleg_cert(
        stake_credential: StakeCredential,
        pool: Ed25519KeyHash,
        d_rep: DRep,
    ) -> Self {
        Self::StakeVoteDelegCert(StakeVoteDelegCert::new(stake_credential, pool, d_rep))
    }

    /// Register a stake credential and delegate to a pool in a single cert
    pub fn new_stake_reg_deleg_cert(
        stake_credential: StakeCredential,
        pool: Ed25519KeyHash,
        deposit: Coin,
    ) -> Self {
        Self::StakeRegDelegCert(StakeRegDelegCert::new(stake_credential, pool, deposit))
    }

    /// Register a stake credential and delegate to a DRep in a single cert
    pub fn new_vote_reg_deleg_cert(
        stake_credential: StakeCredential,
        d_rep: DRep,
        deposit: Coin,
    ) -> Self {
        Self::VoteRegDelegCert(VoteRegDelegCert::new(stake_credential, d_rep, deposit))
    }

    /// Register a stake credential and delegate to a pool and a DRep in a single cert
    pub fn new_stake_vote_reg_deleg_cert(
        stake_credential: StakeCredential,
        pool: Ed25519KeyHash,
        d_rep: DRep,
        deposit: Coin,
    ) -> Self {
        Self::StakeVoteRegDelegCert(StakeVoteRegDelegCert::new(
            stake_credential,
            pool,
            d_rep,
            deposit,
        ))
    }

    pub fn new_auth_committee_hot_cert(
        committee_cold_credential: CommitteeColdCredential,
        committee_hot_credential: CommitteeHotCredential,
    ) -> Self {
        Self::AuthCommitteeHotCert(AuthCommitteeHotCert::new(
            committee_cold_credential,
            committee_hot_credential,
        ))
    }

    pub fn new_resign_committee_cold_cert(
        committee_cold_credential: CommitteeColdCredential,
        anchor: Option<Anchor>,
    ) -> Self {
        Self::ResignCommitteeColdCert(ResignCommitteeColdCert::new(
            committee_cold_credential,
            anchor,
        ))
    }

    pub fn new_reg_drep_cert(
        drep_credential: DrepCredential,
        deposit: Coin,
        anchor: Option<Anchor>,
    ) -> Self {
        Self::RegDrepCert(RegDrepCert::new(drep_credential, deposit, anchor))
    }

    pub fn new_unreg_drep_cert(drep_credential: DrepCredential, deposit: Coin) -> Self {
        Self::UnregDrepCert(UnregDrepCert::new(drep_credential, deposit))
    }

    pub fn new_update_drep_cert(drep_credential: DrepCredential, anchor: Option<Anchor>) -> Self {
        Self::UpdateDrepCert(UpdateDrepCert::new(drep_credential, anchor))
    }
}

pub type CommitteeColdCredential = Credential;

pub type CommitteeHotCredential = Credential;

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
pub enum Credential {
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

impl Credential {
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

#[derive(Clone, Debug)]
pub struct DNSName {
    pub inner: String,
    pub encodings: Option<DNSNameEncoding>,
}

impl DNSName {
    pub fn get(&self) -> &String {
        &self.inner
    }

    pub fn new(inner: String) -> Result<Self, DeserializeError> {
        if inner.len() > 128 {
            return Err(DeserializeError::new(
                "DNSName",
                DeserializeFailure::RangeCheck {
                    found: inner.len() as isize,
                    min: Some(0),
                    max: Some(128),
                },
            ));
        }
        Ok(Self {
            inner,
            encodings: None,
        })
    }
}

impl TryFrom<String> for DNSName {
    type Error = DeserializeError;

    fn try_from(inner: String) -> Result<Self, Self::Error> {
        DNSName::new(inner)
    }
}

impl serde::Serialize for DNSName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.inner.serialize(serializer)
    }
}

impl<'de> serde::de::Deserialize<'de> for DNSName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let inner = <String as serde::de::Deserialize>::deserialize(deserializer)?;
        Self::new(inner.clone()).map_err(|_e| {
            serde::de::Error::invalid_value(serde::de::Unexpected::Str(&inner), &"invalid DNSName")
        })
    }
}

impl schemars::JsonSchema for DNSName {
    fn schema_name() -> String {
        String::from("DNSName")
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        String::json_schema(gen)
    }

    fn is_referenceable() -> bool {
        String::is_referenceable()
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub enum DRep {
    Key {
        pool: Ed25519KeyHash,
        #[serde(skip)]
        len_encoding: LenEncoding,
        #[serde(skip)]
        index_0_encoding: Option<cbor_event::Sz>,
        #[serde(skip)]
        pool_encoding: StringEncoding,
    },
    Script {
        script_hash: ScriptHash,
        #[serde(skip)]
        len_encoding: LenEncoding,
        #[serde(skip)]
        index_0_encoding: Option<cbor_event::Sz>,
        #[serde(skip)]
        script_hash_encoding: StringEncoding,
    },
    AlwaysAbstain {
        #[serde(skip)]
        always_abstain_encoding: Option<cbor_event::Sz>,
        #[serde(skip)]
        len_encoding: LenEncoding,
    },
    AlwaysNoConfidence {
        #[serde(skip)]
        always_no_confidence_encoding: Option<cbor_event::Sz>,
        #[serde(skip)]
        len_encoding: LenEncoding,
    },
}

impl DRep {
    pub fn new_key(pool: Ed25519KeyHash) -> Self {
        Self::Key {
            pool,
            len_encoding: LenEncoding::default(),
            index_0_encoding: None,
            pool_encoding: StringEncoding::default(),
        }
    }

    pub fn new_script(script_hash: ScriptHash) -> Self {
        Self::Script {
            script_hash,
            len_encoding: LenEncoding::default(),
            index_0_encoding: None,
            script_hash_encoding: StringEncoding::default(),
        }
    }

    pub fn new_always_abstain() -> Self {
        Self::AlwaysAbstain {
            always_abstain_encoding: None,
            len_encoding: LenEncoding::default(),
        }
    }

    pub fn new_always_no_confidence() -> Self {
        Self::AlwaysNoConfidence {
            always_no_confidence_encoding: None,
            len_encoding: LenEncoding::default(),
        }
    }
}

pub type DrepCredential = Credential;

#[derive(Clone, Debug)]
pub struct Ipv4 {
    pub inner: Vec<u8>,
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
                    found: inner.len() as isize,
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

#[derive(Clone, Debug)]
pub struct Ipv6 {
    pub inner: Vec<u8>,
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
                    found: inner.len() as isize,
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
pub struct MultiHostName {
    /// A SRV DNS record
    pub dns_name: DNSName,
    #[serde(skip)]
    pub encodings: Option<MultiHostNameEncoding>,
}

impl MultiHostName {
    /// * `dns_name` - A SRV DNS record
    pub fn new(dns_name: DNSName) -> Self {
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
    pub pool_owners: SetEd25519KeyHash,
    pub relays: Vec<Relay>,
    pub pool_metadata: Option<PoolMetadata>,
    #[serde(skip)]
    pub encodings: Option<PoolParamsEncoding>,
}

impl PoolParams {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        operator: Ed25519KeyHash,
        vrf_keyhash: VRFKeyHash,
        pledge: Coin,
        cost: Coin,
        margin: UnitInterval,
        reward_account: RewardAccount,
        pool_owners: SetEd25519KeyHash,
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
    pub pool: Ed25519KeyHash,
    pub epoch: Epoch,
    #[serde(skip)]
    pub encodings: Option<PoolRetirementEncoding>,
}

impl PoolRetirement {
    pub fn new(pool: Ed25519KeyHash, epoch: Epoch) -> Self {
        Self {
            pool,
            epoch,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct RegCert {
    pub stake_credential: StakeCredential,
    pub deposit: Coin,
    #[serde(skip)]
    pub encodings: Option<RegCertEncoding>,
}

impl RegCert {
    pub fn new(stake_credential: StakeCredential, deposit: Coin) -> Self {
        Self {
            stake_credential,
            deposit,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct RegDrepCert {
    pub drep_credential: DrepCredential,
    pub deposit: Coin,
    pub anchor: Option<Anchor>,
    #[serde(skip)]
    pub encodings: Option<RegDrepCertEncoding>,
}

impl RegDrepCert {
    pub fn new(drep_credential: DrepCredential, deposit: Coin, anchor: Option<Anchor>) -> Self {
        Self {
            drep_credential,
            deposit,
            anchor,
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

    pub fn new_single_host_name(port: Option<Port>, dns_name: DNSName) -> Self {
        Self::SingleHostName(SingleHostName::new(port, dns_name))
    }

    pub fn new_multi_host_name(dns_name: DNSName) -> Self {
        Self::MultiHostName(MultiHostName::new(dns_name))
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ResignCommitteeColdCert {
    pub committee_cold_credential: CommitteeColdCredential,
    pub anchor: Option<Anchor>,
    #[serde(skip)]
    pub encodings: Option<ResignCommitteeColdCertEncoding>,
}

impl ResignCommitteeColdCert {
    pub fn new(committee_cold_credential: CommitteeColdCredential, anchor: Option<Anchor>) -> Self {
        Self {
            committee_cold_credential,
            anchor,
            encodings: None,
        }
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
    /// An A or AAAA DNS record
    pub dns_name: DNSName,
    #[serde(skip)]
    pub encodings: Option<SingleHostNameEncoding>,
}

impl SingleHostName {
    /// * `dns_name` - An A or AAAA DNS record
    pub fn new(port: Option<Port>, dns_name: DNSName) -> Self {
        Self {
            port,
            dns_name,
            encodings: None,
        }
    }
}

pub type StakeCredential = Credential;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct StakeDelegation {
    pub stake_credential: StakeCredential,
    pub pool: Ed25519KeyHash,
    #[serde(skip)]
    pub encodings: Option<StakeDelegationEncoding>,
}

impl StakeDelegation {
    pub fn new(stake_credential: StakeCredential, pool: Ed25519KeyHash) -> Self {
        Self {
            stake_credential,
            pool,
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
pub struct StakeRegDelegCert {
    pub stake_credential: StakeCredential,
    pub pool: Ed25519KeyHash,
    pub deposit: Coin,
    #[serde(skip)]
    pub encodings: Option<StakeRegDelegCertEncoding>,
}

impl StakeRegDelegCert {
    pub fn new(stake_credential: StakeCredential, pool: Ed25519KeyHash, deposit: Coin) -> Self {
        Self {
            stake_credential,
            pool,
            deposit,
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

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct StakeVoteDelegCert {
    pub stake_credential: StakeCredential,
    pub pool: Ed25519KeyHash,
    pub d_rep: DRep,
    #[serde(skip)]
    pub encodings: Option<StakeVoteDelegCertEncoding>,
}

impl StakeVoteDelegCert {
    pub fn new(stake_credential: StakeCredential, pool: Ed25519KeyHash, d_rep: DRep) -> Self {
        Self {
            stake_credential,
            pool,
            d_rep,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct StakeVoteRegDelegCert {
    pub stake_credential: StakeCredential,
    pub pool: Ed25519KeyHash,
    pub d_rep: DRep,
    pub deposit: Coin,
    #[serde(skip)]
    pub encodings: Option<StakeVoteRegDelegCertEncoding>,
}

impl StakeVoteRegDelegCert {
    pub fn new(
        stake_credential: StakeCredential,
        pool: Ed25519KeyHash,
        d_rep: DRep,
        deposit: Coin,
    ) -> Self {
        Self {
            stake_credential,
            pool,
            d_rep,
            deposit,
            encodings: None,
        }
    }
}

impl From<DNSName> for String {
    fn from(wrapper: DNSName) -> Self {
        wrapper.inner
    }
}

impl From<Url> for String {
    fn from(wrapper: Url) -> Self {
        wrapper.inner
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct UnregCert {
    pub stake_credential: StakeCredential,
    pub deposit: Coin,
    #[serde(skip)]
    pub encodings: Option<UnregCertEncoding>,
}

impl UnregCert {
    pub fn new(stake_credential: StakeCredential, deposit: Coin) -> Self {
        Self {
            stake_credential,
            deposit,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct UnregDrepCert {
    pub drep_credential: DrepCredential,
    pub deposit: Coin,
    #[serde(skip)]
    pub encodings: Option<UnregDrepCertEncoding>,
}

impl UnregDrepCert {
    pub fn new(drep_credential: DrepCredential, deposit: Coin) -> Self {
        Self {
            drep_credential,
            deposit,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct UpdateDrepCert {
    pub drep_credential: DrepCredential,
    pub anchor: Option<Anchor>,
    #[serde(skip)]
    pub encodings: Option<UpdateDrepCertEncoding>,
}

impl UpdateDrepCert {
    pub fn new(drep_credential: DrepCredential, anchor: Option<Anchor>) -> Self {
        Self {
            drep_credential,
            anchor,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Url {
    pub inner: String,
    pub encodings: Option<UrlEncoding>,
}

impl Url {
    pub fn get(&self) -> &String {
        &self.inner
    }

    pub fn new(inner: String) -> Result<Self, DeserializeError> {
        if inner.len() > 128 {
            return Err(DeserializeError::new(
                "Url",
                DeserializeFailure::RangeCheck {
                    found: inner.len() as isize,
                    min: Some(0),
                    max: Some(128),
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

impl serde::Serialize for Url {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.inner.serialize(serializer)
    }
}

impl<'de> serde::de::Deserialize<'de> for Url {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let inner = <String as serde::de::Deserialize>::deserialize(deserializer)?;
        Self::new(inner.clone()).map_err(|_e| {
            serde::de::Error::invalid_value(serde::de::Unexpected::Str(&inner), &"invalid Url")
        })
    }
}

impl schemars::JsonSchema for Url {
    fn schema_name() -> String {
        String::from("Url")
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        String::json_schema(gen)
    }

    fn is_referenceable() -> bool {
        String::is_referenceable()
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct VoteDelegCert {
    pub stake_credential: StakeCredential,
    pub d_rep: DRep,
    #[serde(skip)]
    pub encodings: Option<VoteDelegCertEncoding>,
}

impl VoteDelegCert {
    pub fn new(stake_credential: StakeCredential, d_rep: DRep) -> Self {
        Self {
            stake_credential,
            d_rep,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct VoteRegDelegCert {
    pub stake_credential: StakeCredential,
    pub d_rep: DRep,
    pub deposit: Coin,
    #[serde(skip)]
    pub encodings: Option<VoteRegDelegCertEncoding>,
}

impl VoteRegDelegCert {
    pub fn new(stake_credential: StakeCredential, d_rep: DRep, deposit: Coin) -> Self {
        Self {
            stake_credential,
            d_rep,
            deposit,
            encodings: None,
        }
    }
}
