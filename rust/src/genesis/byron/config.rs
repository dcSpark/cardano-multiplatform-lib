//! Blockchain network specific config (ProtocolMagic)
//!
//! there are some settings that need to be set in order to guarantee
//! operability with the appropriate network or different option.
//!

use cbor_event::{self, de::Deserializer, se::Serializer};
use std::{
    collections::BTreeMap,
    fmt,
    io::{BufRead, Write},
    time::{Duration, SystemTime},
};

use crate::{legacy_address, utils, fees, chain_crypto::{Ed25519Bip32, self, Ed25519}, crypto::BlockHeaderHash, genesis::network_info::NetworkInfo};


/// this is the protocol magic number
///
/// it is meant to be used on some places in order to guarantee
/// incompatibility between forks, test network and the main-net.
///
/// # Default
///
/// The default value is set to the mainnet
///
/// ```
/// use cardano_multiplatform_lib::genesis::byron::config::ProtocolMagic;
///
/// assert_eq!(ProtocolMagic::default(), ProtocolMagic::new(0x2D964A09));
/// ```
///
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
#[cfg_attr(feature = "generic-serialization", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ProtocolMagic(pub u32);
impl ProtocolMagic {
    #[deprecated]
    pub fn new(val: u32) -> Self {
        ProtocolMagic(val)
    }
}
impl fmt::Display for ProtocolMagic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl ::std::ops::Deref for ProtocolMagic {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<u32> for ProtocolMagic {
    fn from(v: u32) -> Self {
        ProtocolMagic(v)
    }
}
impl Default for ProtocolMagic {
    fn default() -> Self {
        NetworkInfo::mainnet().protocol_magic()
    }
}
impl cbor_event::se::Serialize for ProtocolMagic {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(self.0 as u64)
    }
}
impl cbor_event::Deserialize for ProtocolMagic {
    fn deserialize<R: BufRead>(reader: &mut Deserializer<R>) -> cbor_event::Result<Self> {
        let v = reader.unsigned_integer()? as u32;
        Ok(ProtocolMagic::from(v))
    }
}

/// Configuration for the wallet-crypto
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[cfg_attr(feature = "generic-serialization", derive(Serialize, Deserialize))]
pub struct Config {
    pub protocol_magic: ProtocolMagic,
}
impl Config {
    pub fn new(protocol_magic: ProtocolMagic) -> Self {
        Config {
            protocol_magic: protocol_magic,
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
    pub fee_policy: fees::LinearFee,
    pub avvm_distr: BTreeMap<chain_crypto::PublicKey<Ed25519>, utils::Coin>, // AVVM = Ada Voucher Vending Machine
    // TODO: convert to ByronAddress without affecting order
    pub non_avvm_balances: BTreeMap<legacy_address::Addr, utils::Coin>,
    pub boot_stakeholders: BTreeMap<legacy_address::StakeholderId, BootStakeholder>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootStakeholder {
    pub weight: BootStakeWeight,
    pub issuer_pk: chain_crypto::PublicKey<Ed25519Bip32>,
    pub delegate_pk: chain_crypto::PublicKey<Ed25519Bip32>,
    pub cert: chain_crypto::Signature<(), Ed25519Bip32>,
}

pub type BootStakeWeight = u16;
