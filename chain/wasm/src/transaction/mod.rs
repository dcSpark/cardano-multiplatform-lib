// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use super::{
    Value, BootstrapWitnessList, CertificateList, NativeScriptList, NetworkId, PlutusDataList,
    PlutusV1ScriptList, PlutusV2ScriptList, RedeemerList, Slot, TransactionInputList,
    TransactionOutputList, Update, VkeywitnessList, Withdrawals,
};
use crate::Script;
use crate::address::Address;
use crate::assets::{Coin, Mint};
use crate::auxdata::AuxiliaryData;
use cml_crypto_wasm::{
    AuxiliaryDataHash, DatumHash, Ed25519KeyHash, ScriptDataHash, TransactionHash,
};
use crate::plutus::PlutusData;
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

pub mod utils;

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct AlonzoTxOut(cml_chain::transaction::AlonzoTxOut);

#[wasm_bindgen]
impl AlonzoTxOut {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<AlonzoTxOut, JsValue> {
        cml_chain::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
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

    pub fn from_json(json: &str) -> Result<AlonzoTxOut, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn address(&self) -> Address {
        self.0.address.clone().into()
    }

    pub fn amount(&self) -> Value {
        self.0.amount.clone().into()
    }

    pub fn datum_hash(&self) -> DatumHash {
        self.0.datum_hash.clone().into()
    }

