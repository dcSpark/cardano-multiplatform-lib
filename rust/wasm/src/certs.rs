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

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct Certificate(pub(crate) core::Certificate);

#[wasm_bindgen]

impl Certificate {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<Certificate, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<Certificate, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn new_stake_registration(stake_credential: &StakeCredential) -> Self {
        Self(core::Certificate::new_stake_registration(stake_credential.clone().into()))
    }

    pub fn new_stake_deregistration(stake_credential: &StakeCredential) -> Self {
        Self(core::Certificate::new_stake_deregistration(stake_credential.clone().into()))
    }

    pub fn new_stake_delegation(stake_credential: &StakeCredential, pool_keyhash: &PoolKeyhash) -> Self {
        Self(core::Certificate::new_stake_delegation(stake_credential.clone().into(), pool_keyhash.clone().into()))
    }

    pub fn new_pool_registration(pool_params: &PoolParams) -> Self {
        Self(core::Certificate::new_pool_registration(pool_params.clone().into()))
    }

    pub fn new_pool_retirement(pool_keyhash: &PoolKeyhash, epoch: Epoch) -> Self {
        Self(core::Certificate::new_pool_retirement(pool_keyhash.clone().into(), epoch))
    }

    pub fn new_genesis_key_delegation(genesishash: &Genesishash, genesis_delegate_hash: &GenesisDelegateHash, vrf_keyhash: &VrfKeyhash) -> Self {
        Self(core::Certificate::new_genesis_key_delegation(genesishash.clone().into(), genesis_delegate_hash.clone().into(), vrf_keyhash.clone().into()))
    }

    pub fn new_move_instantaneous_rewards_cert(move_instantaneous_reward: &MoveInstantaneousReward) -> Self {
        Self(core::Certificate::new_move_instantaneous_rewards_cert(move_instantaneous_reward.clone().into()))
    }

    pub fn kind(&self) -> CertificateKind {
        match &self.0 {
            core::Certificate::StakeRegistration(_) => CertificateKind::StakeRegistration,
            core::Certificate::StakeDeregistration(_) => CertificateKind::StakeDeregistration,
            core::Certificate::StakeDelegation(_) => CertificateKind::StakeDelegation,
            core::Certificate::PoolRegistration(_) => CertificateKind::PoolRegistration,
            core::Certificate::PoolRetirement(_) => CertificateKind::PoolRetirement,
            core::Certificate::GenesisKeyDelegation(_) => CertificateKind::GenesisKeyDelegation,
            core::Certificate::MoveInstantaneousRewardsCert(_) => CertificateKind::MoveInstantaneousRewardsCert,
        }
    }

    pub fn as_stake_registration(&self) -> Option<StakeRegistration> {
        match &self.0 {
            core::Certificate::StakeRegistration(stake_registration) => Some(stake_registration.clone().into()),
            _ => None,
        }
    }

    pub fn as_stake_deregistration(&self) -> Option<StakeDeregistration> {
        match &self.0 {
            core::Certificate::StakeDeregistration(stake_deregistration) => Some(stake_deregistration.clone().into()),
            _ => None,
        }
    }

    pub fn as_stake_delegation(&self) -> Option<StakeDelegation> {
        match &self.0 {
            core::Certificate::StakeDelegation(stake_delegation) => Some(stake_delegation.clone().into()),
            _ => None,
        }
    }

    pub fn as_pool_registration(&self) -> Option<PoolRegistration> {
        match &self.0 {
            core::Certificate::PoolRegistration(pool_registration) => Some(pool_registration.clone().into()),
            _ => None,
        }
    }

    pub fn as_pool_retirement(&self) -> Option<PoolRetirement> {
        match &self.0 {
            core::Certificate::PoolRetirement(pool_retirement) => Some(pool_retirement.clone().into()),
            _ => None,
        }
    }

    pub fn as_genesis_key_delegation(&self) -> Option<GenesisKeyDelegation> {
        match &self.0 {
            core::Certificate::GenesisKeyDelegation(genesis_key_delegation) => Some(genesis_key_delegation.clone().into()),
            _ => None,
        }
    }

    pub fn as_move_instantaneous_rewards_cert(&self) -> Option<MoveInstantaneousRewardsCert> {
        match &self.0 {
            core::Certificate::MoveInstantaneousRewardsCert(move_instantaneous_rewards_cert) => Some(move_instantaneous_rewards_cert.clone().into()),
            _ => None,
        }
    }
}

