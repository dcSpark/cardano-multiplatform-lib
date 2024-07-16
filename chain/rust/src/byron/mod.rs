#[cfg(not(feature = "used_from_wasm"))]
use noop_proc_macro::wasm_bindgen;
#[cfg(feature = "used_from_wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

use std::io::{BufRead, Write};

use cml_crypto::{chain_crypto::hash::Blake2b224, Bip32PublicKey, PublicKey};

use crate::Coin;

// This library was code-generated using an experimental CDDL to rust tool:
// https://github.com/Emurgo/cddl-codegen

use cbor_event::{self, de::Deserializer, se::Serializer};

pub use self::crc32::Crc32;
pub use cml_core::network::ProtocolMagic;
pub use utils::{
    make_daedalus_bootstrap_witness, make_icarus_bootstrap_witness, AddressId, ByronAddressError,
    ByronScript, ParseExtendedAddrError, StakeholderId,
};

mod base58;
mod crc32;
mod serialization;
mod utils;

//#![allow(clippy::too_many_arguments)]

// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    serde::Deserialize,
    serde::Serialize,
    schemars::JsonSchema,
)]
pub struct AddrAttributes {
    pub stake_distribution: Option<StakeDistribution>,
    pub derivation_path: Option<HDAddressPayload>,
    pub protocol_magic: Option<ProtocolMagic>,
}

impl AddrAttributes {
    pub fn new() -> Self {
        Self {
            stake_distribution: None,
            derivation_path: None,
            protocol_magic: None,
        }
    }
}

impl Default for AddrAttributes {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(
    Copy,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Clone,
    Debug,
    Hash,
    serde::Deserialize,
    serde::Serialize,
    schemars::JsonSchema,
)]
#[wasm_bindgen]
pub enum ByronAddrType {
    PublicKey = 0,
    Script = 1,
    Redeem = 2,
}

#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    serde::Deserialize,
    serde::Serialize,
    schemars::JsonSchema,
)]
pub struct AddressContent {
    pub address_id: AddressId,
    pub addr_attributes: AddrAttributes,
    pub addr_type: ByronAddrType,
}

impl AddressContent {
    pub fn new(
        address_id: AddressId,
        addr_attributes: AddrAttributes,
        addr_type: ByronAddrType,
    ) -> Self {
        Self {
            address_id,
            addr_attributes,
            addr_type,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ByronAddress {
    pub content: AddressContent,
    pub crc: Crc32,
}

impl ByronAddress {
    /// Create a ByronAddress from an already calculated CRC32.
    /// Does not validate the crc whatsoever.
    /// use From<AddressContent> to calculate the CRC from the content
    pub fn new(content: AddressContent, crc: Crc32) -> Self {
        Self { content, crc }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ByronTxOut {
    pub address: ByronAddress,
    pub amount: Coin,
}

impl ByronTxOut {
    pub fn new(address: ByronAddress, amount: Coin) -> Self {
        Self { address, amount }
    }
}

#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    serde::Deserialize,
    serde::Serialize,
    schemars::JsonSchema,
)]
pub struct HDAddressPayload(pub Vec<u8>);

impl HDAddressPayload {
    pub fn get(&self) -> &Vec<u8> {
        &self.0
    }

    pub fn new(inner: Vec<u8>) -> Self {
        Self(inner)
    }
}

impl From<Vec<u8>> for HDAddressPayload {
    fn from(inner: Vec<u8>) -> Self {
        HDAddressPayload::new(inner)
    }
}

impl From<HDAddressPayload> for Vec<u8> {
    fn from(wrapper: HDAddressPayload) -> Self {
        wrapper.0
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub enum SpendingData {
    SpendingDataPubKey(Bip32PublicKey),
    SpendingDataScript(ByronScript),
    SpendingDataRedeem(PublicKey),
}

impl SpendingData {
    pub fn new_spending_data_pub_key(pubkey: Bip32PublicKey) -> Self {
        Self::SpendingDataPubKey(pubkey)
    }

    pub fn new_spending_data_script(script: ByronScript) -> Self {
        Self::SpendingDataScript(script)
    }

    pub fn new_spending_data_redeem(redeem: PublicKey) -> Self {
        Self::SpendingDataRedeem(redeem)
    }
}

#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    serde::Deserialize,
    serde::Serialize,
    schemars::JsonSchema,
)]
pub enum StakeDistribution {
    SingleKey(StakeholderId),
    BootstrapEra,
}

impl StakeDistribution {
    pub fn new_single_key(stakeholder_id: StakeholderId) -> Self {
        Self::SingleKey(stakeholder_id)
    }

    pub fn new_bootstrap_era() -> Self {
        Self::BootstrapEra
    }
}

// #[cfg(test)]
// mod tests {
//     use cml_core::serialization::ToBytes;
//     use super::*;

//     #[test]
//     fn tx_output_decoding() {
//         let tx_out = ByronTxOut::from_bytes(
//             hex::decode("8282d818582183581cc6eb29e2cbb7b616b28c83da505a08253c33ec371319261ad93e558ca0001a1102942c1b00000005f817ddfc").unwrap()
//         ).unwrap();
//         assert_eq!(tx_out.address().to_base58(), "Ae2tdPwUPEZGexC4LXgsr1BJ1PppXk71zpuRkboFopVpSDcykQvpyYJXCJf");
//         assert!(tx_out.to_json().unwrap().contains("Ae2tdPwUPEZGexC4LXgsr1BJ1PppXk71zpuRkboFopVpSDcykQvpyYJXCJf"));
//     }
// }
