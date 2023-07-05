// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use super::{
    NativeScriptList, PlutusV1ScriptList, PlutusV2ScriptList,
};
pub use cml_core_wasm::metadata::{Metadata, TransactionMetadatum, TransactionMetadatumLabel};
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct AlonzoAuxData(cml_chain::auxdata::AlonzoAuxData);

#[wasm_bindgen]
impl AlonzoAuxData {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<AlonzoAuxData, JsValue> {
        cml_chain::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<AlonzoAuxData, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

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

    pub fn new() -> Self {
        Self(cml_chain::auxdata::AlonzoAuxData::new())
    }
}

impl From<cml_chain::auxdata::AlonzoAuxData> for AlonzoAuxData {
    fn from(native: cml_chain::auxdata::AlonzoAuxData) -> Self {
        Self(native)
    }
}

impl From<AlonzoAuxData> for cml_chain::auxdata::AlonzoAuxData {
    fn from(wasm: AlonzoAuxData) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::auxdata::AlonzoAuxData> for AlonzoAuxData {
    fn as_ref(&self) -> &cml_chain::auxdata::AlonzoAuxData {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct AuxiliaryData(cml_chain::auxdata::AuxiliaryData);

#[wasm_bindgen]
impl AuxiliaryData {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<AuxiliaryData, JsValue> {
        cml_chain::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<AuxiliaryData, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

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

    pub fn new_alonzo(alonzo: &AlonzoAuxData) -> Self {
        Self(cml_chain::auxdata::AuxiliaryData::new_alonzo(
            alonzo.clone().into(),
        ))
    }

    pub fn kind(&self) -> AuxiliaryDataKind {
        match &self.0 {
            cml_chain::auxdata::AuxiliaryData::Shelley { .. } => AuxiliaryDataKind::Shelley,
            cml_chain::auxdata::AuxiliaryData::ShelleyMA(_) => AuxiliaryDataKind::ShelleyMA,
            cml_chain::auxdata::AuxiliaryData::Alonzo(_) => AuxiliaryDataKind::Alonzo,
        }
    }

    pub fn as_shelley(&self) -> Option<ShelleyAuxData> {
        match &self.0 {
            cml_chain::auxdata::AuxiliaryData::Shelley(shelley) => {
                Some(shelley.clone().into())
            }
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

    pub fn as_alonzo(&self) -> Option<AlonzoAuxData> {
        match &self.0 {
            cml_chain::auxdata::AuxiliaryData::Alonzo(alonzo) => Some(alonzo.clone().into()),
            _ => None,
        }
    }
}

impl From<cml_chain::auxdata::AuxiliaryData> for AuxiliaryData {
    fn from(native: cml_chain::auxdata::AuxiliaryData) -> Self {
        Self(native)
    }
}

impl From<AuxiliaryData> for cml_chain::auxdata::AuxiliaryData {
    fn from(wasm: AuxiliaryData) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::auxdata::AuxiliaryData> for AuxiliaryData {
    fn as_ref(&self) -> &cml_chain::auxdata::AuxiliaryData {
        &self.0
    }
}

#[wasm_bindgen]
pub enum AuxiliaryDataKind {
    Shelley,
    ShelleyMA,
    Alonzo,
}

pub type ShelleyAuxData = Metadata;

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ShelleyMaAuxData(cml_chain::auxdata::ShelleyMaAuxData);

#[wasm_bindgen]
impl ShelleyMaAuxData {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<ShelleyMaAuxData, JsValue> {
        cml_chain::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<ShelleyMaAuxData, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

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

impl From<cml_chain::auxdata::ShelleyMaAuxData> for ShelleyMaAuxData {
    fn from(native: cml_chain::auxdata::ShelleyMaAuxData) -> Self {
        Self(native)
    }
}

impl From<ShelleyMaAuxData> for cml_chain::auxdata::ShelleyMaAuxData {
    fn from(wasm: ShelleyMaAuxData) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::auxdata::ShelleyMaAuxData> for ShelleyMaAuxData {
    fn as_ref(&self) -> &cml_chain::auxdata::ShelleyMaAuxData {
        &self.0
    }
}
