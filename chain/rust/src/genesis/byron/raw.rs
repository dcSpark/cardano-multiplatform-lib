use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::config;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct GenesisData {
    pub avvmDistr: HashMap<String, String>,
    pub nonAvvmBalances: HashMap<String, String>,
    pub bootStakeholders: HashMap<String, config::BootStakeWeight>,
    pub heavyDelegation: HashMap<String, HeavyDelegation>,
    pub protocolConsts: ProtocolConsts,
    pub startTime: u64,
    pub blockVersionData: BlockVersionData,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct ProtocolConsts {
    pub k: usize,
    pub protocolMagic: u32,
    //pub vssMaxTTL: u32,
    //pub vssMinTTL: u32,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct BlockVersionData {
    //pub heavyDelThd: String,
    //pub maxBlockSize: String,
    //pub maxHeaderSize: String,
    //pub maxProposalSize: String,
    //pub maxTxSize: String,
    //pub mpcThd: String,
    //pub scriptVersion: u32,
    pub slotDuration: String,
    //pub softforkRule: SoftforkRule,
    pub txFeePolicy: TxFeePolicy,
    //pub unlockStakeEpoch: String,
    //pub updateImplicit: String,
    //pub updateProposalThd: String,
    //pub updateVoteThd: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct TxFeePolicy {
    pub summand: String,
    pub multiplier: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct SoftforkRule {
    pub initThd: String,
    pub minThd: String,
    pub thdDecrement: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct HeavyDelegation {
    pub issuerPk: String,
    pub delegatePk: String,
    pub cert: String,
}
