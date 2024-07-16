use cml_crypto::{Ed25519KeyHash, VRFKeyHash};
use fraction::Fraction;
use std::collections::BTreeMap;

use crate::{address::Address, block::ProtocolVersion, Coin};

/// A subset of the Shelley genesis data. The genesis data is a JSON file
/// is something completely different from a epoch genesis block and the Byron genesis block
#[derive(Debug, Clone)]
pub struct ShelleyGenesisData {
    pub active_slots_coeff: Fraction,
    pub epoch_length: u64,
    pub gen_delegs: BTreeMap<Ed25519KeyHash, ShelleyGenesisDelegations>,
    pub initial_funds: BTreeMap<Address, Coin>,
    pub max_kes_evolutions: u64,
    pub max_lovelace_supply: Coin,
    pub network_id: u64,
    pub network_magic: u64,
    pub protocol_params: ShelleyGenesisProtocolParameters,
    pub security_param: u64,
    pub slot_length: u64,
    pub slots_per_kes_period: u64,
    pub staking: Option<ShelleyGenesisStaking>,
    pub system_start: chrono::DateTime<chrono::Utc>,
    pub update_quorum: u64,
}

#[derive(Debug, Clone)]
pub struct ShelleyGenesisDelegations {
    pub delegate: Ed25519KeyHash,
    pub vrf: VRFKeyHash,
}

#[derive(Debug, Clone)]
pub struct ShelleyGenesisStaking {
    pub pools: BTreeMap<Ed25519KeyHash, crate::certs::PoolParams>,
    // initial delegations of staking key -> pool id
    pub stake: BTreeMap<Ed25519KeyHash, Ed25519KeyHash>,
}

#[derive(Debug, Clone)]
pub struct ShelleyGenesisProtocolParameters {
    pub a0: Fraction,
    pub decentralisation_param: Fraction,
    pub e_max: u64,
    pub extra_entropy: ShelleyGenesisExtraEntropy,
    pub key_deposit: Coin,
    pub max_block_body_size: u64,
    pub max_block_header_size: u64,
    pub max_tx_size: u64,
    pub min_fee_a: Coin,
    pub min_fee_b: Coin,
    pub min_pool_cost: Coin,
    pub min_utxo_value: Coin,
    pub n_opt: u64,
    pub pool_deposit: Coin,
    pub protocol_version: ProtocolVersion,
    pub rho: Fraction,
    pub tau: Fraction,
}

#[derive(Debug, Clone)]
pub struct ShelleyGenesisExtraEntropy {
    pub tag: String,
}
