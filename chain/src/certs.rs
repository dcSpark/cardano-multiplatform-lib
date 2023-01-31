use super::*;

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
        pool_keyhash: Ed25519KeyHash,
    ) -> Self {
        Self::StakeDelegation(StakeDelegation::new(stake_credential, pool_keyhash))
    }

    pub fn new_pool_registration(pool_params: PoolParams) -> Self {
        Self::PoolRegistration(PoolRegistration::new(pool_params))
    }

    pub fn new_pool_retirement(pool_keyhash: Ed25519KeyHash, epoch: Epoch) -> Self {
        Self::PoolRetirement(PoolRetirement::new(pool_keyhash, epoch))
    }

    pub fn new_genesis_key_delegation(
        genesishash: GenesisHash,
        genesis_delegate_hash: GenesisDelegateHash,
        vrf_keyhash: VRFKeyHash,
    ) -> Self {
        Self::GenesisKeyDelegation(GenesisKeyDelegation::new(
            genesishash,
            genesis_delegate_hash,
            vrf_keyhash,
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

impl From<DnsName> for String {
    fn from(wrapper: DnsName) -> Self {
        wrapper.inner
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct GenesisKeyDelegation {
    pub genesishash: GenesisHash,
    pub genesis_delegate_hash: GenesisDelegateHash,
    pub vrf_keyhash: VRFKeyHash,
    #[serde(skip)]
    pub encodings: Option<GenesisKeyDelegationEncoding>,
}

impl GenesisKeyDelegation {
    pub fn new(
        genesishash: GenesisHash,
        genesis_delegate_hash: GenesisDelegateHash,
        vrf_keyhash: VRFKeyHash,
    ) -> Self {
        Self {
            genesishash,
            genesis_delegate_hash,
            vrf_keyhash,
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
pub struct MoveInstantaneousReward {
    pub index_0: I0OrI1,
    pub index_1: OrderedHashMap<StakeCredential, DeltaCoin>,
    pub coin: Coin,
    #[serde(skip)]
    pub encodings: Option<MoveInstantaneousRewardEncoding>,
}

impl MoveInstantaneousReward {
    pub fn new(
        index_0: I0OrI1,
        index_1: OrderedHashMap<StakeCredential, DeltaCoin>,
        coin: Coin,
    ) -> Self {
        Self {
            index_0,
            index_1,
            coin,
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
    pub pool_owners: Vec<AddrKeyhash>,
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
        pool_owners: Vec<AddrKeyhash>,
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
    pub pool_keyhash: Ed25519KeyHash,
    pub epoch: Epoch,
    #[serde(skip)]
    pub encodings: Option<PoolRetirementEncoding>,
}

impl PoolRetirement {
    pub fn new(pool_keyhash: Ed25519KeyHash, epoch: Epoch) -> Self {
        Self {
            pool_keyhash,
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

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct StakeDelegation {
    pub stake_credential: StakeCredential,
    pub pool_keyhash: Ed25519KeyHash,
    #[serde(skip)]
    pub encodings: Option<StakeDelegationEncoding>,
}

impl StakeDelegation {
    pub fn new(stake_credential: StakeCredential, pool_keyhash: Ed25519KeyHash) -> Self {
        Self {
            stake_credential,
            pool_keyhash,
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

impl From<Url> for String {
    fn from(wrapper: Url) -> Self {
        wrapper.inner
    }
}
