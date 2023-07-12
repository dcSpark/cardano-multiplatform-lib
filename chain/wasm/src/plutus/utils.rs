use cml_chain::plutus::Language;
use cml_core_wasm::impl_wasm_conversions;
use wasm_bindgen::prelude::{wasm_bindgen, JsError, JsValue};
use cml_crypto_wasm::ScriptHash;
use crate::{PlutusDataList, RedeemerList, plutus::PlutusData};

use super::{ExUnits, PlutusV1Script, PlutusV2Script};

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ConstrPlutusData(cml_chain::plutus::ConstrPlutusData);

#[wasm_bindgen]
impl ConstrPlutusData {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<ConstrPlutusData, JsValue> {
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

    pub fn from_json(json: &str) -> Result<ConstrPlutusData, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn alternative(&self) -> u64 {
        self.0.alternative
    }

    pub fn fields(&self) -> PlutusDataList {
        self.0.fields.clone().into()
    }

    pub fn new(alternative: u64, fields: &PlutusDataList) -> Self {
        Self(cml_chain::plutus::ConstrPlutusData::new(
            alternative,
            fields.clone().into(),
        ))
    }
}

impl From<cml_chain::plutus::ConstrPlutusData> for ConstrPlutusData {
    fn from(native: cml_chain::plutus::ConstrPlutusData) -> Self {
        Self(native)
    }
}

impl From<ConstrPlutusData> for cml_chain::plutus::ConstrPlutusData {
    fn from(wasm: ConstrPlutusData) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::plutus::ConstrPlutusData> for ConstrPlutusData {
    fn as_ref(&self) -> &cml_chain::plutus::ConstrPlutusData {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct PlutusMap(cml_chain::plutus::PlutusMap);

impl_wasm_conversions!(cml_chain::plutus::PlutusMap, PlutusMap);

#[wasm_bindgen]
impl PlutusMap {
    pub fn new() -> Self {
        Self(cml_chain::plutus::PlutusMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Replaces all datums of a given key, if any exist.
    pub fn set(&mut self, key: &PlutusData, value: &PlutusData) {
        self.0.set(key.clone().into(), value.clone().into())
    }

    /// Gets the plutus datum corresponding to a given key, if it exists.
    /// Note: In the case of duplicate keys this only returns the first datum.
    /// This is an extremely rare occurence on-chain but can happen.
    pub fn get(&self, key: &PlutusData) -> Option<PlutusData> {
        self.0.get(key.as_ref()).map(|pd| pd.clone().into())
    }

    /// In the extremely unlikely situation there are duplicate keys, this gets all of a single key
    pub fn get_all(&self, key: &PlutusData) -> Option<PlutusDataList> {
        self
            .0
            .get_all(key.as_ref())
            .map(|datums| datums
                .into_iter()
                .map(|d| d.clone().into())
                .collect::<Vec<_>>()
                .into())
    }

    pub fn keys(&self) -> PlutusDataList {
        PlutusDataList(self.0.entries.iter().map(|(k, _v)| k.clone()).collect::<Vec<_>>())
    }
}

/// Version-agnostic Plutus script
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct PlutusScript(cml_chain::plutus::utils::PlutusScript);

#[wasm_bindgen]
impl PlutusScript {
    pub fn hash(&self) -> ScriptHash {
        self.0.hash().into()
    }

    pub fn as_v1(&self) -> Option<PlutusV1Script> {
        match &self.0 {
            cml_chain::plutus::utils::PlutusScript::PlutusV1(v1) => Some(v1.clone().into()),
            _=> None,
        }
    }

    pub fn as_v2(&self) -> Option<PlutusV2Script> {
        match &self.0 {
            cml_chain::plutus::utils::PlutusScript::PlutusV2(v2) => Some(v2.clone().into()),
            _=> None,
        }
    }

    pub fn version(&self) -> Language {
        self.0.version()
    }
}

impl PlutusV1Script {
    pub fn hash(&self) -> ScriptHash {
        self.0.hash().into()
    }
}

impl PlutusV2Script {
    pub fn hash(&self) -> ScriptHash {
        self.0.hash().into()
    }
}

impl ExUnits {
    pub fn checked_add(&self, other: &ExUnits) -> Result<ExUnits, JsError> {
        self.0.checked_add(other.as_ref())
            .map(Into::into)
            .map_err(Into::into)
    }
}

pub fn compute_total_ex_units(redeemers: &RedeemerList) -> Result<ExUnits, JsError> {
    cml_chain::plutus::utils::compute_total_ex_units(redeemers.as_ref())
        .map(Into::into)
        .map_err(Into::into)
}