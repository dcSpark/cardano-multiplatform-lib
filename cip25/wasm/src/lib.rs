#![allow(
    clippy::len_without_is_empty,
    clippy::too_many_arguments,
    clippy::new_without_default
)]

pub mod utils;

pub use utils::LabelMetadata;

pub use core::CIP25Version;

use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

/// This is the entire metadata schema for CIP-25
/// It can be parsed by passing in the CBOR bytes of the entire transaction metadata
/// or by passing in an existing Metadata struct.
/// Parsing from CBOR bytes should be marginally faster.
#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct CIP25Metadata(pub(crate) core::CIP25Metadata);

#[wasm_bindgen]
impl CIP25Metadata {
    /// Serialize to CBOR bytes compatible with tx metadata
    /// Does not guarantee any specific type of CBOR format and should NOT
    /// be used with round-tripping. It will ignore all non-CIP25 keys.
    /// Use core::metadate crate for round-tripping metadata.
    pub fn to_bytes(&self) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0)
    }

    /// Deserialize from CBOR bytes compatible with tx metadata
    /// Does not guarantee any specific type of CBOR format and should NOT
    /// be used with round-tripping. It will ignore all non-CIP25 keys.
    /// Use core::metadate crate for round-tripping metadata.
    pub fn from_bytes(data: Vec<u8>) -> Result<CIP25Metadata, JsValue> {
        use core::serialization::FromBytes;
        FromBytes::from_bytes(data)
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

    pub fn from_json(json: &str) -> Result<CIP25Metadata, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    /// The core details of the CIP25 spec
    pub fn key_721(&self) -> LabelMetadata {
        self.0.key_721.clone().into()
    }

    pub fn new(key_721: &LabelMetadata) -> Self {
        Self(core::CIP25Metadata::new(key_721.clone().into()))
    }
}

impl From<core::CIP25Metadata> for CIP25Metadata {
    fn from(native: core::CIP25Metadata) -> Self {
        Self(native)
    }
}

impl From<CIP25Metadata> for core::CIP25Metadata {
    fn from(wasm: CIP25Metadata) -> Self {
        wasm.0
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
pub struct ChunkableString(pub(crate) core::ChunkableString);

#[wasm_bindgen]
impl ChunkableString {
    pub fn to_bytes(&self) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<ChunkableString, JsValue> {
        use core::serialization::FromBytes;
        FromBytes::from_bytes(data)
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

    pub fn from_json(json: &str) -> Result<ChunkableString, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn new_single(single: &String64) -> Self {
        Self(core::ChunkableString::new_single(single.clone().into()))
    }

    pub fn new_chunked(chunked: &String64s) -> Self {
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

    pub fn as_chunked(&self) -> Option<String64s> {
        match &self.0 {
            core::ChunkableString::Chunked(chunked) => Some(chunked.clone().into()),
            _ => None,
        }
    }
}

impl From<core::ChunkableString> for ChunkableString {
    fn from(native: core::ChunkableString) -> Self {
        Self(native)
    }
}

impl From<ChunkableString> for core::ChunkableString {
    fn from(wasm: ChunkableString) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]
pub enum ChunkableStringKind {
    Single,
    Chunked,
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct FilesDetails(pub(crate) core::FilesDetails);

#[wasm_bindgen]
impl FilesDetails {
    pub fn to_bytes(&self) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<FilesDetails, JsValue> {
        use core::serialization::FromBytes;
        FromBytes::from_bytes(data)
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

    pub fn from_json(json: &str) -> Result<FilesDetails, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

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

impl From<core::FilesDetails> for FilesDetails {
    fn from(native: core::FilesDetails) -> Self {
        Self(native)
    }
}

impl From<FilesDetails> for core::FilesDetails {
    fn from(wasm: FilesDetails) -> Self {
        wasm.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct FilesDetailss(pub(crate) Vec<core::FilesDetails>);

#[wasm_bindgen]
impl FilesDetailss {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> FilesDetails {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &FilesDetails) {
        self.0.push(elem.clone().into());
    }
}

impl From<Vec<core::FilesDetails>> for FilesDetailss {
    fn from(native: Vec<core::FilesDetails>) -> Self {
        Self(native)
    }
}

impl From<FilesDetailss> for Vec<core::FilesDetails> {
    fn from(wrapper: FilesDetailss) -> Self {
        wrapper.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MetadataDetails(pub(crate) core::MetadataDetails);

#[wasm_bindgen]
impl MetadataDetails {
    pub fn to_bytes(&self) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<MetadataDetails, JsValue> {
        use core::serialization::FromBytes;
        FromBytes::from_bytes(data)
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

    pub fn from_json(json: &str) -> Result<MetadataDetails, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

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

    pub fn set_files(&mut self, files: &FilesDetailss) {
        self.0.files = Some(files.clone().into())
    }

    pub fn files(&self) -> Option<FilesDetailss> {
        self.0.files.clone().map(std::convert::Into::into)
    }

    pub fn new(name: &String64, image: &ChunkableString) -> Self {
        Self(core::MetadataDetails::new(
            name.clone().into(),
            image.clone().into(),
        ))
    }
}

impl From<core::MetadataDetails> for MetadataDetails {
    fn from(native: core::MetadataDetails) -> Self {
        Self(native)
    }
}

impl From<MetadataDetails> for core::MetadataDetails {
    fn from(wasm: MetadataDetails) -> Self {
        wasm.0
    }
}

/// A String of at most 64 bytes.
/// This is to conform with Cardano metadata restrictions.
#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct String64(pub(crate) core::String64);

#[wasm_bindgen]
impl String64 {
    pub fn to_bytes(&self) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<String64, JsValue> {
        use core::serialization::FromBytes;
        FromBytes::from_bytes(data)
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

    pub fn from_json(json: &str) -> Result<String64, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn get(&self) -> String {
        self.0.get().clone()
    }
}

impl From<core::String64> for String64 {
    fn from(native: core::String64) -> Self {
        Self(native)
    }
}

impl From<String64> for core::String64 {
    fn from(wasm: String64) -> Self {
        wasm.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct String64s(pub(crate) Vec<core::String64>);

#[wasm_bindgen]
impl String64s {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> String64 {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &String64) {
        self.0.push(elem.clone().into());
    }
}

impl From<Vec<core::String64>> for String64s {
    fn from(native: Vec<core::String64>) -> Self {
        Self(native)
    }
}

impl From<String64s> for Vec<core::String64> {
    fn from(wrapper: String64s) -> Self {
        wrapper.0
    }
}
