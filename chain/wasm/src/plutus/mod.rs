pub mod utils;

// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use crate::utils::BigInteger;

use super::{IntList, PlutusDataList, SubCoin};
pub use cml_chain::plutus::{Language, RedeemerTag};
use cml_core_wasm::{impl_wasm_cbor_json_api, impl_wasm_conversions};
pub use utils::{ConstrPlutusData, PlutusMap};
use wasm_bindgen::prelude::{wasm_bindgen, JsError, JsValue};

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct CostModels(cml_chain::plutus::CostModels);

impl_wasm_cbor_json_api!(CostModels);

impl_wasm_conversions!(cml_chain::plutus::CostModels, CostModels);

#[wasm_bindgen]
impl CostModels {
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

    pub fn set_plutus_v3(&mut self, plutus_v3: &IntList) {
        self.0.plutus_v3 = Some(plutus_v3.clone().into())
    }

    pub fn plutus_v3(&self) -> Option<IntList> {
        self.0.plutus_v3.clone().map(std::convert::Into::into)
    }

    pub fn new() -> Self {
        Self(cml_chain::plutus::CostModels::new())
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ExUnitPrices(cml_chain::plutus::ExUnitPrices);

impl_wasm_cbor_json_api!(ExUnitPrices);

impl_wasm_conversions!(cml_chain::plutus::ExUnitPrices, ExUnitPrices);

#[wasm_bindgen]
impl ExUnitPrices {
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

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ExUnits(cml_chain::plutus::ExUnits);

impl_wasm_cbor_json_api!(ExUnits);

impl_wasm_conversions!(cml_chain::plutus::ExUnits, ExUnits);

#[wasm_bindgen]
impl ExUnits {
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

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct PlutusData(cml_chain::plutus::PlutusData);

impl_wasm_cbor_json_api!(PlutusData);

impl_wasm_conversions!(cml_chain::plutus::PlutusData, PlutusData);

#[wasm_bindgen]
impl PlutusData {
    pub fn new_constr_plutus_data(constr_plutus_data: &ConstrPlutusData) -> Self {
        Self(cml_chain::plutus::PlutusData::new_constr_plutus_data(
            constr_plutus_data.clone().into(),
        ))
    }

    pub fn new_map(map: &PlutusMap) -> Self {
        Self(cml_chain::plutus::PlutusData::new_map(map.clone().into()))
    }

    pub fn new_list(list: &PlutusDataList) -> Self {
        Self(cml_chain::plutus::PlutusData::new_list(list.clone().into()))
    }

    pub fn new_integer(big_int: &BigInteger) -> Self {
        Self(cml_chain::plutus::PlutusData::new_integer(
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
            cml_chain::plutus::PlutusData::Integer(_) => PlutusDataKind::Integer,
            cml_chain::plutus::PlutusData::Bytes { .. } => PlutusDataKind::Bytes,
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

    pub fn as_map(&self) -> Option<PlutusMap> {
        match &self.0 {
            cml_chain::plutus::PlutusData::Map(map) => Some(map.clone().into()),
            _ => None,
        }
    }

    pub fn as_list(&self) -> Option<PlutusDataList> {
        match &self.0 {
            cml_chain::plutus::PlutusData::List { list, .. } => Some(list.clone().into()),
            _ => None,
        }
    }

    pub fn as_integer(&self) -> Option<BigInteger> {
        match &self.0 {
            cml_chain::plutus::PlutusData::Integer(big_int) => Some(big_int.clone().into()),
            _ => None,
        }
    }

    pub fn as_bytes(&self) -> Option<Vec<u8>> {
        match &self.0 {
            cml_chain::plutus::PlutusData::Bytes { bytes, .. } => Some(bytes.clone()),
            _ => None,
        }
    }
}

#[wasm_bindgen]
pub enum PlutusDataKind {
    ConstrPlutusData,
    Map,
    List,
    Integer,
    Bytes,
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct PlutusV1Script(cml_chain::plutus::PlutusV1Script);

impl_wasm_cbor_json_api!(PlutusV1Script);

impl_wasm_conversions!(cml_chain::plutus::PlutusV1Script, PlutusV1Script);

#[wasm_bindgen]
impl PlutusV1Script {
    pub fn get(&self) -> Vec<u8> {
        self.0.get().clone()
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct PlutusV2Script(cml_chain::plutus::PlutusV2Script);

impl_wasm_cbor_json_api!(PlutusV2Script);

impl_wasm_conversions!(cml_chain::plutus::PlutusV2Script, PlutusV2Script);

#[wasm_bindgen]
impl PlutusV2Script {
    pub fn get(&self) -> Vec<u8> {
        self.0.get().clone()
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct PlutusV3Script(cml_chain::plutus::PlutusV3Script);

impl_wasm_cbor_json_api!(PlutusV3Script);

impl_wasm_conversions!(cml_chain::plutus::PlutusV3Script, PlutusV3Script);

#[wasm_bindgen]
impl PlutusV3Script {
    pub fn get(&self) -> Vec<u8> {
        self.0.get().clone()
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct Redeemer(cml_chain::plutus::Redeemer);

impl_wasm_cbor_json_api!(Redeemer);

impl_wasm_conversions!(cml_chain::plutus::Redeemer, Redeemer);

#[wasm_bindgen]
impl Redeemer {
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
            tag,
            index,
            data.clone().into(),
            ex_units.clone().into(),
        ))
    }
}