impl From<core::Certificate> for Certificate {
    fn from(native: core::Certificate) -> Self {
        Self(native)
    }
}

impl From<Certificate> for core::Certificate {
    fn from(wasm: Certificate) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct DnsName(pub(crate) core::DnsName);

#[wasm_bindgen]

impl DnsName {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<DnsName, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<DnsName, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn get(&self) -> String {
        self.0.get().clone().clone()
    }
}

impl From<core::DnsName> for DnsName {
    fn from(native: core::DnsName) -> Self {
        Self(native)
    }
}

impl From<DnsName> for core::DnsName {
    fn from(wasm: DnsName) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct GenesisKeyDelegation(pub(crate) core::GenesisKeyDelegation);

#[wasm_bindgen]

impl GenesisKeyDelegation {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<GenesisKeyDelegation, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<GenesisKeyDelegation, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn genesishash(&self) -> Genesishash {
        self.0.genesishash.clone().into()
    }

    pub fn genesis_delegate_hash(&self) -> GenesisDelegateHash {
        self.0.genesis_delegate_hash.clone().into()
    }

    pub fn vrf_keyhash(&self) -> VrfKeyhash {
        self.0.vrf_keyhash.clone().into()
    }

    pub fn new(genesishash: &Genesishash, genesis_delegate_hash: &GenesisDelegateHash, vrf_keyhash: &VrfKeyhash) -> Self {
        Self(core::GenesisKeyDelegation::new(genesishash.clone().into(), genesis_delegate_hash.clone().into(), vrf_keyhash.clone().into()))
    }
}

impl From<core::GenesisKeyDelegation> for GenesisKeyDelegation {
    fn from(native: core::GenesisKeyDelegation) -> Self {
        Self(native)
    }
}

impl From<GenesisKeyDelegation> for core::GenesisKeyDelegation {
    fn from(wasm: GenesisKeyDelegation) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct Ipv4(pub(crate) core::Ipv4);

#[wasm_bindgen]

impl Ipv4 {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<Ipv4, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<Ipv4, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn get(&self) -> Vec<u8> {
        self.0.get().clone().clone()
    }
}

impl From<core::Ipv4> for Ipv4 {
    fn from(native: core::Ipv4) -> Self {
        Self(native)
    }
}

impl From<Ipv4> for core::Ipv4 {
    fn from(wasm: Ipv4) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct Ipv6(pub(crate) core::Ipv6);

#[wasm_bindgen]

impl Ipv6 {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<Ipv6, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<Ipv6, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn get(&self) -> Vec<u8> {
        self.0.get().clone().clone()
    }
}

impl From<core::Ipv6> for Ipv6 {
    fn from(native: core::Ipv6) -> Self {
        Self(native)
    }
}

impl From<Ipv6> for core::Ipv6 {
    fn from(wasm: Ipv6) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct MoveInstantaneousReward(pub(crate) core::MoveInstantaneousReward);

#[wasm_bindgen]

impl MoveInstantaneousReward {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<MoveInstantaneousReward, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<MoveInstantaneousReward, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn index_0(&self) -> I0OrI1 {
        self.0.index_0.clone().into()
    }

    pub fn index_1(&self) -> MapStakeCredentialToDeltaCoin {
        self.0.index_1.clone().into()
    }

    pub fn coin(&self) -> Coin {
        self.0.coin
    }

    pub fn new(index_0: &I0OrI1, index_1: &MapStakeCredentialToDeltaCoin, coin: Coin) -> Self {
        Self(core::MoveInstantaneousReward::new(index_0.clone().into(), index_1.clone().into(), coin))
    }
}

impl From<core::MoveInstantaneousReward> for MoveInstantaneousReward {
    fn from(native: core::MoveInstantaneousReward) -> Self {
        Self(native)
    }
}

impl From<MoveInstantaneousReward> for core::MoveInstantaneousReward {
    fn from(wasm: MoveInstantaneousReward) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct MoveInstantaneousRewardsCert(pub(crate) core::MoveInstantaneousRewardsCert);

#[wasm_bindgen]

impl MoveInstantaneousRewardsCert {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<MoveInstantaneousRewardsCert, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<MoveInstantaneousRewardsCert, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn move_instantaneous_reward(&self) -> MoveInstantaneousReward {
        self.0.move_instantaneous_reward.clone().into()
    }

