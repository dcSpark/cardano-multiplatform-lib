// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

pub mod serialization;

use crate::byron::transaction::ByronAttributes;
use crate::byron::{Blake2b256, ByronPubKey, ByronSignature, ByronUpdateId, EpochId};
use cml_chain::utils::BigInteger;
use std::collections::BTreeMap;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct Bvermod {
    pub script_version: Vec<u16>,
    pub slot_duration: Vec<BigInteger>,
    pub max_block_size: Vec<BigInteger>,
    pub max_header_size: Vec<BigInteger>,
    pub max_tx_size: Vec<BigInteger>,
    pub max_proposal_size: Vec<BigInteger>,
    pub mpc_thd: Vec<u64>,
    pub heavy_del_thd: Vec<u64>,
    pub update_vote_thd: Vec<u64>,
    pub update_proposal_thd: Vec<u64>,
    pub update_implicit: Vec<u64>,
    pub soft_fork_rule: Vec<SoftForkRule>,
    pub tx_fee_policy: Vec<ByronTxFeePolicy>,
    pub unlock_stake_epoch: Vec<EpochId>,
}

impl Bvermod {
    pub fn new(
        script_version: Vec<u16>,
        slot_duration: Vec<BigInteger>,
        max_block_size: Vec<BigInteger>,
        max_header_size: Vec<BigInteger>,
        max_tx_size: Vec<BigInteger>,
        max_proposal_size: Vec<BigInteger>,
        mpc_thd: Vec<u64>,
        heavy_del_thd: Vec<u64>,
        update_vote_thd: Vec<u64>,
        update_proposal_thd: Vec<u64>,
        update_implicit: Vec<u64>,
        soft_fork_rule: Vec<SoftForkRule>,
        tx_fee_policy: Vec<ByronTxFeePolicy>,
        unlock_stake_epoch: Vec<EpochId>,
    ) -> Self {
        Self {
            script_version,
            slot_duration,
            max_block_size,
            max_header_size,
            max_tx_size,
            max_proposal_size,
            mpc_thd,
            heavy_del_thd,
            update_vote_thd,
            update_proposal_thd,
            update_implicit,
            soft_fork_rule,
            tx_fee_policy,
            unlock_stake_epoch,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ByronBlockVersion {
    pub u16: u16,
    pub u162: u16,
    pub u8: u8,
}

impl ByronBlockVersion {
    pub fn new(u16: u16, u162: u16, u8: u8) -> Self {
        Self { u16, u162, u8 }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ByronSoftwareVersion {
    pub application_name: String,
    pub u32: u32,
}

impl ByronSoftwareVersion {
    pub fn new(application_name: String, u32: u32) -> Self {
        Self {
            application_name,
            u32,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ByronTxFeePolicy {
    pub index_1: StdFeePolicy,
}

impl ByronTxFeePolicy {
    pub fn new(index_1: StdFeePolicy) -> Self {
        Self { index_1 }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ByronUpdate {
    pub proposal: Vec<ByronUpdateProposal>,
    pub votes: Vec<ByronUpdateVote>,
}

impl ByronUpdate {
    pub fn new(proposal: Vec<ByronUpdateProposal>, votes: Vec<ByronUpdateVote>) -> Self {
        Self { proposal, votes }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ByronUpdateData {
    pub blake2b256: Blake2b256,
    pub blake2b2562: Blake2b256,
    pub blake2b2563: Blake2b256,
    pub blake2b2564: Blake2b256,
}

impl ByronUpdateData {
    pub fn new(
        blake2b256: Blake2b256,
        blake2b2562: Blake2b256,
        blake2b2563: Blake2b256,
        blake2b2564: Blake2b256,
    ) -> Self {
        Self {
            blake2b256,
            blake2b2562,
            blake2b2563,
            blake2b2564,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ByronUpdateProposal {
    pub block_version: ByronBlockVersion,
    pub block_version_mod: Bvermod,
    pub software_version: ByronSoftwareVersion,
    pub data: BTreeMap<SystemTag, ByronUpdateData>,
    pub byron_attributes: ByronAttributes,
    pub from: ByronPubKey,
    pub signature: ByronSignature,
}

impl ByronUpdateProposal {
    pub fn new(
        block_version: ByronBlockVersion,
        block_version_mod: Bvermod,
        software_version: ByronSoftwareVersion,
        data: BTreeMap<SystemTag, ByronUpdateData>,
        byron_attributes: ByronAttributes,
        from: ByronPubKey,
        signature: ByronSignature,
    ) -> Self {
        Self {
            block_version,
            block_version_mod,
            software_version,
            data,
            byron_attributes,
            from,
            signature,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ByronUpdateVote {
    pub voter: ByronPubKey,
    pub proposal_id: ByronUpdateId,
    pub vote: bool,
    pub signature: ByronSignature,
}

impl ByronUpdateVote {
    pub fn new(
        voter: ByronPubKey,
        proposal_id: ByronUpdateId,
        vote: bool,
        signature: ByronSignature,
    ) -> Self {
        Self {
            voter,
            proposal_id,
            vote,
            signature,
        }
    }
}

pub type CoinPortion = u64;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct SoftForkRule {
    pub coin_portion: CoinPortion,
    pub coin_portion2: CoinPortion,
    pub coin_portion3: CoinPortion,
}

impl SoftForkRule {
    pub fn new(
        coin_portion: CoinPortion,
        coin_portion2: CoinPortion,
        coin_portion3: CoinPortion,
    ) -> Self {
        Self {
            coin_portion,
            coin_portion2,
            coin_portion3,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct StdFeePolicy {
    pub big_integer: BigInteger,
    pub big_integer2: BigInteger,
}

impl StdFeePolicy {
    pub fn new(big_integer: BigInteger, big_integer2: BigInteger) -> Self {
        Self {
            big_integer,
            big_integer2,
        }
    }
}

pub type SystemTag = String;
