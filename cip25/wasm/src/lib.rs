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
pub use utils::LabelMetadata;

pub use core::CIP25Version;

use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

/// This is the entire metadata schema for CIP-25
/// It can be parsed by passing in the CBOR bytes of the entire transaction metadata
/// or by passing in an existing Metadata struct.
/// Parsing from CBOR bytes should be marginally faster.
#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct CIP25Metadata(core::CIP25Metadata);

impl_wasm_conversions!(core::CIP25Metadata, CIP25Metadata);

// we manually write to_cbor_bytes/from_cbor_bytes so we can add the comments

impl_wasm_json_api!(CIP25Metadata);

#[wasm_bindgen]
impl CIP25Metadata {
    /// Serialize to CBOR bytes compatible with tx metadata
    /// Does not guarantee any specific type of CBOR format and should NOT
    /// be used with round-tripping. It will ignore all non-CIP25 keys.
    /// Use core::metadate crate for round-tripping metadata.
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0)
    }

    /// Deserialize from CBOR bytes compatible with tx metadata
    /// Does not guarantee any specific type of CBOR format and should NOT
    /// be used with round-tripping. It will ignore all non-CIP25 keys.
    /// Use core::metadate crate for round-tripping metadata.
    pub fn from_cbor_bytes(data: Vec<u8>) -> Result<CIP25Metadata, JsValue> {
        use core::serialization::FromBytes;
        FromBytes::from_bytes(data)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_cbor_bytes: {e}")))
    }

    /// The core details of the CIP25 spec
    pub fn key_721(&self) -> LabelMetadata {
        self.0.key_721.clone().into()
    }

    pub fn new(key_721: &LabelMetadata) -> Self {
        Self(core::CIP25Metadata::new(key_721.clone().into()))
    }
}

/// A String that may or may not be chunked into 64-byte chunks to be able
/// to conform to Cardano TX Metadata limitations.
/// Most users should simply use ChunkableString::from_string() and ChunkableString::to_string()
/// and avoid the explicit single/chunk interface:
/// ```javascript
/// let chunkableString = CIP25.ChunkableString.from_string("this can be any length and will automatically be chunked if needed");
/// ```
#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ChunkableString(core::ChunkableString);

impl_wasm_conversions!(core::ChunkableString, ChunkableString);

impl_wasm_cbor_json_api_cbor_event_serialize!(ChunkableString);

#[wasm_bindgen]
impl ChunkableString {
    pub fn new_single(single: &String64) -> Self {
        Self(core::ChunkableString::new_single(single.clone().into()))
    }

    pub fn new_chunked(chunked: &String64List) -> Self {
        Self(core::ChunkableString::new_chunked(chunked.clone().into()))
    }

    pub fn kind(&self) -> ChunkableStringKind {
        match &self.0 {
            core::ChunkableString::Single(_) => ChunkableStringKind::Single,
            core::ChunkableString::Chunked(_) => ChunkableStringKind::Chunked,
        }
    }

    pub fn as_single(&self) -> Option<String64> {
        match &self.0 {
            core::ChunkableString::Single(single) => Some(single.clone().into()),
            _ => None,
        }
    }

    pub fn as_chunked(&self) -> Option<String64List> {
        match &self.0 {
            core::ChunkableString::Chunked(chunked) => Some(chunked.clone().into()),
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
pub struct FilesDetails(core::FilesDetails);

impl_wasm_conversions!(core::FilesDetails, FilesDetails);

impl_wasm_cbor_json_api_cbor_event_serialize!(FilesDetails);

#[wasm_bindgen]
impl FilesDetails {
    pub fn name(&self) -> String64 {
        self.0.name.clone().into()
    }

    pub fn media_type(&self) -> String64 {
        self.0.media_type.clone().into()
    }

    pub fn src(&self) -> ChunkableString {
        self.0.src.clone().into()
    }

    pub fn new(name: &String64, media_type: &String64, src: &ChunkableString) -> Self {
        Self(core::FilesDetails::new(
            name.clone().into(),
            media_type.clone().into(),
            src.clone().into(),
        ))
    }
}

impl_wasm_list!(core::FilesDetails, FilesDetails, FilesDetailsList);

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MetadataDetails(core::MetadataDetails);

impl_wasm_conversions!(core::MetadataDetails, MetadataDetails);

impl_wasm_cbor_json_api_cbor_event_serialize!(MetadataDetails);

#[wasm_bindgen]
impl MetadataDetails {
    pub fn name(&self) -> String64 {
        self.0.name.clone().into()
    }

    pub fn image(&self) -> ChunkableString {
        self.0.image.clone().into()
    }

    pub fn set_media_type(&mut self, media_type: &String64) {
        self.0.media_type = Some(media_type.clone().into())
    }

    pub fn media_type(&self) -> Option<String64> {
        self.0.media_type.clone().map(std::convert::Into::into)
    }

    pub fn set_description(&mut self, description: &ChunkableString) {
        self.0.description = Some(description.clone().into())
    }

    pub fn description(&self) -> Option<ChunkableString> {
        self.0.description.clone().map(std::convert::Into::into)
    }

    pub fn set_files(&mut self, files: &FilesDetailsList) {
        self.0.files = Some(files.clone().into())
    }

    pub fn files(&self) -> Option<FilesDetailsList> {
        self.0.files.clone().map(std::convert::Into::into)
    }

    pub fn new(name: &String64, image: &ChunkableString) -> Self {
        Self(core::MetadataDetails::new(
            name.clone().into(),
            image.clone().into(),
        ))
    }
}

/// A String of at most 64 bytes.
/// This is to conform with Cardano metadata restrictions.
#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct String64(core::String64);

impl_wasm_conversions!(core::String64, String64);

impl_wasm_cbor_json_api_cbor_event_serialize!(String64);

#[wasm_bindgen]
impl String64 {
    pub fn get(&self) -> String {
        self.0.get().clone()
    }
}

impl_wasm_list!(core::String64, String64, String64List);
