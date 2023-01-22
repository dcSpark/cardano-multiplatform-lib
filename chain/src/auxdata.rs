/// TX auxiliary data
use super::*;

// TX metadata (i.e. metadatums)
pub use cml_core::metadata::*;

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