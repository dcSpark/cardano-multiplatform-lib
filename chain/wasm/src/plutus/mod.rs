// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use super::{IntList, MapPlutusDataToPlutusData, PlutusDataList, SubCoin};
use crate::utils::BigInt;
pub use cml_chain::plutus::{Language, RedeemerTag};
use cml_core::ordered_hash_map::OrderedHashMap;
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

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

    pub fn constructor(&self) -> u64 {
        self.0.constructor
    }

    pub fn fields(&self) -> PlutusDataList {
        self.0.fields.clone().into()
    }

    pub fn new(constructor: u64, fields: &PlutusDataList) -> Self {
        Self(cml_chain::plutus::ConstrPlutusData::new(
            constructor,
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
pub struct CostModels(cml_chain::plutus::CostModels);

#[wasm_bindgen]
impl CostModels {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<CostModels, JsValue> {
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

    pub fn from_json(json: &str) -> Result<CostModels, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn set_plutus_v1(&mut self, plutus_v1: &IntList) {
        self.0.plutus_v1 = Some(plutus_v1.clone().into())
    }

    pub fn plutus_v1(&self) -> Option<IntList> {
        self.0.plutus_v1.clone().map(std::convert::Into::into)
    }

    pub fn set_plutus_v2(&mut self, plutus_v2: &IntList) {
        self.0.plutus_v2 = Some(plutus_v2.clone().into())
    }

    pub fn plutus_v2(&self) -> Option<IntList> {
        self.0.plutus_v2.clone().map(std::convert::Into::into)
    }

    pub fn new() -> Self {
        Self(cml_chain::plutus::CostModels::new())
    }
}

impl From<cml_chain::plutus::CostModels> for CostModels {
    fn from(native: cml_chain::plutus::CostModels) -> Self {
        Self(native)
    }
}

impl From<CostModels> for cml_chain::plutus::CostModels {
    fn from(wasm: CostModels) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::plutus::CostModels> for CostModels {
    fn as_ref(&self) -> &cml_chain::plutus::CostModels {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ExUnitPrices(cml_chain::plutus::ExUnitPrices);

#[wasm_bindgen]
impl ExUnitPrices {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<ExUnitPrices, JsValue> {
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

    pub fn from_json(json: &str) -> Result<ExUnitPrices, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn mem_price(&self) -> SubCoin {
        self.0.mem_price.clone().into()
    }

    pub fn step_price(&self) -> SubCoin {
        self.0.step_price.clone().into()
    }

    pub fn new(mem_price: &SubCoin, step_price: &SubCoin) -> Self {
        Self(cml_chain::plutus::ExUnitPrices::new(
            mem_price.clone().into(),
            step_price.clone().into(),
        ))
    }
}

impl From<cml_chain::plutus::ExUnitPrices> for ExUnitPrices {
    fn from(native: cml_chain::plutus::ExUnitPrices) -> Self {
        Self(native)
    }
}

impl From<ExUnitPrices> for cml_chain::plutus::ExUnitPrices {
    fn from(wasm: ExUnitPrices) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::plutus::ExUnitPrices> for ExUnitPrices {
    fn as_ref(&self) -> &cml_chain::plutus::ExUnitPrices {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ExUnits(cml_chain::plutus::ExUnits);

#[wasm_bindgen]
impl ExUnits {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<ExUnits, JsValue> {
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

    pub fn from_json(json: &str) -> Result<ExUnits, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn mem(&self) -> u64 {
        self.0.mem
    }

    pub fn steps(&self) -> u64 {
        self.0.steps
    }

    pub fn new(mem: u64, steps: u64) -> Self {
        Self(cml_chain::plutus::ExUnits::new(mem, steps))
    }
}

impl From<cml_chain::plutus::ExUnits> for ExUnits {
    fn from(native: cml_chain::plutus::ExUnits) -> Self {
        Self(native)
    }
}

impl From<ExUnits> for cml_chain::plutus::ExUnits {
    fn from(wasm: ExUnits) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::plutus::ExUnits> for ExUnits {
    fn as_ref(&self) -> &cml_chain::plutus::ExUnits {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct PlutusData(cml_chain::plutus::PlutusData);

#[wasm_bindgen]
impl PlutusData {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<PlutusData, JsValue> {
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

    pub fn from_json(json: &str) -> Result<PlutusData, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn new_constr_plutus_data(constr_plutus_data: &ConstrPlutusData) -> Self {
        Self(cml_chain::plutus::PlutusData::new_constr_plutus_data(
            constr_plutus_data.clone().into(),
        ))
    }

    pub fn new_map(map: &MapPlutusDataToPlutusData) -> Self {
        Self(cml_chain::plutus::PlutusData::new_map(map.clone().into()))
    }

    pub fn new_list(list: &PlutusDataList) -> Self {
        Self(cml_chain::plutus::PlutusData::new_list(list.clone().into()))
    }

    pub fn new_big_int(big_int: &BigInt) -> Self {
        Self(cml_chain::plutus::PlutusData::new_big_int(
            big_int.clone().into(),
        ))
    }

    pub fn new_bytes(bytes: Vec<u8>) -> Self {
        Self(cml_chain::plutus::PlutusData::new_bytes(bytes))
    }

    pub fn kind(&self) -> PlutusDataKind {
        match &self.0 {
            cml_chain::plutus::PlutusData::ConstrPlutusData(_) => PlutusDataKind::ConstrPlutusData,
            cml_chain::plutus::PlutusData::Map { .. } => PlutusDataKind::Map,
            cml_chain::plutus::PlutusData::List { .. } => PlutusDataKind::List,
            cml_chain::plutus::PlutusData::BigInt(_) => PlutusDataKind::BigInt,
            cml_chain::plutus::PlutusData::Bytes{ .. } => PlutusDataKind::Bytes,
        }
    }

    pub fn as_constr_plutus_data(&self) -> Option<ConstrPlutusData> {
        match &self.0 {
            cml_chain::plutus::PlutusData::ConstrPlutusData(constr_plutus_data) => {
                Some(constr_plutus_data.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_map(&self) -> Option<MapPlutusDataToPlutusData> {
        match &self.0 {
            cml_chain::plutus::PlutusData::Map { map, .. } => Some(map.clone().into()),
            _ => None,
        }
    }

    pub fn as_list(&self) -> Option<PlutusDataList> {
        match &self.0 {
            cml_chain::plutus::PlutusData::List { list, .. } => Some(list.clone().into()),
            _ => None,
        }
    }

    pub fn as_big_int(&self) -> Option<BigInt> {
        match &self.0 {
            cml_chain::plutus::PlutusData::BigInt(big_int) => Some(big_int.clone().into()),
            _ => None,
        }
    }

    pub fn as_bytes(&self) -> Option<Vec<u8>> {
        match &self.0 {
            cml_chain::plutus::PlutusData::Bytes{ bytes, .. } => Some(bytes.clone().into()),
            _ => None,
        }
    }
}

impl From<cml_chain::plutus::PlutusData> for PlutusData {
    fn from(native: cml_chain::plutus::PlutusData) -> Self {
        Self(native)
    }
}

impl From<PlutusData> for cml_chain::plutus::PlutusData {
    fn from(wasm: PlutusData) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::plutus::PlutusData> for PlutusData {
    fn as_ref(&self) -> &cml_chain::plutus::PlutusData {
        &self.0
    }
}

#[wasm_bindgen]
pub enum PlutusDataKind {
    ConstrPlutusData,
    Map,
    List,
    BigInt,
    Bytes,
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct PlutusV1Script(cml_chain::plutus::PlutusV1Script);

#[wasm_bindgen]
impl PlutusV1Script {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<PlutusV1Script, JsValue> {
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

    pub fn from_json(json: &str) -> Result<PlutusV1Script, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn get(&self) -> Vec<u8> {
        self.0.get().clone()
    }
}

impl From<cml_chain::plutus::PlutusV1Script> for PlutusV1Script {
    fn from(native: cml_chain::plutus::PlutusV1Script) -> Self {
        Self(native)
    }
}

impl From<PlutusV1Script> for cml_chain::plutus::PlutusV1Script {
    fn from(wasm: PlutusV1Script) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::plutus::PlutusV1Script> for PlutusV1Script {
    fn as_ref(&self) -> &cml_chain::plutus::PlutusV1Script {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct PlutusV2Script(cml_chain::plutus::PlutusV2Script);

#[wasm_bindgen]
impl PlutusV2Script {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<PlutusV2Script, JsValue> {
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

    pub fn from_json(json: &str) -> Result<PlutusV2Script, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn get(&self) -> Vec<u8> {
        self.0.get().clone()
    }
}

impl From<cml_chain::plutus::PlutusV2Script> for PlutusV2Script {
    fn from(native: cml_chain::plutus::PlutusV2Script) -> Self {
        Self(native)
    }
}

impl From<PlutusV2Script> for cml_chain::plutus::PlutusV2Script {
    fn from(wasm: PlutusV2Script) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::plutus::PlutusV2Script> for PlutusV2Script {
    fn as_ref(&self) -> &cml_chain::plutus::PlutusV2Script {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct Redeemer(cml_chain::plutus::Redeemer);

#[wasm_bindgen]
impl Redeemer {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<Redeemer, JsValue> {
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

    pub fn from_json(json: &str) -> Result<Redeemer, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn tag(&self) -> RedeemerTag {
        self.0.tag
    }

    pub fn index(&self) -> u64 {
        self.0.index
    }

    pub fn data(&self) -> PlutusData {
        self.0.data.clone().into()
    }

    pub fn ex_units(&self) -> ExUnits {
        self.0.ex_units.clone().into()
    }

    pub fn new(tag: RedeemerTag, index: u64, data: &PlutusData, ex_units: &ExUnits) -> Self {
        Self(cml_chain::plutus::Redeemer::new(
            tag.into(),
            index,
            data.clone().into(),
            ex_units.clone().into(),
        ))
    }
}

impl From<cml_chain::plutus::Redeemer> for Redeemer {
    fn from(native: cml_chain::plutus::Redeemer) -> Self {
        Self(native)
    }
}

impl From<Redeemer> for cml_chain::plutus::Redeemer {
    fn from(wasm: Redeemer) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::plutus::Redeemer> for Redeemer {
    fn as_ref(&self) -> &cml_chain::plutus::Redeemer {
        &self.0
    }
}
