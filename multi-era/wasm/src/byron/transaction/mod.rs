// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use crate::byron::{Blake2b256, ByronPubKey, ByronSignature, ByronTxId};
use crate::{ByronTxInList, ByronTxOutList};
use std::collections::BTreeMap;
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronAttributes(cml_multi_era::byron::transaction::ByronAttributes);

#[wasm_bindgen]
impl ByronAttributes {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, key: &ByronAny, value: &ByronAny) -> Option<ByronAny> {
        self.0
            .insert(key.clone().into(), value.clone().into())
            .map(Into::into)
    }

    pub fn get(&self, key: &ByronAny) -> Option<ByronAny> {
        self.0.get(key.as_ref()).map(|v| v.clone().into())
    }

    pub fn keys(&self) -> ByronAnyList {
        ByronAnyList(self.0.iter().map(|(k, _v)| k.clone()).collect::<Vec<_>>())
    }
}

impl From<cml_multi_era::byron::transaction::ByronAttributes> for ByronAttributes {
    fn from(native: cml_multi_era::byron::transaction::ByronAttributes) -> Self {
        Self(native)
    }
}

impl From<ByronAttributes> for cml_multi_era::byron::transaction::ByronAttributes {
    fn from(wasm: ByronAttributes) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::byron::transaction::ByronAttributes> for ByronAttributes {
    fn as_ref(&self) -> &cml_multi_era::byron::transaction::ByronAttributes {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronPkWitness(cml_multi_era::byron::transaction::ByronPkWitness);

#[wasm_bindgen]
impl ByronPkWitness {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_multi_era::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<ByronPkWitness, JsValue> {
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

    pub fn from_json(json: &str) -> Result<ByronPkWitness, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn index_1(&self) -> ByronPkWitnessEntry {
        self.0.index_1.clone().into()
    }

    pub fn new(index_1: &ByronPkWitnessEntry) -> Self {
        Self(cml_multi_era::byron::transaction::ByronPkWitness::new(
            index_1.clone().into(),
        ))
    }
}

impl From<cml_multi_era::byron::transaction::ByronPkWitness> for ByronPkWitness {
    fn from(native: cml_multi_era::byron::transaction::ByronPkWitness) -> Self {
        Self(native)
    }
}

impl From<ByronPkWitness> for cml_multi_era::byron::transaction::ByronPkWitness {
    fn from(wasm: ByronPkWitness) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::byron::transaction::ByronPkWitness> for ByronPkWitness {
    fn as_ref(&self) -> &cml_multi_era::byron::transaction::ByronPkWitness {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronPkWitnessEntry(cml_multi_era::byron::transaction::ByronPkWitnessEntry);

#[wasm_bindgen]
impl ByronPkWitnessEntry {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_multi_era::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<ByronPkWitnessEntry, JsValue> {
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

    pub fn from_json(json: &str) -> Result<ByronPkWitnessEntry, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn byron_pub_key(&self) -> ByronPubKey {
        self.0.byron_pub_key.clone()
    }

    pub fn byron_signature(&self) -> ByronSignature {
        self.0.byron_signature.clone()
    }

    pub fn new(byron_pub_key: ByronPubKey, byron_signature: ByronSignature) -> Self {
        Self(cml_multi_era::byron::transaction::ByronPkWitnessEntry::new(
            byron_pub_key,
            byron_signature,
        ))
    }
}

impl From<cml_multi_era::byron::transaction::ByronPkWitnessEntry> for ByronPkWitnessEntry {
    fn from(native: cml_multi_era::byron::transaction::ByronPkWitnessEntry) -> Self {
        Self(native)
    }
}

impl From<ByronPkWitnessEntry> for cml_multi_era::byron::transaction::ByronPkWitnessEntry {
    fn from(wasm: ByronPkWitnessEntry) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::byron::transaction::ByronPkWitnessEntry> for ByronPkWitnessEntry {
    fn as_ref(&self) -> &cml_multi_era::byron::transaction::ByronPkWitnessEntry {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronRedeemWitness(cml_multi_era::byron::transaction::ByronRedeemWitness);

#[wasm_bindgen]
impl ByronRedeemWitness {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_multi_era::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<ByronRedeemWitness, JsValue> {
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

    pub fn from_json(json: &str) -> Result<ByronRedeemWitness, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn index_1(&self) -> ByronRedeemerWitnessEntry {
        self.0.index_1.clone().into()
    }

    pub fn new(index_1: &ByronRedeemerWitnessEntry) -> Self {
        Self(cml_multi_era::byron::transaction::ByronRedeemWitness::new(
            index_1.clone().into(),
        ))
    }
}

impl From<cml_multi_era::byron::transaction::ByronRedeemWitness> for ByronRedeemWitness {
    fn from(native: cml_multi_era::byron::transaction::ByronRedeemWitness) -> Self {
        Self(native)
    }
}

impl From<ByronRedeemWitness> for cml_multi_era::byron::transaction::ByronRedeemWitness {
    fn from(wasm: ByronRedeemWitness) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::byron::transaction::ByronRedeemWitness> for ByronRedeemWitness {
    fn as_ref(&self) -> &cml_multi_era::byron::transaction::ByronRedeemWitness {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronRedeemerScript(cml_multi_era::byron::transaction::ByronRedeemerScript);

#[wasm_bindgen]
impl ByronRedeemerScript {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_multi_era::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<ByronRedeemerScript, JsValue> {
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

    pub fn from_json(json: &str) -> Result<ByronRedeemerScript, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn u16(&self) -> u16 {
        self.0.u16
    }

    pub fn index_1(&self) -> Vec<u8> {
        self.0.index_1.clone()
    }

    pub fn new(u16: u16, index_1: Vec<u8>) -> Self {
        Self(cml_multi_era::byron::transaction::ByronRedeemerScript::new(
            u16, index_1,
        ))
    }
}

impl From<cml_multi_era::byron::transaction::ByronRedeemerScript> for ByronRedeemerScript {
    fn from(native: cml_multi_era::byron::transaction::ByronRedeemerScript) -> Self {
        Self(native)
    }
}

impl From<ByronRedeemerScript> for cml_multi_era::byron::transaction::ByronRedeemerScript {
    fn from(wasm: ByronRedeemerScript) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::byron::transaction::ByronRedeemerScript> for ByronRedeemerScript {
    fn as_ref(&self) -> &cml_multi_era::byron::transaction::ByronRedeemerScript {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronRedeemerWitnessEntry(cml_multi_era::byron::transaction::ByronRedeemerWitnessEntry);

#[wasm_bindgen]
impl ByronRedeemerWitnessEntry {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_multi_era::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<ByronRedeemerWitnessEntry, JsValue> {
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

    pub fn from_json(json: &str) -> Result<ByronRedeemerWitnessEntry, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn byron_pub_key(&self) -> ByronPubKey {
        self.0.byron_pub_key.clone()
    }

    pub fn byron_signature(&self) -> ByronSignature {
        self.0.byron_signature.clone()
    }

    pub fn new(byron_pub_key: ByronPubKey, byron_signature: ByronSignature) -> Self {
        Self(
            cml_multi_era::byron::transaction::ByronRedeemerWitnessEntry::new(
                byron_pub_key,
                byron_signature,
            ),
        )
    }
}

impl From<cml_multi_era::byron::transaction::ByronRedeemerWitnessEntry>
    for ByronRedeemerWitnessEntry
{
    fn from(native: cml_multi_era::byron::transaction::ByronRedeemerWitnessEntry) -> Self {
        Self(native)
    }
}

impl From<ByronRedeemerWitnessEntry>
    for cml_multi_era::byron::transaction::ByronRedeemerWitnessEntry
{
    fn from(wasm: ByronRedeemerWitnessEntry) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::byron::transaction::ByronRedeemerWitnessEntry>
    for ByronRedeemerWitnessEntry
{
    fn as_ref(&self) -> &cml_multi_era::byron::transaction::ByronRedeemerWitnessEntry {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronScriptWitness(cml_multi_era::byron::transaction::ByronScriptWitness);

#[wasm_bindgen]
impl ByronScriptWitness {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_multi_era::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<ByronScriptWitness, JsValue> {
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

    pub fn from_json(json: &str) -> Result<ByronScriptWitness, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn index_1(&self) -> ByronScriptWitnessEntry {
        self.0.index_1.clone().into()
    }

    pub fn new(index_1: &ByronScriptWitnessEntry) -> Self {
        Self(cml_multi_era::byron::transaction::ByronScriptWitness::new(
            index_1.clone().into(),
        ))
    }
}

impl From<cml_multi_era::byron::transaction::ByronScriptWitness> for ByronScriptWitness {
    fn from(native: cml_multi_era::byron::transaction::ByronScriptWitness) -> Self {
        Self(native)
    }
}

impl From<ByronScriptWitness> for cml_multi_era::byron::transaction::ByronScriptWitness {
    fn from(wasm: ByronScriptWitness) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::byron::transaction::ByronScriptWitness> for ByronScriptWitness {
    fn as_ref(&self) -> &cml_multi_era::byron::transaction::ByronScriptWitness {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronScriptWitnessEntry(cml_multi_era::byron::transaction::ByronScriptWitnessEntry);

#[wasm_bindgen]
impl ByronScriptWitnessEntry {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_multi_era::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<ByronScriptWitnessEntry, JsValue> {
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

    pub fn from_json(json: &str) -> Result<ByronScriptWitnessEntry, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn byron_validator_script(&self) -> ByronValidatorScript {
        self.0.byron_validator_script.clone().into()
    }

    pub fn byron_redeemer_script(&self) -> ByronRedeemerScript {
        self.0.byron_redeemer_script.clone().into()
    }

    pub fn new(
        byron_validator_script: &ByronValidatorScript,
        byron_redeemer_script: &ByronRedeemerScript,
    ) -> Self {
        Self(
            cml_multi_era::byron::transaction::ByronScriptWitnessEntry::new(
                byron_validator_script.clone().into(),
                byron_redeemer_script.clone().into(),
            ),
        )
    }
}

impl From<cml_multi_era::byron::transaction::ByronScriptWitnessEntry> for ByronScriptWitnessEntry {
    fn from(native: cml_multi_era::byron::transaction::ByronScriptWitnessEntry) -> Self {
        Self(native)
    }
}

impl From<ByronScriptWitnessEntry> for cml_multi_era::byron::transaction::ByronScriptWitnessEntry {
    fn from(wasm: ByronScriptWitnessEntry) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::byron::transaction::ByronScriptWitnessEntry> for ByronScriptWitnessEntry {
    fn as_ref(&self) -> &cml_multi_era::byron::transaction::ByronScriptWitnessEntry {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronTx(cml_multi_era::byron::transaction::ByronTx);

#[wasm_bindgen]
impl ByronTx {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_multi_era::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<ByronTx, JsValue> {
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

    pub fn from_json(json: &str) -> Result<ByronTx, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn inputs(&self) -> ByronTxInList {
        self.0.inputs.clone().into()
    }

    pub fn outputs(&self) -> ByronTxOutList {
        self.0.outputs.clone().into()
    }

    pub fn attrs(&self) -> ByronAttributes {
        self.0.attrs.clone().into()
    }

    pub fn new(inputs: &ByronTxInList, outputs: &ByronTxOutList, attrs: &ByronAttributes) -> Self {
        Self(cml_multi_era::byron::transaction::ByronTx::new(
            inputs.clone().into(),
            outputs.clone().into(),
            attrs.clone().into(),
        ))
    }
}

impl From<cml_multi_era::byron::transaction::ByronTx> for ByronTx {
    fn from(native: cml_multi_era::byron::transaction::ByronTx) -> Self {
        Self(native)
    }
}

impl From<ByronTx> for cml_multi_era::byron::transaction::ByronTx {
    fn from(wasm: ByronTx) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::byron::transaction::ByronTx> for ByronTx {
    fn as_ref(&self) -> &cml_multi_era::byron::transaction::ByronTx {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronTxIn(cml_multi_era::byron::transaction::ByronTxIn);

#[wasm_bindgen]
impl ByronTxIn {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_multi_era::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<ByronTxIn, JsValue> {
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

    pub fn from_json(json: &str) -> Result<ByronTxIn, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn new_byron_tx_in_regular(byron_tx_in_regular: &ByronTxInRegular) -> Self {
        Self(
            cml_multi_era::byron::transaction::ByronTxIn::new_byron_tx_in_regular(
                byron_tx_in_regular.clone().into(),
            ),
        )
    }

    pub fn new_byron_tx_in_genesis(byron_tx_in_genesis: &ByronTxInGenesis) -> Self {
        Self(
            cml_multi_era::byron::transaction::ByronTxIn::new_byron_tx_in_genesis(
                byron_tx_in_genesis.clone().into(),
            ),
        )
    }

    pub fn kind(&self) -> ByronTxInKind {
        match &self.0 {
            cml_multi_era::byron::transaction::ByronTxIn::ByronTxInRegular(_) => {
                ByronTxInKind::ByronTxInRegular
            }
            cml_multi_era::byron::transaction::ByronTxIn::ByronTxInGenesis(_) => {
                ByronTxInKind::ByronTxInGenesis
            }
        }
    }

    pub fn as_byron_tx_in_regular(&self) -> Option<ByronTxInRegular> {
        match &self.0 {
            cml_multi_era::byron::transaction::ByronTxIn::ByronTxInRegular(byron_tx_in_regular) => {
                Some(byron_tx_in_regular.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_byron_tx_in_genesis(&self) -> Option<ByronTxInGenesis> {
        match &self.0 {
            cml_multi_era::byron::transaction::ByronTxIn::ByronTxInGenesis(byron_tx_in_genesis) => {
                Some(byron_tx_in_genesis.clone().into())
            }
            _ => None,
        }
    }
}

impl From<cml_multi_era::byron::transaction::ByronTxIn> for ByronTxIn {
    fn from(native: cml_multi_era::byron::transaction::ByronTxIn) -> Self {
        Self(native)
    }
}

impl From<ByronTxIn> for cml_multi_era::byron::transaction::ByronTxIn {
    fn from(wasm: ByronTxIn) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::byron::transaction::ByronTxIn> for ByronTxIn {
    fn as_ref(&self) -> &cml_multi_era::byron::transaction::ByronTxIn {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronTxInGenesis(cml_multi_era::byron::transaction::ByronTxInGenesis);

#[wasm_bindgen]
impl ByronTxInGenesis {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_multi_era::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<ByronTxInGenesis, JsValue> {
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

    pub fn from_json(json: &str) -> Result<ByronTxInGenesis, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn u8(&self) -> u8 {
        self.0.u8
    }

    pub fn index_1(&self) -> Vec<u8> {
        self.0.index_1.clone()
    }

    pub fn new(u8: u8, index_1: Vec<u8>) -> Self {
        Self(cml_multi_era::byron::transaction::ByronTxInGenesis::new(
            u8, index_1,
        ))
    }
}

impl From<cml_multi_era::byron::transaction::ByronTxInGenesis> for ByronTxInGenesis {
    fn from(native: cml_multi_era::byron::transaction::ByronTxInGenesis) -> Self {
        Self(native)
    }
}

impl From<ByronTxInGenesis> for cml_multi_era::byron::transaction::ByronTxInGenesis {
    fn from(wasm: ByronTxInGenesis) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::byron::transaction::ByronTxInGenesis> for ByronTxInGenesis {
    fn as_ref(&self) -> &cml_multi_era::byron::transaction::ByronTxInGenesis {
        &self.0
    }
}

#[wasm_bindgen]
pub enum ByronTxInKind {
    ByronTxInRegular,
    ByronTxInGenesis,
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronTxInRegular(cml_multi_era::byron::transaction::ByronTxInRegular);

#[wasm_bindgen]
impl ByronTxInRegular {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_multi_era::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<ByronTxInRegular, JsValue> {
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

    pub fn from_json(json: &str) -> Result<ByronTxInRegular, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn index_1(&self) -> ByronTxOutPtr {
        self.0.index_1.clone().into()
    }

    pub fn new(index_1: &ByronTxOutPtr) -> Self {
        Self(cml_multi_era::byron::transaction::ByronTxInRegular::new(
            index_1.clone().into(),
        ))
    }
}

impl From<cml_multi_era::byron::transaction::ByronTxInRegular> for ByronTxInRegular {
    fn from(native: cml_multi_era::byron::transaction::ByronTxInRegular) -> Self {
        Self(native)
    }
}

impl From<ByronTxInRegular> for cml_multi_era::byron::transaction::ByronTxInRegular {
    fn from(wasm: ByronTxInRegular) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::byron::transaction::ByronTxInRegular> for ByronTxInRegular {
    fn as_ref(&self) -> &cml_multi_era::byron::transaction::ByronTxInRegular {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronTxOutPtr(cml_multi_era::byron::transaction::ByronTxOutPtr);

#[wasm_bindgen]
impl ByronTxOutPtr {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_multi_era::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<ByronTxOutPtr, JsValue> {
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

    pub fn from_json(json: &str) -> Result<ByronTxOutPtr, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn byron_tx_id(&self) -> ByronTxId {
        self.0.byron_tx_id.clone().into()
    }

    pub fn u32(&self) -> u32 {
        self.0.u32
    }

    pub fn new(byron_tx_id: &ByronTxId, u32: u32) -> Self {
        Self(cml_multi_era::byron::transaction::ByronTxOutPtr::new(
            byron_tx_id.clone().into(),
            u32,
        ))
    }
}

impl From<cml_multi_era::byron::transaction::ByronTxOutPtr> for ByronTxOutPtr {
    fn from(native: cml_multi_era::byron::transaction::ByronTxOutPtr) -> Self {
        Self(native)
    }
}

impl From<ByronTxOutPtr> for cml_multi_era::byron::transaction::ByronTxOutPtr {
    fn from(wasm: ByronTxOutPtr) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::byron::transaction::ByronTxOutPtr> for ByronTxOutPtr {
    fn as_ref(&self) -> &cml_multi_era::byron::transaction::ByronTxOutPtr {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronTxProof(cml_multi_era::byron::transaction::ByronTxProof);

#[wasm_bindgen]
impl ByronTxProof {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_multi_era::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<ByronTxProof, JsValue> {
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

    pub fn from_json(json: &str) -> Result<ByronTxProof, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn u32(&self) -> u32 {
        self.0.u32
    }

    pub fn blake2b256(&self) -> Blake2b256 {
        self.0.blake2b256.clone().into()
    }

    pub fn blake2b2562(&self) -> Blake2b256 {
        self.0.blake2b2562.clone().into()
    }

    pub fn new(u32: u32, blake2b256: &Blake2b256, blake2b2562: &Blake2b256) -> Self {
        Self(cml_multi_era::byron::transaction::ByronTxProof::new(
            u32,
            blake2b256.clone().into(),
            blake2b2562.clone().into(),
        ))
    }
}

impl From<cml_multi_era::byron::transaction::ByronTxProof> for ByronTxProof {
    fn from(native: cml_multi_era::byron::transaction::ByronTxProof) -> Self {
        Self(native)
    }
}

impl From<ByronTxProof> for cml_multi_era::byron::transaction::ByronTxProof {
    fn from(wasm: ByronTxProof) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::byron::transaction::ByronTxProof> for ByronTxProof {
    fn as_ref(&self) -> &cml_multi_era::byron::transaction::ByronTxProof {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronTxWitness(cml_multi_era::byron::transaction::ByronTxWitness);

#[wasm_bindgen]
impl ByronTxWitness {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_multi_era::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<ByronTxWitness, JsValue> {
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

    pub fn from_json(json: &str) -> Result<ByronTxWitness, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn new_byron_pk_witness(index_1: &ByronPkWitnessEntry) -> Self {
        Self(
            cml_multi_era::byron::transaction::ByronTxWitness::new_byron_pk_witness(
                index_1.clone().into(),
            ),
        )
    }

    pub fn new_byron_script_witness(index_1: &ByronScriptWitnessEntry) -> Self {
        Self(
            cml_multi_era::byron::transaction::ByronTxWitness::new_byron_script_witness(
                index_1.clone().into(),
            ),
        )
    }

    pub fn new_byron_redeem_witness(index_1: &ByronRedeemerWitnessEntry) -> Self {
        Self(
            cml_multi_era::byron::transaction::ByronTxWitness::new_byron_redeem_witness(
                index_1.clone().into(),
            ),
        )
    }

    pub fn kind(&self) -> ByronTxWitnessKind {
        match &self.0 {
            cml_multi_era::byron::transaction::ByronTxWitness::ByronPkWitness(_) => {
                ByronTxWitnessKind::ByronPkWitness
            }
            cml_multi_era::byron::transaction::ByronTxWitness::ByronScriptWitness(_) => {
                ByronTxWitnessKind::ByronScriptWitness
            }
            cml_multi_era::byron::transaction::ByronTxWitness::ByronRedeemWitness(_) => {
                ByronTxWitnessKind::ByronRedeemWitness
            }
        }
    }

    pub fn as_byron_pk_witness(&self) -> Option<ByronPkWitness> {
        match &self.0 {
            cml_multi_era::byron::transaction::ByronTxWitness::ByronPkWitness(byron_pk_witness) => {
                Some(byron_pk_witness.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_byron_script_witness(&self) -> Option<ByronScriptWitness> {
        match &self.0 {
            cml_multi_era::byron::transaction::ByronTxWitness::ByronScriptWitness(
                byron_script_witness,
            ) => Some(byron_script_witness.clone().into()),
            _ => None,
        }
    }

    pub fn as_byron_redeem_witness(&self) -> Option<ByronRedeemWitness> {
        match &self.0 {
            cml_multi_era::byron::transaction::ByronTxWitness::ByronRedeemWitness(
                byron_redeem_witness,
            ) => Some(byron_redeem_witness.clone().into()),
            _ => None,
        }
    }
}

impl From<cml_multi_era::byron::transaction::ByronTxWitness> for ByronTxWitness {
    fn from(native: cml_multi_era::byron::transaction::ByronTxWitness) -> Self {
        Self(native)
    }
}

impl From<ByronTxWitness> for cml_multi_era::byron::transaction::ByronTxWitness {
    fn from(wasm: ByronTxWitness) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::byron::transaction::ByronTxWitness> for ByronTxWitness {
    fn as_ref(&self) -> &cml_multi_era::byron::transaction::ByronTxWitness {
        &self.0
    }
}

#[wasm_bindgen]
pub enum ByronTxWitnessKind {
    ByronPkWitness,
    ByronScriptWitness,
    ByronRedeemWitness,
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronValidatorScript(cml_multi_era::byron::transaction::ByronValidatorScript);

#[wasm_bindgen]
impl ByronValidatorScript {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_multi_era::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<ByronValidatorScript, JsValue> {
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

    pub fn from_json(json: &str) -> Result<ByronValidatorScript, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn u16(&self) -> u16 {
        self.0.u16
    }

    pub fn index_1(&self) -> Vec<u8> {
        self.0.index_1.clone()
    }

    pub fn new(u16: u16, index_1: Vec<u8>) -> Self {
        Self(cml_multi_era::byron::transaction::ByronValidatorScript::new(u16, index_1))
    }
}

impl From<cml_multi_era::byron::transaction::ByronValidatorScript> for ByronValidatorScript {
    fn from(native: cml_multi_era::byron::transaction::ByronValidatorScript) -> Self {
        Self(native)
    }
}

impl From<ByronValidatorScript> for cml_multi_era::byron::transaction::ByronValidatorScript {
    fn from(wasm: ByronValidatorScript) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::byron::transaction::ByronValidatorScript> for ByronValidatorScript {
    fn as_ref(&self) -> &cml_multi_era::byron::transaction::ByronValidatorScript {
        &self.0
    }
}
