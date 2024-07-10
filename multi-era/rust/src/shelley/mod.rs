// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

pub mod cbor_encodings;
pub mod serialization;
pub mod utils;

use cbor_encodings::{
    MultisigAllEncoding, MultisigAnyEncoding, MultisigNOfKEncoding, MultisigPubkeyEncoding,
    ShelleyBlockEncoding, ShelleyHeaderBodyEncoding, ShelleyHeaderEncoding,
    ShelleyMultiHostNameEncoding, ShelleyPoolRegistrationEncoding,
    ShelleyProtocolParamUpdateEncoding, ShelleySingleHostNameEncoding,
    ShelleyTransactionBodyEncoding, ShelleyTransactionEncoding, ShelleyTransactionOutputEncoding,
    ShelleyTransactionWitnessSetEncoding, ShelleyUpdateEncoding,
};
use cml_chain::address::{Address, RewardAccount};
use cml_chain::assets::Coin;
use cml_chain::auxdata::Metadata;
use cml_chain::block::{OperationalCert, ProtocolVersion};
use cml_chain::certs::{
    Ipv4, Ipv6, PoolMetadata, PoolRetirement, SingleHostAddr, StakeCredential, StakeDelegation,
    StakeDeregistration, StakeRegistration,
};
use cml_chain::crypto::{
    AuxiliaryDataHash, BlockBodyHash, BlockHeaderHash, BootstrapWitness, Ed25519KeyHash,
    GenesisHash, KESSignature, Nonce, VRFCert, VRFVkey, Vkey, Vkeywitness,
};
use cml_chain::transaction::TransactionInput;
use cml_chain::{Epoch, Port, Rational, UnitInterval, Withdrawals};
use cml_core::ordered_hash_map::OrderedHashMap;
use cml_core::{DeserializeError, DeserializeFailure, TransactionIndex};
use cml_crypto::{GenesisDelegateHash, VRFKeyHash};
use std::collections::BTreeMap;
use std::convert::TryFrom;

use crate::allegra::MIRPot;

use self::cbor_encodings::{
    GenesisKeyDelegationEncoding, ProtocolVersionStructEncoding, ShelleyDNSNameEncoding,
    ShelleyMoveInstantaneousRewardEncoding, ShelleyMoveInstantaneousRewardsCertEncoding,
    ShelleyPoolParamsEncoding,
};
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct GenesisKeyDelegation {
    pub genesis_hash: GenesisHash,
    pub genesis_delegate_hash: GenesisDelegateHash,
    pub vrf_key_hash: VRFKeyHash,
    #[serde(skip)]
    pub encodings: Option<GenesisKeyDelegationEncoding>,
}

