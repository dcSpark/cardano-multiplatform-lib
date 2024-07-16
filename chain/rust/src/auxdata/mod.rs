// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

pub mod cbor_encodings;
pub mod metadata;
pub mod serialization;
pub mod utils;

use crate::plutus::{PlutusV1Script, PlutusV2Script, PlutusV3Script};
use crate::transaction::NativeScript;
use cbor_encodings::{ConwayFormatAuxDataEncoding, ShelleyMAFormatAuxDataEncoding};

pub use metadata::*;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub enum AuxiliaryData {
    Shelley(ShelleyFormatAuxData),
    ShelleyMA(ShelleyMAFormatAuxData),
    Conway(ConwayFormatAuxData),
}

impl AuxiliaryData {
    pub fn new_shelley(shelley: ShelleyFormatAuxData) -> Self {
        Self::Shelley(shelley)
    }

    pub fn new_shelley_ma(shelley_ma: ShelleyMAFormatAuxData) -> Self {
        Self::ShelleyMA(shelley_ma)
    }

    pub fn new_conway(conway: ConwayFormatAuxData) -> Self {
        Self::Conway(conway)
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ConwayFormatAuxData {
    pub metadata: Option<Metadata>,
    pub native_scripts: Option<Vec<NativeScript>>,
    pub plutus_v1_scripts: Option<Vec<PlutusV1Script>>,
    pub plutus_v2_scripts: Option<Vec<PlutusV2Script>>,
    pub plutus_v3_scripts: Option<Vec<PlutusV3Script>>,
    #[serde(skip)]
    pub encodings: Option<ConwayFormatAuxDataEncoding>,
}

impl ConwayFormatAuxData {
    pub fn new() -> Self {
        Self {
            metadata: None,
            native_scripts: None,
            plutus_v1_scripts: None,
            plutus_v2_scripts: None,
            plutus_v3_scripts: None,
            encodings: None,
        }
    }
}

impl Default for ConwayFormatAuxData {
    fn default() -> Self {
        Self::new()
    }
}

pub type ShelleyFormatAuxData = Metadata;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ShelleyMAFormatAuxData {
    pub transaction_metadata: Metadata,
    pub auxiliary_scripts: Vec<NativeScript>,
    #[serde(skip)]
    pub encodings: Option<ShelleyMAFormatAuxDataEncoding>,
}

impl ShelleyMAFormatAuxData {
    pub fn new(transaction_metadata: Metadata, auxiliary_scripts: Vec<NativeScript>) -> Self {
        Self {
            transaction_metadata,
            auxiliary_scripts,
            encodings: None,
        }
    }
}
