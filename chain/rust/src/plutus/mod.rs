// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

pub mod cbor_encodings;
pub mod serialization;
pub mod utils;

#[cfg(not(feature = "used_from_wasm"))]
use noop_proc_macro::wasm_bindgen;
#[cfg(feature = "used_from_wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

use self::cbor_encodings::{
    LegacyRedeemerEncoding, PlutusV3ScriptEncoding, RedeemerKeyEncoding, RedeemerValEncoding,
};

use super::{Rational, SubCoin};
use crate::utils::BigInteger;
use cbor_encodings::{
    CostModelsEncoding, ExUnitPricesEncoding, ExUnitsEncoding, PlutusV1ScriptEncoding,
    PlutusV2ScriptEncoding,
};

use cml_core::ordered_hash_map::OrderedHashMap;
use cml_core::serialization::{LenEncoding, Serialize, StringEncoding};
use cml_crypto::{blake2b256, DatumHash};

pub use utils::{ConstrPlutusData, PlutusMap, PlutusScript};

#[derive(Clone, Debug, Default)]
pub struct CostModels {
    pub inner: OrderedHashMap<u64, Vec<i64>>,
    pub encodings: Option<CostModelsEncoding>,
}

impl CostModels {
    pub fn new(inner: OrderedHashMap<u64, Vec<i64>>) -> Self {
        Self {
            inner,
            encodings: None,
        }
    }
}

impl From<OrderedHashMap<u64, Vec<i64>>> for CostModels {
    fn from(inner: OrderedHashMap<u64, Vec<i64>>) -> Self {
        CostModels::new(inner.clone())
    }
}

impl From<CostModels> for OrderedHashMap<u64, Vec<i64>> {
    fn from(wrapper: CostModels) -> Self {
        wrapper.inner
    }
}

impl serde::Serialize for CostModels {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.inner.serialize(serializer)
    }
}

impl<'de> serde::de::Deserialize<'de> for CostModels {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let inner =
            <OrderedHashMap<u64, Vec<i64>> as serde::de::Deserialize>::deserialize(deserializer)?;
        Ok(Self::new(inner))
    }
}

impl schemars::JsonSchema for CostModels {
    fn schema_name() -> String {
        String::from("CostModels")
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        OrderedHashMap::<u64, Vec<i64>>::json_schema(gen)
    }

    fn is_referenceable() -> bool {
        OrderedHashMap::<u64, Vec<i64>>::is_referenceable()
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

#[derive(
    Clone, Debug, derivative::Derivative, serde::Deserialize, serde::Serialize, schemars::JsonSchema,
)]
#[derivative(PartialEq, Hash, Eq)]
pub struct ExUnits {
    pub mem: u64,
    pub steps: u64,
    #[serde(skip)]
    #[derivative(PartialEq = "ignore", Hash = "ignore")]
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
#[wasm_bindgen]
pub enum Language {
    PlutusV1,
    PlutusV2,
    PlutusV3,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct LegacyRedeemer {
    pub tag: RedeemerTag,
    pub index: u64,
    pub data: PlutusData,
    pub ex_units: ExUnits,
    #[serde(skip)]
    pub encodings: Option<LegacyRedeemerEncoding>,
}

impl LegacyRedeemer {
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

#[derive(Clone, Debug, derivative::Derivative)]
#[derivative(
    Eq,
    PartialEq,
    Ord = "feature_allow_slow_enum",
    PartialOrd = "feature_allow_slow_enum",
    Hash
)]
pub enum PlutusData {
    ConstrPlutusData(ConstrPlutusData),
    Map(PlutusMap),
    List {
        list: Vec<PlutusData>,
        #[derivative(
            PartialEq = "ignore",
            Ord = "ignore",
            PartialOrd = "ignore",
            Hash = "ignore"
        )]
        list_encoding: LenEncoding,
    },
    Integer(BigInteger),
    Bytes {
        bytes: Vec<u8>,
        #[derivative(
            PartialEq = "ignore",
            Ord = "ignore",
            PartialOrd = "ignore",
            Hash = "ignore"
        )]
        bytes_encoding: StringEncoding,
    },
}

impl PlutusData {
    pub fn new_constr_plutus_data(constr_plutus_data: ConstrPlutusData) -> Self {
        Self::ConstrPlutusData(constr_plutus_data)
    }

    pub fn new_map(map: PlutusMap) -> Self {
        Self::Map(map)
    }

    pub fn new_list(list: Vec<PlutusData>) -> Self {
        Self::List {
            list,
            list_encoding: LenEncoding::default(),
        }
    }

    pub fn new_integer(integer: BigInteger) -> Self {
        Self::Integer(integer)
    }

    pub fn new_bytes(bytes: Vec<u8>) -> Self {
        Self::Bytes {
            bytes,
            bytes_encoding: StringEncoding::default(),
        }
    }

    pub fn hash(&self) -> DatumHash {
        DatumHash::from(blake2b256(&self.to_cbor_bytes()))
    }
}

#[derive(Clone, Debug, derivative::Derivative)]
#[derivative(Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PlutusV1Script {
    pub inner: Vec<u8>,
    #[derivative(
        PartialEq = "ignore",
        Ord = "ignore",
        PartialOrd = "ignore",
        Hash = "ignore"
    )]
    pub encodings: Option<PlutusV1ScriptEncoding>,
}

impl PlutusV1Script {
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

impl serde::Serialize for PlutusV1Script {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&hex::encode(self.inner.clone()))
    }
}

