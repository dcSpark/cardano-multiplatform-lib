#![allow(
    clippy::len_without_is_empty,
    clippy::too_many_arguments,
    clippy::new_without_default
)]
impl_wasm_list_needs_into!(
    cml_cip25::CIP25String64,
    CIP25String64,
    CIP25String64List,
    true,
    false
);
impl_wasm_list_needs_into!(
    cml_cip25::CIP25FilesDetails,
    CIP25FilesDetails,
    CIP25FilesDetailsList,
    true,
    false
);
// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

pub use crate::CIP25LabelMetadata;

use cml_core_wasm::{
    impl_wasm_cbor_json_api_cbor_event_serialize, impl_wasm_conversions, impl_wasm_list_needs_into,
};
use std::collections::BTreeMap;
use wasm_bindgen::prelude::{wasm_bindgen, JsError};

/// A String that may or may not be chunked into 64-byte chunks to be able
/// to conform to Cardano TX Metadata limitations.
/// Most users should simply use CIP25ChunkableString::from_string() and CIP25ChunkableString::to_string()
/// and avoid the explicit single/chunk interface:
/// ```javascript
/// let chunkableString = CIP25.CIP25ChunkableString.from_string("this can be any length and will automatically be chunked if needed");
/// ```
#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct CIP25ChunkableString(cml_cip25::CIP25ChunkableString);

impl_wasm_cbor_json_api_cbor_event_serialize!(CIP25ChunkableString);

impl_wasm_conversions!(cml_cip25::CIP25ChunkableString, CIP25ChunkableString);

#[wasm_bindgen]
impl CIP25ChunkableString {
    pub fn new_single(single: &CIP25String64) -> Self {
        Self(cml_cip25::CIP25ChunkableString::new_single(
            single.clone().into(),
        ))
    }

    pub fn new_chunked(chunked: &CIP25String64List) -> Self {
        Self(cml_cip25::CIP25ChunkableString::new_chunked(
            chunked.clone().into(),
        ))
    }

    pub fn kind(&self) -> CIP25ChunkableStringKind {
        match &self.0 {
            cml_cip25::CIP25ChunkableString::Single(_) => CIP25ChunkableStringKind::Single,
            cml_cip25::CIP25ChunkableString::Chunked(_) => CIP25ChunkableStringKind::Chunked,
        }
    }

    pub fn as_single(&self) -> Option<CIP25String64> {
        match &self.0 {
            cml_cip25::CIP25ChunkableString::Single(single) => Some(single.clone().into()),
            _ => None,
        }
    }

    pub fn as_chunked(&self) -> Option<CIP25String64List> {
        match &self.0 {
            cml_cip25::CIP25ChunkableString::Chunked(chunked) => Some(chunked.clone().into()),
            _ => None,
        }
    }
}

#[wasm_bindgen]
pub enum CIP25ChunkableStringKind {
    Single,
    Chunked,
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct CIP25FilesDetails(cml_cip25::CIP25FilesDetails);

impl_wasm_cbor_json_api_cbor_event_serialize!(CIP25FilesDetails);

impl_wasm_conversions!(cml_cip25::CIP25FilesDetails, CIP25FilesDetails);

#[wasm_bindgen]
impl CIP25FilesDetails {
    pub fn name(&self) -> CIP25String64 {
        self.0.name.clone().into()
    }

    pub fn media_type(&self) -> CIP25String64 {
        self.0.media_type.clone().into()
    }

    pub fn src(&self) -> CIP25ChunkableString {
        self.0.src.clone().into()
    }

    pub fn new(
        name: &CIP25String64,
        media_type: &CIP25String64,
        src: &CIP25ChunkableString,
    ) -> Self {
        Self(cml_cip25::CIP25FilesDetails::new(
            name.clone().into(),
            media_type.clone().into(),
            src.clone().into(),
        ))
    }
}

/// This is the entire metadata schema for CIP-25
/// It can be parsed by passing in the CBOR bytes of the entire transaction metadata
/// or by passing in an existing Metadata struct.
/// Parsing from CBOR bytes should be marginally faster.
/// 
/// Careful: `to_cbor_bytes`, `from_cbor_bytes`, `to_cbor_hex`, `from_cbor_hex` will:
/// 1. ignore all non-CIP25 keys
/// 2. not support round-trip serialization
/// Use  `cml_chain::auxdata::Metadata` / `TransactionMetadatum` for round-tripping metadata.
#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct CIP25Metadata(cml_cip25::CIP25Metadata);

impl_wasm_cbor_json_api_cbor_event_serialize!(CIP25Metadata);

impl_wasm_conversions!(cml_cip25::CIP25Metadata, CIP25Metadata);

#[wasm_bindgen]
impl CIP25Metadata {
    pub fn key_721(&self) -> CIP25LabelMetadata {
        self.0.key_721.clone().into()
    }

    pub fn new(key_721: &CIP25LabelMetadata) -> Self {
        Self(cml_cip25::CIP25Metadata::new(key_721.clone().into()))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct CIP25MetadataDetails(cml_cip25::CIP25MetadataDetails);

impl_wasm_cbor_json_api_cbor_event_serialize!(CIP25MetadataDetails);

impl_wasm_conversions!(cml_cip25::CIP25MetadataDetails, CIP25MetadataDetails);

#[wasm_bindgen]
impl CIP25MetadataDetails {
    pub fn name(&self) -> CIP25String64 {
        self.0.name.clone().into()
    }

    pub fn image(&self) -> CIP25ChunkableString {
        self.0.image.clone().into()
    }

    pub fn set_media_type(&mut self, media_type: &CIP25String64) {
        self.0.media_type = Some(media_type.clone().into())
    }

    pub fn media_type(&self) -> Option<CIP25String64> {
        self.0.media_type.clone().map(std::convert::Into::into)
    }

    pub fn set_description(&mut self, description: &CIP25ChunkableString) {
        self.0.description = Some(description.clone().into())
    }

    pub fn description(&self) -> Option<CIP25ChunkableString> {
        self.0.description.clone().map(std::convert::Into::into)
    }

    pub fn set_files(&mut self, files: &CIP25FilesDetailsList) {
        self.0.files = Some(files.clone().into())
    }

    pub fn files(&self) -> Option<CIP25FilesDetailsList> {
        self.0.files.clone().map(std::convert::Into::into)
    }

    pub fn new(name: &CIP25String64, image: &CIP25ChunkableString) -> Self {
        Self(cml_cip25::CIP25MetadataDetails::new(
            name.clone().into(),
            image.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct CIP25String64(cml_cip25::CIP25String64);

impl_wasm_cbor_json_api_cbor_event_serialize!(CIP25String64);

impl_wasm_conversions!(cml_cip25::CIP25String64, CIP25String64);

#[wasm_bindgen]
impl CIP25String64 {
    pub fn new(inner: String) -> Result<CIP25String64, JsError> {
        cml_cip25::CIP25String64::new(inner)
            .map(Into::into)
            .map_err(Into::into)
    }

    pub fn get(&self) -> String {
        self.0.get().clone()
    }
}
