pub mod utils;

// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use crate::utils::BigInteger;

use super::{PlutusDataList, SubCoin};
use crate::{LegacyRedeemerList, MapRedeemerKeyToRedeemerVal};
pub use cml_chain::plutus::{Language, RedeemerTag};
use cml_core_wasm::{impl_wasm_cbor_json_api, impl_wasm_conversions, impl_wasm_map};
pub use utils::{ConstrPlutusData, PlutusMap};
use wasm_bindgen::prelude::wasm_bindgen;

impl_wasm_map!(
    u64,
    Vec<i64>,
    u64,
    Vec<i64>,
    Vec<u64>,
    MapU64ToArrI64,
    true,
    true,
    true,
    false
);

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct CostModels(cml_chain::plutus::CostModels);

impl_wasm_cbor_json_api!(CostModels);

impl_wasm_conversions!(cml_chain::plutus::CostModels, CostModels);

#[wasm_bindgen]
impl CostModels {
    pub fn inner(&self) -> MapU64ToArrI64 {
        self.0.inner.clone().into()
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
pub struct LegacyRedeemer(cml_chain::plutus::LegacyRedeemer);

impl_wasm_cbor_json_api!(LegacyRedeemer);

impl_wasm_conversions!(cml_chain::plutus::LegacyRedeemer, LegacyRedeemer);

#[wasm_bindgen]
impl LegacyRedeemer {
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
        Self(cml_chain::plutus::LegacyRedeemer::new(
            tag,
            index,
            data.clone().into(),
            ex_units.clone().into(),
        ))
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

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct PlutusV2Script(cml_chain::plutus::PlutusV2Script);

impl_wasm_cbor_json_api!(PlutusV2Script);

impl_wasm_conversions!(cml_chain::plutus::PlutusV2Script, PlutusV2Script);

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct PlutusV3Script(cml_chain::plutus::PlutusV3Script);

impl_wasm_cbor_json_api!(PlutusV3Script);

impl_wasm_conversions!(cml_chain::plutus::PlutusV3Script, PlutusV3Script);

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct RedeemerKey(cml_chain::plutus::RedeemerKey);

impl_wasm_cbor_json_api!(RedeemerKey);

impl_wasm_conversions!(cml_chain::plutus::RedeemerKey, RedeemerKey);

#[wasm_bindgen]
impl RedeemerKey {
    pub fn tag(&self) -> RedeemerTag {
        self.0.tag
    }

    pub fn index(&self) -> u64 {
        self.0.index
    }

    pub fn new(tag: RedeemerTag, index: u64) -> Self {
        Self(cml_chain::plutus::RedeemerKey::new(tag, index))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct RedeemerVal(cml_chain::plutus::RedeemerVal);

impl_wasm_cbor_json_api!(RedeemerVal);

impl_wasm_conversions!(cml_chain::plutus::RedeemerVal, RedeemerVal);

#[wasm_bindgen]
impl RedeemerVal {
    pub fn data(&self) -> PlutusData {
        self.0.data.clone().into()
    }

    pub fn ex_units(&self) -> ExUnits {
        self.0.ex_units.clone().into()
    }

    pub fn new(data: &PlutusData, ex_units: &ExUnits) -> Self {
        Self(cml_chain::plutus::RedeemerVal::new(
            data.clone().into(),
            ex_units.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct Redeemers(cml_chain::plutus::Redeemers);

impl_wasm_cbor_json_api!(Redeemers);

impl_wasm_conversions!(cml_chain::plutus::Redeemers, Redeemers);

#[wasm_bindgen]
impl Redeemers {
    pub fn new_arr_legacy_redeemer(arr_legacy_redeemer: &LegacyRedeemerList) -> Self {
        Self(cml_chain::plutus::Redeemers::new_arr_legacy_redeemer(
            arr_legacy_redeemer.clone().into(),
        ))
    }

    pub fn new_map_redeemer_key_to_redeemer_val(
        map_redeemer_key_to_redeemer_val: &MapRedeemerKeyToRedeemerVal,
    ) -> Self {
        Self(
            cml_chain::plutus::Redeemers::new_map_redeemer_key_to_redeemer_val(
                map_redeemer_key_to_redeemer_val.clone().into(),
            ),
        )
    }

    pub fn kind(&self) -> RedeemersKind {
        match &self.0 {
            cml_chain::plutus::Redeemers::ArrLegacyRedeemer { .. } => {
                RedeemersKind::ArrLegacyRedeemer
            }
            cml_chain::plutus::Redeemers::MapRedeemerKeyToRedeemerVal { .. } => {
                RedeemersKind::MapRedeemerKeyToRedeemerVal
            }
        }
    }

    pub fn as_arr_legacy_redeemer(&self) -> Option<LegacyRedeemerList> {
        match &self.0 {
            cml_chain::plutus::Redeemers::ArrLegacyRedeemer {
                arr_legacy_redeemer,
                ..
            } => Some(arr_legacy_redeemer.clone().into()),
            _ => None,
        }
    }

    pub fn as_map_redeemer_key_to_redeemer_val(&self) -> Option<MapRedeemerKeyToRedeemerVal> {
        match &self.0 {
            cml_chain::plutus::Redeemers::MapRedeemerKeyToRedeemerVal {
                map_redeemer_key_to_redeemer_val,
                ..
            } => Some(map_redeemer_key_to_redeemer_val.clone().into()),
            _ => None,
        }
    }
}

#[wasm_bindgen]
pub enum RedeemersKind {
    ArrLegacyRedeemer,
    MapRedeemerKeyToRedeemerVal,
}
