#![allow(
    clippy::len_without_is_empty,
    clippy::too_many_arguments,
    clippy::new_without_default
)]
// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

pub mod crc32;
pub mod utils;

pub use cml_chain::byron::ByronAddrType;
use cml_crypto_wasm::{PublicKey, Bip32PublicKey};
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};
pub use self::crc32::Crc32;
pub use self::utils::{AddressId, ByronScript, StakeholderId, ProtocolMagic};

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct AddrAttributes(cml_chain::byron::AddrAttributes);

#[wasm_bindgen]
impl AddrAttributes {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_core::serialization::ToBytes::to_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<AddrAttributes, JsValue> {
        cml_core::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<AddrAttributes, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

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
        self.0.protocol_magic = Some(protocol_magic.clone().into())
    }

    pub fn protocol_magic(&self) -> Option<ProtocolMagic> {
        self.0.protocol_magic.clone().map(std::convert::Into::into)
    }

    pub fn new() -> Self {
        Self(cml_chain::byron::AddrAttributes::new())
    }
}

impl From<cml_chain::byron::AddrAttributes> for AddrAttributes {
    fn from(native: cml_chain::byron::AddrAttributes) -> Self {
        Self(native)
    }
}

impl From<AddrAttributes> for cml_chain::byron::AddrAttributes {
    fn from(wasm: AddrAttributes) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::byron::AddrAttributes> for AddrAttributes {
    fn as_ref(&self) -> &cml_chain::byron::AddrAttributes {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct AddressContent(cml_chain::byron::AddressContent);

#[wasm_bindgen]
impl AddressContent {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_core::serialization::ToBytes::to_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<AddressContent, JsValue> {
        cml_core::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<AddressContent, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn address_id(&self) -> AddressId {
        self.0.address_id.clone().into()
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
            addr_type.into(),
        ))
    }
}

impl From<cml_chain::byron::AddressContent> for AddressContent {
    fn from(native: cml_chain::byron::AddressContent) -> Self {
        Self(native)
    }
}

impl From<AddressContent> for cml_chain::byron::AddressContent {
    fn from(wasm: AddressContent) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::byron::AddressContent> for AddressContent {
    fn as_ref(&self) -> &cml_chain::byron::AddressContent {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronAddress(cml_chain::byron::ByronAddress);

#[wasm_bindgen]
impl ByronAddress {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_core::serialization::ToBytes::to_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<ByronAddress, JsValue> {
        cml_core::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<ByronAddress, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn content(&self) -> AddressContent {
        self.0.content.clone().into()
    }

    pub fn crc(&self) -> Crc32 {
        self.0.crc.clone().into()
    }

    pub fn new(content: &AddressContent, crc: &Crc32) -> Self {
        Self(cml_chain::byron::ByronAddress::new(
            content.clone().into(),
            crc.clone().into(),
        ))
    }
}

impl From<cml_chain::byron::ByronAddress> for ByronAddress {
    fn from(native: cml_chain::byron::ByronAddress) -> Self {
        Self(native)
    }
}

impl From<ByronAddress> for cml_chain::byron::ByronAddress {
    fn from(wasm: ByronAddress) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::byron::ByronAddress> for ByronAddress {
    fn as_ref(&self) -> &cml_chain::byron::ByronAddress {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct HDAddressPayload(cml_chain::byron::HDAddressPayload);

#[wasm_bindgen]
impl HDAddressPayload {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_core::serialization::ToBytes::to_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<HDAddressPayload, JsValue> {
        cml_core::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<HDAddressPayload, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn get(&self) -> Vec<u8> {
        self.0.get().clone()
    }
}

impl From<cml_chain::byron::HDAddressPayload> for HDAddressPayload {
    fn from(native: cml_chain::byron::HDAddressPayload) -> Self {
        Self(native)
    }
}

impl From<HDAddressPayload> for cml_chain::byron::HDAddressPayload {
    fn from(wasm: HDAddressPayload) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::byron::HDAddressPayload> for HDAddressPayload {
    fn as_ref(&self) -> &cml_chain::byron::HDAddressPayload {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct SpendingData(cml_chain::byron::SpendingData);

#[wasm_bindgen]
impl SpendingData {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_core::serialization::ToBytes::to_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<SpendingData, JsValue> {
        cml_core::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<SpendingData, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

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
            cml_chain::byron::SpendingData::SpendingDataPubKey(_) => SpendingDataKind::SpendingDataPubKey,
            cml_chain::byron::SpendingData::SpendingDataScript(_) => SpendingDataKind::SpendingDataScript,
            cml_chain::byron::SpendingData::SpendingDataRedeem(_) => SpendingDataKind::SpendingDataRedeem,
        }
    }

    pub fn as_spending_data_pub_key(&self) -> Option<Bip32PublicKey> {
        match &self.0 {
            cml_chain::byron::SpendingData::SpendingDataPubKey(pubkey) => Some(pubkey.clone().into()),
            _ => None,
        }
    }

    pub fn as_spending_data_script(&self) -> Option<ByronScript> {
        match &self.0 {
            cml_chain::byron::SpendingData::SpendingDataScript(script) => Some(script.clone().into()),
            _ => None,
        }
    }

    pub fn as_spending_data_redeem(&self) -> Option<PublicKey> {
        match &self.0 {
            cml_chain::byron::SpendingData::SpendingDataRedeem(redeem) => Some(redeem.clone().into()),
            _ => None,
        }
    }
}

impl From<cml_chain::byron::SpendingData> for SpendingData {
    fn from(native: cml_chain::byron::SpendingData) -> Self {
        Self(native)
    }
}

impl From<SpendingData> for cml_chain::byron::SpendingData {
    fn from(wasm: SpendingData) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::byron::SpendingData> for SpendingData {
    fn as_ref(&self) -> &cml_chain::byron::SpendingData {
        &self.0
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

#[wasm_bindgen]
impl StakeDistribution {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_core::serialization::ToBytes::to_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<StakeDistribution, JsValue> {
        cml_core::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<StakeDistribution, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

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
            cml_chain::byron::StakeDistribution::BootstrapEra => StakeDistributionKind::BootstrapEra,
        }
    }

    pub fn as_single_key(&self) -> Option<StakeholderId> {
        match &self.0 {
            cml_chain::byron::StakeDistribution::SingleKey(stakeholder_id) => {
                Some(stakeholder_id.clone().into())
            }
            _ => None,
        }
    }
}

impl From<cml_chain::byron::StakeDistribution> for StakeDistribution {
    fn from(native: cml_chain::byron::StakeDistribution) -> Self {
        Self(native)
    }
}

impl From<StakeDistribution> for cml_chain::byron::StakeDistribution {
    fn from(wasm: StakeDistribution) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::byron::StakeDistribution> for StakeDistribution {
    fn as_ref(&self) -> &cml_chain::byron::StakeDistribution {
        &self.0
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

#[wasm_bindgen]
impl ByronTxOut {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_core::serialization::ToBytes::to_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<ByronTxOut, JsValue> {
        cml_core::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<ByronTxOut, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn address(&self) -> ByronAddress {
        self.0.address.clone().into()
    }

    pub fn amount(&self) -> u64 {
        self.0.amount
    }

    pub fn new(address: &ByronAddress, amount: u64) -> Self {
        Self(cml_chain::byron::ByronTxOut::new(address.clone().into(), amount))
    }
}

impl From<cml_chain::byron::ByronTxOut> for ByronTxOut {
    fn from(native: cml_chain::byron::ByronTxOut) -> Self {
        Self(native)
    }
}

impl From<ByronTxOut> for cml_chain::byron::ByronTxOut {
    fn from(wasm: ByronTxOut) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::byron::ByronTxOut> for ByronTxOut {
    fn as_ref(&self) -> &cml_chain::byron::ByronTxOut {
        &self.0
    }
}
