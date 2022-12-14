use super::*;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct AlonzoAuxData {
    pub key_0: Option<Metadata>,
    pub key_1: Option<Vec<NativeScript>>,
    pub key_2: Option<Vec<PlutusV1Script>>,
    pub key_3: Option<Vec<PlutusV2Script>>,
    #[serde(skip)]
    pub encodings: Option<AlonzoAuxDataEncoding>,
}

impl AlonzoAuxData {
    pub fn new() -> Self {
        Self {
            key_0: None,
            key_1: None,
            key_2: None,
            key_3: None,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub enum AuxiliaryData {
    ShelleyAuxData {
        shelley_aux_data: ShelleyAuxData,
        #[serde(skip)]
        shelley_aux_data_encoding: LenEncoding,
        #[serde(skip)]
        shelley_aux_data_key_encodings: BTreeMap<u64, Option<cbor_event::Sz>>,
    }
    ,
    ShelleyMaAuxData(ShelleyMaAuxData),
    AlonzoAuxData(AlonzoAuxData),
}

impl AuxiliaryData {
    pub fn new_shelley_aux_data(shelley_aux_data: ShelleyAuxData) -> Self {
        Self::ShelleyAuxData {
            shelley_aux_data,
            shelley_aux_data_encoding: LenEncoding::default(),
            shelley_aux_data_key_encodings: BTreeMap::new(),
        }
    }

    pub fn new_shelley_ma_aux_data(shelley_ma_aux_data: ShelleyMaAuxData) -> Self {
        Self::ShelleyMaAuxData(shelley_ma_aux_data)
    }

    pub fn new_alonzo_aux_data(alonzo_aux_data: AlonzoAuxData) -> Self {
        Self::AlonzoAuxData(alonzo_aux_data)
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ShelleyMaAuxData {
    pub transaction_metadata: Metadata,
    pub auxiliary_scripts: Vec<NativeScript>,
    #[serde(skip)]
    pub encodings: Option<ShelleyMaAuxDataEncoding>,
}

impl ShelleyMaAuxData {
    pub fn new(transaction_metadata: Metadata, auxiliary_scripts: Vec<NativeScript>) -> Self {
        Self {
            transaction_metadata,
            auxiliary_scripts,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema, Derivative)]
#[derivative(Eq, PartialEq, Ord="feature_allow_slow_enum", PartialOrd="feature_allow_slow_enum", Hash)]
pub enum TransactionMetadatum {
    MapTransactionMetadatumToTransactionMetadatum {
        map_transaction_metadatum_to_transaction_metadatum: OrderedHashMap<TransactionMetadatum, TransactionMetadatum>,
        #[derivative(PartialEq="ignore", Ord="ignore", PartialOrd="ignore", Hash="ignore")]
        #[serde(skip)]
        map_transaction_metadatum_to_transaction_metadatum_encoding: LenEncoding,
    }
    ,
    ArrTransactionMetadatum {
        arr_transaction_metadatum: Vec<TransactionMetadatum>,
        #[derivative(PartialEq="ignore", Ord="ignore", PartialOrd="ignore", Hash="ignore")]
        #[serde(skip)]
        arr_transaction_metadatum_encoding: LenEncoding,
    }
    ,
    Int(Int),
    Bytes {
        bytes: Vec<u8>,
        #[derivative(PartialEq="ignore", Ord="ignore", PartialOrd="ignore", Hash="ignore")]
        #[serde(skip)]
        bytes_encoding: StringEncoding,
    }
    ,
    Text {
        text: String,
        #[derivative(PartialEq="ignore", Ord="ignore", PartialOrd="ignore", Hash="ignore")]
        #[serde(skip)]
        text_encoding: StringEncoding,
    }
    ,
}

impl TransactionMetadatum {
    pub fn new_map_transaction_metadatum_to_transaction_metadatum(map_transaction_metadatum_to_transaction_metadatum: OrderedHashMap<TransactionMetadatum, TransactionMetadatum>) -> Self {
        Self::MapTransactionMetadatumToTransactionMetadatum {
            map_transaction_metadatum_to_transaction_metadatum,
            map_transaction_metadatum_to_transaction_metadatum_encoding: LenEncoding::default(),
        }
    }

    pub fn new_arr_transaction_metadatum(arr_transaction_metadatum: Vec<TransactionMetadatum>) -> Self {
        Self::ArrTransactionMetadatum {
            arr_transaction_metadatum,
            arr_transaction_metadatum_encoding: LenEncoding::default(),
        }
    }

    pub fn new_int(int: Int) -> Self {
        Self::Int(int)
    }

    pub fn new_bytes(bytes: Vec<u8>) -> Self {
        Self::Bytes {
            bytes,
            bytes_encoding: StringEncoding::default(),
        }
    }

    pub fn new_text(text: String) -> Self {
        Self::Text {
            text,
            text_encoding: StringEncoding::default(),
        }
    }
}

use super::*;