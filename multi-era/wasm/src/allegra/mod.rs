// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use cml_chain_wasm::assets::Coin;
use cml_chain_wasm::auxdata::{ShelleyAuxData, ShelleyMaAuxData};
use cml_crypto_wasm::AuxiliaryDataHash;
use cml_chain_wasm::Withdrawals;
use crate::shelley::{ShelleyHeader, ShelleyUpdate};
use cml_chain_wasm::{
    BootstrapWitnessList, NativeScriptList, VkeywitnessList, TransactionInputList, CertificateList, 
};
use crate::{
    AllegraTransactionBodyList, AllegraTransactionWitnessSetList,
    MapTransactionIndexToAllegraAuxiliaryData, ShelleyTransactionOutputList,
};
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct AllegraAuxiliaryData(cml_multi_era::allegra::AllegraAuxiliaryData);

#[wasm_bindgen]
impl AllegraAuxiliaryData {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_core::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<AllegraAuxiliaryData, JsValue> {
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

    pub fn from_json(json: &str) -> Result<AllegraAuxiliaryData, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn new_shelley_aux_data(shelley_aux_data: &ShelleyAuxData) -> Self {
        Self(
            cml_multi_era::allegra::AllegraAuxiliaryData::new_shelley_aux_data(
                shelley_aux_data.clone().into(),
            ),
        )
    }

    pub fn new_shelley_ma_aux_data(shelley_ma_aux_data: &ShelleyMaAuxData) -> Self {
        Self(
            cml_multi_era::allegra::AllegraAuxiliaryData::new_shelley_ma_aux_data(
                shelley_ma_aux_data.clone().into(),
            ),
        )
    }

    pub fn kind(&self) -> AllegraAuxiliaryDataKind {
        match &self.0 {
            cml_multi_era::allegra::AllegraAuxiliaryData::ShelleyAuxData(_) => {
                AllegraAuxiliaryDataKind::ShelleyAuxData
            }
            cml_multi_era::allegra::AllegraAuxiliaryData::ShelleyMaAuxData(_) => {
                AllegraAuxiliaryDataKind::ShelleyMaAuxData
            }
        }
    }

