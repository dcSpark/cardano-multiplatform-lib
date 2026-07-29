// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

extern crate alloc;
use alloc::vec::Vec;
pub mod cbor_encodings;
pub mod serialization;
// cddl-codegen extern re-export contract: this crate's hand-written root lib.rs must re-export
// each name below (`pub use <your_module>::<Name>;`) so the generated glue resolves against the
// user-owned definition. See the extern types section of docs/output_format.
pub use crate::Metadata;

use crate::generated::plutus::{PlutusV1Script, PlutusV2Script, PlutusV3Script};
use crate::generated::transaction::NativeScript;
use cbor_encodings::{ConwayFormatAuxDataEncoding, ShelleyMAFormatAuxDataEncoding};

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