impl<'de> serde::de::Deserialize<'de> for PlutusV1Script {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let s = <String as serde::de::Deserialize>::deserialize(deserializer)?;
        hex::decode(&s).map(PlutusV1Script::new).map_err(|_e| {
            serde::de::Error::invalid_value(serde::de::Unexpected::Str(&s), &"invalid hex bytes")
        })
    }
}

impl schemars::JsonSchema for PlutusV1Script {
    fn schema_name() -> String {
        String::from("PlutusV1Script")
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        String::json_schema(gen)
    }

    fn is_referenceable() -> bool {
        String::is_referenceable()
    }
}

#[derive(Clone, Debug, derivative::Derivative)]
#[derivative(Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PlutusV2Script {
    pub inner: Vec<u8>,
    #[derivative(
        PartialEq = "ignore",
        Ord = "ignore",
        PartialOrd = "ignore",
        Hash = "ignore"
    )]
    pub encodings: Option<PlutusV2ScriptEncoding>,
}

impl PlutusV2Script {
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

impl serde::Serialize for PlutusV2Script {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&hex::encode(self.inner.clone()))
    }
}

impl<'de> serde::de::Deserialize<'de> for PlutusV2Script {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let s = <String as serde::de::Deserialize>::deserialize(deserializer)?;
        hex::decode(&s).map(PlutusV2Script::new).map_err(|_e| {
            serde::de::Error::invalid_value(serde::de::Unexpected::Str(&s), &"invalid hex bytes")
        })
    }
}

impl schemars::JsonSchema for PlutusV2Script {
    fn schema_name() -> String {
        String::from("PlutusV2Script")
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        String::json_schema(gen)
    }

    fn is_referenceable() -> bool {
        String::is_referenceable()
    }
}

#[derive(Clone, Debug, derivative::Derivative)]
#[derivative(Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PlutusV3Script {
    pub inner: Vec<u8>,
    #[derivative(
        PartialEq = "ignore",
        Ord = "ignore",
        PartialOrd = "ignore",
        Hash = "ignore"
    )]
    pub encodings: Option<PlutusV3ScriptEncoding>,
}

impl PlutusV3Script {
    pub fn new(inner: Vec<u8>) -> Self {
        Self {
            inner,
            encodings: None,
        }
    }
}

impl From<Vec<u8>> for PlutusV3Script {
    fn from(inner: Vec<u8>) -> Self {
        PlutusV3Script::new(inner)
    }
}

impl From<PlutusV3Script> for Vec<u8> {
    fn from(wrapper: PlutusV3Script) -> Self {
        wrapper.inner
    }
}

impl serde::Serialize for PlutusV3Script {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&hex::encode(self.inner.clone()))
    }
}

impl<'de> serde::de::Deserialize<'de> for PlutusV3Script {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let s = <String as serde::de::Deserialize>::deserialize(deserializer)?;
        hex::decode(&s).map(PlutusV3Script::new).map_err(|_e| {
            serde::de::Error::invalid_value(serde::de::Unexpected::Str(&s), &"invalid hex bytes")
        })
    }
}

impl schemars::JsonSchema for PlutusV3Script {
    fn schema_name() -> String {
        String::from("PlutusV3Script")
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        String::json_schema(gen)
    }

    fn is_referenceable() -> bool {
        String::is_referenceable()
    }
}

#[derive(
    Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema, derivative::Derivative,
)]
#[derivative(Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RedeemerKey {
    pub tag: RedeemerTag,
    pub index: u64,
    #[derivative(
        PartialEq = "ignore",
        Ord = "ignore",
        PartialOrd = "ignore",
        Hash = "ignore"
    )]
    #[serde(skip)]
    pub encodings: Option<RedeemerKeyEncoding>,
}

impl RedeemerKey {
    pub fn new(tag: RedeemerTag, index: u64) -> Self {
        Self {
            tag,
            index,
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
#[wasm_bindgen]
pub enum RedeemerTag {
    Spend,
    Mint,
    Cert,
    Reward,
    Voting,
    Proposing,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct RedeemerVal {
    pub data: PlutusData,
    pub ex_units: ExUnits,
    #[serde(skip)]
    pub encodings: Option<RedeemerValEncoding>,
}

impl RedeemerVal {
    pub fn new(data: PlutusData, ex_units: ExUnits) -> Self {
        Self {
            data,
            ex_units,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub enum Redeemers {
    ArrLegacyRedeemer {
        arr_legacy_redeemer: Vec<LegacyRedeemer>,
        #[serde(skip)]
        arr_legacy_redeemer_encoding: LenEncoding,
    },
    MapRedeemerKeyToRedeemerVal {
        map_redeemer_key_to_redeemer_val: OrderedHashMap<RedeemerKey, RedeemerVal>,
        #[serde(skip)]
        map_redeemer_key_to_redeemer_val_encoding: LenEncoding,
    },
}

impl Redeemers {
    pub fn new_arr_legacy_redeemer(arr_legacy_redeemer: Vec<LegacyRedeemer>) -> Self {
        Self::ArrLegacyRedeemer {
            arr_legacy_redeemer,
            arr_legacy_redeemer_encoding: LenEncoding::default(),
        }
    }

    pub fn new_map_redeemer_key_to_redeemer_val(
        map_redeemer_key_to_redeemer_val: OrderedHashMap<RedeemerKey, RedeemerVal>,
    ) -> Self {
        Self::MapRedeemerKeyToRedeemerVal {
            map_redeemer_key_to_redeemer_val,
            map_redeemer_key_to_redeemer_val_encoding: LenEncoding::default(),
        }
    }
}