    pub fn new(move_instantaneous_reward: &MoveInstantaneousReward) -> Self {
        Self(core::MoveInstantaneousRewardsCert::new(move_instantaneous_reward.clone().into()))
    }
}

impl From<core::MoveInstantaneousRewardsCert> for MoveInstantaneousRewardsCert {
    fn from(native: core::MoveInstantaneousRewardsCert) -> Self {
        Self(native)
    }
}

impl From<MoveInstantaneousRewardsCert> for core::MoveInstantaneousRewardsCert {
    fn from(wasm: MoveInstantaneousRewardsCert) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct MultiHostName(pub(crate) core::MultiHostName);

#[wasm_bindgen]

impl MultiHostName {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<MultiHostName, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<MultiHostName, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn dns_name(&self) -> DnsName {
        self.0.dns_name.clone().into()
    }

    pub fn new(dns_name: &DnsName) -> Self {
        Self(core::MultiHostName::new(dns_name.clone().into()))
    }
}

impl From<core::MultiHostName> for MultiHostName {
    fn from(native: core::MultiHostName) -> Self {
        Self(native)
    }
}

impl From<MultiHostName> for core::MultiHostName {
    fn from(wasm: MultiHostName) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct PoolMetadata(pub(crate) core::PoolMetadata);

#[wasm_bindgen]

impl PoolMetadata {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<PoolMetadata, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<PoolMetadata, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn url(&self) -> Url {
        self.0.url.clone().into()
    }

    pub fn pool_metadata_hash(&self) -> PoolMetadataHash {
        self.0.pool_metadata_hash.clone().into()
    }

    pub fn new(url: &Url, pool_metadata_hash: &PoolMetadataHash) -> Self {
        Self(core::PoolMetadata::new(url.clone().into(), pool_metadata_hash.clone().into()))
    }
}

impl From<core::PoolMetadata> for PoolMetadata {
    fn from(native: core::PoolMetadata) -> Self {
        Self(native)
    }
}

impl From<PoolMetadata> for core::PoolMetadata {
    fn from(wasm: PoolMetadata) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct PoolParams(pub(crate) core::PoolParams);

#[wasm_bindgen]

impl PoolParams {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<PoolParams, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<PoolParams, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn operator(&self) -> PoolKeyhash {
        self.0.operator.clone().into()
    }

    pub fn vrf_keyhash(&self) -> VrfKeyhash {
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

    pub fn pool_owners(&self) -> AddrKeyhashs {
        self.0.pool_owners.clone().into()
    }

    pub fn relays(&self) -> Relays {
        self.0.relays.clone().into()
    }

    pub fn pool_metadata(&self) -> Option<PoolMetadata> {
        self.0.pool_metadata.clone().map(std::convert::Into::into)
    }

    pub fn new(operator: &PoolKeyhash, vrf_keyhash: &VrfKeyhash, pledge: Coin, cost: Coin, margin: &UnitInterval, reward_account: &RewardAccount, pool_owners: &AddrKeyhashs, relays: &Relays, pool_metadata: Option<PoolMetadata>) -> Self {
        Self(core::PoolParams::new(operator.clone().into(), vrf_keyhash.clone().into(), pledge, cost, margin.clone().into(), reward_account.clone().into(), pool_owners.clone().into(), relays.clone().into(), pool_metadata.map(Into::into)))
    }
}

impl From<core::PoolParams> for PoolParams {
    fn from(native: core::PoolParams) -> Self {
        Self(native)
    }
}

impl From<PoolParams> for core::PoolParams {
    fn from(wasm: PoolParams) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct PoolRegistration(pub(crate) core::PoolRegistration);

#[wasm_bindgen]

impl PoolRegistration {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<PoolRegistration, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<PoolRegistration, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn pool_params(&self) -> PoolParams {
        self.0.pool_params.clone().into()
    }