impl GenesisKeyDelegation {
    pub fn new(
        genesis_hash: GenesisHash,
        genesis_delegate_hash: GenesisDelegateHash,
        vrf_key_hash: VRFKeyHash,
    ) -> Self {
        Self {
            genesis_hash,
            genesis_delegate_hash,
            vrf_key_hash,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct MultisigAll {
    pub multisig_scripts: Vec<MultisigScript>,
    #[serde(skip)]
    pub encodings: Option<MultisigAllEncoding>,
}

impl MultisigAll {
    pub fn new(multisig_scripts: Vec<MultisigScript>) -> Self {
        Self {
            multisig_scripts,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct MultisigAny {
    pub multisig_scripts: Vec<MultisigScript>,
    #[serde(skip)]
    pub encodings: Option<MultisigAnyEncoding>,
}

impl MultisigAny {
    pub fn new(multisig_scripts: Vec<MultisigScript>) -> Self {
        Self {
            multisig_scripts,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct MultisigNOfK {
    pub n: u64,
    pub multisig_scripts: Vec<MultisigScript>,
    #[serde(skip)]
    pub encodings: Option<MultisigNOfKEncoding>,
}

impl MultisigNOfK {
    pub fn new(n: u64, multisig_scripts: Vec<MultisigScript>) -> Self {
        Self {
            n,
            multisig_scripts,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct MultisigPubkey {
    pub ed25519_key_hash: Ed25519KeyHash,
    #[serde(skip)]
    pub encodings: Option<MultisigPubkeyEncoding>,
}

impl MultisigPubkey {
    pub fn new(ed25519_key_hash: Ed25519KeyHash) -> Self {
        Self {
            ed25519_key_hash,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub enum MultisigScript {
    MultisigPubkey(MultisigPubkey),
    MultisigAll(MultisigAll),
    MultisigAny(MultisigAny),
    MultisigNOfK(MultisigNOfK),
}

impl MultisigScript {
    pub fn new_multisig_pubkey(ed25519_key_hash: Ed25519KeyHash) -> Self {
        Self::MultisigPubkey(MultisigPubkey::new(ed25519_key_hash))
    }

    pub fn new_multisig_all(multisig_scripts: Vec<MultisigScript>) -> Self {
        Self::MultisigAll(MultisigAll::new(multisig_scripts))
    }

    pub fn new_multisig_any(multisig_scripts: Vec<MultisigScript>) -> Self {
        Self::MultisigAny(MultisigAny::new(multisig_scripts))
    }

    pub fn new_multisig_n_of_k(n: u64, multisig_scripts: Vec<MultisigScript>) -> Self {
        Self::MultisigNOfK(MultisigNOfK::new(n, multisig_scripts))
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ProtocolVersionStruct {
    pub protocol_version: ProtocolVersion,
    #[serde(skip)]
    pub encodings: Option<ProtocolVersionStructEncoding>,
}

impl ProtocolVersionStruct {
    pub fn new(protocol_version: ProtocolVersion) -> Self {
        Self {
            protocol_version,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ShelleyBlock {
    pub header: ShelleyHeader,
    pub transaction_bodies: Vec<ShelleyTransactionBody>,
    pub transaction_witness_sets: Vec<ShelleyTransactionWitnessSet>,
    pub transaction_metadata_set: OrderedHashMap<TransactionIndex, Metadata>,
    #[serde(skip)]
    pub encodings: Option<ShelleyBlockEncoding>,
}

impl ShelleyBlock {
    pub fn new(
        header: ShelleyHeader,
        transaction_bodies: Vec<ShelleyTransactionBody>,
        transaction_witness_sets: Vec<ShelleyTransactionWitnessSet>,
        transaction_metadata_set: OrderedHashMap<TransactionIndex, Metadata>,
    ) -> Self {
        Self {
            header,
            transaction_bodies,
            transaction_witness_sets,
            transaction_metadata_set,
            encodings: None,
        }
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub enum ShelleyCertificate {
    StakeRegistration(StakeRegistration),
    StakeDeregistration(StakeDeregistration),
    StakeDelegation(StakeDelegation),
    ShelleyPoolRegistration(ShelleyPoolRegistration),
    PoolRetirement(PoolRetirement),
    GenesisKeyDelegation(GenesisKeyDelegation),
    ShelleyMoveInstantaneousRewardsCert(ShelleyMoveInstantaneousRewardsCert),
}

impl ShelleyCertificate {
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

    pub fn new_shelley_pool_registration(pool_params: ShelleyPoolParams) -> Self {
        Self::ShelleyPoolRegistration(ShelleyPoolRegistration::new(pool_params))
    }

    pub fn new_pool_retirement(ed25519_key_hash: Ed25519KeyHash, epoch: Epoch) -> Self {
        Self::PoolRetirement(PoolRetirement::new(ed25519_key_hash, epoch))
    }

    pub fn new_genesis_key_delegation(
        genesis_hash: GenesisHash,
        genesis_delegate_hash: GenesisDelegateHash,
        vrf_key_hash: VRFKeyHash,
    ) -> Self {
        Self::GenesisKeyDelegation(GenesisKeyDelegation::new(
            genesis_hash,
            genesis_delegate_hash,
            vrf_key_hash,
        ))
    }

    pub fn new_shelley_move_instantaneous_rewards_cert(
        shelley_move_instantaneous_reward: ShelleyMoveInstantaneousReward,
    ) -> Self {
        Self::ShelleyMoveInstantaneousRewardsCert(ShelleyMoveInstantaneousRewardsCert::new(
            shelley_move_instantaneous_reward,
        ))
    }
}

#[derive(Clone, Debug)]
pub struct ShelleyDNSName {
    pub inner: String,
    pub encodings: Option<ShelleyDNSNameEncoding>,
}

impl ShelleyDNSName {
    pub fn get(&self) -> &String {
        &self.inner
    }

    pub fn new(inner: String) -> Result<Self, DeserializeError> {
        if inner.len() > 64 {
            return Err(DeserializeError::new(
                "ShelleyDNSName",
                DeserializeFailure::RangeCheck {
                    found: inner.len() as isize,
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

impl TryFrom<String> for ShelleyDNSName {
    type Error = DeserializeError;

    fn try_from(inner: String) -> Result<Self, Self::Error> {
        ShelleyDNSName::new(inner)
    }
}

impl serde::Serialize for ShelleyDNSName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.inner.serialize(serializer)
    }
}

impl<'de> serde::de::Deserialize<'de> for ShelleyDNSName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let inner = <String as serde::de::Deserialize>::deserialize(deserializer)?;
        Self::new(inner.clone()).map_err(|_e| {
            serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(&inner),
                &"invalid ShelleyDNSName",
            )
        })
    }
}

impl schemars::JsonSchema for ShelleyDNSName {
    fn schema_name() -> String {
        String::from("ShelleyDNSName")
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        String::json_schema(gen)
    }

    fn is_referenceable() -> bool {
        String::is_referenceable()
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ShelleyHeader {
    pub body: ShelleyHeaderBody,
    pub signature: KESSignature,
    #[serde(skip)]
    pub encodings: Option<ShelleyHeaderEncoding>,
}

impl ShelleyHeader {
    pub fn new(body: ShelleyHeaderBody, signature: KESSignature) -> Self {
        Self {
            body,
            signature,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ShelleyHeaderBody {
    pub block_number: u64,
    pub slot: u64,
    pub prev_hash: Option<BlockHeaderHash>,
    pub issuer_vkey: Vkey,
    pub vrf_vkey: VRFVkey,
    pub nonce_vrf: VRFCert,
    pub leader_vrf: VRFCert,
    pub block_body_size: u64,
    pub block_body_hash: BlockBodyHash,
    pub operational_cert: OperationalCert,
    pub protocol_version: ProtocolVersion,
    #[serde(skip)]
    pub encodings: Option<ShelleyHeaderBodyEncoding>,
}

impl ShelleyHeaderBody {
    pub fn new(
        block_number: u64,
        slot: u64,
        prev_hash: Option<BlockHeaderHash>,
        issuer_vkey: Vkey,
        vrf_vkey: VRFVkey,
        nonce_vrf: VRFCert,
        leader_vrf: VRFCert,
        block_body_size: u64,
        block_body_hash: BlockBodyHash,
        operational_cert: OperationalCert,
        protocol_version: ProtocolVersion,
    ) -> Self {
        Self {
            block_number,
            slot,
            prev_hash,
            issuer_vkey,
            vrf_vkey,
            nonce_vrf,
            leader_vrf,
            block_body_size,
            block_body_hash,
            operational_cert,
            protocol_version,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ShelleyMoveInstantaneousReward {
    pub pot: MIRPot,
    pub to_stake_credentials: OrderedHashMap<StakeCredential, Coin>,
    #[serde(skip)]
    pub encodings: Option<ShelleyMoveInstantaneousRewardEncoding>,
}

impl ShelleyMoveInstantaneousReward {
    pub fn new(pot: MIRPot, to_stake_credentials: OrderedHashMap<StakeCredential, Coin>) -> Self {
        Self {
            pot,
            to_stake_credentials,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ShelleyMoveInstantaneousRewardsCert {
    pub shelley_move_instantaneous_reward: ShelleyMoveInstantaneousReward,
    #[serde(skip)]
    pub encodings: Option<ShelleyMoveInstantaneousRewardsCertEncoding>,
}

impl ShelleyMoveInstantaneousRewardsCert {
    pub fn new(shelley_move_instantaneous_reward: ShelleyMoveInstantaneousReward) -> Self {
        Self {
            shelley_move_instantaneous_reward,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ShelleyMultiHostName {
    /// A SRV DNS record
    pub shelley_dns_name: ShelleyDNSName,
    #[serde(skip)]
    pub encodings: Option<ShelleyMultiHostNameEncoding>,
}

impl ShelleyMultiHostName {
    /// * `shelley_dns_name` - A SRV DNS record
    pub fn new(shelley_dns_name: ShelleyDNSName) -> Self {
        Self {
            shelley_dns_name,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ShelleyPoolParams {
    pub operator: Ed25519KeyHash,
    pub vrf_keyhash: VRFKeyHash,
    pub pledge: Coin,
    pub cost: Coin,
    pub margin: UnitInterval,
    pub reward_account: RewardAccount,
    pub pool_owners: Vec<Ed25519KeyHash>,
    pub relays: Vec<ShelleyRelay>,
    pub pool_metadata: Option<PoolMetadata>,
    #[serde(skip)]
    pub encodings: Option<ShelleyPoolParamsEncoding>,
}

impl ShelleyPoolParams {
    pub fn new(
        operator: Ed25519KeyHash,
        vrf_keyhash: VRFKeyHash,
        pledge: Coin,
        cost: Coin,
        margin: UnitInterval,
        reward_account: RewardAccount,
        pool_owners: Vec<Ed25519KeyHash>,
        relays: Vec<ShelleyRelay>,
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
pub struct ShelleyPoolRegistration {
    pub pool_params: ShelleyPoolParams,
    #[serde(skip)]
    pub encodings: Option<ShelleyPoolRegistrationEncoding>,
}

impl ShelleyPoolRegistration {
    pub fn new(pool_params: ShelleyPoolParams) -> Self {
        Self {
            pool_params,
            encodings: None,
        }
    }
}

pub type ShelleyProposedProtocolParameterUpdates =
    OrderedHashMap<GenesisHash, ShelleyProtocolParamUpdate>;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ShelleyProtocolParamUpdate {
    pub minfee_a: Option<u64>,
    pub minfee_b: Option<u64>,
    pub max_block_body_size: Option<u64>,
    pub max_transaction_size: Option<u64>,
    pub max_block_header_size: Option<u64>,
    pub key_deposit: Option<Coin>,
    pub pool_deposit: Option<Coin>,
    pub maximum_epoch: Option<Epoch>,
    pub n_opt: Option<u64>,
    pub pool_pledge_influence: Option<Rational>,
    pub expansion_rate: Option<UnitInterval>,
    pub treasury_growth_rate: Option<UnitInterval>,
    pub decentralization_constant: Option<UnitInterval>,
    pub extra_entropy: Option<Nonce>,
    pub protocol_version: Option<ProtocolVersionStruct>,
    pub min_utxo_value: Option<Coin>,
    #[serde(skip)]
    pub encodings: Option<ShelleyProtocolParamUpdateEncoding>,
}

impl ShelleyProtocolParamUpdate {
    pub fn new() -> Self {
        Self {
            minfee_a: None,
            minfee_b: None,
            max_block_body_size: None,
            max_transaction_size: None,
            max_block_header_size: None,
            key_deposit: None,
            pool_deposit: None,
            maximum_epoch: None,
            n_opt: None,
            pool_pledge_influence: None,
            expansion_rate: None,
            treasury_growth_rate: None,
            decentralization_constant: None,
            extra_entropy: None,
            protocol_version: None,
            min_utxo_value: None,
            encodings: None,
        }
    }
}

impl Default for ShelleyProtocolParamUpdate {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub enum ShelleyRelay {
    SingleHostAddr(SingleHostAddr),
    ShelleySingleHostName(ShelleySingleHostName),
    ShelleyMultiHostName(ShelleyMultiHostName),
}

impl ShelleyRelay {
    pub fn new_single_host_addr(
        port: Option<Port>,
        ipv4: Option<Ipv4>,
        ipv6: Option<Ipv6>,
    ) -> Self {
        Self::SingleHostAddr(SingleHostAddr::new(port, ipv4, ipv6))
    }

    pub fn new_shelley_single_host_name(
        port: Option<Port>,
        shelley_dns_name: ShelleyDNSName,
    ) -> Self {
        Self::ShelleySingleHostName(ShelleySingleHostName::new(port, shelley_dns_name))
    }

    pub fn new_shelley_multi_host_name(shelley_dns_name: ShelleyDNSName) -> Self {
        Self::ShelleyMultiHostName(ShelleyMultiHostName::new(shelley_dns_name))
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ShelleySingleHostName {
    pub port: Option<Port>,
    /// An A or AAAA DNS record
    pub shelley_dns_name: ShelleyDNSName,
    #[serde(skip)]
    pub encodings: Option<ShelleySingleHostNameEncoding>,
}

impl ShelleySingleHostName {
    /// * `shelley_dns_name` - An A or AAAA DNS record
    pub fn new(port: Option<Port>, shelley_dns_name: ShelleyDNSName) -> Self {
        Self {
            port,
            shelley_dns_name,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ShelleyTransaction {
    pub body: ShelleyTransactionBody,
    pub witness_set: ShelleyTransactionWitnessSet,
    pub metadata: Option<Metadata>,
    #[serde(skip)]
    pub encodings: Option<ShelleyTransactionEncoding>,
}

impl ShelleyTransaction {
    pub fn new(
        body: ShelleyTransactionBody,
        witness_set: ShelleyTransactionWitnessSet,
        metadata: Option<Metadata>,
    ) -> Self {
        Self {
            body,
            witness_set,
            metadata,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ShelleyTransactionBody {
    pub inputs: Vec<TransactionInput>,
    pub outputs: Vec<ShelleyTransactionOutput>,
    pub fee: Coin,
    pub ttl: u64,
    pub certs: Option<Vec<ShelleyCertificate>>,
    pub withdrawals: Option<Withdrawals>,
    pub update: Option<ShelleyUpdate>,
    pub auxiliary_data_hash: Option<AuxiliaryDataHash>,
    #[serde(skip)]
    pub encodings: Option<ShelleyTransactionBodyEncoding>,
}

impl ShelleyTransactionBody {
    pub fn new(
        inputs: Vec<TransactionInput>,
        outputs: Vec<ShelleyTransactionOutput>,
        fee: Coin,
        ttl: u64,
    ) -> Self {
        Self {
            inputs,
            outputs,
            fee,
            ttl,
            certs: None,
            withdrawals: None,
            update: None,
            auxiliary_data_hash: None,
            encodings: None,
        }
    }
}

pub type ShelleyTransactionIndex = u16;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ShelleyTransactionOutput {
    pub address: Address,
    pub amount: Coin,
    #[serde(skip)]
    pub encodings: Option<ShelleyTransactionOutputEncoding>,
}

impl ShelleyTransactionOutput {
    pub fn new(address: Address, amount: Coin) -> Self {
        Self {
            address,
            amount,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ShelleyTransactionWitnessSet {
    pub vkeywitnesses: Option<Vec<Vkeywitness>>,
    pub native_scripts: Option<Vec<MultisigScript>>,
    pub bootstrap_witnesses: Option<Vec<BootstrapWitness>>,
    #[serde(skip)]
    pub encodings: Option<ShelleyTransactionWitnessSetEncoding>,
}

impl ShelleyTransactionWitnessSet {
    pub fn new() -> Self {
        Self {
            vkeywitnesses: None,
            native_scripts: None,
            bootstrap_witnesses: None,
            encodings: None,
        }
    }
}

impl Default for ShelleyTransactionWitnessSet {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ShelleyUpdate {
    pub shelley_proposed_protocol_parameter_updates: ShelleyProposedProtocolParameterUpdates,
    pub epoch: Epoch,
    #[serde(skip)]
    pub encodings: Option<ShelleyUpdateEncoding>,
}

impl ShelleyUpdate {
    pub fn new(
        shelley_proposed_protocol_parameter_updates: ShelleyProposedProtocolParameterUpdates,
        epoch: Epoch,
    ) -> Self {
        Self {
            shelley_proposed_protocol_parameter_updates,
            epoch,
            encodings: None,
        }
    }
}

impl From<ShelleyDNSName> for String {
    fn from(wrapper: ShelleyDNSName) -> Self {
        wrapper.inner
    }
}
