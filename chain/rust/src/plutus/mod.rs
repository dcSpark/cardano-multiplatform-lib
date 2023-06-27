// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

pub mod cbor_encodings;
pub mod serialization;
pub mod utils;

use super::{Rational, SubCoin};
use crate::utils::BigInt;
use cbor_encodings::{
    CostModelsEncoding, ExUnitPricesEncoding, ExUnitsEncoding, PlutusV1ScriptEncoding,
    PlutusV2ScriptEncoding, RedeemerEncoding,
};
use cml_core::ordered_hash_map::OrderedHashMap;
use cml_core::serialization::{LenEncoding, StringEncoding};
use cml_core::Int;

pub use utils::{ConstrPlutusData, PlutusScript};


#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct CostModels {
    pub plutus_v1: Option<Vec<Int>>,
    pub plutus_v2: Option<Vec<Int>>,
    #[serde(skip)]
    pub encodings: Option<CostModelsEncoding>,
}

impl CostModels {
    pub fn new() -> Self {
        Self {
            plutus_v1: None,
            plutus_v2: None,
            encodings: None,
        }
    }
}

impl Default for CostModels {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ExUnitPrices {
    pub mem_price: SubCoin,
    pub step_price: SubCoin,
    #[serde(skip)]
    pub encodings: Option<ExUnitPricesEncoding>,
}

impl ExUnitPrices {
    pub fn new(mem_price: SubCoin, step_price: SubCoin) -> Self {
        Self {
            mem_price,
            step_price,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, derivative::Derivative, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
#[derivative(PartialEq, Hash, Eq)]
pub struct ExUnits {
    pub mem: u64,
    pub steps: u64,
    #[serde(skip)]
    #[derivative(
        PartialEq = "ignore",
        Hash = "ignore",
    )]
    pub encodings: Option<ExUnitsEncoding>,
}

impl ExUnits {
    pub fn new(mem: u64, steps: u64) -> Self {
        Self {
            mem,
            steps,
            encodings: None,
        }
    }
}

#[derive(
    Copy,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Clone,
    Debug,
    serde::Deserialize,
    serde::Serialize,
    schemars::JsonSchema,
)]
#[wasm_bindgen::prelude::wasm_bindgen]
pub enum Language {
    PlutusV1,
    PlutusV2,
}

#[derive(
    Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema, derivative::Derivative,
)]
#[derivative(
    Eq,
    PartialEq,
    Ord = "feature_allow_slow_enum",
    PartialOrd = "feature_allow_slow_enum",
    Hash
)]
pub enum PlutusData {
    ConstrPlutusData(ConstrPlutusData),
    Map {
        map: OrderedHashMap<PlutusData, PlutusData>,
        #[derivative(
            PartialEq = "ignore",
            Ord = "ignore",
            PartialOrd = "ignore",
            Hash = "ignore"
        )]
        #[serde(skip)]
        map_encoding: LenEncoding,
    },
    List {
        list: Vec<PlutusData>,
        #[derivative(
            PartialEq = "ignore",
            Ord = "ignore",
            PartialOrd = "ignore",
            Hash = "ignore"
        )]
        #[serde(skip)]
        list_encoding: LenEncoding,
    },
    BigInt(BigInt),
    Bytes {
        bytes: Vec<u8>,
        #[derivative(
            PartialEq = "ignore",
            Ord = "ignore",
            PartialOrd = "ignore",
            Hash = "ignore"
        )]
        #[serde(skip)]
        bytes_encoding: StringEncoding,
    },
}

impl PlutusData {
    pub fn new_constr_plutus_data(constr_plutus_data: ConstrPlutusData) -> Self {
        Self::ConstrPlutusData(constr_plutus_data)
    }

    pub fn new_map(map: OrderedHashMap<PlutusData, PlutusData>) -> Self {
        Self::Map {
            map,
            map_encoding: LenEncoding::default(),
        }
    }

    pub fn new_list(list: Vec<PlutusData>) -> Self {
        Self::List {
            list,
            list_encoding: LenEncoding::default(),
        }
    }

    pub fn new_big_int(big_int: BigInt) -> Self {
        Self::BigInt(big_int)
    }

    pub fn new_bytes(bytes: Vec<u8>) -> Self {
        Self::Bytes {
            bytes,
            bytes_encoding: StringEncoding::default(),
        }
    }
}

#[derive(Clone, Debug, derivative::Derivative, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
#[derivative(Hash, PartialEq, Eq)]
pub struct PlutusV1Script {
    pub inner: Vec<u8>,
    #[serde(skip)]
    #[derivative(
        PartialEq = "ignore",
        Hash = "ignore",
    )]
    pub encodings: Option<PlutusV1ScriptEncoding>,
}

impl PlutusV1Script {
    pub fn get(&self) -> &Vec<u8> {
        &self.inner
    }

    pub fn new(inner: Vec<u8>) -> Self {
        Self {
            inner,
            encodings: None,
        }
    }
}

impl From<Vec<u8>> for PlutusV1Script {
    fn from(inner: Vec<u8>) -> Self {
        PlutusV1Script::new(inner)
    }
}

impl From<PlutusV1Script> for Vec<u8> {
    fn from(wrapper: PlutusV1Script) -> Self {
        wrapper.inner
    }
}

#[derive(Clone, Debug, derivative::Derivative, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
#[derivative(Hash, PartialEq, Eq)]
pub struct PlutusV2Script {
    pub inner: Vec<u8>,
    #[serde(skip)]
    #[derivative(
        PartialEq = "ignore",
        Hash = "ignore",
    )]
    pub encodings: Option<PlutusV2ScriptEncoding>,
}

impl PlutusV2Script {
    pub fn get(&self) -> &Vec<u8> {
        &self.inner
    }

    pub fn new(inner: Vec<u8>) -> Self {
        Self {
            inner,
            encodings: None,
        }
    }
}

impl From<Vec<u8>> for PlutusV2Script {
    fn from(inner: Vec<u8>) -> Self {
        PlutusV2Script::new(inner)
    }
}

impl From<PlutusV2Script> for Vec<u8> {
    fn from(wrapper: PlutusV2Script) -> Self {
        wrapper.inner
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct Redeemer {
    pub tag: RedeemerTag,
    pub index: u64,
    pub data: PlutusData,
    pub ex_units: ExUnits,
    #[serde(skip)]
    pub encodings: Option<RedeemerEncoding>,
}

impl Redeemer {
    pub fn new(tag: RedeemerTag, index: u64, data: PlutusData, ex_units: ExUnits) -> Self {
        Self {
            tag,
            index,
            data,
            ex_units,
            encodings: None,
        }
    }
}

#[derive(
    Copy,
    Eq,
    Hash,
    PartialEq,
    Ord,
    PartialOrd,
    Clone,
    Debug,
    serde::Deserialize,
    serde::Serialize,
    schemars::JsonSchema,
)]
#[wasm_bindgen::prelude::wasm_bindgen]
pub enum RedeemerTag {
    Spend,
    Mint,
    Cert,
    Reward,
}
