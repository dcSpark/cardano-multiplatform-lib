use crate::{NativeScriptList, PlutusV1ScriptList, PlutusV2ScriptList};
use wasm_bindgen::prelude::wasm_bindgen;

use super::{AuxiliaryData, Metadata};

#[wasm_bindgen]
impl AuxiliaryData {
    pub fn new() -> Self {
        cml_chain::auxdata::AuxiliaryData::new().into()
    }

    pub fn metadata(&self) -> Option<Metadata> {
        self.0.metadata().map(|m| m.clone().into())
    }

    pub fn native_scripts(&self) -> Option<NativeScriptList> {
        self.0.native_scripts().cloned().map(Into::into)
    }

    pub fn plutus_v1_scripts(&self) -> Option<PlutusV1ScriptList> {
        self.0.plutus_v1_scripts().cloned().map(Into::into)
    }

    pub fn plutus_v2_scripts(&self) -> Option<PlutusV2ScriptList> {
        self.0.plutus_v2_scripts().cloned().map(Into::into)
    }

    /// Warning: overwrites any conflicting metadatum labels present
    pub fn add_metadata(&mut self, other: &Metadata) {
        self.0.add_metadata(other.clone().into())
    }

    /// Warning: does not check for duplicates and may migrate eras
    pub fn add_native_scripts(&mut self, scripts: &NativeScriptList) {
        self.0.add_native_scripts(scripts.clone().into())
    }

    /// Warning: does not check for duplicates and may migrate eras
    pub fn add_plutus_v1_scripts(&mut self, scripts: &PlutusV1ScriptList) {
        self.0.add_plutus_v1_scripts(scripts.clone().into())
    }

    /// Warning: does not check for duplicates and may migrate eras
    pub fn add_plutus_v2_scripts(&mut self, scripts: &PlutusV2ScriptList) {
        self.0.add_plutus_v2_scripts(scripts.clone().into())
    }

    /// Adds everything present in other to self
    /// May change the era the aux data is in if necessary
    /// Warning: overwrites any metadatum labels present
    /// also does not check for duplicates in scripts
    pub fn add(&mut self, other: &AuxiliaryData) {
        self.0.add(other.clone().into())
    }
}
