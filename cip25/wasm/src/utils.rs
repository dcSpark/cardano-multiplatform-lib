use crate::*;

use wasm_bindgen::prelude::{JsError};

use cml_core_wasm::metadata::Metadata;

#[wasm_bindgen]
impl CIP25Metadata {
    /// Create a Metadata containing only the CIP25 schema
    pub fn to_metadata(&self) -> Result<Metadata, JsError> {
        self.0.to_metadata().map(Metadata::from).map_err(Into::into)
    }

    /// Read the CIP25 schema from a Metadata. Ignores all other data besides CIP25
    /// Can fail if the Metadata does not conform to CIP25
    pub fn from_metadata(metadata: &Metadata) -> Result<CIP25Metadata, JsError> {
        core::CIP25Metadata::from_metadata(metadata.as_ref()).map(Self).map_err(Into::into)
    }

    /// Add to an existing metadata (could be empty) the full CIP25 metadata
    pub fn add_to_metadata(&self, metadata: &mut Metadata) -> Result<(), JsError> {
        self.0.add_to_metadata(metadata.as_mut()).map_err(Into::into)
    }

}

#[wasm_bindgen]
impl String64 {
    pub fn new(s: &str) -> Result<String64, JsError> {
        core::String64::new_str(s).map(Self).map_err(Into::into)
    }

    pub fn to_str(&self) -> String {
        self.0.to_str().to_owned()
    }

    pub fn get_str(&self) -> String {
        self.0.get().clone()
    }
}

#[wasm_bindgen]

impl ChunkableString {
    pub fn from_string(str: &str) -> Self {
        Self(core::ChunkableString::from(str))
    }

    pub fn to_string(&self) -> String {
        String::from(&self.0)
    }
}