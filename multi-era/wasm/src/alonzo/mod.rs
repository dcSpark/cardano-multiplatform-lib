// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use cml_chain_wasm::{
    GenesisHashList,
    TransactionIndex,
    ProtocolVersionStruct,
};
use cml_chain_wasm::assets::{Coin, Mint};
use cml_chain_wasm::auxdata::{Metadata, ShelleyAuxData, ShelleyMaAuxData};
use cml_chain_wasm::crypto::{Nonce};
use cml_chain_wasm::plutus::{ExUnitPrices, ExUnits};
use cml_chain_wasm::transaction::{RequiredSigners};
use cml_chain_wasm::{Epoch, NetworkId, Rational, UnitInterval, Withdrawals};
use crate::shelley::ShelleyHeader;
use cml_chain_wasm::{
    CertificateList, IntList, NativeScriptList, PlutusDataList, PlutusV1ScriptList,
    RedeemerList, TransactionInputList, VkeywitnessList, BootstrapWitnessList
};
use cml_crypto_wasm::{AuxiliaryDataHash, GenesisHash, ScriptDataHash};
use crate::{
    AlonzoTransactionBodyList, AlonzoTransactionWitnessSetList, AlonzoTxOutList, MapTransactionIndexToAlonzoAuxiliaryData,
};
use cml_core::ordered_hash_map::OrderedHashMap;
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct AlonzoAuxiliaryData(cml_multi_era::alonzo::AlonzoAuxiliaryData);

#[wasm_bindgen]
impl AlonzoAuxiliaryData {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_core::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<AlonzoAuxiliaryData, JsValue> {
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

    pub fn from_json(json: &str) -> Result<AlonzoAuxiliaryData, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn new_shelley(shelley: &ShelleyAuxData) -> Self {
        Self(cml_multi_era::alonzo::AlonzoAuxiliaryData::new_shelley(
            shelley.clone().into(),
        ))
    }

    pub fn new_shelley_m_a(shelley_m_a: &ShelleyMaAuxData) -> Self {
        Self(cml_multi_era::alonzo::AlonzoAuxiliaryData::new_shelley_m_a(
            shelley_m_a.clone().into(),
        ))
    }

    pub fn new_alonzo(alonzo: &AlonzoOnlyAuxData) -> Self {
        Self(cml_multi_era::alonzo::AlonzoAuxiliaryData::new_alonzo(
            alonzo.clone().into(),
        ))
    }

    pub fn kind(&self) -> AlonzoAuxiliaryDataKind {
        match &self.0 {
            cml_multi_era::alonzo::AlonzoAuxiliaryData::Shelley(_) => {
                AlonzoAuxiliaryDataKind::Shelley
            }
            cml_multi_era::alonzo::AlonzoAuxiliaryData::ShelleyMA(_) => {
                AlonzoAuxiliaryDataKind::ShelleyMA
            }
            cml_multi_era::alonzo::AlonzoAuxiliaryData::Alonzo(_) => {
                AlonzoAuxiliaryDataKind::Alonzo
            }
        }
    }

