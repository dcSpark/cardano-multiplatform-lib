use crate::assets::Coin;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Parsing of the JSON representation of the Shelley genesis block
/// Note: for a lot of these fields, I didn't check what the max valid size is in the Haskell code
///       so I just used u64 everywhere

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct ShelleyGenesisData {
    // convert lossless JSON floats to string to avoid lossy Rust f64
    pub activeSlotsCoeff: serde_json::Number,
    pub epochLength: u64,
    pub genDelegs: HashMap<String, ShelleyGenesisDelegations>,
    pub initialFunds: HashMap<String, Coin>,
    pub maxKESEvolutions: u64,
    pub maxLovelaceSupply: Coin,
    pub networkId: String,
    pub networkMagic: u64,
    pub protocolParams: ShelleyGenesisProtocolParameters,
    pub securityParam: u64,
    pub slotLength: serde_json::Number,
    pub slotsPerKESPeriod: u64,
    pub staking: Option<ShelleyGenesisStaking>,
    pub systemStart: String,
    pub updateQuorum: u64,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct ShelleyGenesisDelegations {
    pub delegate: String,
    pub vrf: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct ShelleyGenesisProtocolParameters {
    // this uses the serde_json arbitrary_precision feature to avoid lossy Rust f64
    pub a0: serde_json::Number,
    // this uses the serde_json arbitrary_precision feature to avoid lossy Rust f64
    pub decentralisationParam: serde_json::Number,
    pub eMax: u64,
    pub extraEntropy: ShelleyGenesisExtraEntropy,
    pub keyDeposit: Coin,
    pub maxBlockBodySize: u64,
    pub maxBlockHeaderSize: u64,
    pub maxTxSize: u64,
    pub minFeeA: Coin,
    pub minFeeB: Coin,
    pub minPoolCost: Coin,
    pub minUTxOValue: Coin,
    pub nOpt: u64,
    pub poolDeposit: Coin,
    pub protocolVersion: ShelleyGenesisProtocolVersion,
    // this uses the serde_json arbitrary_precision feature to avoid lossy Rust f64
    pub rho: serde_json::Number,
    // this uses the serde_json arbitrary_precision feature to avoid lossy Rust f64
    pub tau: serde_json::Number,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct ShelleyGenesisExtraEntropy {
    pub tag: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct ShelleyGenesisProtocolVersion {
    pub major: u64,
    pub minor: u64,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct ShelleyGenesisStaking {
    pub pools: HashMap<String, ShelleyGenesisPool>,
    pub stake: HashMap<String, String>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct ShelleyGenesisPool {
    pub cost: Coin,
    // this uses the serde_json arbitrary_precision feature to avoid lossy Rust f64
    pub margin: serde_json::Number,
    pub metadata: Option<ShelleyGenesisPoolMetadata>,
    pub owners: Vec<String>,
    pub pledge: Coin,
    pub publicKey: String,
    pub relays: Vec<RelayTypeMap>,
    pub rewardAccount: ShelleyGenesisRewardAccount,
    pub vrf: String,
}

// TODO: there are other relay types, but I can't find the JSON type for them
//       and I can't find any usage of them in the wild anyway
// The key here defines the relay type
// ex:
// - single host address
type RelayTypeMap = HashMap<String, ShelleyGenesisPoolSingleHotsRelay>;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct ShelleyGenesisPoolSingleHotsRelay {
    pub IPv6: Option<String>,
    pub port: Option<u16>,
    pub IPv4: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct ShelleyGenesisPoolMetadata {
    pub hash: String,
    pub url: String,
}
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct ShelleyGenesisRewardAccount {
    pub network: String,
    pub credential: ShelleyGenesisCredential,
}
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct ShelleyGenesisCredential {
    // for some reason there actually is a space in the JSON key emitted by the Haskell node
    // both key hash and keyHash are accepted
    #[serde(alias = "key hash")]
    pub keyHash: String,
}
