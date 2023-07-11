// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use crate::allegra::{AllegraAuxiliaryData, AllegraTransactionWitnessSet};
use cml_chain_wasm::assets::{Coin, Mint};
use cml_chain_wasm::Withdrawals;
use crate::shelley::{ShelleyHeader, ShelleyUpdate};
use cml_chain_wasm::{
    CertificateList, TransactionInputList
};
use cml_crypto_wasm::{AuxiliaryDataHash};
use crate::{
    AllegraTransactionWitnessSetList, MapTransactionIndexToAllegraAuxiliaryData,
    MaryTransactionBodyList, ShelleyTxOutList,
};
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MaryBlock(cml_multi_era::mary::MaryBlock);

#[wasm_bindgen]
impl MaryBlock {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_core::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<MaryBlock, JsValue> {
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

    pub fn from_json(json: &str) -> Result<MaryBlock, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn header(&self) -> ShelleyHeader {
        self.0.header.clone().into()
    }

    pub fn transaction_bodies(&self) -> MaryTransactionBodyList {
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
        transaction_bodies: &MaryTransactionBodyList,
        transaction_witness_sets: &AllegraTransactionWitnessSetList,
        auxiliary_data_set: &MapTransactionIndexToAllegraAuxiliaryData,
    ) -> Self {
        Self(cml_multi_era::mary::MaryBlock::new(
            header.clone().into(),
            transaction_bodies.clone().into(),
            transaction_witness_sets.clone().into(),
            auxiliary_data_set.clone().into(),
        ))
    }
}

impl From<cml_multi_era::mary::MaryBlock> for MaryBlock {
    fn from(native: cml_multi_era::mary::MaryBlock) -> Self {
        Self(native)
    }
}

impl From<MaryBlock> for cml_multi_era::mary::MaryBlock {
    fn from(wasm: MaryBlock) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::mary::MaryBlock> for MaryBlock {
    fn as_ref(&self) -> &cml_multi_era::mary::MaryBlock {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MaryTransaction(cml_multi_era::mary::MaryTransaction);

#[wasm_bindgen]
impl MaryTransaction {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_core::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<MaryTransaction, JsValue> {
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

    pub fn from_json(json: &str) -> Result<MaryTransaction, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn body(&self) -> MaryTransactionBody {
        self.0.body.clone().into()
    }

    pub fn witness_set(&self) -> AllegraTransactionWitnessSet {
        self.0.witness_set.clone().into()
    }

    pub fn auxiliary_data(&self) -> Option<AllegraAuxiliaryData> {
        self.0.auxiliary_data.clone().map(std::convert::Into::into)
    }

    pub fn new(
        body: &MaryTransactionBody,
        witness_set: &AllegraTransactionWitnessSet,
        auxiliary_data: Option<AllegraAuxiliaryData>,
    ) -> Self {
        Self(cml_multi_era::mary::MaryTransaction::new(
            body.clone().into(),
            witness_set.clone().into(),
            auxiliary_data.map(Into::into),
        ))
    }
}

impl From<cml_multi_era::mary::MaryTransaction> for MaryTransaction {
    fn from(native: cml_multi_era::mary::MaryTransaction) -> Self {
        Self(native)
    }
}

impl From<MaryTransaction> for cml_multi_era::mary::MaryTransaction {
    fn from(wasm: MaryTransaction) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::mary::MaryTransaction> for MaryTransaction {
    fn as_ref(&self) -> &cml_multi_era::mary::MaryTransaction {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MaryTransactionBody(cml_multi_era::mary::MaryTransactionBody);

#[wasm_bindgen]
impl MaryTransactionBody {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_core::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<MaryTransactionBody, JsValue> {
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

    pub fn from_json(json: &str) -> Result<MaryTransactionBody, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn inputs(&self) -> TransactionInputList {
        self.0.inputs.clone().into()
    }

    pub fn outputs(&self) -> ShelleyTxOutList {
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

    pub fn set_mint(&mut self, mint: &Mint) {
        self.0.mint = Some(mint.clone().into())
    }

    pub fn mint(&self) -> Option<Mint> {
        self.0.mint.clone().map(std::convert::Into::into)
    }

    pub fn new(inputs: &TransactionInputList, outputs: &ShelleyTxOutList, fee: Coin) -> Self {
        Self(cml_multi_era::mary::MaryTransactionBody::new(
            inputs.clone().into(),
            outputs.clone().into(),
            fee,
        ))
    }
}

impl From<cml_multi_era::mary::MaryTransactionBody> for MaryTransactionBody {
    fn from(native: cml_multi_era::mary::MaryTransactionBody) -> Self {
        Self(native)
    }
}

impl From<MaryTransactionBody> for cml_multi_era::mary::MaryTransactionBody {
    fn from(wasm: MaryTransactionBody) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::mary::MaryTransactionBody> for MaryTransactionBody {
    fn as_ref(&self) -> &cml_multi_era::mary::MaryTransactionBody {
        &self.0
    }
}
