#![allow(
    clippy::len_without_is_empty,
    clippy::too_many_arguments,
    clippy::new_without_default
)]
// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

pub mod crc32;
pub mod utils;

pub use self::crc32::Crc32;
pub use self::utils::{AddressId, ByronScript, ProtocolMagic, StakeholderId};
pub use cml_chain::byron::ByronAddrType;
use cml_core_wasm::{impl_wasm_cbor_event_serialize_api, impl_wasm_conversions};
use cml_crypto_wasm::{Bip32PublicKey, PublicKey};
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct AddrAttributes(cml_chain::byron::AddrAttributes);

impl_wasm_conversions!(cml_chain::byron::AddrAttributes, AddrAttributes);

impl_wasm_cbor_event_serialize_api!(AddrAttributes);

#[wasm_bindgen]
impl AddrAttributes {
    pub fn set_stake_distribution(&mut self, stake_distribution: &StakeDistribution) {
        self.0.stake_distribution = Some(stake_distribution.clone().into())
    }

    pub fn stake_distribution(&self) -> Option<StakeDistribution> {
        self.0
            .stake_distribution
            .clone()
            .map(std::convert::Into::into)
    }

    pub fn set_derivation_path(&mut self, derivation_path: &HDAddressPayload) {
        self.0.derivation_path = Some(derivation_path.clone().into())
    }

    pub fn derivation_path(&self) -> Option<HDAddressPayload> {
        self.0.derivation_path.clone().map(std::convert::Into::into)
    }

    pub fn set_protocol_magic(&mut self, protocol_magic: &ProtocolMagic) {
        self.0.protocol_magic = Some((*protocol_magic).into())
    }

    pub fn protocol_magic(&self) -> Option<ProtocolMagic> {
        self.0.protocol_magic.map(std::convert::Into::into)
    }

