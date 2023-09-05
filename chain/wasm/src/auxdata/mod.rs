// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use super::{NativeScriptList, PlutusV1ScriptList, PlutusV2ScriptList, PlutusV3ScriptList};
pub use cml_core_wasm::metadata::{Metadata, TransactionMetadatum, TransactionMetadatumLabel};
use cml_core_wasm::{impl_wasm_cbor_json_api, impl_wasm_conversions};
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct AuxiliaryData(cml_chain::auxdata::AuxiliaryData);

impl_wasm_cbor_json_api!(AuxiliaryData);

impl_wasm_conversions!(cml_chain::auxdata::AuxiliaryData, AuxiliaryData);

#[wasm_bindgen]
impl AuxiliaryData {
    pub fn new_shelley(shelley: &ShelleyAuxData) -> Self {
        Self(cml_chain::auxdata::AuxiliaryData::new_shelley(
            shelley.clone().into(),
        ))
    }

    pub fn new_shelley_m_a(shelley_m_a: &ShelleyMaAuxData) -> Self {
        Self(cml_chain::auxdata::AuxiliaryData::new_shelley_m_a(
            shelley_m_a.clone().into(),
        ))
    }

    pub fn new_conway(alonzo: &ConwayAuxData) -> Self {
        Self(cml_chain::auxdata::AuxiliaryData::new_conway(
            alonzo.clone().into(),
        ))
    }

    pub fn kind(&self) -> AuxiliaryDataKind {
        match &self.0 {
            cml_chain::auxdata::AuxiliaryData::Shelley { .. } => AuxiliaryDataKind::Shelley,
            cml_chain::auxdata::AuxiliaryData::ShelleyMA(_) => AuxiliaryDataKind::ShelleyMA,
            cml_chain::auxdata::AuxiliaryData::Conway(_) => AuxiliaryDataKind::Conway,
        }
    }

    pub fn as_shelley(&self) -> Option<ShelleyAuxData> {
        match &self.0 {
            cml_chain::auxdata::AuxiliaryData::Shelley(shelley) => Some(shelley.clone().into()),
            _ => None,
        }
    }

    pub fn as_shelley_m_a(&self) -> Option<ShelleyMaAuxData> {
        match &self.0 {
            cml_chain::auxdata::AuxiliaryData::ShelleyMA(shelley_m_a) => {
                Some(shelley_m_a.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_alonzo(&self) -> Option<ConwayAuxData> {
        match &self.0 {
            cml_chain::auxdata::AuxiliaryData::Conway(alonzo) => Some(alonzo.clone().into()),
            _ => None,
        }
    }
}

#[wasm_bindgen]
pub enum AuxiliaryDataKind {
    Shelley,
    ShelleyMA,
    Conway,
}

pub type ShelleyAuxData = Metadata;

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ShelleyMaAuxData(cml_chain::auxdata::ShelleyMaAuxData);

impl_wasm_cbor_json_api!(ShelleyMaAuxData);

impl_wasm_conversions!(cml_chain::auxdata::ShelleyMaAuxData, ShelleyMaAuxData);

#[wasm_bindgen]
impl ShelleyMaAuxData {
    pub fn transaction_metadata(&self) -> Metadata {
        self.0.transaction_metadata.clone().into()
    }

    pub fn auxiliary_scripts(&self) -> NativeScriptList {
        self.0.auxiliary_scripts.clone().into()
    }

    pub fn new(transaction_metadata: &Metadata, auxiliary_scripts: &NativeScriptList) -> Self {
        Self(cml_chain::auxdata::ShelleyMaAuxData::new(
            transaction_metadata.clone().into(),
            auxiliary_scripts.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ConwayAuxData(cml_chain::auxdata::ConwayAuxData);

impl_wasm_cbor_json_api!(ConwayAuxData);

impl_wasm_conversions!(cml_chain::auxdata::ConwayAuxData, ConwayAuxData);

#[wasm_bindgen]
impl ConwayAuxData {
    pub fn set_metadata(&mut self, metadata: &Metadata) {
        self.0.metadata = Some(metadata.clone().into())
    }

    pub fn metadata(&self) -> Option<Metadata> {
        self.0.metadata.clone().map(std::convert::Into::into)
    }

    pub fn set_native_scripts(&mut self, native_scripts: &NativeScriptList) {
        self.0.native_scripts = Some(native_scripts.clone().into())
    }

    pub fn native_scripts(&self) -> Option<NativeScriptList> {
        self.0.native_scripts.clone().map(std::convert::Into::into)
    }

    pub fn set_plutus_v1_scripts(&mut self, plutus_v1_scripts: &PlutusV1ScriptList) {
        self.0.plutus_v1_scripts = Some(plutus_v1_scripts.clone().into())
    }

    pub fn plutus_v1_scripts(&self) -> Option<PlutusV1ScriptList> {
        self.0
            .plutus_v1_scripts
            .clone()
            .map(std::convert::Into::into)
    }

    pub fn set_plutus_v2_scripts(&mut self, plutus_v2_scripts: &PlutusV2ScriptList) {
        self.0.plutus_v2_scripts = Some(plutus_v2_scripts.clone().into())
    }

    pub fn plutus_v2_scripts(&self) -> Option<PlutusV2ScriptList> {
        self.0
            .plutus_v2_scripts
            .clone()
            .map(std::convert::Into::into)
    }

        pub fn set_plutus_v3_scripts(&mut self, plutus_v3_scripts: &PlutusV3ScriptList) {
        self.0.plutus_v3_scripts = Some(plutus_v3_scripts.clone().into())
    }

    pub fn plutus_v3_scripts(&self) -> Option<PlutusV3ScriptList> {
        self.0
            .plutus_v3_scripts
            .clone()
            .map(std::convert::Into::into)
    }

    pub fn new() -> Self {
        Self(cml_chain::auxdata::ConwayAuxData::new())
    }
}