    pub fn new(pool_params: &PoolParams) -> Self {
        Self(core::PoolRegistration::new(pool_params.clone().into()))
    }
}

impl From<core::PoolRegistration> for PoolRegistration {
    fn from(native: core::PoolRegistration) -> Self {
        Self(native)
    }
}

impl From<PoolRegistration> for core::PoolRegistration {
    fn from(wasm: PoolRegistration) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct PoolRetirement(pub(crate) core::PoolRetirement);

#[wasm_bindgen]

impl PoolRetirement {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<PoolRetirement, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<PoolRetirement, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn pool_keyhash(&self) -> PoolKeyhash {
        self.0.pool_keyhash.clone().into()
    }

    pub fn epoch(&self) -> Epoch {
        self.0.epoch
    }

    pub fn new(pool_keyhash: &PoolKeyhash, epoch: Epoch) -> Self {
        Self(core::PoolRetirement::new(pool_keyhash.clone().into(), epoch))
    }
}

impl From<core::PoolRetirement> for PoolRetirement {
    fn from(native: core::PoolRetirement) -> Self {
        Self(native)
    }
}

impl From<PoolRetirement> for core::PoolRetirement {
    fn from(wasm: PoolRetirement) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

pub enum RelayKind {
    SingleHostAddr,
    SingleHostName,
    MultiHostName,
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct Relay(pub(crate) core::Relay);

#[wasm_bindgen]

impl Relay {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<Relay, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<Relay, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn new_single_host_addr(port: Option<Port>, ipv4: Option<Ipv4>, ipv6: Option<Ipv6>) -> Self {
        Self(core::Relay::new_single_host_addr(port, ipv4.map(Into::into), ipv6.map(Into::into)))
    }

    pub fn new_single_host_name(port: Option<Port>, dns_name: &DnsName) -> Self {
        Self(core::Relay::new_single_host_name(port, dns_name.clone().into()))
    }

    pub fn new_multi_host_name(dns_name: &DnsName) -> Self {
        Self(core::Relay::new_multi_host_name(dns_name.clone().into()))
    }

    pub fn kind(&self) -> RelayKind {
        match &self.0 {
            core::Relay::SingleHostAddr(_) => RelayKind::SingleHostAddr,
            core::Relay::SingleHostName(_) => RelayKind::SingleHostName,
            core::Relay::MultiHostName(_) => RelayKind::MultiHostName,
        }
    }

    pub fn as_single_host_addr(&self) -> Option<SingleHostAddr> {
        match &self.0 {
            core::Relay::SingleHostAddr(single_host_addr) => Some(single_host_addr.clone().into()),
            _ => None,
        }
    }

    pub fn as_single_host_name(&self) -> Option<SingleHostName> {
        match &self.0 {
            core::Relay::SingleHostName(single_host_name) => Some(single_host_name.clone().into()),
            _ => None,
        }
    }

    pub fn as_multi_host_name(&self) -> Option<MultiHostName> {
        match &self.0 {
            core::Relay::MultiHostName(multi_host_name) => Some(multi_host_name.clone().into()),
            _ => None,
        }
    }
}

impl From<core::Relay> for Relay {
    fn from(native: core::Relay) -> Self {
        Self(native)
    }
}

impl From<Relay> for core::Relay {
    fn from(wasm: Relay) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct SingleHostAddr(pub(crate) core::SingleHostAddr);

#[wasm_bindgen]

impl SingleHostAddr {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<SingleHostAddr, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<SingleHostAddr, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
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
        Self(core::SingleHostAddr::new(port, ipv4.map(Into::into), ipv6.map(Into::into)))
    }
}

impl From<core::SingleHostAddr> for SingleHostAddr {
    fn from(native: core::SingleHostAddr) -> Self {
        Self(native)
    }
}

impl From<SingleHostAddr> for core::SingleHostAddr {
    fn from(wasm: SingleHostAddr) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct SingleHostName(pub(crate) core::SingleHostName);

#[wasm_bindgen]

impl SingleHostName {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<SingleHostName, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<SingleHostName, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn port(&self) -> Option<Port> {
        self.0.port
    }

    pub fn dns_name(&self) -> DnsName {
        self.0.dns_name.clone().into()
    }

    pub fn new(port: Option<Port>, dns_name: &DnsName) -> Self {
        Self(core::SingleHostName::new(port, dns_name.clone().into()))
    }
}

impl From<core::SingleHostName> for SingleHostName {
    fn from(native: core::SingleHostName) -> Self {
        Self(native)
    }
}

impl From<SingleHostName> for core::SingleHostName {
    fn from(wasm: SingleHostName) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

pub enum StakeCredentialKind {
    StakeCredential0,
    StakeCredential1,
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct StakeCredential(pub(crate) core::StakeCredential);

#[wasm_bindgen]

impl StakeCredential {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<StakeCredential, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<StakeCredential, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn new_stake_credential0(addr_keyhash: &AddrKeyhash) -> Self {
        Self(core::StakeCredential::new_stake_credential0(addr_keyhash.clone().into()))
    }