    pub fn as_shelley_aux_data(&self) -> Option<ShelleyAuxData> {
        match &self.0 {
            cml_multi_era::allegra::AllegraAuxiliaryData::ShelleyAuxData(shelley_aux_data) => {
                Some(shelley_aux_data.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_shelley_ma_aux_data(&self) -> Option<ShelleyMaAuxData> {
        match &self.0 {
            cml_multi_era::allegra::AllegraAuxiliaryData::ShelleyMaAuxData(shelley_ma_aux_data) => {
                Some(shelley_ma_aux_data.clone().into())
            }
            _ => None,
        }
    }
}

impl From<cml_multi_era::allegra::AllegraAuxiliaryData> for AllegraAuxiliaryData {
    fn from(native: cml_multi_era::allegra::AllegraAuxiliaryData) -> Self {
        Self(native)
    }
}

impl From<AllegraAuxiliaryData> for cml_multi_era::allegra::AllegraAuxiliaryData {
    fn from(wasm: AllegraAuxiliaryData) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::allegra::AllegraAuxiliaryData> for AllegraAuxiliaryData {
    fn as_ref(&self) -> &cml_multi_era::allegra::AllegraAuxiliaryData {
        &self.0
    }
}

#[wasm_bindgen]
pub enum AllegraAuxiliaryDataKind {
    ShelleyAuxData,
    ShelleyMaAuxData,
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct AllegraBlock(cml_multi_era::allegra::AllegraBlock);

#[wasm_bindgen]
impl AllegraBlock {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_core::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<AllegraBlock, JsValue> {
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

    pub fn from_json(json: &str) -> Result<AllegraBlock, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn header(&self) -> ShelleyHeader {
        self.0.header.clone().into()
    }

    pub fn transaction_bodies(&self) -> AllegraTransactionBodyList {
        self.0.transaction_bodies.clone().into()
    }

    pub fn transaction_witness_sets(&self) -> AllegraTransactionWitnessSetList {
        self.0.transaction_witness_sets.clone().into()
    }

    pub fn auxiliary_data_set(&self) -> MapTransactionIndexToAllegraAuxiliaryData {
        self.0.auxiliary_data_set.clone().into()
    }

    pub fn new(
        header: &ShelleyHeader,
        transaction_bodies: &AllegraTransactionBodyList,
        transaction_witness_sets: &AllegraTransactionWitnessSetList,
        auxiliary_data_set: &MapTransactionIndexToAllegraAuxiliaryData,
    ) -> Self {
        Self(cml_multi_era::allegra::AllegraBlock::new(
            header.clone().into(),
            transaction_bodies.clone().into(),
            transaction_witness_sets.clone().into(),
            auxiliary_data_set.clone().into(),
        ))
    }
}

impl From<cml_multi_era::allegra::AllegraBlock> for AllegraBlock {
    fn from(native: cml_multi_era::allegra::AllegraBlock) -> Self {
        Self(native)
    }
}

impl From<AllegraBlock> for cml_multi_era::allegra::AllegraBlock {
    fn from(wasm: AllegraBlock) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::allegra::AllegraBlock> for AllegraBlock {
    fn as_ref(&self) -> &cml_multi_era::allegra::AllegraBlock {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct AllegraTransaction(cml_multi_era::allegra::AllegraTransaction);

#[wasm_bindgen]
impl AllegraTransaction {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_core::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<AllegraTransaction, JsValue> {
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

    pub fn from_json(json: &str) -> Result<AllegraTransaction, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn body(&self) -> AllegraTransactionBody {
        self.0.body.clone().into()
    }

    pub fn witness_set(&self) -> AllegraTransactionWitnessSet {
        self.0.witness_set.clone().into()
    }

    pub fn auxiliary_data(&self) -> Option<AllegraAuxiliaryData> {
        self.0.auxiliary_data.clone().map(std::convert::Into::into)
    }

    pub fn new(
        body: &AllegraTransactionBody,
        witness_set: &AllegraTransactionWitnessSet,
        auxiliary_data: Option<AllegraAuxiliaryData>,
    ) -> Self {
        Self(cml_multi_era::allegra::AllegraTransaction::new(
            body.clone().into(),
            witness_set.clone().into(),
            auxiliary_data.map(Into::into),
        ))
    }
}

impl From<cml_multi_era::allegra::AllegraTransaction> for AllegraTransaction {
    fn from(native: cml_multi_era::allegra::AllegraTransaction) -> Self {
        Self(native)
    }
}

impl From<AllegraTransaction> for cml_multi_era::allegra::AllegraTransaction {
    fn from(wasm: AllegraTransaction) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::allegra::AllegraTransaction> for AllegraTransaction {
    fn as_ref(&self) -> &cml_multi_era::allegra::AllegraTransaction {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct AllegraTransactionBody(cml_multi_era::allegra::AllegraTransactionBody);

#[wasm_bindgen]
impl AllegraTransactionBody {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_core::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<AllegraTransactionBody, JsValue> {
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

    pub fn from_json(json: &str) -> Result<AllegraTransactionBody, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn inputs(&self) -> TransactionInputList {
        self.0.inputs.clone().into()
    }

    pub fn outputs(&self) -> ShelleyTransactionOutputList {
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

    pub fn set_update(&mut self, update: &ShelleyUpdate) {
        self.0.update = Some(update.clone().into())
    }

    pub fn update(&self) -> Option<ShelleyUpdate> {
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

    pub fn new(
        inputs: &TransactionInputList,
        outputs: &ShelleyTransactionOutputList,
        fee: Coin,
    ) -> Self {
        Self(cml_multi_era::allegra::AllegraTransactionBody::new(
            inputs.clone().into(),
            outputs.clone().into(),
            fee,
        ))
    }
}

impl From<cml_multi_era::allegra::AllegraTransactionBody> for AllegraTransactionBody {
    fn from(native: cml_multi_era::allegra::AllegraTransactionBody) -> Self {
        Self(native)
    }
}

impl From<AllegraTransactionBody> for cml_multi_era::allegra::AllegraTransactionBody {
    fn from(wasm: AllegraTransactionBody) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::allegra::AllegraTransactionBody> for AllegraTransactionBody {
    fn as_ref(&self) -> &cml_multi_era::allegra::AllegraTransactionBody {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct AllegraTransactionWitnessSet(cml_multi_era::allegra::AllegraTransactionWitnessSet);

#[wasm_bindgen]
impl AllegraTransactionWitnessSet {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_core::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<AllegraTransactionWitnessSet, JsValue> {
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

    pub fn from_json(json: &str) -> Result<AllegraTransactionWitnessSet, JsValue> {
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

    pub fn new() -> Self {
        Self(cml_multi_era::allegra::AllegraTransactionWitnessSet::new())
    }
}

impl From<cml_multi_era::allegra::AllegraTransactionWitnessSet> for AllegraTransactionWitnessSet {
    fn from(native: cml_multi_era::allegra::AllegraTransactionWitnessSet) -> Self {
        Self(native)
    }
}

impl From<AllegraTransactionWitnessSet> for cml_multi_era::allegra::AllegraTransactionWitnessSet {
    fn from(wasm: AllegraTransactionWitnessSet) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::allegra::AllegraTransactionWitnessSet> for AllegraTransactionWitnessSet {
    fn as_ref(&self) -> &cml_multi_era::allegra::AllegraTransactionWitnessSet {
        &self.0
    }
}
