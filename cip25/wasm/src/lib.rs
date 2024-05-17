#![allow(
    clippy::len_without_is_empty,
    clippy::too_many_arguments,
    clippy::new_without_default
)]

pub mod utils;

use cml_core_wasm::{
    impl_wasm_cbor_json_api_cbor_event_serialize, impl_wasm_conversions, impl_wasm_json_api,
    impl_wasm_list,
};
pub use utils::CIP25LabelMetadata;

pub use cml_cip25::CIP25Version;

use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

/// This is the entire metadata schema for CIP-25
/// It can be parsed by passing in the CBOR bytes of the entire transaction metadata
/// or by passing in an existing Metadata struct.
/// Parsing from CBOR bytes should be marginally faster.
#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct CIP25Metadata(cml_cip25::CIP25Metadata);

impl_wasm_conversions!(cml_cip25::CIP25Metadata, CIP25Metadata);

// we manually write to_cbor_bytes/from_cbor_bytes so we can add the comments

impl_wasm_json_api!(CIP25Metadata);

#[wasm_bindgen]
impl CIP25Metadata {
    /// Serialize to CBOR bytes compatible with tx metadata
    /// Does not guarantee any specific type of CBOR format and should NOT
    /// be used with round-tripping. It will ignore all non-CIP25 keys.
    /// Use cml_cip25::metadate crate for round-tripping metadata.
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        use cml_cip25::serialization::ToBytes;
        ToBytes::to_bytes(&self.0)
    }

    /// Deserialize from CBOR bytes compatible with tx metadata
    /// Does not guarantee any specific type of CBOR format and should NOT
    /// be used with round-tripping. It will ignore all non-CIP25 keys.
    /// Use cml_cip25::metadate crate for round-tripping metadata.
    pub fn from_cbor_bytes(data: Vec<u8>) -> Result<CIP25Metadata, JsValue> {
        use cml_cip25::serialization::FromBytes;
        FromBytes::from_bytes(data)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_cbor_bytes: {e}")))
    }

    /// The core details of the CIP25 spec
    pub fn key_721(&self) -> CIP25LabelMetadata {
        self.0.key_721.clone().into()
    }

    pub fn new(key_721: &CIP25LabelMetadata) -> Self {
        Self(cml_cip25::CIP25Metadata::new(key_721.clone().into()))
    }
}

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

impl_wasm_conversions!(cml_cip25::CIP25ChunkableString, CIP25ChunkableString);

impl_wasm_cbor_json_api_cbor_event_serialize!(CIP25ChunkableString);

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

    pub fn kind(&self) -> ChunkableStringKind {
        match &self.0 {
            cml_cip25::CIP25ChunkableString::Single(_) => ChunkableStringKind::Single,
            cml_cip25::CIP25ChunkableString::Chunked(_) => ChunkableStringKind::Chunked,
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
pub enum ChunkableStringKind {
    Single,
    Chunked,
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct CIP25FilesDetails(cml_cip25::CIP25FilesDetails);

impl_wasm_conversions!(cml_cip25::CIP25FilesDetails, CIP25FilesDetails);

impl_wasm_cbor_json_api_cbor_event_serialize!(CIP25FilesDetails);

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

impl_wasm_list!(
    cml_cip25::CIP25FilesDetails,
    CIP25FilesDetails,
    FilesDetailsList
);

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct CIP25MetadataDetails(cml_cip25::CIP25MetadataDetails);

impl_wasm_conversions!(cml_cip25::CIP25MetadataDetails, CIP25MetadataDetails);

impl_wasm_cbor_json_api_cbor_event_serialize!(CIP25MetadataDetails);

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

    pub fn set_files(&mut self, files: &FilesDetailsList) {
        self.0.files = Some(files.clone().into())
    }

    pub fn files(&self) -> Option<FilesDetailsList> {
        self.0.files.clone().map(std::convert::Into::into)
    }

    pub fn new(name: &CIP25String64, image: &CIP25ChunkableString) -> Self {
        Self(cml_cip25::CIP25MetadataDetails::new(
            name.clone().into(),
            image.clone().into(),
        ))
    }
}

/// A String of at most 64 bytes.
/// This is to conform with Cardano metadata restrictions.
#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct CIP25String64(cml_cip25::CIP25String64);

impl_wasm_conversions!(cml_cip25::CIP25String64, CIP25String64);

impl_wasm_cbor_json_api_cbor_event_serialize!(CIP25String64);

#[wasm_bindgen]
impl CIP25String64 {
    pub fn get(&self) -> String {
        self.0.get().clone()
    }
}

impl_wasm_list!(cml_cip25::CIP25String64, CIP25String64, CIP25String64List);
