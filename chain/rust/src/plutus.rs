use super::*;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema, Derivative)]
#[derivative(Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct BigInt {
    #[derivative(
        PartialEq = "ignore",
        Ord = "ignore",
        PartialOrd = "ignore",
        Hash = "ignore"
    )]
    #[serde(skip)]
    pub encodings: Option<BigIntEncoding>,
}

impl BigInt {
    pub fn new() -> Self {
        Self { encodings: None }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema, Derivative)]
#[derivative(Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ConstrPlutusData {
    pub index_0: u64,
    pub plutus_datas: Vec<PlutusData>,
    #[derivative(
        PartialEq = "ignore",
        Ord = "ignore",
        PartialOrd = "ignore",
        Hash = "ignore"
    )]
    #[serde(skip)]
    pub encodings: Option<ConstrPlutusDataEncoding>,
}

impl ConstrPlutusData {
    pub fn new(index_0: u64, plutus_datas: Vec<PlutusData>) -> Self {
        Self {
            index_0,
            plutus_datas,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct Costmdls {
    pub key_0: Option<Vec<Int>>,
    pub key_1: Option<Vec<Int>>,
    #[serde(skip)]
    pub encodings: Option<CostmdlsEncoding>,
}

impl Costmdls {
    pub fn new() -> Self {
        Self {
            key_0: None,
            key_1: None,
            encodings: None,
        }
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

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ExUnits {
    pub mem: u64,
    pub steps: u64,
    #[serde(skip)]
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

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub enum Language {
    I0 {
        #[serde(skip)]
        i0_encoding: Option<cbor_event::Sz>,
    },
    I1 {
        #[serde(skip)]
        i1_encoding: Option<cbor_event::Sz>,
    },
}

impl Language {
    pub fn new_i0() -> Self {
        Self::I0 { i0_encoding: None }
    }

    pub fn new_i1() -> Self {
        Self::I1 { i1_encoding: None }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema, Derivative)]
#[derivative(
    Eq,
    PartialEq,
    Ord = "feature_allow_slow_enum",
    PartialOrd = "feature_allow_slow_enum",
    Hash
)]
pub enum PlutusData {
    ConstrPlutusData(ConstrPlutusData),
    MapPlutusDataToPlutusData {
        map_plutus_data_to_plutus_data: OrderedHashMap<PlutusData, PlutusData>,
        #[derivative(
            PartialEq = "ignore",
            Ord = "ignore",
            PartialOrd = "ignore",
            Hash = "ignore"
        )]
        #[serde(skip)]
        map_plutus_data_to_plutus_data_encoding: LenEncoding,
    },
    ArrPlutusData {
        arr_plutus_data: Vec<PlutusData>,
        #[derivative(
            PartialEq = "ignore",
            Ord = "ignore",
            PartialOrd = "ignore",
            Hash = "ignore"
        )]
        #[serde(skip)]
        arr_plutus_data_encoding: LenEncoding,
    },
    BigInt(BigInt),
    BoundedBytes {
        bounded_bytes: BoundedBytes,
        #[derivative(
            PartialEq = "ignore",
            Ord = "ignore",
            PartialOrd = "ignore",
            Hash = "ignore"
        )]
        #[serde(skip)]
        bounded_bytes_encoding: StringEncoding,
    },
}

impl PlutusData {
    pub fn new_constr_plutus_data(constr_plutus_data: ConstrPlutusData) -> Self {
        Self::ConstrPlutusData(constr_plutus_data)
    }

    pub fn new_map_plutus_data_to_plutus_data(
        map_plutus_data_to_plutus_data: OrderedHashMap<PlutusData, PlutusData>,
    ) -> Self {
        Self::MapPlutusDataToPlutusData {
            map_plutus_data_to_plutus_data,
            map_plutus_data_to_plutus_data_encoding: LenEncoding::default(),
        }
    }

    pub fn new_arr_plutus_data(arr_plutus_data: Vec<PlutusData>) -> Self {
        Self::ArrPlutusData {
            arr_plutus_data,
            arr_plutus_data_encoding: LenEncoding::default(),
        }
    }

    pub fn new_big_int(big_int: BigInt) -> Self {
        Self::BigInt(big_int)
    }

    pub fn new_bounded_bytes(bounded_bytes: BoundedBytes) -> Self {
        Self::BoundedBytes {
            bounded_bytes,
            bounded_bytes_encoding: StringEncoding::default(),
        }
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

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub enum RedeemerTag {
    I0 {
        #[serde(skip)]
        i0_encoding: Option<cbor_event::Sz>,
    },
    I1 {
        #[serde(skip)]
        i1_encoding: Option<cbor_event::Sz>,
    },
    I2 {
        #[serde(skip)]
        i2_encoding: Option<cbor_event::Sz>,
    },
    I3 {
        #[serde(skip)]
        i3_encoding: Option<cbor_event::Sz>,
    },
}

impl RedeemerTag {
    pub fn new_i0() -> Self {
        Self::I0 { i0_encoding: None }
    }

    pub fn new_i1() -> Self {
        Self::I1 { i1_encoding: None }
    }

    pub fn new_i2() -> Self {
        Self::I2 { i2_encoding: None }
    }

    pub fn new_i3() -> Self {
        Self::I3 { i3_encoding: None }
    }
}

use super::*;
