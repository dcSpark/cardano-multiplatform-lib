// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

pub mod cbor_encodings;
pub mod serialization;
pub mod utils;

use super::Int;
use crate::plutus::{PlutusV1Script, PlutusV2Script};
use crate::transaction::NativeScript;
use cbor_encodings::{AlonzoAuxDataEncoding, ShelleyMaAuxDataEncoding};
use cml_core::error::*;
use cml_core::ordered_hash_map::OrderedHashMap;
use cml_core::serialization::{LenEncoding, StringEncoding};
use std::collections::BTreeMap;
use std::convert::TryFrom;

pub use cml_core::metadata::*;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct AlonzoAuxData {
    pub metadata: Option<Metadata>,
    pub native_scripts: Option<Vec<NativeScript>>,
    pub plutus_v1_scripts: Option<Vec<PlutusV1Script>>,
    pub plutus_v2_scripts: Option<Vec<PlutusV2Script>>,
    #[serde(skip)]
    pub encodings: Option<AlonzoAuxDataEncoding>,
}

impl AlonzoAuxData {
    pub fn new() -> Self {
        Self {
            metadata: None,
            native_scripts: None,
            plutus_v1_scripts: None,
            plutus_v2_scripts: None,
            encodings: None,
        }
    }
}

impl Default for AlonzoAuxData {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub enum AuxiliaryData {
    Shelley(ShelleyAuxData),
    ShelleyMA(ShelleyMaAuxData),
    Alonzo(AlonzoAuxData),
}

impl AuxiliaryData {
    pub fn new_shelley(shelley: ShelleyAuxData) -> Self {
        Self::Shelley(shelley)
    }

    pub fn new_shelley_m_a(shelley_m_a: ShelleyMaAuxData) -> Self {
        Self::ShelleyMA(shelley_m_a)
    }

    pub fn new_alonzo(alonzo: AlonzoAuxData) -> Self {
        Self::Alonzo(alonzo)
    }
}

pub type ShelleyAuxData = Metadata;

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
