pub type PlutusV1Script = Vec<u8>;

pub type PlutusV2Script = Vec<u8>;

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct BigInt(pub(crate) core::BigInt);

#[wasm_bindgen]

impl BigInt {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<BigInt, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<BigInt, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn new() -> Self {
        Self(core::BigInt::new())
    }
}

impl From<core::BigInt> for BigInt {
    fn from(native: core::BigInt) -> Self {
        Self(native)
    }
}

impl From<BigInt> for core::BigInt {
    fn from(wasm: BigInt) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct ConstrPlutusData(pub(crate) core::ConstrPlutusData);

#[wasm_bindgen]

impl ConstrPlutusData {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<ConstrPlutusData, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<ConstrPlutusData, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn index_0(&self) -> u64 {
        self.0.index_0
    }

    pub fn plutus_datas(&self) -> PlutusDatas {
        self.0.plutus_datas.clone().into()
    }

    pub fn new(index_0: u64, plutus_datas: &PlutusDatas) -> Self {
        Self(core::ConstrPlutusData::new(index_0, plutus_datas.clone().into()))
    }
}

impl From<core::ConstrPlutusData> for ConstrPlutusData {
    fn from(native: core::ConstrPlutusData) -> Self {
        Self(native)
    }
}

impl From<ConstrPlutusData> for core::ConstrPlutusData {
    fn from(wasm: ConstrPlutusData) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct Costmdls(pub(crate) core::Costmdls);

#[wasm_bindgen]

impl Costmdls {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<Costmdls, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<Costmdls, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn set_key_0(&mut self, key_0: &Ints) {
        self.0.key_0 = Some(key_0.clone().into())
    }

    pub fn key_0(&self) -> Option<Ints> {
        self.0.key_0.clone().map(std::convert::Into::into)
    }

    pub fn set_key_1(&mut self, key_1: &Ints) {
        self.0.key_1 = Some(key_1.clone().into())
    }

    pub fn key_1(&self) -> Option<Ints> {
        self.0.key_1.clone().map(std::convert::Into::into)
    }

    pub fn new() -> Self {
        Self(core::Costmdls::new())
    }
}

impl From<core::Costmdls> for Costmdls {
    fn from(native: core::Costmdls) -> Self {
        Self(native)
    }
}

impl From<Costmdls> for core::Costmdls {
    fn from(wasm: Costmdls) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct ExUnitPrices(pub(crate) core::ExUnitPrices);

#[wasm_bindgen]

impl ExUnitPrices {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<ExUnitPrices, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<ExUnitPrices, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn mem_price(&self) -> SubCoin {
        self.0.mem_price.clone().into()
    }

    pub fn step_price(&self) -> SubCoin {
        self.0.step_price.clone().into()
    }

    pub fn new(mem_price: &SubCoin, step_price: &SubCoin) -> Self {
        Self(core::ExUnitPrices::new(mem_price.clone().into(), step_price.clone().into()))
    }
}

impl From<core::ExUnitPrices> for ExUnitPrices {
    fn from(native: core::ExUnitPrices) -> Self {
        Self(native)
    }
}

impl From<ExUnitPrices> for core::ExUnitPrices {
    fn from(wasm: ExUnitPrices) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct ExUnits(pub(crate) core::ExUnits);

#[wasm_bindgen]

impl ExUnits {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<ExUnits, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<ExUnits, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn mem(&self) -> u64 {
        self.0.mem
    }

    pub fn steps(&self) -> u64 {
        self.0.steps
    }

    pub fn new(mem: u64, steps: u64) -> Self {
        Self(core::ExUnits::new(mem, steps))
    }
}

impl From<core::ExUnits> for ExUnits {
    fn from(native: core::ExUnits) -> Self {
        Self(native)
    }
}

impl From<ExUnits> for core::ExUnits {
    fn from(wasm: ExUnits) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

pub enum LanguageKind {
    I0,
    I1,
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct Language(pub(crate) core::Language);

#[wasm_bindgen]

impl Language {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<Language, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<Language, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn new_i0() -> Self {
        Self(core::Language::new_i0())
    }

    pub fn new_i1() -> Self {
        Self(core::Language::new_i1())
    }

    pub fn kind(&self) -> LanguageKind {
        match &self.0 {
            core::Language::I0{ .. } => LanguageKind::I0,
            core::Language::I1{ .. } => LanguageKind::I1,
        }
    }
}

impl From<core::Language> for Language {
    fn from(native: core::Language) -> Self {
        Self(native)
    }
}

impl From<Language> for core::Language {
    fn from(wasm: Language) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

pub enum PlutusDataKind {
    ConstrPlutusData,
    MapPlutusDataToPlutusData,
    ArrPlutusData,
    BigInt,
    BoundedBytes,
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct PlutusData(pub(crate) core::PlutusData);

#[wasm_bindgen]

impl PlutusData {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<PlutusData, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<PlutusData, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn new_constr_plutus_data(constr_plutus_data: &ConstrPlutusData) -> Self {
        Self(core::PlutusData::new_constr_plutus_data(constr_plutus_data.clone().into()))
    }

