// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use crate::{
    GenesisHashList, MapStakeCredentialToCoin, MapTransactionIndexToMetadata, MultisigScriptList,
    ShelleyCertificateList, ShelleyRelayList, ShelleyTransactionBodyList,
    ShelleyTransactionOutputList, ShelleyTransactionWitnessSetList,
};
use cml_chain_wasm::address::{Address, RewardAccount};
use cml_chain_wasm::assets::Coin;
use cml_chain_wasm::auxdata::Metadata;
use cml_chain_wasm::block::{OperationalCert, ProtocolVersion};
use cml_chain_wasm::certs::{
    Ipv4, Ipv6, PoolMetadata, PoolRetirement, SingleHostAddr, StakeCredential, StakeDelegation,
    StakeDeregistration, StakeRegistration,
};
use cml_chain_wasm::crypto::{KESSignature, Nonce, VRFCert, Vkey};
use cml_chain_wasm::{
    BootstrapWitnessList, Ed25519KeyHashList, TransactionInputList, VkeywitnessList,
};
use cml_chain_wasm::{Epoch, Port, Rational, UnitInterval, Withdrawals};
use cml_core::ordered_hash_map::OrderedHashMap;
use cml_core_wasm::{impl_wasm_cbor_json_api, impl_wasm_conversions};
use cml_crypto_wasm::{
    AuxiliaryDataHash, BlockBodyHash, BlockHeaderHash, Ed25519KeyHash, GenesisDelegateHash,
    GenesisHash, VRFKeyHash, VRFVkey,
};
use cml_multi_era::allegra::MIRPot;
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct GenesisKeyDelegation(cml_multi_era::shelley::GenesisKeyDelegation);

impl_wasm_cbor_json_api!(GenesisKeyDelegation);

impl_wasm_conversions!(
    cml_multi_era::shelley::GenesisKeyDelegation,
    GenesisKeyDelegation
);

#[wasm_bindgen]
impl GenesisKeyDelegation {
    pub fn genesis_hash(&self) -> GenesisHash {
        self.0.genesis_hash.into()
    }

    pub fn genesis_delegate_hash(&self) -> GenesisDelegateHash {
        self.0.genesis_delegate_hash.into()
    }

    pub fn vrf_key_hash(&self) -> VRFKeyHash {
        self.0.vrf_key_hash.into()
    }