    pub fn new_stake_credential1(scripthash: &Scripthash) -> Self {
        Self(core::StakeCredential::new_stake_credential1(scripthash.clone().into()))
    }

    pub fn kind(&self) -> StakeCredentialKind {
        match &self.0 {
            core::StakeCredential::StakeCredential0(_) => StakeCredentialKind::StakeCredential0,
            core::StakeCredential::StakeCredential1(_) => StakeCredentialKind::StakeCredential1,
        }
    }

    pub fn as_stake_credential0(&self) -> Option<StakeCredential0> {
        match &self.0 {
            core::StakeCredential::StakeCredential0(stake_credential0) => Some(stake_credential0.clone().into()),
            _ => None,
        }
    }

    pub fn as_stake_credential1(&self) -> Option<StakeCredential1> {
        match &self.0 {
            core::StakeCredential::StakeCredential1(stake_credential1) => Some(stake_credential1.clone().into()),
            _ => None,
        }
    }
}

impl From<core::StakeCredential> for StakeCredential {
    fn from(native: core::StakeCredential) -> Self {
        Self(native)
    }
}

impl From<StakeCredential> for core::StakeCredential {
    fn from(wasm: StakeCredential) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct StakeDelegation(pub(crate) core::StakeDelegation);

#[wasm_bindgen]

impl StakeDelegation {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<StakeDelegation, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<StakeDelegation, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn stake_credential(&self) -> StakeCredential {
        self.0.stake_credential.clone().into()
    }

    pub fn pool_keyhash(&self) -> PoolKeyhash {
        self.0.pool_keyhash.clone().into()
    }

    pub fn new(stake_credential: &StakeCredential, pool_keyhash: &PoolKeyhash) -> Self {
        Self(core::StakeDelegation::new(stake_credential.clone().into(), pool_keyhash.clone().into()))
    }
}

impl From<core::StakeDelegation> for StakeDelegation {
    fn from(native: core::StakeDelegation) -> Self {
        Self(native)
    }
}

impl From<StakeDelegation> for core::StakeDelegation {
    fn from(wasm: StakeDelegation) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct StakeDeregistration(pub(crate) core::StakeDeregistration);

#[wasm_bindgen]

impl StakeDeregistration {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<StakeDeregistration, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<StakeDeregistration, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn stake_credential(&self) -> StakeCredential {
        self.0.stake_credential.clone().into()
    }

    pub fn new(stake_credential: &StakeCredential) -> Self {
        Self(core::StakeDeregistration::new(stake_credential.clone().into()))
    }
}

impl From<core::StakeDeregistration> for StakeDeregistration {
    fn from(native: core::StakeDeregistration) -> Self {
        Self(native)
    }
}

impl From<StakeDeregistration> for core::StakeDeregistration {
    fn from(wasm: StakeDeregistration) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct StakeRegistration(pub(crate) core::StakeRegistration);

#[wasm_bindgen]

impl StakeRegistration {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<StakeRegistration, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<StakeRegistration, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn stake_credential(&self) -> StakeCredential {
        self.0.stake_credential.clone().into()
    }

    pub fn new(stake_credential: &StakeCredential) -> Self {
        Self(core::StakeRegistration::new(stake_credential.clone().into()))
    }
}

impl From<core::StakeRegistration> for StakeRegistration {
    fn from(native: core::StakeRegistration) -> Self {
        Self(native)
    }
}

impl From<StakeRegistration> for core::StakeRegistration {
    fn from(wasm: StakeRegistration) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct Url(pub(crate) core::Url);

#[wasm_bindgen]

impl Url {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<Url, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<Url, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn get(&self) -> String {
        self.0.get().clone().clone()
    }
}

impl From<core::Url> for Url {
    fn from(native: core::Url) -> Self {
        Self(native)
    }
}

impl From<Url> for core::Url {
    fn from(wasm: Url) -> Self {
        wasm.0
    }
}

use super::*;