    pub fn as_shelley(&self) -> Option<ShelleyAuxData> {
        match &self.0 {
            cml_multi_era::alonzo::AlonzoAuxiliaryData::Shelley(shelley) => {
                Some(shelley.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_shelley_m_a(&self) -> Option<ShelleyMaAuxData> {
        match &self.0 {
            cml_multi_era::alonzo::AlonzoAuxiliaryData::ShelleyMA(shelley_m_a) => {
                Some(shelley_m_a.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_alonzo(&self) -> Option<AlonzoOnlyAuxData> {
        match &self.0 {
            cml_multi_era::alonzo::AlonzoAuxiliaryData::Alonzo(alonzo) => {
                Some(alonzo.clone().into())
            }
            _ => None,
        }
    }
}

impl From<cml_multi_era::alonzo::AlonzoAuxiliaryData> for AlonzoAuxiliaryData {
    fn from(native: cml_multi_era::alonzo::AlonzoAuxiliaryData) -> Self {
        Self(native)
    }
}

impl From<AlonzoAuxiliaryData> for cml_multi_era::alonzo::AlonzoAuxiliaryData {
    fn from(wasm: AlonzoAuxiliaryData) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::alonzo::AlonzoAuxiliaryData> for AlonzoAuxiliaryData {
    fn as_ref(&self) -> &cml_multi_era::alonzo::AlonzoAuxiliaryData {
        &self.0
    }
}

#[wasm_bindgen]
pub enum AlonzoAuxiliaryDataKind {
    Shelley,
    ShelleyMA,
    Alonzo,
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct AlonzoBlock(cml_multi_era::alonzo::AlonzoBlock);

#[wasm_bindgen]
impl AlonzoBlock {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_core::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<AlonzoBlock, JsValue> {
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

    pub fn from_json(json: &str) -> Result<AlonzoBlock, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn header(&self) -> ShelleyHeader {
        self.0.header.clone().into()
    }

    pub fn transaction_bodies(&self) -> AlonzoTransactionBodyList {
        self.0.transaction_bodies.clone().into()
    }

    pub fn transaction_witness_sets(&self) -> AlonzoTransactionWitnessSetList {
        self.0.transaction_witness_sets.clone().into()
    }

    pub fn auxiliary_data_set(&self) -> MapTransactionIndexToAlonzoAuxiliaryData {
        self.0.auxiliary_data_set.clone().into()
    }

    pub fn invalid_transactions(&self) -> Vec<TransactionIndex> {
        self.0.invalid_transactions.clone()
    }

    pub fn new(
        header: &ShelleyHeader,
        transaction_bodies: &AlonzoTransactionBodyList,
        transaction_witness_sets: &AlonzoTransactionWitnessSetList,
        auxiliary_data_set: &MapTransactionIndexToAlonzoAuxiliaryData,
        invalid_transactions: Vec<TransactionIndex>,
    ) -> Self {
        Self(cml_multi_era::alonzo::AlonzoBlock::new(
            header.clone().into(),
            transaction_bodies.clone().into(),
            transaction_witness_sets.clone().into(),
            auxiliary_data_set.clone().into(),
            invalid_transactions,
        ))
    }
}

impl From<cml_multi_era::alonzo::AlonzoBlock> for AlonzoBlock {
    fn from(native: cml_multi_era::alonzo::AlonzoBlock) -> Self {
        Self(native)
    }
}

impl From<AlonzoBlock> for cml_multi_era::alonzo::AlonzoBlock {
    fn from(wasm: AlonzoBlock) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::alonzo::AlonzoBlock> for AlonzoBlock {
    fn as_ref(&self) -> &cml_multi_era::alonzo::AlonzoBlock {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct AlonzoCostmdls(cml_multi_era::alonzo::AlonzoCostmdls);

#[wasm_bindgen]
impl AlonzoCostmdls {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_core::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<AlonzoCostmdls, JsValue> {
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

    pub fn from_json(json: &str) -> Result<AlonzoCostmdls, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn plutus_v1(&self) -> IntList {
        self.0.plutus_v1.clone().into()
    }

    pub fn new(plutus_v1: &IntList) -> Self {
        Self(cml_multi_era::alonzo::AlonzoCostmdls::new(
            plutus_v1.clone().into(),
        ))
    }
}

impl From<cml_multi_era::alonzo::AlonzoCostmdls> for AlonzoCostmdls {
    fn from(native: cml_multi_era::alonzo::AlonzoCostmdls) -> Self {
        Self(native)
    }
}

impl From<AlonzoCostmdls> for cml_multi_era::alonzo::AlonzoCostmdls {
    fn from(wasm: AlonzoCostmdls) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::alonzo::AlonzoCostmdls> for AlonzoCostmdls {
    fn as_ref(&self) -> &cml_multi_era::alonzo::AlonzoCostmdls {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct AlonzoOnlyAuxData(cml_multi_era::alonzo::AlonzoOnlyAuxData);

#[wasm_bindgen]
impl AlonzoOnlyAuxData {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_core::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<AlonzoOnlyAuxData, JsValue> {
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

    pub fn from_json(json: &str) -> Result<AlonzoOnlyAuxData, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn set_metadata(&mut self, metadata: &Metadata) {
        self.0.metadata = Some(metadata.clone().into())
    }

    pub fn metadata(&self) -> Option<Metadata> {
        self.0.metadata.clone().map(std::convert::Into::into)
    }

    pub fn set_native_scripts(&mut self, native_scripts: &NativeScriptList) {
        self.0.native_scripts = Some(native_scripts.clone().into())
    }

    pub fn native_scripts(&self) -> Option<NativeScriptList> {
        self.0.native_scripts.clone().map(std::convert::Into::into)
    }

    pub fn set_plutus_v1_scripts(&mut self, plutus_v1_scripts: &PlutusV1ScriptList) {
        self.0.plutus_v1_scripts = Some(plutus_v1_scripts.clone().into())
    }

    pub fn plutus_v1_scripts(&self) -> Option<PlutusV1ScriptList> {
        self.0
            .plutus_v1_scripts
            .clone()
            .map(std::convert::Into::into)
    }

    pub fn new() -> Self {
        Self(cml_multi_era::alonzo::AlonzoOnlyAuxData::new())
    }
}

impl From<cml_multi_era::alonzo::AlonzoOnlyAuxData> for AlonzoOnlyAuxData {
    fn from(native: cml_multi_era::alonzo::AlonzoOnlyAuxData) -> Self {
        Self(native)
    }
}

impl From<AlonzoOnlyAuxData> for cml_multi_era::alonzo::AlonzoOnlyAuxData {
    fn from(wasm: AlonzoOnlyAuxData) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::alonzo::AlonzoOnlyAuxData> for AlonzoOnlyAuxData {
    fn as_ref(&self) -> &cml_multi_era::alonzo::AlonzoOnlyAuxData {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct AlonzoProposedProtocolParameterUpdates(
    cml_multi_era::alonzo::AlonzoProposedProtocolParameterUpdates,
);

#[wasm_bindgen]
impl AlonzoProposedProtocolParameterUpdates {
    pub fn new() -> Self {
        Self(OrderedHashMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(
        &mut self,
        key: &GenesisHash,
        value: &AlonzoProtocolParamUpdate,
    ) -> Option<AlonzoProtocolParamUpdate> {
        self.0
            .insert(key.clone().into(), value.clone().into())
            .map(Into::into)
    }

    pub fn get(&self, key: &GenesisHash) -> Option<AlonzoProtocolParamUpdate> {
        self.0.get(key.as_ref()).map(|v| v.clone().into())
    }

    pub fn keys(&self) -> GenesisHashList {
        self.0.iter().map(|(k, _v)| k.clone()).collect::<Vec<_>>().into()
    }
}

impl From<cml_multi_era::alonzo::AlonzoProposedProtocolParameterUpdates>
    for AlonzoProposedProtocolParameterUpdates
{
    fn from(native: cml_multi_era::alonzo::AlonzoProposedProtocolParameterUpdates) -> Self {
        Self(native)
    }
}

impl From<AlonzoProposedProtocolParameterUpdates>
    for cml_multi_era::alonzo::AlonzoProposedProtocolParameterUpdates
{
    fn from(wasm: AlonzoProposedProtocolParameterUpdates) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::alonzo::AlonzoProposedProtocolParameterUpdates>
    for AlonzoProposedProtocolParameterUpdates
{
    fn as_ref(&self) -> &cml_multi_era::alonzo::AlonzoProposedProtocolParameterUpdates {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct AlonzoProtocolParamUpdate(cml_multi_era::alonzo::AlonzoProtocolParamUpdate);

#[wasm_bindgen]
impl AlonzoProtocolParamUpdate {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_core::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<AlonzoProtocolParamUpdate, JsValue> {
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

    pub fn from_json(json: &str) -> Result<AlonzoProtocolParamUpdate, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

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

    pub fn set_min_pool_cost(&mut self, min_pool_cost: Coin) {
        self.0.min_pool_cost = Some(min_pool_cost)
    }

    pub fn min_pool_cost(&self) -> Option<Coin> {
        self.0.min_pool_cost
    }

    pub fn set_ada_per_utxo_byte(&mut self, ada_per_utxo_byte: Coin) {
        self.0.ada_per_utxo_byte = Some(ada_per_utxo_byte)
    }

    pub fn ada_per_utxo_byte(&self) -> Option<Coin> {
        self.0.ada_per_utxo_byte
    }

    pub fn set_cost_models_for_script_languages(
        &mut self,
        cost_models_for_script_languages: &AlonzoCostmdls,
    ) {
        self.0.cost_models_for_script_languages =
            Some(cost_models_for_script_languages.clone().into())
    }

    pub fn cost_models_for_script_languages(&self) -> Option<AlonzoCostmdls> {
        self.0
            .cost_models_for_script_languages
            .clone()
            .map(std::convert::Into::into)
    }

    pub fn set_execution_costs(&mut self, execution_costs: &ExUnitPrices) {
        self.0.execution_costs = Some(execution_costs.clone().into())
    }

    pub fn execution_costs(&self) -> Option<ExUnitPrices> {
        self.0.execution_costs.clone().map(std::convert::Into::into)
    }

    pub fn set_max_tx_ex_units(&mut self, max_tx_ex_units: &ExUnits) {
        self.0.max_tx_ex_units = Some(max_tx_ex_units.clone().into())
    }

    pub fn max_tx_ex_units(&self) -> Option<ExUnits> {
        self.0.max_tx_ex_units.clone().map(std::convert::Into::into)
    }

    pub fn set_max_block_ex_units(&mut self, max_block_ex_units: &ExUnits) {
        self.0.max_block_ex_units = Some(max_block_ex_units.clone().into())
    }

    pub fn max_block_ex_units(&self) -> Option<ExUnits> {
        self.0
            .max_block_ex_units
            .clone()
            .map(std::convert::Into::into)
    }

    pub fn set_max(&mut self, max: u64) {
        self.0.max = Some(max)
    }

    pub fn max(&self) -> Option<u64> {
        self.0.max
    }

    pub fn set_collateral_percentage(&mut self, collateral_percentage: u64) {
        self.0.collateral_percentage = Some(collateral_percentage)
    }

    pub fn collateral_percentage(&self) -> Option<u64> {
        self.0.collateral_percentage
    }

    pub fn set_max_collateral_inputs(&mut self, max_collateral_inputs: u64) {
        self.0.max_collateral_inputs = Some(max_collateral_inputs)
    }

    pub fn max_collateral_inputs(&self) -> Option<u64> {
        self.0.max_collateral_inputs
    }

    pub fn new() -> Self {
        Self(cml_multi_era::alonzo::AlonzoProtocolParamUpdate::new())
    }
}

impl From<cml_multi_era::alonzo::AlonzoProtocolParamUpdate> for AlonzoProtocolParamUpdate {
    fn from(native: cml_multi_era::alonzo::AlonzoProtocolParamUpdate) -> Self {
        Self(native)
    }
}

impl From<AlonzoProtocolParamUpdate> for cml_multi_era::alonzo::AlonzoProtocolParamUpdate {
    fn from(wasm: AlonzoProtocolParamUpdate) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::alonzo::AlonzoProtocolParamUpdate> for AlonzoProtocolParamUpdate {
    fn as_ref(&self) -> &cml_multi_era::alonzo::AlonzoProtocolParamUpdate {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct AlonzoTransaction(cml_multi_era::alonzo::AlonzoTransaction);

#[wasm_bindgen]
impl AlonzoTransaction {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_core::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<AlonzoTransaction, JsValue> {
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

    pub fn from_json(json: &str) -> Result<AlonzoTransaction, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn body(&self) -> AlonzoTransactionBody {
        self.0.body.clone().into()
    }

    pub fn witness_set(&self) -> AlonzoTransactionWitnessSet {
        self.0.witness_set.clone().into()
    }

    pub fn is_valid(&self) -> bool {
        self.0.is_valid
    }

    pub fn auxiliary_data(&self) -> Option<AlonzoAuxiliaryData> {
        self.0.auxiliary_data.clone().map(std::convert::Into::into)
    }

    pub fn new(
        body: &AlonzoTransactionBody,
        witness_set: &AlonzoTransactionWitnessSet,
        is_valid: bool,
        auxiliary_data: Option<AlonzoAuxiliaryData>,
    ) -> Self {
        Self(cml_multi_era::alonzo::AlonzoTransaction::new(
            body.clone().into(),
            witness_set.clone().into(),
            is_valid,
            auxiliary_data.map(Into::into),
        ))
    }
}

impl From<cml_multi_era::alonzo::AlonzoTransaction> for AlonzoTransaction {
    fn from(native: cml_multi_era::alonzo::AlonzoTransaction) -> Self {
        Self(native)
    }
}

impl From<AlonzoTransaction> for cml_multi_era::alonzo::AlonzoTransaction {
    fn from(wasm: AlonzoTransaction) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::alonzo::AlonzoTransaction> for AlonzoTransaction {
    fn as_ref(&self) -> &cml_multi_era::alonzo::AlonzoTransaction {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct AlonzoTransactionBody(cml_multi_era::alonzo::AlonzoTransactionBody);

#[wasm_bindgen]
impl AlonzoTransactionBody {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_core::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<AlonzoTransactionBody, JsValue> {
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

    pub fn from_json(json: &str) -> Result<AlonzoTransactionBody, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn inputs(&self) -> TransactionInputList {
        self.0.inputs.clone().into()
    }

    pub fn outputs(&self) -> AlonzoTxOutList {
        self.0.outputs.clone().into()
    }

    pub fn fee(&self) -> Coin {
        self.0.fee
    }

    pub fn set_ttl(&mut self, ttl: u64) {
        self.0.ttl = Some(ttl)
    }

    pub fn ttl(&self) -> Option<u64> {
        self.0.ttl
    }

    pub fn set_certs(&mut self, certs: &CertificateList) {
        self.0.certs = Some(certs.clone().into())
    }

    pub fn certs(&self) -> Option<CertificateList> {
        self.0.certs.clone().map(std::convert::Into::into)
    }

    pub fn set_withdrawals(&mut self, withdrawals: &Withdrawals) {
        self.0.withdrawals = Some(withdrawals.clone().into())
    }

    pub fn withdrawals(&self) -> Option<Withdrawals> {
        self.0.withdrawals.clone().map(std::convert::Into::into)
    }

    pub fn set_update(&mut self, update: &AlonzoUpdate) {
        self.0.update = Some(update.clone().into())
    }

    pub fn update(&self) -> Option<AlonzoUpdate> {
        self.0.update.clone().map(std::convert::Into::into)
    }

    pub fn set_auxiliary_data_hash(&mut self, auxiliary_data_hash: &AuxiliaryDataHash) {
        self.0.auxiliary_data_hash = Some(auxiliary_data_hash.clone().into())
    }

    pub fn auxiliary_data_hash(&self) -> Option<AuxiliaryDataHash> {
        self.0
            .auxiliary_data_hash
            .clone()
            .map(std::convert::Into::into)
    }

    pub fn set_validity_interval_start(&mut self, validity_interval_start: u64) {
        self.0.validity_interval_start = Some(validity_interval_start)
    }

    pub fn validity_interval_start(&self) -> Option<u64> {
        self.0.validity_interval_start
    }

    pub fn set_mint(&mut self, mint: &Mint) {
        self.0.mint = Some(mint.clone().into())
    }

    pub fn mint(&self) -> Option<Mint> {
        self.0.mint.clone().map(std::convert::Into::into)
    }

    pub fn set_script_data_hash(&mut self, script_data_hash: &ScriptDataHash) {
        self.0.script_data_hash = Some(script_data_hash.clone().into())
    }

    pub fn script_data_hash(&self) -> Option<ScriptDataHash> {
        self.0
            .script_data_hash
            .clone()
            .map(std::convert::Into::into)
    }

    pub fn set_collateral_inputs(&mut self, collateral_inputs: &TransactionInputList) {
        self.0.collateral_inputs = Some(collateral_inputs.clone().into())
    }

    pub fn collateral_inputs(&self) -> Option<TransactionInputList> {
        self.0
            .collateral_inputs
            .clone()
            .map(std::convert::Into::into)
    }

    pub fn set_required_signers(&mut self, required_signers: &RequiredSigners) {
        self.0.required_signers = Some(required_signers.clone().into())
    }

    pub fn required_signers(&self) -> Option<RequiredSigners> {
        self.0
            .required_signers
            .clone()
            .map(std::convert::Into::into)
    }

    pub fn set_network_id(&mut self, network_id: NetworkId) {
        self.0.network_id = Some(network_id)
    }

    pub fn network_id(&self) -> Option<NetworkId> {
        self.0.network_id
    }

    pub fn new(
        inputs: &TransactionInputList,
        outputs: &AlonzoTransactionOutputList,
        fee: Coin,
    ) -> Self {
        Self(cml_multi_era::alonzo::AlonzoTransactionBody::new(
            inputs.clone().into(),
            outputs.clone().into(),
            fee,
        ))
    }
}

impl From<cml_multi_era::alonzo::AlonzoTransactionBody> for AlonzoTransactionBody {
    fn from(native: cml_multi_era::alonzo::AlonzoTransactionBody) -> Self {
        Self(native)
    }
}

impl From<AlonzoTransactionBody> for cml_multi_era::alonzo::AlonzoTransactionBody {
    fn from(wasm: AlonzoTransactionBody) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::alonzo::AlonzoTransactionBody> for AlonzoTransactionBody {
    fn as_ref(&self) -> &cml_multi_era::alonzo::AlonzoTransactionBody {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct AlonzoTransactionOutput(cml_multi_era::alonzo::AlonzoTransactionOutput);

#[wasm_bindgen]
impl AlonzoTransactionOutput {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_multi_era::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<AlonzoTransactionOutput, JsValue> {
        cml_multi_era::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
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

    pub fn from_json(json: &str) -> Result<AlonzoTransactionOutput, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn new_shelley_tx_out(shelley_tx_out: &ShelleyTxOut) -> Self {
        Self(
            cml_multi_era::alonzo::AlonzoTransactionOutput::new_shelley_tx_out(
                shelley_tx_out.clone().into(),
            ),
        )
    }

    pub fn new_alonzo_tx_out(alonzo_tx_out: &AlonzoTxOut) -> Self {
        Self(
            cml_multi_era::alonzo::AlonzoTransactionOutput::new_alonzo_tx_out(
                alonzo_tx_out.clone().into(),
            ),
        )
    }

    pub fn kind(&self) -> AlonzoTransactionOutputKind {
        match &self.0 {
            cml_multi_era::alonzo::AlonzoTransactionOutput::ShelleyTxOut(_) => {
                AlonzoTransactionOutputKind::ShelleyTxOut
            }
            cml_multi_era::alonzo::AlonzoTransactionOutput::AlonzoTxOut(_) => {
                AlonzoTransactionOutputKind::AlonzoTxOut
            }
        }
    }

    pub fn as_shelley_tx_out(&self) -> Option<ShelleyTxOut> {
        match &self.0 {
            cml_multi_era::alonzo::AlonzoTransactionOutput::ShelleyTxOut(shelley_tx_out) => {
                Some(shelley_tx_out.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_alonzo_tx_out(&self) -> Option<AlonzoTxOut> {
        match &self.0 {
            cml_multi_era::alonzo::AlonzoTransactionOutput::AlonzoTxOut(alonzo_tx_out) => {
                Some(alonzo_tx_out.clone().into())
            }
            _ => None,
        }
    }
}

impl From<cml_multi_era::alonzo::AlonzoTransactionOutput> for AlonzoTransactionOutput {
    fn from(native: cml_multi_era::alonzo::AlonzoTransactionOutput) -> Self {
        Self(native)
    }
}

impl From<AlonzoTransactionOutput> for cml_multi_era::alonzo::AlonzoTransactionOutput {
    fn from(wasm: AlonzoTransactionOutput) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::alonzo::AlonzoTransactionOutput> for AlonzoTransactionOutput {
    fn as_ref(&self) -> &cml_multi_era::alonzo::AlonzoTransactionOutput {
        &self.0
    }
}

#[wasm_bindgen]
pub enum AlonzoTransactionOutputKind {
    ShelleyTxOut,
    AlonzoTxOut,
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct AlonzoTransactionWitnessSet(cml_multi_era::alonzo::AlonzoTransactionWitnessSet);

#[wasm_bindgen]
impl AlonzoTransactionWitnessSet {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_core::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<AlonzoTransactionWitnessSet, JsValue> {
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

    pub fn from_json(json: &str) -> Result<AlonzoTransactionWitnessSet, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn set_vkeywitnesses(&mut self, vkeywitnesses: &VkeywitnessList) {
        self.0.vkeywitnesses = Some(vkeywitnesses.clone().into())
    }

    pub fn vkeywitnesses(&self) -> Option<VkeywitnessList> {
        self.0.vkeywitnesses.clone().map(std::convert::Into::into)
    }

    pub fn set_native_scripts(&mut self, native_scripts: &NativeScriptList) {
        self.0.native_scripts = Some(native_scripts.clone().into())
    }

    pub fn native_scripts(&self) -> Option<NativeScriptList> {
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

    pub fn set_plutus_v1_scripts(&mut self, plutus_v1_scripts: &PlutusV1ScriptList) {
        self.0.plutus_v1_scripts = Some(plutus_v1_scripts.clone().into())
    }

    pub fn plutus_v1_scripts(&self) -> Option<PlutusV1ScriptList> {
        self.0
            .plutus_v1_scripts
            .clone()
            .map(std::convert::Into::into)
    }

    pub fn set_plutus_datums(&mut self, plutus_datums: &PlutusDataList) {
        self.0.plutus_datums = Some(plutus_datums.clone().into())
    }

    pub fn plutus_datums(&self) -> Option<PlutusDataList> {
        self.0.plutus_datums.clone().map(std::convert::Into::into)
    }

    pub fn set_redeemers(&mut self, redeemers: &RedeemerList) {
        self.0.redeemers = Some(redeemers.clone().into())
    }

    pub fn redeemers(&self) -> Option<RedeemerList> {
        self.0.redeemers.clone().map(std::convert::Into::into)
    }

    pub fn new() -> Self {
        Self(cml_multi_era::alonzo::AlonzoTransactionWitnessSet::new())
    }
}

impl From<cml_multi_era::alonzo::AlonzoTransactionWitnessSet> for AlonzoTransactionWitnessSet {
    fn from(native: cml_multi_era::alonzo::AlonzoTransactionWitnessSet) -> Self {
        Self(native)
    }
}

impl From<AlonzoTransactionWitnessSet> for cml_multi_era::alonzo::AlonzoTransactionWitnessSet {
    fn from(wasm: AlonzoTransactionWitnessSet) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::alonzo::AlonzoTransactionWitnessSet> for AlonzoTransactionWitnessSet {
    fn as_ref(&self) -> &cml_multi_era::alonzo::AlonzoTransactionWitnessSet {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct AlonzoUpdate(cml_multi_era::alonzo::AlonzoUpdate);

#[wasm_bindgen]
impl AlonzoUpdate {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_core::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<AlonzoUpdate, JsValue> {
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

    pub fn from_json(json: &str) -> Result<AlonzoUpdate, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn proposed_protocol_parameter_updates(&self) -> AlonzoProposedProtocolParameterUpdates {
        self.0.proposed_protocol_parameter_updates.clone().into()
    }

    pub fn epoch(&self) -> Epoch {
        self.0.epoch
    }

    pub fn new(
        proposed_protocol_parameter_updates: &AlonzoProposedProtocolParameterUpdates,
        epoch: Epoch,
    ) -> Self {
        Self(cml_multi_era::alonzo::AlonzoUpdate::new(
            proposed_protocol_parameter_updates.clone().into(),
            epoch,
        ))
    }
}

impl From<cml_multi_era::alonzo::AlonzoUpdate> for AlonzoUpdate {
    fn from(native: cml_multi_era::alonzo::AlonzoUpdate) -> Self {
        Self(native)
    }
}

impl From<AlonzoUpdate> for cml_multi_era::alonzo::AlonzoUpdate {
    fn from(wasm: AlonzoUpdate) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::alonzo::AlonzoUpdate> for AlonzoUpdate {
    fn as_ref(&self) -> &cml_multi_era::alonzo::AlonzoUpdate {
        &self.0
    }
}