    pub fn new(
        genesis_hash: &GenesisHash,
        genesis_delegate_hash: &GenesisDelegateHash,
        vrf_key_hash: &VRFKeyHash,
    ) -> Self {
        Self(cml_multi_era::shelley::GenesisKeyDelegation::new(
            genesis_hash.clone().into(),
            genesis_delegate_hash.clone().into(),
            vrf_key_hash.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MultisigAll(cml_multi_era::shelley::MultisigAll);

impl_wasm_cbor_json_api!(MultisigAll);

impl_wasm_conversions!(cml_multi_era::shelley::MultisigAll, MultisigAll);

#[wasm_bindgen]
impl MultisigAll {
    pub fn multisig_scripts(&self) -> MultisigScriptList {
        self.0.multisig_scripts.clone().into()
    }

    pub fn new(multisig_scripts: &MultisigScriptList) -> Self {
        Self(cml_multi_era::shelley::MultisigAll::new(
            multisig_scripts.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MultisigAny(cml_multi_era::shelley::MultisigAny);

impl_wasm_cbor_json_api!(MultisigAny);

impl_wasm_conversions!(cml_multi_era::shelley::MultisigAny, MultisigAny);

#[wasm_bindgen]
impl MultisigAny {
    pub fn multisig_scripts(&self) -> MultisigScriptList {
        self.0.multisig_scripts.clone().into()
    }

    pub fn new(multisig_scripts: &MultisigScriptList) -> Self {
        Self(cml_multi_era::shelley::MultisigAny::new(
            multisig_scripts.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MultisigNOfK(cml_multi_era::shelley::MultisigNOfK);

impl_wasm_cbor_json_api!(MultisigNOfK);

impl_wasm_conversions!(cml_multi_era::shelley::MultisigNOfK, MultisigNOfK);

#[wasm_bindgen]
impl MultisigNOfK {
    pub fn n(&self) -> u64 {
        self.0.n
    }

    pub fn multisig_scripts(&self) -> MultisigScriptList {
        self.0.multisig_scripts.clone().into()
    }

    pub fn new(n: u64, multisig_scripts: &MultisigScriptList) -> Self {
        Self(cml_multi_era::shelley::MultisigNOfK::new(
            n,
            multisig_scripts.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MultisigPubkey(cml_multi_era::shelley::MultisigPubkey);

impl_wasm_cbor_json_api!(MultisigPubkey);

impl_wasm_conversions!(cml_multi_era::shelley::MultisigPubkey, MultisigPubkey);

#[wasm_bindgen]
impl MultisigPubkey {
    pub fn ed25519_key_hash(&self) -> Ed25519KeyHash {
        self.0.ed25519_key_hash.into()
    }

    pub fn new(ed25519_key_hash: &Ed25519KeyHash) -> Self {
        Self(cml_multi_era::shelley::MultisigPubkey::new(
            ed25519_key_hash.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MultisigScript(cml_multi_era::shelley::MultisigScript);

impl_wasm_cbor_json_api!(MultisigScript);

impl_wasm_conversions!(cml_multi_era::shelley::MultisigScript, MultisigScript);

#[wasm_bindgen]
impl MultisigScript {
    pub fn new_multisig_pubkey(ed25519_key_hash: &Ed25519KeyHash) -> Self {
        Self(cml_multi_era::shelley::MultisigScript::new_multisig_pubkey(
            ed25519_key_hash.clone().into(),
        ))
    }

    pub fn new_multisig_all(multisig_scripts: &MultisigScriptList) -> Self {
        Self(cml_multi_era::shelley::MultisigScript::new_multisig_all(
            multisig_scripts.clone().into(),
        ))
    }

    pub fn new_multisig_any(multisig_scripts: &MultisigScriptList) -> Self {
        Self(cml_multi_era::shelley::MultisigScript::new_multisig_any(
            multisig_scripts.clone().into(),
        ))
    }

    pub fn new_multisig_n_of_k(n: u64, multisig_scripts: &MultisigScriptList) -> Self {
        Self(cml_multi_era::shelley::MultisigScript::new_multisig_n_of_k(
            n,
            multisig_scripts.clone().into(),
        ))
    }

    pub fn kind(&self) -> MultisigScriptKind {
        match &self.0 {
            cml_multi_era::shelley::MultisigScript::MultisigPubkey(_) => {
                MultisigScriptKind::MultisigPubkey
            }
            cml_multi_era::shelley::MultisigScript::MultisigAll(_) => {
                MultisigScriptKind::MultisigAll
            }
            cml_multi_era::shelley::MultisigScript::MultisigAny(_) => {
                MultisigScriptKind::MultisigAny
            }
            cml_multi_era::shelley::MultisigScript::MultisigNOfK(_) => {
                MultisigScriptKind::MultisigNOfK
            }
        }
    }

    pub fn as_multisig_pubkey(&self) -> Option<MultisigPubkey> {
        match &self.0 {
            cml_multi_era::shelley::MultisigScript::MultisigPubkey(multisig_pubkey) => {
                Some(multisig_pubkey.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_multisig_all(&self) -> Option<MultisigAll> {
        match &self.0 {
            cml_multi_era::shelley::MultisigScript::MultisigAll(multisig_all) => {
                Some(multisig_all.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_multisig_any(&self) -> Option<MultisigAny> {
        match &self.0 {
            cml_multi_era::shelley::MultisigScript::MultisigAny(multisig_any) => {
                Some(multisig_any.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_multisig_n_of_k(&self) -> Option<MultisigNOfK> {
        match &self.0 {
            cml_multi_era::shelley::MultisigScript::MultisigNOfK(multisig_n_of_k) => {
                Some(multisig_n_of_k.clone().into())
            }
            _ => None,
        }
    }
}

#[wasm_bindgen]
pub enum MultisigScriptKind {
    MultisigPubkey,
    MultisigAll,
    MultisigAny,
    MultisigNOfK,
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ProtocolVersionStruct(cml_multi_era::shelley::ProtocolVersionStruct);

impl_wasm_cbor_json_api!(ProtocolVersionStruct);

impl_wasm_conversions!(
    cml_multi_era::shelley::ProtocolVersionStruct,
    ProtocolVersionStruct
);

#[wasm_bindgen]
impl ProtocolVersionStruct {
    pub fn protocol_version(&self) -> ProtocolVersion {
        self.0.protocol_version.clone().into()
    }

    pub fn new(protocol_version: &ProtocolVersion) -> Self {
        Self(cml_multi_era::shelley::ProtocolVersionStruct::new(
            protocol_version.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ShelleyBlock(cml_multi_era::shelley::ShelleyBlock);

impl_wasm_cbor_json_api!(ShelleyBlock);

impl_wasm_conversions!(cml_multi_era::shelley::ShelleyBlock, ShelleyBlock);

#[wasm_bindgen]
impl ShelleyBlock {
    pub fn header(&self) -> ShelleyHeader {
        self.0.header.clone().into()
    }

    pub fn transaction_bodies(&self) -> ShelleyTransactionBodyList {
        self.0.transaction_bodies.clone().into()
    }

    pub fn transaction_witness_sets(&self) -> ShelleyTransactionWitnessSetList {
        self.0.transaction_witness_sets.clone().into()
    }

    pub fn transaction_metadata_set(&self) -> MapTransactionIndexToMetadata {
        self.0.transaction_metadata_set.clone().into()
    }

    pub fn new(
        header: &ShelleyHeader,
        transaction_bodies: &ShelleyTransactionBodyList,
        transaction_witness_sets: &ShelleyTransactionWitnessSetList,
        transaction_metadata_set: &MapTransactionIndexToMetadata,
    ) -> Self {
        Self(cml_multi_era::shelley::ShelleyBlock::new(
            header.clone().into(),
            transaction_bodies.clone().into(),
            transaction_witness_sets.clone().into(),
            transaction_metadata_set.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ShelleyCertificate(cml_multi_era::shelley::ShelleyCertificate);

impl_wasm_cbor_json_api!(ShelleyCertificate);

impl_wasm_conversions!(
    cml_multi_era::shelley::ShelleyCertificate,
    ShelleyCertificate
);

#[wasm_bindgen]
impl ShelleyCertificate {
    pub fn new_stake_registration(stake_credential: &StakeCredential) -> Self {
        Self(
            cml_multi_era::shelley::ShelleyCertificate::new_stake_registration(
                stake_credential.clone().into(),
            ),
        )
    }

    pub fn new_stake_deregistration(stake_credential: &StakeCredential) -> Self {
        Self(
            cml_multi_era::shelley::ShelleyCertificate::new_stake_deregistration(
                stake_credential.clone().into(),
            ),
        )
    }

    pub fn new_stake_delegation(
        stake_credential: &StakeCredential,
        ed25519_key_hash: &Ed25519KeyHash,
    ) -> Self {
        Self(
            cml_multi_era::shelley::ShelleyCertificate::new_stake_delegation(
                stake_credential.clone().into(),
                ed25519_key_hash.clone().into(),
            ),
        )
    }

    pub fn new_shelley_pool_registration(pool_params: &ShelleyPoolParams) -> Self {
        Self(
            cml_multi_era::shelley::ShelleyCertificate::new_shelley_pool_registration(
                pool_params.clone().into(),
            ),
        )
    }

    pub fn new_pool_retirement(ed25519_key_hash: &Ed25519KeyHash, epoch: Epoch) -> Self {
        Self(
            cml_multi_era::shelley::ShelleyCertificate::new_pool_retirement(
                ed25519_key_hash.clone().into(),
                epoch,
            ),
        )
    }

    pub fn new_genesis_key_delegation(
        genesis_hash: &GenesisHash,
        genesis_delegate_hash: &GenesisDelegateHash,
        vrf_key_hash: &VRFKeyHash,
    ) -> Self {
        Self(
            cml_multi_era::shelley::ShelleyCertificate::new_genesis_key_delegation(
                genesis_hash.clone().into(),
                genesis_delegate_hash.clone().into(),
                vrf_key_hash.clone().into(),
            ),
        )
    }

    pub fn new_shelley_move_instantaneous_rewards_cert(
        shelley_move_instantaneous_reward: &ShelleyMoveInstantaneousReward,
    ) -> Self {
        Self(
            cml_multi_era::shelley::ShelleyCertificate::new_shelley_move_instantaneous_rewards_cert(
                shelley_move_instantaneous_reward.clone().into(),
            ),
        )
    }

    pub fn kind(&self) -> ShelleyCertificateKind {
        match &self.0 {
            cml_multi_era::shelley::ShelleyCertificate::StakeRegistration(_) => {
                ShelleyCertificateKind::StakeRegistration
            }
            cml_multi_era::shelley::ShelleyCertificate::StakeDeregistration(_) => {
                ShelleyCertificateKind::StakeDeregistration
            }
            cml_multi_era::shelley::ShelleyCertificate::StakeDelegation(_) => {
                ShelleyCertificateKind::StakeDelegation
            }
            cml_multi_era::shelley::ShelleyCertificate::ShelleyPoolRegistration(_) => {
                ShelleyCertificateKind::ShelleyPoolRegistration
            }
            cml_multi_era::shelley::ShelleyCertificate::PoolRetirement(_) => {
                ShelleyCertificateKind::PoolRetirement
            }
            cml_multi_era::shelley::ShelleyCertificate::GenesisKeyDelegation(_) => {
                ShelleyCertificateKind::GenesisKeyDelegation
            }
            cml_multi_era::shelley::ShelleyCertificate::ShelleyMoveInstantaneousRewardsCert(_) => {
                ShelleyCertificateKind::ShelleyMoveInstantaneousRewardsCert
            }
        }
    }

    pub fn as_stake_registration(&self) -> Option<StakeRegistration> {
        match &self.0 {
            cml_multi_era::shelley::ShelleyCertificate::StakeRegistration(stake_registration) => {
                Some(stake_registration.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_stake_deregistration(&self) -> Option<StakeDeregistration> {
        match &self.0 {
            cml_multi_era::shelley::ShelleyCertificate::StakeDeregistration(
                stake_deregistration,
            ) => Some(stake_deregistration.clone().into()),
            _ => None,
        }
    }

    pub fn as_stake_delegation(&self) -> Option<StakeDelegation> {
        match &self.0 {
            cml_multi_era::shelley::ShelleyCertificate::StakeDelegation(stake_delegation) => {
                Some(stake_delegation.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_shelley_pool_registration(&self) -> Option<ShelleyPoolRegistration> {
        match &self.0 {
            cml_multi_era::shelley::ShelleyCertificate::ShelleyPoolRegistration(
                shelley_pool_registration,
            ) => Some(shelley_pool_registration.clone().into()),
            _ => None,
        }
    }

    pub fn as_pool_retirement(&self) -> Option<PoolRetirement> {
        match &self.0 {
            cml_multi_era::shelley::ShelleyCertificate::PoolRetirement(pool_retirement) => {
                Some(pool_retirement.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_genesis_key_delegation(&self) -> Option<GenesisKeyDelegation> {
        match &self.0 {
            cml_multi_era::shelley::ShelleyCertificate::GenesisKeyDelegation(
                genesis_key_delegation,
            ) => Some(genesis_key_delegation.clone().into()),
            _ => None,
        }
    }

    pub fn as_shelley_move_instantaneous_rewards_cert(
        &self,
    ) -> Option<ShelleyMoveInstantaneousRewardsCert> {
        match &self.0 {
            cml_multi_era::shelley::ShelleyCertificate::ShelleyMoveInstantaneousRewardsCert(
                shelley_move_instantaneous_rewards_cert,
            ) => Some(shelley_move_instantaneous_rewards_cert.clone().into()),
            _ => None,
        }
    }
}

#[wasm_bindgen]
pub enum ShelleyCertificateKind {
    StakeRegistration,
    StakeDeregistration,
    StakeDelegation,
    ShelleyPoolRegistration,
    PoolRetirement,
    GenesisKeyDelegation,
    ShelleyMoveInstantaneousRewardsCert,
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ShelleyDNSName(cml_multi_era::shelley::ShelleyDNSName);

impl_wasm_cbor_json_api!(ShelleyDNSName);

impl_wasm_conversions!(cml_multi_era::shelley::ShelleyDNSName, ShelleyDNSName);

#[wasm_bindgen]
impl ShelleyDNSName {
    pub fn get(&self) -> String {
        self.0.get().clone()
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ShelleyHeader(cml_multi_era::shelley::ShelleyHeader);

impl_wasm_cbor_json_api!(ShelleyHeader);

impl_wasm_conversions!(cml_multi_era::shelley::ShelleyHeader, ShelleyHeader);

#[wasm_bindgen]
impl ShelleyHeader {
    pub fn body(&self) -> ShelleyHeaderBody {
        self.0.body.clone().into()
    }

    pub fn signature(&self) -> KESSignature {
        self.0.signature.clone().into()
    }

    pub fn new(body: &ShelleyHeaderBody, signature: &KESSignature) -> Self {
        Self(cml_multi_era::shelley::ShelleyHeader::new(
            body.clone().into(),
            signature.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ShelleyHeaderBody(cml_multi_era::shelley::ShelleyHeaderBody);

impl_wasm_cbor_json_api!(ShelleyHeaderBody);

impl_wasm_conversions!(cml_multi_era::shelley::ShelleyHeaderBody, ShelleyHeaderBody);

#[wasm_bindgen]
impl ShelleyHeaderBody {
    pub fn block_number(&self) -> u64 {
        self.0.block_number
    }

    pub fn slot(&self) -> u64 {
        self.0.slot
    }

    pub fn prev_hash(&self) -> Option<BlockHeaderHash> {
        self.0.prev_hash.map(std::convert::Into::into)
    }

    pub fn issuer_vkey(&self) -> Vkey {
        self.0.issuer_vkey.clone().into()
    }

    pub fn vrf_vkey(&self) -> VRFVkey {
        self.0.vrf_vkey.into()
    }

    pub fn nonce_vrf(&self) -> VRFCert {
        self.0.nonce_vrf.clone().into()
    }

    pub fn leader_vrf(&self) -> VRFCert {
        self.0.leader_vrf.clone().into()
    }

    pub fn block_body_size(&self) -> u64 {
        self.0.block_body_size
    }

    pub fn block_body_hash(&self) -> BlockBodyHash {
        self.0.block_body_hash.into()
    }

    pub fn operational_cert(&self) -> OperationalCert {
        self.0.operational_cert.clone().into()
    }

    pub fn protocol_version(&self) -> ProtocolVersion {
        self.0.protocol_version.clone().into()
    }

    pub fn new(
        block_number: u64,
        slot: u64,
        prev_hash: Option<BlockHeaderHash>,
        issuer_vkey: &Vkey,
        vrf_vkey: &VRFVkey,
        nonce_vrf: &VRFCert,
        leader_vrf: &VRFCert,
        block_body_size: u64,
        block_body_hash: &BlockBodyHash,
        operational_cert: &OperationalCert,
        protocol_version: &ProtocolVersion,
    ) -> Self {
        Self(cml_multi_era::shelley::ShelleyHeaderBody::new(
            block_number,
            slot,
            prev_hash.map(Into::into),
            issuer_vkey.clone().into(),
            vrf_vkey.clone().into(),
            nonce_vrf.clone().into(),
            leader_vrf.clone().into(),
            block_body_size,
            block_body_hash.clone().into(),
            operational_cert.clone().into(),
            protocol_version.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ShelleyMoveInstantaneousReward(cml_multi_era::shelley::ShelleyMoveInstantaneousReward);

impl_wasm_cbor_json_api!(ShelleyMoveInstantaneousReward);

impl_wasm_conversions!(
    cml_multi_era::shelley::ShelleyMoveInstantaneousReward,
    ShelleyMoveInstantaneousReward
);

#[wasm_bindgen]
impl ShelleyMoveInstantaneousReward {
    pub fn pot(&self) -> MIRPot {
        self.0.pot
    }

    pub fn to_stake_credentials(&self) -> MapStakeCredentialToCoin {
        self.0.to_stake_credentials.clone().into()
    }

    pub fn new(pot: MIRPot, to_stake_credentials: &MapStakeCredentialToCoin) -> Self {
        Self(cml_multi_era::shelley::ShelleyMoveInstantaneousReward::new(
            pot,
            to_stake_credentials.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ShelleyMoveInstantaneousRewardsCert(
    cml_multi_era::shelley::ShelleyMoveInstantaneousRewardsCert,
);

impl_wasm_cbor_json_api!(ShelleyMoveInstantaneousRewardsCert);

impl_wasm_conversions!(
    cml_multi_era::shelley::ShelleyMoveInstantaneousRewardsCert,
    ShelleyMoveInstantaneousRewardsCert
);

#[wasm_bindgen]
impl ShelleyMoveInstantaneousRewardsCert {
    pub fn shelley_move_instantaneous_reward(&self) -> ShelleyMoveInstantaneousReward {
        self.0.shelley_move_instantaneous_reward.clone().into()
    }

    pub fn new(shelley_move_instantaneous_reward: &ShelleyMoveInstantaneousReward) -> Self {
        Self(
            cml_multi_era::shelley::ShelleyMoveInstantaneousRewardsCert::new(
                shelley_move_instantaneous_reward.clone().into(),
            ),
        )
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ShelleyMultiHostName(cml_multi_era::shelley::ShelleyMultiHostName);

impl_wasm_cbor_json_api!(ShelleyMultiHostName);

impl_wasm_conversions!(
    cml_multi_era::shelley::ShelleyMultiHostName,
    ShelleyMultiHostName
);

#[wasm_bindgen]
impl ShelleyMultiHostName {
    pub fn shelley_dns_name(&self) -> ShelleyDNSName {
        self.0.shelley_dns_name.clone().into()
    }

    /// * `shelley_dns_name` - A SRV DNS record
    pub fn new(shelley_dns_name: &ShelleyDNSName) -> Self {
        Self(cml_multi_era::shelley::ShelleyMultiHostName::new(
            shelley_dns_name.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ShelleyPoolParams(cml_multi_era::shelley::ShelleyPoolParams);

impl_wasm_cbor_json_api!(ShelleyPoolParams);

impl_wasm_conversions!(cml_multi_era::shelley::ShelleyPoolParams, ShelleyPoolParams);

#[wasm_bindgen]
impl ShelleyPoolParams {
    pub fn operator(&self) -> Ed25519KeyHash {
        self.0.operator.into()
    }

    pub fn vrf_keyhash(&self) -> VRFKeyHash {
        self.0.vrf_keyhash.into()
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

    pub fn relays(&self) -> ShelleyRelayList {
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
        relays: &ShelleyRelayList,
        pool_metadata: Option<PoolMetadata>,
    ) -> Self {
        Self(cml_multi_era::shelley::ShelleyPoolParams::new(
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
pub struct ShelleyPoolRegistration(cml_multi_era::shelley::ShelleyPoolRegistration);

impl_wasm_cbor_json_api!(ShelleyPoolRegistration);

impl_wasm_conversions!(
    cml_multi_era::shelley::ShelleyPoolRegistration,
    ShelleyPoolRegistration
);

#[wasm_bindgen]
impl ShelleyPoolRegistration {
    pub fn pool_params(&self) -> ShelleyPoolParams {
        self.0.pool_params.clone().into()
    }

    pub fn new(pool_params: &ShelleyPoolParams) -> Self {
        Self(cml_multi_era::shelley::ShelleyPoolRegistration::new(
            pool_params.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ShelleyProposedProtocolParameterUpdates(
    cml_multi_era::shelley::ShelleyProposedProtocolParameterUpdates,
);

impl_wasm_conversions!(
    cml_multi_era::shelley::ShelleyProposedProtocolParameterUpdates,
    ShelleyProposedProtocolParameterUpdates
);

#[wasm_bindgen]
impl ShelleyProposedProtocolParameterUpdates {
    pub fn new() -> Self {
        Self(OrderedHashMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(
        &mut self,
        key: &GenesisHash,
        value: &ShelleyProtocolParamUpdate,
    ) -> Option<ShelleyProtocolParamUpdate> {
        self.0
            .insert(key.clone().into(), value.clone().into())
            .map(Into::into)
    }

    pub fn get(&self, key: &GenesisHash) -> Option<ShelleyProtocolParamUpdate> {
        self.0.get(key.as_ref()).map(|v| v.clone().into())
    }

    pub fn keys(&self) -> GenesisHashList {
        self.0.iter().map(|(k, _v)| *k).collect::<Vec<_>>().into()
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ShelleyProtocolParamUpdate(cml_multi_era::shelley::ShelleyProtocolParamUpdate);

impl_wasm_cbor_json_api!(ShelleyProtocolParamUpdate);

impl_wasm_conversions!(
    cml_multi_era::shelley::ShelleyProtocolParamUpdate,
    ShelleyProtocolParamUpdate
);

#[wasm_bindgen]
impl ShelleyProtocolParamUpdate {
    pub fn set_minfee_a(&mut self, minfee_a: u64) {
        self.0.minfee_a = Some(minfee_a)
    }

    pub fn minfee_a(&self) -> Option<u64> {
        self.0.minfee_a
    }

    pub fn set_minfee_b(&mut self, minfee_b: u64) {
        self.0.minfee_b = Some(minfee_b)
    }

    pub fn minfee_b(&self) -> Option<u64> {
        self.0.minfee_b
    }

    pub fn set_max_block_body_size(&mut self, max_block_body_size: u64) {
        self.0.max_block_body_size = Some(max_block_body_size)
    }

    pub fn max_block_body_size(&self) -> Option<u64> {
        self.0.max_block_body_size
    }

    pub fn set_max_transaction_size(&mut self, max_transaction_size: u64) {
        self.0.max_transaction_size = Some(max_transaction_size)
    }

    pub fn max_transaction_size(&self) -> Option<u64> {
        self.0.max_transaction_size
    }

    pub fn set_max_block_header_size(&mut self, max_block_header_size: u64) {
        self.0.max_block_header_size = Some(max_block_header_size)
    }

    pub fn max_block_header_size(&self) -> Option<u64> {
        self.0.max_block_header_size
    }

    pub fn set_key_deposit(&mut self, key_deposit: Coin) {
        self.0.key_deposit = Some(key_deposit)
    }

    pub fn key_deposit(&self) -> Option<Coin> {
        self.0.key_deposit
    }

    pub fn set_pool_deposit(&mut self, pool_deposit: Coin) {
        self.0.pool_deposit = Some(pool_deposit)
    }

    pub fn pool_deposit(&self) -> Option<Coin> {
        self.0.pool_deposit
    }

    pub fn set_maximum_epoch(&mut self, maximum_epoch: Epoch) {
        self.0.maximum_epoch = Some(maximum_epoch)
    }

    pub fn maximum_epoch(&self) -> Option<Epoch> {
        self.0.maximum_epoch
    }

    pub fn set_n_opt(&mut self, n_opt: u64) {
        self.0.n_opt = Some(n_opt)
    }

    pub fn n_opt(&self) -> Option<u64> {
        self.0.n_opt
    }

    pub fn set_pool_pledge_influence(&mut self, pool_pledge_influence: &Rational) {
        self.0.pool_pledge_influence = Some(pool_pledge_influence.clone().into())
    }

    pub fn pool_pledge_influence(&self) -> Option<Rational> {
        self.0
            .pool_pledge_influence
            .clone()
            .map(std::convert::Into::into)
    }

    pub fn set_expansion_rate(&mut self, expansion_rate: &UnitInterval) {
        self.0.expansion_rate = Some(expansion_rate.clone().into())
    }

    pub fn expansion_rate(&self) -> Option<UnitInterval> {
        self.0.expansion_rate.clone().map(std::convert::Into::into)
    }

    pub fn set_treasury_growth_rate(&mut self, treasury_growth_rate: &UnitInterval) {
        self.0.treasury_growth_rate = Some(treasury_growth_rate.clone().into())
    }

    pub fn treasury_growth_rate(&self) -> Option<UnitInterval> {
        self.0
            .treasury_growth_rate
            .clone()
            .map(std::convert::Into::into)
    }

    pub fn set_decentralization_constant(&mut self, decentralization_constant: &UnitInterval) {
        self.0.decentralization_constant = Some(decentralization_constant.clone().into())
    }

    pub fn decentralization_constant(&self) -> Option<UnitInterval> {
        self.0
            .decentralization_constant
            .clone()
            .map(std::convert::Into::into)
    }

    pub fn set_extra_entropy(&mut self, extra_entropy: &Nonce) {
        self.0.extra_entropy = Some(extra_entropy.clone().into())
    }

    pub fn extra_entropy(&self) -> Option<Nonce> {
        self.0.extra_entropy.clone().map(std::convert::Into::into)
    }

    pub fn set_protocol_version(&mut self, protocol_version: &ProtocolVersionStruct) {
        self.0.protocol_version = Some(protocol_version.clone().into())
    }

    pub fn protocol_version(&self) -> Option<ProtocolVersionStruct> {
        self.0
            .protocol_version
            .clone()
            .map(std::convert::Into::into)
    }

    pub fn set_min_utxo_value(&mut self, min_utxo_value: Coin) {
        self.0.min_utxo_value = Some(min_utxo_value)
    }

    pub fn min_utxo_value(&self) -> Option<Coin> {
        self.0.min_utxo_value
    }

    pub fn new() -> Self {
        Self(cml_multi_era::shelley::ShelleyProtocolParamUpdate::new())
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ShelleyRelay(cml_multi_era::shelley::ShelleyRelay);

impl_wasm_cbor_json_api!(ShelleyRelay);

impl_wasm_conversions!(cml_multi_era::shelley::ShelleyRelay, ShelleyRelay);

#[wasm_bindgen]
impl ShelleyRelay {
    pub fn new_single_host_addr(
        port: Option<Port>,
        ipv4: Option<Ipv4>,
        ipv6: Option<Ipv6>,
    ) -> Self {
        Self(cml_multi_era::shelley::ShelleyRelay::new_single_host_addr(
            port,
            ipv4.map(Into::into),
            ipv6.map(Into::into),
        ))
    }

    pub fn new_shelley_single_host_name(
        port: Option<Port>,
        shelley_dns_name: &ShelleyDNSName,
    ) -> Self {
        Self(
            cml_multi_era::shelley::ShelleyRelay::new_shelley_single_host_name(
                port,
                shelley_dns_name.clone().into(),
            ),
        )
    }

    pub fn new_shelley_multi_host_name(shelley_dns_name: &ShelleyDNSName) -> Self {
        Self(
            cml_multi_era::shelley::ShelleyRelay::new_shelley_multi_host_name(
                shelley_dns_name.clone().into(),
            ),
        )
    }

    pub fn kind(&self) -> ShelleyRelayKind {
        match &self.0 {
            cml_multi_era::shelley::ShelleyRelay::SingleHostAddr(_) => {
                ShelleyRelayKind::SingleHostAddr
            }
            cml_multi_era::shelley::ShelleyRelay::ShelleySingleHostName(_) => {
                ShelleyRelayKind::ShelleySingleHostName
            }
            cml_multi_era::shelley::ShelleyRelay::ShelleyMultiHostName(_) => {
                ShelleyRelayKind::ShelleyMultiHostName
            }
        }
    }

    pub fn as_single_host_addr(&self) -> Option<SingleHostAddr> {
        match &self.0 {
            cml_multi_era::shelley::ShelleyRelay::SingleHostAddr(single_host_addr) => {
                Some(single_host_addr.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_shelley_single_host_name(&self) -> Option<ShelleySingleHostName> {
        match &self.0 {
            cml_multi_era::shelley::ShelleyRelay::ShelleySingleHostName(
                shelley_single_host_name,
            ) => Some(shelley_single_host_name.clone().into()),
            _ => None,
        }
    }

    pub fn as_shelley_multi_host_name(&self) -> Option<ShelleyMultiHostName> {
        match &self.0 {
            cml_multi_era::shelley::ShelleyRelay::ShelleyMultiHostName(shelley_multi_host_name) => {
                Some(shelley_multi_host_name.clone().into())
            }
            _ => None,
        }
    }
}

#[wasm_bindgen]
pub enum ShelleyRelayKind {
    SingleHostAddr,
    ShelleySingleHostName,
    ShelleyMultiHostName,
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ShelleySingleHostName(cml_multi_era::shelley::ShelleySingleHostName);

impl_wasm_cbor_json_api!(ShelleySingleHostName);

impl_wasm_conversions!(
    cml_multi_era::shelley::ShelleySingleHostName,
    ShelleySingleHostName
);

#[wasm_bindgen]
impl ShelleySingleHostName {
    pub fn port(&self) -> Option<Port> {
        self.0.port
    }

    pub fn shelley_dns_name(&self) -> ShelleyDNSName {
        self.0.shelley_dns_name.clone().into()
    }

    /// * `shelley_dns_name` - An A or AAAA DNS record
    pub fn new(port: Option<Port>, shelley_dns_name: &ShelleyDNSName) -> Self {
        Self(cml_multi_era::shelley::ShelleySingleHostName::new(
            port,
            shelley_dns_name.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ShelleyTransaction(cml_multi_era::shelley::ShelleyTransaction);

impl_wasm_cbor_json_api!(ShelleyTransaction);

impl_wasm_conversions!(
    cml_multi_era::shelley::ShelleyTransaction,
    ShelleyTransaction
);

#[wasm_bindgen]
impl ShelleyTransaction {
    pub fn body(&self) -> ShelleyTransactionBody {
        self.0.body.clone().into()
    }

    pub fn witness_set(&self) -> ShelleyTransactionWitnessSet {
        self.0.witness_set.clone().into()
    }

    pub fn metadata(&self) -> Option<Metadata> {
        self.0.metadata.clone().map(std::convert::Into::into)
    }

    pub fn new(
        body: &ShelleyTransactionBody,
        witness_set: &ShelleyTransactionWitnessSet,
        metadata: Option<Metadata>,
    ) -> Self {
        Self(cml_multi_era::shelley::ShelleyTransaction::new(
            body.clone().into(),
            witness_set.clone().into(),
            metadata.map(Into::into),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ShelleyTransactionBody(cml_multi_era::shelley::ShelleyTransactionBody);

impl_wasm_cbor_json_api!(ShelleyTransactionBody);

impl_wasm_conversions!(
    cml_multi_era::shelley::ShelleyTransactionBody,
    ShelleyTransactionBody
);

#[wasm_bindgen]
impl ShelleyTransactionBody {
    pub fn inputs(&self) -> TransactionInputList {
        self.0.inputs.clone().into()
    }

    pub fn outputs(&self) -> ShelleyTransactionOutputList {
        self.0.outputs.clone().into()
    }

    pub fn fee(&self) -> Coin {
        self.0.fee
    }

    pub fn ttl(&self) -> u64 {
        self.0.ttl
    }

    pub fn set_certs(&mut self, certs: &ShelleyCertificateList) {
        self.0.certs = Some(certs.clone().into())
    }

    pub fn certs(&self) -> Option<ShelleyCertificateList> {
        self.0.certs.clone().map(std::convert::Into::into)
    }

    pub fn set_withdrawals(&mut self, withdrawals: &Withdrawals) {
        self.0.withdrawals = Some(withdrawals.clone().into())
    }

    pub fn withdrawals(&self) -> Option<Withdrawals> {
        self.0.withdrawals.clone().map(std::convert::Into::into)
    }

    pub fn set_update(&mut self, update: &ShelleyUpdate) {
        self.0.update = Some(update.clone().into())
    }

    pub fn update(&self) -> Option<ShelleyUpdate> {
        self.0.update.clone().map(std::convert::Into::into)
    }

    pub fn set_auxiliary_data_hash(&mut self, auxiliary_data_hash: &AuxiliaryDataHash) {
        self.0.auxiliary_data_hash = Some(auxiliary_data_hash.clone().into())
    }

    pub fn auxiliary_data_hash(&self) -> Option<AuxiliaryDataHash> {
        self.0.auxiliary_data_hash.map(std::convert::Into::into)
    }

    pub fn new(
        inputs: &TransactionInputList,
        outputs: &ShelleyTransactionOutputList,
        fee: Coin,
        ttl: u64,
    ) -> Self {
        Self(cml_multi_era::shelley::ShelleyTransactionBody::new(
            inputs.clone().into(),
            outputs.clone().into(),
            fee,
            ttl,
        ))
    }
}

pub type ShelleyTransactionIndex = u16;

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ShelleyTransactionOutput(cml_multi_era::shelley::ShelleyTransactionOutput);

impl_wasm_cbor_json_api!(ShelleyTransactionOutput);

impl_wasm_conversions!(
    cml_multi_era::shelley::ShelleyTransactionOutput,
    ShelleyTransactionOutput
);

#[wasm_bindgen]
impl ShelleyTransactionOutput {
    pub fn address(&self) -> Address {
        self.0.address.clone().into()
    }

    pub fn amount(&self) -> Coin {
        self.0.amount
    }

    pub fn new(address: &Address, amount: Coin) -> Self {
        Self(cml_multi_era::shelley::ShelleyTransactionOutput::new(
            address.clone().into(),
            amount,
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ShelleyTransactionWitnessSet(cml_multi_era::shelley::ShelleyTransactionWitnessSet);

impl_wasm_cbor_json_api!(ShelleyTransactionWitnessSet);

impl_wasm_conversions!(
    cml_multi_era::shelley::ShelleyTransactionWitnessSet,
    ShelleyTransactionWitnessSet
);

#[wasm_bindgen]
impl ShelleyTransactionWitnessSet {
    pub fn set_vkeywitnesses(&mut self, vkeywitnesses: &VkeywitnessList) {
        self.0.vkeywitnesses = Some(vkeywitnesses.clone().into())
    }

    pub fn vkeywitnesses(&self) -> Option<VkeywitnessList> {
        self.0.vkeywitnesses.clone().map(std::convert::Into::into)
    }

    pub fn set_native_scripts(&mut self, native_scripts: &MultisigScriptList) {
        self.0.native_scripts = Some(native_scripts.clone().into())
    }

    pub fn native_scripts(&self) -> Option<MultisigScriptList> {
        self.0.native_scripts.clone().map(std::convert::Into::into)
    }

    pub fn set_bootstrap_witnesses(&mut self, bootstrap_witnesses: &BootstrapWitnessList) {
        self.0.bootstrap_witnesses = Some(bootstrap_witnesses.clone().into())
    }

    pub fn bootstrap_witnesses(&self) -> Option<BootstrapWitnessList> {
        self.0
            .bootstrap_witnesses
            .clone()
            .map(std::convert::Into::into)
    }

    pub fn new() -> Self {
        Self(cml_multi_era::shelley::ShelleyTransactionWitnessSet::new())
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ShelleyUpdate(cml_multi_era::shelley::ShelleyUpdate);

impl_wasm_cbor_json_api!(ShelleyUpdate);

impl_wasm_conversions!(cml_multi_era::shelley::ShelleyUpdate, ShelleyUpdate);

#[wasm_bindgen]
impl ShelleyUpdate {
    pub fn shelley_proposed_protocol_parameter_updates(
        &self,
    ) -> ShelleyProposedProtocolParameterUpdates {
        self.0
            .shelley_proposed_protocol_parameter_updates
            .clone()
            .into()
    }

    pub fn epoch(&self) -> Epoch {
        self.0.epoch
    }

    pub fn new(
        shelley_proposed_protocol_parameter_updates: &ShelleyProposedProtocolParameterUpdates,
        epoch: Epoch,
    ) -> Self {
        Self(cml_multi_era::shelley::ShelleyUpdate::new(
            shelley_proposed_protocol_parameter_updates.clone().into(),
            epoch,
        ))
    }
}