    pub fn new() -> Self {
        Self(cml_chain::byron::AddrAttributes::new())
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct AddressContent(cml_chain::byron::AddressContent);

impl_wasm_conversions!(cml_chain::byron::AddressContent, AddressContent);

impl_wasm_cbor_event_serialize_api!(AddressContent);

#[wasm_bindgen]
impl AddressContent {
    pub fn address_id(&self) -> AddressId {
        self.0.address_id.into()
    }

    pub fn addr_attributes(&self) -> AddrAttributes {
        self.0.addr_attributes.clone().into()
    }

    pub fn addr_type(&self) -> ByronAddrType {
        self.0.addr_type
    }

    pub fn new(
        address_id: &AddressId,
        addr_attributes: &AddrAttributes,
        addr_type: ByronAddrType,
    ) -> Self {
        Self(cml_chain::byron::AddressContent::new(
            address_id.clone().into(),
            addr_attributes.clone().into(),
            addr_type,
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronAddress(cml_chain::byron::ByronAddress);

impl_wasm_conversions!(cml_chain::byron::ByronAddress, ByronAddress);

impl_wasm_cbor_event_serialize_api!(ByronAddress);

#[wasm_bindgen]
impl ByronAddress {
    pub fn content(&self) -> AddressContent {
        self.0.content.clone().into()
    }

    pub fn crc(&self) -> Crc32 {
        self.0.crc.into()
    }

    pub fn new(content: &AddressContent, crc: &Crc32) -> Self {
        Self(cml_chain::byron::ByronAddress::new(
            content.clone().into(),
            (*crc).into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct HDAddressPayload(cml_chain::byron::HDAddressPayload);

impl_wasm_conversions!(cml_chain::byron::HDAddressPayload, HDAddressPayload);

impl_wasm_cbor_event_serialize_api!(HDAddressPayload);

#[wasm_bindgen]
impl HDAddressPayload {
    pub fn get(&self) -> Vec<u8> {
        self.0.get().clone()
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct SpendingData(cml_chain::byron::SpendingData);

impl_wasm_conversions!(cml_chain::byron::SpendingData, SpendingData);

impl_wasm_cbor_event_serialize_api!(SpendingData);

#[wasm_bindgen]
impl SpendingData {
    pub fn new_spending_data_pub_key(pubkey: &Bip32PublicKey) -> Self {
        Self(cml_chain::byron::SpendingData::new_spending_data_pub_key(
            (*pubkey).clone().into(),
        ))
    }

    pub fn new_spending_data_script(script: &ByronScript) -> Self {
        Self(cml_chain::byron::SpendingData::new_spending_data_script(
            script.clone().into(),
        ))
    }

    pub fn new_spending_data_redeem(redeem: &PublicKey) -> Self {
        Self(cml_chain::byron::SpendingData::new_spending_data_redeem(
            redeem.clone().into(),
        ))
    }

    pub fn kind(&self) -> SpendingDataKind {
        match &self.0 {
            cml_chain::byron::SpendingData::SpendingDataPubKey(_) => {
                SpendingDataKind::SpendingDataPubKey
            }
            cml_chain::byron::SpendingData::SpendingDataScript(_) => {
                SpendingDataKind::SpendingDataScript
            }
            cml_chain::byron::SpendingData::SpendingDataRedeem(_) => {
                SpendingDataKind::SpendingDataRedeem
            }
        }
    }

    pub fn as_spending_data_pub_key(&self) -> Option<Bip32PublicKey> {
        match &self.0 {
            cml_chain::byron::SpendingData::SpendingDataPubKey(pubkey) => {
                Some(pubkey.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_spending_data_script(&self) -> Option<ByronScript> {
        match &self.0 {
            cml_chain::byron::SpendingData::SpendingDataScript(script) => Some((*script).into()),
            _ => None,
        }
    }

    pub fn as_spending_data_redeem(&self) -> Option<PublicKey> {
        match &self.0 {
            cml_chain::byron::SpendingData::SpendingDataRedeem(redeem) => {
                Some(redeem.clone().into())
            }
            _ => None,
        }
    }
}

#[wasm_bindgen]
pub enum SpendingDataKind {
    SpendingDataPubKey,
    SpendingDataScript,
    SpendingDataRedeem,
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct StakeDistribution(cml_chain::byron::StakeDistribution);

impl_wasm_conversions!(cml_chain::byron::StakeDistribution, StakeDistribution);

impl_wasm_cbor_event_serialize_api!(StakeDistribution);

#[wasm_bindgen]
impl StakeDistribution {
    pub fn new_single_key(stakeholder_id: &StakeholderId) -> Self {
        Self(cml_chain::byron::StakeDistribution::new_single_key(
            stakeholder_id.clone().into(),
        ))
    }

    pub fn new_bootstrap_era() -> Self {
        Self(cml_chain::byron::StakeDistribution::new_bootstrap_era())
    }

    pub fn kind(&self) -> StakeDistributionKind {
        match &self.0 {
            cml_chain::byron::StakeDistribution::SingleKey(_) => StakeDistributionKind::SingleKey,
            cml_chain::byron::StakeDistribution::BootstrapEra => {
                StakeDistributionKind::BootstrapEra
            }
        }
    }

    pub fn as_single_key(&self) -> Option<StakeholderId> {
        match &self.0 {
            cml_chain::byron::StakeDistribution::SingleKey(stakeholder_id) => {
                Some((*stakeholder_id).into())
            }
            _ => None,
        }
    }
}

#[wasm_bindgen]
pub enum StakeDistributionKind {
    SingleKey,
    BootstrapEra,
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronTxOut(cml_chain::byron::ByronTxOut);

impl_wasm_conversions!(cml_chain::byron::ByronTxOut, ByronTxOut);

impl_wasm_cbor_event_serialize_api!(ByronTxOut);

#[wasm_bindgen]
impl ByronTxOut {
    pub fn address(&self) -> ByronAddress {
        self.0.address.clone().into()
    }

    pub fn amount(&self) -> u64 {
        self.0.amount
    }

    pub fn new(address: &ByronAddress, amount: u64) -> Self {
        Self(cml_chain::byron::ByronTxOut::new(
            address.clone().into(),
            amount,
        ))
    }
}
