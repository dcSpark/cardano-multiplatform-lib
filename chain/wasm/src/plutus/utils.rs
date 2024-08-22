use crate::{
    plutus::{PlutusData, Redeemers},
    LegacyRedeemerList, PlutusDataList,
};
use cml_chain::plutus::Language;
use cml_core_wasm::{
    impl_raw_bytes_api, impl_wasm_cbor_api, impl_wasm_cbor_json_api, impl_wasm_conversions,
};
use cml_crypto_wasm::ScriptHash;
use wasm_bindgen::prelude::{wasm_bindgen, JsError};

use super::{ExUnits, PlutusV1Script, PlutusV2Script, PlutusV3Script};

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ConstrPlutusData(cml_chain::plutus::ConstrPlutusData);

impl_wasm_conversions!(cml_chain::plutus::ConstrPlutusData, ConstrPlutusData);

impl_wasm_cbor_json_api!(ConstrPlutusData);

#[wasm_bindgen]
impl ConstrPlutusData {
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

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct PlutusMap(cml_chain::plutus::PlutusMap);

impl_wasm_conversions!(cml_chain::plutus::PlutusMap, PlutusMap);

impl_wasm_cbor_api!(PlutusMap);

#[wasm_bindgen]
impl PlutusMap {
    pub fn new() -> Self {
        Self(cml_chain::plutus::PlutusMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
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
        self.0
            .get_all(key.as_ref())
            .map(|datums| datums.into_iter().cloned().collect::<Vec<_>>().into())
    }

    pub fn keys(&self) -> PlutusDataList {
        PlutusDataList(
            self.0
                .entries
                .iter()
                .map(|(k, _v)| k.clone())
                .collect::<Vec<_>>(),
        )
    }
}

/// Version-agnostic Plutus script
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct PlutusScript(cml_chain::plutus::utils::PlutusScript);

impl_wasm_conversions!(cml_chain::plutus::utils::PlutusScript, PlutusScript);

#[wasm_bindgen]
impl PlutusScript {
    pub fn from_v1(script: &PlutusV1Script) -> Self {
        cml_chain::plutus::utils::PlutusScript::PlutusV1(script.as_ref().clone()).into()
    }

    pub fn from_v2(script: &PlutusV2Script) -> Self {
        cml_chain::plutus::utils::PlutusScript::PlutusV2(script.as_ref().clone()).into()
    }

    pub fn from_v3(script: &PlutusV3Script) -> Self {
        cml_chain::plutus::utils::PlutusScript::PlutusV3(script.as_ref().clone()).into()
    }

    pub fn hash(&self) -> ScriptHash {
        self.0.hash().into()
    }

    pub fn as_v1(&self) -> Option<PlutusV1Script> {
        match &self.0 {
            cml_chain::plutus::utils::PlutusScript::PlutusV1(v1) => Some(v1.clone().into()),
            _ => None,
        }
    }

    pub fn as_v2(&self) -> Option<PlutusV2Script> {
        match &self.0 {
            cml_chain::plutus::utils::PlutusScript::PlutusV2(v2) => Some(v2.clone().into()),
            _ => None,
        }
    }

    pub fn as_v3(&self) -> Option<PlutusV3Script> {
        match &self.0 {
            cml_chain::plutus::utils::PlutusScript::PlutusV3(v3) => Some(v3.clone().into()),
            _ => None,
        }
    }

    pub fn version(&self) -> Language {
        self.0.version()
    }
}

#[wasm_bindgen]
impl PlutusV1Script {
    pub fn hash(&self) -> ScriptHash {
        self.0.hash().into()
    }
}

#[wasm_bindgen]
impl PlutusV2Script {
    pub fn hash(&self) -> ScriptHash {
        self.0.hash().into()
    }
}

#[wasm_bindgen]
impl PlutusV3Script {
    pub fn hash(&self) -> ScriptHash {
        self.0.hash().into()
    }
}

impl_raw_bytes_api!(cml_chain::plutus::PlutusV1Script, PlutusV1Script);

impl_raw_bytes_api!(cml_chain::plutus::PlutusV2Script, PlutusV2Script);

impl_raw_bytes_api!(cml_chain::plutus::PlutusV3Script, PlutusV3Script);

#[wasm_bindgen]
impl Redeemers {
    pub fn to_flat_format(&self) -> LegacyRedeemerList {
        self.0.clone().to_flat_format().into()
    }
}

#[wasm_bindgen]
impl ExUnits {
    pub fn checked_add(&self, other: &ExUnits) -> Result<ExUnits, JsError> {
        self.0
            .checked_add(other.as_ref())
            .map(Into::into)
            .map_err(Into::into)
    }
}

#[wasm_bindgen]
pub fn compute_total_ex_units(redeemers: &Redeemers) -> Result<ExUnits, JsError> {
    cml_chain::plutus::utils::compute_total_ex_units(redeemers.to_flat_format().as_ref())
        .map(Into::into)
        .map_err(Into::into)
}