    pub fn new(address: &Address, amount: &Value, datum_hash: &DatumHash) -> Self {
        Self(cml_chain::transaction::AlonzoTxOut::new(
            address.clone().into(),
            amount.clone().into(),
            datum_hash.clone().into(),
        ))
    }
}

impl From<cml_chain::transaction::AlonzoTxOut> for AlonzoTxOut {
    fn from(native: cml_chain::transaction::AlonzoTxOut) -> Self {
        Self(native)
    }
}

impl From<AlonzoTxOut> for cml_chain::transaction::AlonzoTxOut {
    fn from(wasm: AlonzoTxOut) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::transaction::AlonzoTxOut> for AlonzoTxOut {
    fn as_ref(&self) -> &cml_chain::transaction::AlonzoTxOut {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct BabbageTxOut(cml_chain::transaction::BabbageTxOut);

#[wasm_bindgen]
impl BabbageTxOut {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<BabbageTxOut, JsValue> {
        cml_chain::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
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

    pub fn from_json(json: &str) -> Result<BabbageTxOut, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn address(&self) -> Address {
        self.0.address.clone().into()
    }

    pub fn amount(&self) -> Value {
        self.0.amount.clone().into()
    }

    pub fn set_datum_option(&mut self, datum_option: &DatumOption) {
        self.0.datum_option = Some(datum_option.clone().into())
    }

    pub fn datum_option(&self) -> Option<DatumOption> {
        self.0.datum_option.clone().map(std::convert::Into::into)
    }

    pub fn set_script_reference(&mut self, script_reference: &ScriptRef) {
        self.0.script_reference = Some(script_reference.clone().into())
    }

    pub fn script_reference(&self) -> Option<ScriptRef> {
        self.0
            .script_reference
            .clone()
            .map(std::convert::Into::into)
    }

    pub fn new(address: &Address, amount: &Value) -> Self {
        Self(cml_chain::transaction::BabbageTxOut::new(
            address.clone().into(),
            amount.clone().into(),
        ))
    }
}

impl From<cml_chain::transaction::BabbageTxOut> for BabbageTxOut {
    fn from(native: cml_chain::transaction::BabbageTxOut) -> Self {
        Self(native)
    }
}

impl From<BabbageTxOut> for cml_chain::transaction::BabbageTxOut {
    fn from(wasm: BabbageTxOut) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::transaction::BabbageTxOut> for BabbageTxOut {
    fn as_ref(&self) -> &cml_chain::transaction::BabbageTxOut {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct DatumOption(cml_chain::transaction::DatumOption);

#[wasm_bindgen]
impl DatumOption {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<DatumOption, JsValue> {
        cml_chain::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
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

    pub fn from_json(json: &str) -> Result<DatumOption, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn new_hash(datum_hash: &DatumHash) -> Self {
        Self(cml_chain::transaction::DatumOption::new_hash(
            datum_hash.clone().into(),
        ))
    }

    pub fn new_datum(datum: &PlutusData) -> Self {
        Self(cml_chain::transaction::DatumOption::new_datum(
            datum.clone().into(),
        ))
    }

    pub fn kind(&self) -> DatumOptionKind {
        match &self.0 {
            cml_chain::transaction::DatumOption::Hash { .. } => DatumOptionKind::Hash,
            cml_chain::transaction::DatumOption::Datum { .. } => DatumOptionKind::Datum,
        }
    }

    pub fn as_hash(&self) -> Option<DatumHash> {
        match &self.0 {
            cml_chain::transaction::DatumOption::Hash { datum_hash, .. } => {
                Some(datum_hash.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_datum(&self) -> Option<PlutusData> {
        match &self.0 {
            cml_chain::transaction::DatumOption::Datum { datum, .. } => Some(datum.clone().into()),
            _ => None,
        }
    }
}

impl From<cml_chain::transaction::DatumOption> for DatumOption {
    fn from(native: cml_chain::transaction::DatumOption) -> Self {
        Self(native)
    }
}

impl From<DatumOption> for cml_chain::transaction::DatumOption {
    fn from(wasm: DatumOption) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::transaction::DatumOption> for DatumOption {
    fn as_ref(&self) -> &cml_chain::transaction::DatumOption {
        &self.0
    }
}

#[wasm_bindgen]
pub enum DatumOptionKind {
    Hash,
    Datum,
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct NativeScript(cml_chain::transaction::NativeScript);

#[wasm_bindgen]
impl NativeScript {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<NativeScript, JsValue> {
        cml_chain::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
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

    pub fn from_json(json: &str) -> Result<NativeScript, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn new_script_pubkey(ed25519_key_hash: &Ed25519KeyHash) -> Self {
        Self(cml_chain::transaction::NativeScript::new_script_pubkey(
            ed25519_key_hash.clone().into(),
        ))
    }

    pub fn new_script_all(native_scripts: &NativeScriptList) -> Self {
        Self(cml_chain::transaction::NativeScript::new_script_all(
            native_scripts.clone().into(),
        ))
    }

    pub fn new_script_any(native_scripts: &NativeScriptList) -> Self {
        Self(cml_chain::transaction::NativeScript::new_script_any(
            native_scripts.clone().into(),
        ))
    }

    pub fn new_script_n_of_k(n: u64, native_scripts: &NativeScriptList) -> Self {
        Self(cml_chain::transaction::NativeScript::new_script_n_of_k(
            n,
            native_scripts.clone().into(),
        ))
    }

    pub fn new_script_invalid_before(before: Slot) -> Self {
        Self(cml_chain::transaction::NativeScript::new_script_invalid_before(before))
    }

    pub fn new_script_invalid_hereafter(after: Slot) -> Self {
        Self(cml_chain::transaction::NativeScript::new_script_invalid_hereafter(after))
    }

    pub fn kind(&self) -> NativeScriptKind {
        match &self.0 {
            cml_chain::transaction::NativeScript::ScriptPubkey(_) => NativeScriptKind::ScriptPubkey,
            cml_chain::transaction::NativeScript::ScriptAll(_) => NativeScriptKind::ScriptAll,
            cml_chain::transaction::NativeScript::ScriptAny(_) => NativeScriptKind::ScriptAny,
            cml_chain::transaction::NativeScript::ScriptNOfK(_) => NativeScriptKind::ScriptNOfK,
            cml_chain::transaction::NativeScript::ScriptInvalidBefore(_) => {
                NativeScriptKind::ScriptInvalidBefore
            }
            cml_chain::transaction::NativeScript::ScriptInvalidHereafter(_) => {
                NativeScriptKind::ScriptInvalidHereafter
            }
        }
    }

    pub fn as_script_pubkey(&self) -> Option<ScriptPubkey> {
        match &self.0 {
            cml_chain::transaction::NativeScript::ScriptPubkey(script_pubkey) => {
                Some(script_pubkey.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_script_all(&self) -> Option<ScriptAll> {
        match &self.0 {
            cml_chain::transaction::NativeScript::ScriptAll(script_all) => {
                Some(script_all.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_script_any(&self) -> Option<ScriptAny> {
        match &self.0 {
            cml_chain::transaction::NativeScript::ScriptAny(script_any) => {
                Some(script_any.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_script_n_of_k(&self) -> Option<ScriptNOfK> {
        match &self.0 {
            cml_chain::transaction::NativeScript::ScriptNOfK(script_n_of_k) => {
                Some(script_n_of_k.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_script_invalid_before(&self) -> Option<ScriptInvalidBefore> {
        match &self.0 {
            cml_chain::transaction::NativeScript::ScriptInvalidBefore(script_invalid_before) => {
                Some(script_invalid_before.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_script_invalid_hereafter(&self) -> Option<ScriptInvalidHereafter> {
        match &self.0 {
            cml_chain::transaction::NativeScript::ScriptInvalidHereafter(
                script_invalid_hereafter,
            ) => Some(script_invalid_hereafter.clone().into()),
            _ => None,
        }
    }
}

impl From<cml_chain::transaction::NativeScript> for NativeScript {
    fn from(native: cml_chain::transaction::NativeScript) -> Self {
        Self(native)
    }
}

impl From<NativeScript> for cml_chain::transaction::NativeScript {
    fn from(wasm: NativeScript) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::transaction::NativeScript> for NativeScript {
    fn as_ref(&self) -> &cml_chain::transaction::NativeScript {
        &self.0
    }
}

#[wasm_bindgen]
pub enum NativeScriptKind {
    ScriptPubkey,
    ScriptAll,
    ScriptAny,
    ScriptNOfK,
    ScriptInvalidBefore,
    ScriptInvalidHereafter,
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct RequiredSigners(Vec<cml_chain::crypto::Ed25519KeyHash>);

#[wasm_bindgen]
impl RequiredSigners {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Ed25519KeyHash {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &Ed25519KeyHash) {
        self.0.push(elem.clone().into());
    }
}

impl From<Vec<cml_chain::crypto::Ed25519KeyHash>> for RequiredSigners {
    fn from(native: Vec<cml_chain::crypto::Ed25519KeyHash>) -> Self {
        Self(native)
    }
}

impl From<RequiredSigners> for Vec<cml_chain::crypto::Ed25519KeyHash> {
    fn from(wasm: RequiredSigners) -> Self {
        wasm.0
    }
}

impl AsRef<Vec<cml_chain::crypto::Ed25519KeyHash>> for RequiredSigners {
    fn as_ref(&self) -> &Vec<cml_chain::crypto::Ed25519KeyHash> {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ScriptAll(cml_chain::transaction::ScriptAll);

#[wasm_bindgen]
impl ScriptAll {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<ScriptAll, JsValue> {
        cml_chain::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
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

    pub fn from_json(json: &str) -> Result<ScriptAll, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn native_scripts(&self) -> NativeScriptList {
        self.0.native_scripts.clone().into()
    }

    pub fn new(native_scripts: &NativeScriptList) -> Self {
        Self(cml_chain::transaction::ScriptAll::new(
            native_scripts.clone().into(),
        ))
    }
}

impl From<cml_chain::transaction::ScriptAll> for ScriptAll {
    fn from(native: cml_chain::transaction::ScriptAll) -> Self {
        Self(native)
    }
}

impl From<ScriptAll> for cml_chain::transaction::ScriptAll {
    fn from(wasm: ScriptAll) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::transaction::ScriptAll> for ScriptAll {
    fn as_ref(&self) -> &cml_chain::transaction::ScriptAll {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ScriptAny(cml_chain::transaction::ScriptAny);

#[wasm_bindgen]
impl ScriptAny {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<ScriptAny, JsValue> {
        cml_chain::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
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

    pub fn from_json(json: &str) -> Result<ScriptAny, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn native_scripts(&self) -> NativeScriptList {
        self.0.native_scripts.clone().into()
    }

    pub fn new(native_scripts: &NativeScriptList) -> Self {
        Self(cml_chain::transaction::ScriptAny::new(
            native_scripts.clone().into(),
        ))
    }
}

impl From<cml_chain::transaction::ScriptAny> for ScriptAny {
    fn from(native: cml_chain::transaction::ScriptAny) -> Self {
        Self(native)
    }
}

impl From<ScriptAny> for cml_chain::transaction::ScriptAny {
    fn from(wasm: ScriptAny) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::transaction::ScriptAny> for ScriptAny {
    fn as_ref(&self) -> &cml_chain::transaction::ScriptAny {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ScriptInvalidBefore(cml_chain::transaction::ScriptInvalidBefore);

#[wasm_bindgen]
impl ScriptInvalidBefore {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<ScriptInvalidBefore, JsValue> {
        cml_chain::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
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

    pub fn from_json(json: &str) -> Result<ScriptInvalidBefore, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn before(&self) -> Slot {
        self.0.before
    }

    pub fn new(before: Slot) -> Self {
        Self(cml_chain::transaction::ScriptInvalidBefore::new(before))
    }
}

impl From<cml_chain::transaction::ScriptInvalidBefore> for ScriptInvalidBefore {
    fn from(native: cml_chain::transaction::ScriptInvalidBefore) -> Self {
        Self(native)
    }
}

impl From<ScriptInvalidBefore> for cml_chain::transaction::ScriptInvalidBefore {
    fn from(wasm: ScriptInvalidBefore) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::transaction::ScriptInvalidBefore> for ScriptInvalidBefore {
    fn as_ref(&self) -> &cml_chain::transaction::ScriptInvalidBefore {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ScriptInvalidHereafter(cml_chain::transaction::ScriptInvalidHereafter);

#[wasm_bindgen]
impl ScriptInvalidHereafter {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<ScriptInvalidHereafter, JsValue> {
        cml_chain::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
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

    pub fn from_json(json: &str) -> Result<ScriptInvalidHereafter, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn after(&self) -> Slot {
        self.0.after
    }

    pub fn new(after: Slot) -> Self {
        Self(cml_chain::transaction::ScriptInvalidHereafter::new(after))
    }
}

impl From<cml_chain::transaction::ScriptInvalidHereafter> for ScriptInvalidHereafter {
    fn from(native: cml_chain::transaction::ScriptInvalidHereafter) -> Self {
        Self(native)
    }
}

impl From<ScriptInvalidHereafter> for cml_chain::transaction::ScriptInvalidHereafter {
    fn from(wasm: ScriptInvalidHereafter) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::transaction::ScriptInvalidHereafter> for ScriptInvalidHereafter {
    fn as_ref(&self) -> &cml_chain::transaction::ScriptInvalidHereafter {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ScriptNOfK(cml_chain::transaction::ScriptNOfK);

#[wasm_bindgen]
impl ScriptNOfK {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<ScriptNOfK, JsValue> {
        cml_chain::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
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

    pub fn from_json(json: &str) -> Result<ScriptNOfK, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn n(&self) -> u64 {
        self.0.n
    }

    pub fn native_scripts(&self) -> NativeScriptList {
        self.0.native_scripts.clone().into()
    }

    pub fn new(n: u64, native_scripts: &NativeScriptList) -> Self {
        Self(cml_chain::transaction::ScriptNOfK::new(
            n,
            native_scripts.clone().into(),
        ))
    }
}

impl From<cml_chain::transaction::ScriptNOfK> for ScriptNOfK {
    fn from(native: cml_chain::transaction::ScriptNOfK) -> Self {
        Self(native)
    }
}

impl From<ScriptNOfK> for cml_chain::transaction::ScriptNOfK {
    fn from(wasm: ScriptNOfK) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::transaction::ScriptNOfK> for ScriptNOfK {
    fn as_ref(&self) -> &cml_chain::transaction::ScriptNOfK {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ScriptPubkey(cml_chain::transaction::ScriptPubkey);

#[wasm_bindgen]
impl ScriptPubkey {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<ScriptPubkey, JsValue> {
        cml_chain::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
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

    pub fn from_json(json: &str) -> Result<ScriptPubkey, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn ed25519_key_hash(&self) -> Ed25519KeyHash {
        self.0.ed25519_key_hash.clone().into()
    }

    pub fn new(ed25519_key_hash: &Ed25519KeyHash) -> Self {
        Self(cml_chain::transaction::ScriptPubkey::new(
            ed25519_key_hash.clone().into(),
        ))
    }
}

impl From<cml_chain::transaction::ScriptPubkey> for ScriptPubkey {
    fn from(native: cml_chain::transaction::ScriptPubkey) -> Self {
        Self(native)
    }
}

impl From<ScriptPubkey> for cml_chain::transaction::ScriptPubkey {
    fn from(wasm: ScriptPubkey) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::transaction::ScriptPubkey> for ScriptPubkey {
    fn as_ref(&self) -> &cml_chain::transaction::ScriptPubkey {
        &self.0
    }
}

pub type ScriptRef = Script;

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ShelleyTxOut(cml_chain::transaction::ShelleyTxOut);

#[wasm_bindgen]
impl ShelleyTxOut {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<ShelleyTxOut, JsValue> {
        cml_chain::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
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

    pub fn from_json(json: &str) -> Result<ShelleyTxOut, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn address(&self) -> Address {
        self.0.address.clone().into()
    }

    pub fn amount(&self) -> Value {
        self.0.amount.clone().into()
    }

    pub fn new(address: &Address, amount: &Value) -> Self {
        Self(cml_chain::transaction::ShelleyTxOut::new(
            address.clone().into(),
            amount.clone().into(),
        ))
    }
}

impl From<cml_chain::transaction::ShelleyTxOut> for ShelleyTxOut {
    fn from(native: cml_chain::transaction::ShelleyTxOut) -> Self {
        Self(native)
    }
}

impl From<ShelleyTxOut> for cml_chain::transaction::ShelleyTxOut {
    fn from(wasm: ShelleyTxOut) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::transaction::ShelleyTxOut> for ShelleyTxOut {
    fn as_ref(&self) -> &cml_chain::transaction::ShelleyTxOut {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct Transaction(cml_chain::transaction::Transaction);

#[wasm_bindgen]
impl Transaction {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<Transaction, JsValue> {
        cml_chain::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
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

    pub fn from_json(json: &str) -> Result<Transaction, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn body(&self) -> TransactionBody {
        self.0.body.clone().into()
    }

    pub fn witness_set(&self) -> TransactionWitnessSet {
        self.0.witness_set.clone().into()
    }

    pub fn is_valid(&self) -> bool {
        self.0.is_valid
    }

    pub fn auxiliary_data(&self) -> Option<AuxiliaryData> {
        self.0.auxiliary_data.clone().map(std::convert::Into::into)
    }

    pub fn new(
        body: &TransactionBody,
        witness_set: &TransactionWitnessSet,
        is_valid: bool,
        auxiliary_data: Option<AuxiliaryData>,
    ) -> Self {
        Self(cml_chain::transaction::Transaction::new(
            body.clone().into(),
            witness_set.clone().into(),
            is_valid,
            auxiliary_data.map(Into::into),
        ))
    }
}

impl From<cml_chain::transaction::Transaction> for Transaction {
    fn from(native: cml_chain::transaction::Transaction) -> Self {
        Self(native)
    }
}

impl From<Transaction> for cml_chain::transaction::Transaction {
    fn from(wasm: Transaction) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::transaction::Transaction> for Transaction {
    fn as_ref(&self) -> &cml_chain::transaction::Transaction {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct TransactionBody(cml_chain::transaction::TransactionBody);

#[wasm_bindgen]
impl TransactionBody {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<TransactionBody, JsValue> {
        cml_chain::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
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

    pub fn from_json(json: &str) -> Result<TransactionBody, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn inputs(&self) -> TransactionInputList {
        self.0.inputs.clone().into()
    }

    pub fn outputs(&self) -> TransactionOutputList {
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

    pub fn set_update(&mut self, update: &Update) {
        self.0.update = Some(update.clone().into())
    }

    pub fn update(&self) -> Option<Update> {
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

    pub fn set_collateral_return(&mut self, collateral_return: &TransactionOutput) {
        self.0.collateral_return = Some(collateral_return.clone().into())
    }

    pub fn collateral_return(&self) -> Option<TransactionOutput> {
        self.0
            .collateral_return
            .clone()
            .map(std::convert::Into::into)
    }

    pub fn set_total_collateral(&mut self, total_collateral: Coin) {
        self.0.total_collateral = Some(total_collateral)
    }

    pub fn total_collateral(&self) -> Option<Coin> {
        self.0.total_collateral
    }

    pub fn set_reference_inputs(&mut self, reference_inputs: &TransactionInputList) {
        self.0.reference_inputs = Some(reference_inputs.clone().into())
    }

    pub fn reference_inputs(&self) -> Option<TransactionInputList> {
        self.0
            .reference_inputs
            .clone()
            .map(std::convert::Into::into)
    }

    pub fn new(inputs: &TransactionInputList, outputs: &TransactionOutputList, fee: Coin) -> Self {
        Self(cml_chain::transaction::TransactionBody::new(
            inputs.clone().into(),
            outputs.clone().into(),
            fee,
        ))
    }
}

impl From<cml_chain::transaction::TransactionBody> for TransactionBody {
    fn from(native: cml_chain::transaction::TransactionBody) -> Self {
        Self(native)
    }
}

impl From<TransactionBody> for cml_chain::transaction::TransactionBody {
    fn from(wasm: TransactionBody) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::transaction::TransactionBody> for TransactionBody {
    fn as_ref(&self) -> &cml_chain::transaction::TransactionBody {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct TransactionInput(cml_chain::transaction::TransactionInput);

#[wasm_bindgen]
impl TransactionInput {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<TransactionInput, JsValue> {
        cml_chain::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
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

    pub fn from_json(json: &str) -> Result<TransactionInput, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn transaction_id(&self) -> TransactionHash {
        self.0.transaction_id.clone().into()
    }

    pub fn index(&self) -> u64 {
        self.0.index
    }

    pub fn new(transaction_id: &TransactionHash, index: u64) -> Self {
        Self(cml_chain::transaction::TransactionInput::new(
            transaction_id.clone().into(),
            index,
        ))
    }
}

impl From<cml_chain::transaction::TransactionInput> for TransactionInput {
    fn from(native: cml_chain::transaction::TransactionInput) -> Self {
        Self(native)
    }
}

impl From<TransactionInput> for cml_chain::transaction::TransactionInput {
    fn from(wasm: TransactionInput) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::transaction::TransactionInput> for TransactionInput {
    fn as_ref(&self) -> &cml_chain::transaction::TransactionInput {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct TransactionOutput(cml_chain::transaction::TransactionOutput);

#[wasm_bindgen]
impl TransactionOutput {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<TransactionOutput, JsValue> {
        cml_chain::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
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

    pub fn from_json(json: &str) -> Result<TransactionOutput, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn new_shelley_tx_out(shelley_tx_out: &ShelleyTxOut) -> Self {
        Self(
            cml_chain::transaction::TransactionOutput::new_shelley_tx_out(
                shelley_tx_out.clone().into(),
            ),
        )
    }

    pub fn new_alonzo_tx_out(alonzo_tx_out: &AlonzoTxOut) -> Self {
        Self(
            cml_chain::transaction::TransactionOutput::new_alonzo_tx_out(
                alonzo_tx_out.clone().into(),
            ),
        )
    }

    pub fn new_babbage_tx_out(babbage_tx_out: &BabbageTxOut) -> Self {
        Self(
            cml_chain::transaction::TransactionOutput::new_babbage_tx_out(
                babbage_tx_out.clone().into(),
            ),
        )
    }

    pub fn kind(&self) -> TransactionOutputKind {
        match &self.0 {
            cml_chain::transaction::TransactionOutput::ShelleyTxOut(_) => {
                TransactionOutputKind::ShelleyTxOut
            }
            cml_chain::transaction::TransactionOutput::AlonzoTxOut(_) => {
                TransactionOutputKind::AlonzoTxOut
            }
            cml_chain::transaction::TransactionOutput::BabbageTxOut(_) => {
                TransactionOutputKind::BabbageTxOut
            }
        }
    }

    pub fn as_shelley_tx_out(&self) -> Option<ShelleyTxOut> {
        match &self.0 {
            cml_chain::transaction::TransactionOutput::ShelleyTxOut(shelley_tx_out) => {
                Some(shelley_tx_out.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_alonzo_tx_out(&self) -> Option<AlonzoTxOut> {
        match &self.0 {
            cml_chain::transaction::TransactionOutput::AlonzoTxOut(alonzo_tx_out) => {
                Some(alonzo_tx_out.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_babbage_tx_out(&self) -> Option<BabbageTxOut> {
        match &self.0 {
            cml_chain::transaction::TransactionOutput::BabbageTxOut(babbage_tx_out) => {
                Some(babbage_tx_out.clone().into())
            }
            _ => None,
        }
    }
}

impl From<cml_chain::transaction::TransactionOutput> for TransactionOutput {
    fn from(native: cml_chain::transaction::TransactionOutput) -> Self {
        Self(native)
    }
}

impl From<TransactionOutput> for cml_chain::transaction::TransactionOutput {
    fn from(wasm: TransactionOutput) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::transaction::TransactionOutput> for TransactionOutput {
    fn as_ref(&self) -> &cml_chain::transaction::TransactionOutput {
        &self.0
    }
}

#[wasm_bindgen]
pub enum TransactionOutputKind {
    ShelleyTxOut,
    AlonzoTxOut,
    BabbageTxOut,
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct TransactionWitnessSet(cml_chain::transaction::TransactionWitnessSet);

#[wasm_bindgen]
impl TransactionWitnessSet {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<TransactionWitnessSet, JsValue> {
        cml_chain::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
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

    pub fn from_json(json: &str) -> Result<TransactionWitnessSet, JsValue> {
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

    pub fn set_plutus_v2_scripts(&mut self, plutus_v2_scripts: &PlutusV2ScriptList) {
        self.0.plutus_v2_scripts = Some(plutus_v2_scripts.clone().into())
    }

    pub fn plutus_v2_scripts(&self) -> Option<PlutusV2ScriptList> {
        self.0
            .plutus_v2_scripts
            .clone()
            .map(std::convert::Into::into)
    }

    pub fn new() -> Self {
        Self(cml_chain::transaction::TransactionWitnessSet::new())
    }
}

impl From<cml_chain::transaction::TransactionWitnessSet> for TransactionWitnessSet {
    fn from(native: cml_chain::transaction::TransactionWitnessSet) -> Self {
        Self(native)
    }
}

impl From<TransactionWitnessSet> for cml_chain::transaction::TransactionWitnessSet {
    fn from(wasm: TransactionWitnessSet) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::transaction::TransactionWitnessSet> for TransactionWitnessSet {
    fn as_ref(&self) -> &cml_chain::transaction::TransactionWitnessSet {
        &self.0
    }
}