    pub fn new_map_plutus_data_to_plutus_data(map_plutus_data_to_plutus_data: &MapPlutusDataToPlutusData) -> Self {
        Self(core::PlutusData::new_map_plutus_data_to_plutus_data(map_plutus_data_to_plutus_data.clone().into()))
    }

    pub fn new_arr_plutus_data(arr_plutus_data: &PlutusDatas) -> Self {
        Self(core::PlutusData::new_arr_plutus_data(arr_plutus_data.clone().into()))
    }

    pub fn new_big_int(big_int: &BigInt) -> Self {
        Self(core::PlutusData::new_big_int(big_int.clone().into()))
    }

    pub fn new_bounded_bytes(bounded_bytes: BoundedBytes) -> Self {
        Self(core::PlutusData::new_bounded_bytes(bounded_bytes))
    }

    pub fn kind(&self) -> PlutusDataKind {
        match &self.0 {
            core::PlutusData::ConstrPlutusData(_) => PlutusDataKind::ConstrPlutusData,
            core::PlutusData::MapPlutusDataToPlutusData{ .. } => PlutusDataKind::MapPlutusDataToPlutusData,
            core::PlutusData::ArrPlutusData{ .. } => PlutusDataKind::ArrPlutusData,
            core::PlutusData::BigInt(_) => PlutusDataKind::BigInt,
            core::PlutusData::BoundedBytes{ .. } => PlutusDataKind::BoundedBytes,
        }
    }

    pub fn as_constr_plutus_data(&self) -> Option<ConstrPlutusData> {
        match &self.0 {
            core::PlutusData::ConstrPlutusData(constr_plutus_data) => Some(constr_plutus_data.clone().into()),
            _ => None,
        }
    }

    pub fn as_map_plutus_data_to_plutus_data(&self) -> Option<MapPlutusDataToPlutusData> {
        match &self.0 {
            core::PlutusData::MapPlutusDataToPlutusData{ map_plutus_data_to_plutus_data, .. } => Some(map_plutus_data_to_plutus_data.clone().into()),
            _ => None,
        }
    }

    pub fn as_arr_plutus_data(&self) -> Option<PlutusDatas> {
        match &self.0 {
            core::PlutusData::ArrPlutusData{ arr_plutus_data, .. } => Some(arr_plutus_data.clone().into()),
            _ => None,
        }
    }

    pub fn as_big_int(&self) -> Option<BigInt> {
        match &self.0 {
            core::PlutusData::BigInt(big_int) => Some(big_int.clone().into()),
            _ => None,
        }
    }

    pub fn as_bounded_bytes(&self) -> Option<BoundedBytes> {
        match &self.0 {
            core::PlutusData::BoundedBytes{ bounded_bytes, .. } => Some(bounded_bytes.clone()),
            _ => None,
        }
    }
}

impl From<core::PlutusData> for PlutusData {
    fn from(native: core::PlutusData) -> Self {
        Self(native)
    }
}

impl From<PlutusData> for core::PlutusData {
    fn from(wasm: PlutusData) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct Redeemer(pub(crate) core::Redeemer);

#[wasm_bindgen]

impl Redeemer {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<Redeemer, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<Redeemer, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn tag(&self) -> RedeemerTag {
        self.0.tag.clone().into()
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

    pub fn new(tag: &RedeemerTag, index: u64, data: &PlutusData, ex_units: &ExUnits) -> Self {
        Self(core::Redeemer::new(tag.clone().into(), index, data.clone().into(), ex_units.clone().into()))
    }
}

impl From<core::Redeemer> for Redeemer {
    fn from(native: core::Redeemer) -> Self {
        Self(native)
    }
}

impl From<Redeemer> for core::Redeemer {
    fn from(wasm: Redeemer) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

pub enum RedeemerTagKind {
    I0,
    I1,
    I2,
    I3,
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct RedeemerTag(pub(crate) core::RedeemerTag);

#[wasm_bindgen]

impl RedeemerTag {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<RedeemerTag, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<RedeemerTag, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn new_i0() -> Self {
        Self(core::RedeemerTag::new_i0())
    }

    pub fn new_i1() -> Self {
        Self(core::RedeemerTag::new_i1())
    }

    pub fn new_i2() -> Self {
        Self(core::RedeemerTag::new_i2())
    }

    pub fn new_i3() -> Self {
        Self(core::RedeemerTag::new_i3())
    }

    pub fn kind(&self) -> RedeemerTagKind {
        match &self.0 {
            core::RedeemerTag::I0{ .. } => RedeemerTagKind::I0,
            core::RedeemerTag::I1{ .. } => RedeemerTagKind::I1,
            core::RedeemerTag::I2{ .. } => RedeemerTagKind::I2,
            core::RedeemerTag::I3{ .. } => RedeemerTagKind::I3,
        }
    }
}

impl From<core::RedeemerTag> for RedeemerTag {
    fn from(native: core::RedeemerTag) -> Self {
        Self(native)
    }
}

impl From<RedeemerTag> for core::RedeemerTag {
    fn from(wasm: RedeemerTag) -> Self {
        wasm.0
    }
}

use super::*;