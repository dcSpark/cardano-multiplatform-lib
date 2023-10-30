//! Blockchain network specific config (ProtocolMagic)
//!
//! there are some settings that need to be set in order to guarantee
//! operability with the appropriate network or different option.
//!

use std::{
    collections::BTreeMap,
    time::{Duration, SystemTime},
};

use crate::{
    fees::LinearFee, Coin,
    byron::{ProtocolMagic, ByronAddress, StakeholderId}
};
use cml_crypto::{
    chain_crypto::{Ed25519Bip32, self, Ed25519}, BlockHeaderHash
};

/// Configuration for the wallet-crypto
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[cfg_attr(feature = "generic-serialization", derive(Serialize, Deserialize))]
pub struct Config {
    pub protocol_magic: ProtocolMagic,
}
impl Config {
    pub fn new(protocol_magic: ProtocolMagic) -> Self {
        Config {
            protocol_magic,
        }
    }
}
impl Default for Config {
    fn default() -> Self {
        Config::new(ProtocolMagic::default())
    }
}

/// A subset of the genesis data. The genesis data is a JSON file
/// whose canonicalized form has the hash 'genesis_prev', which is the
/// parent of the genesis block of epoch 0. (Note that "genesis data"
/// is something completely different from a epoch genesis block. The
/// genesis data is not stored in the chain as a block.)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenesisData {
    // FIXME: genesis_prev shouldn't be here since it's computed *from* the GenesisData.
    pub genesis_prev: BlockHeaderHash,
    pub epoch_stability_depth: usize, // a.k.a. 'k'
    pub start_time: SystemTime,
    pub slot_duration: Duration,
    pub protocol_magic: ProtocolMagic,
    pub fee_policy: LinearFee,
    pub avvm_distr: BTreeMap<chain_crypto::PublicKey<Ed25519>, Coin>, // AVVM = Ada Voucher Vending Machine
    // note: order of the keys here is unspecified in the spec (anything order is valid)
    pub non_avvm_balances: BTreeMap<ByronAddress, Coin>,
    pub boot_stakeholders: BTreeMap<StakeholderId, BootStakeholder>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootStakeholder {
    pub weight: BootStakeWeight,
    pub issuer_pk: chain_crypto::PublicKey<Ed25519Bip32>,
    pub delegate_pk: chain_crypto::PublicKey<Ed25519Bip32>,
    pub cert: chain_crypto::Signature<(), Ed25519Bip32>,
}

pub type BootStakeWeight = u16;
