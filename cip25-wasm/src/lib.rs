#![allow(
    clippy::len_without_is_empty,
    clippy::too_many_arguments,
    clippy::new_without_default
)]

pub mod utils;

use std::collections::BTreeMap;
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct AssetNameV1(pub(crate) core::AssetNameV1);

#[wasm_bindgen]
impl AssetNameV1 {
    pub fn to_bytes(&self) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<AssetNameV1, JsValue> {
        use core::prelude::FromBytes;
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

    pub fn from_json(json: &str) -> Result<AssetNameV1, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn get(&self) -> String64 {
        self.0.get().clone().into()
    }
}

impl From<core::AssetNameV1> for AssetNameV1 {
    fn from(native: core::AssetNameV1) -> Self {
        Self(native)
    }
}

impl From<AssetNameV1> for core::AssetNameV1 {
    fn from(wasm: AssetNameV1) -> Self {
        wasm.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct AssetNameV1s(pub(crate) Vec<core::AssetNameV1>);

#[wasm_bindgen]
impl AssetNameV1s {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> AssetNameV1 {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &AssetNameV1) {
        self.0.push(elem.clone().into());
    }
}

impl From<Vec<core::AssetNameV1>> for AssetNameV1s {
    fn from(native: Vec<core::AssetNameV1>) -> Self {
        Self(native)
    }
}

impl From<AssetNameV1s> for Vec<core::AssetNameV1> {
    fn from(wrapper: AssetNameV1s) -> Self {
        wrapper.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct AssetNameV2(pub(crate) core::AssetNameV2);

#[wasm_bindgen]
impl AssetNameV2 {
    pub fn to_bytes(&self) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<AssetNameV2, JsValue> {
        use core::prelude::FromBytes;
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

    pub fn from_json(json: &str) -> Result<AssetNameV2, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn get(&self) -> Vec<u8> {
        self.0.get().clone()
    }
}

impl From<core::AssetNameV2> for AssetNameV2 {
    fn from(native: core::AssetNameV2) -> Self {
        Self(native)
    }
}

impl From<AssetNameV2> for core::AssetNameV2 {
    fn from(wasm: AssetNameV2) -> Self {
        wasm.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct AssetNameV2s(pub(crate) Vec<core::AssetNameV2>);

#[wasm_bindgen]
impl AssetNameV2s {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> AssetNameV2 {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &AssetNameV2) {
        self.0.push(elem.clone().into());
    }
}

impl From<Vec<core::AssetNameV2>> for AssetNameV2s {
    fn from(native: Vec<core::AssetNameV2>) -> Self {
        Self(native)
    }
}

impl From<AssetNameV2s> for Vec<core::AssetNameV2> {
    fn from(wrapper: AssetNameV2s) -> Self {
        wrapper.0
    }
}

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
        use core::prelude::FromBytes;
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
        use core::prelude::FromBytes;
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
pub struct Data(pub(crate) core::Data);

impl From<core::Data> for Data {
    fn from(native: core::Data) -> Self {
        Self(native)
    }
}

impl From<Data> for core::Data {
    fn from(wrapper: Data) -> Self {
        wrapper.0
    }
}

#[wasm_bindgen]
impl Data {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(
        &mut self,
        key: &PolicyIdV2,
        value: &MapAssetNameV2ToMetadataDetails,
    ) -> Option<MapAssetNameV2ToMetadataDetails> {
        self.0
            .insert(key.clone().into(), value.clone().into())
            .map(Into::into)
    }

    pub fn get(&self, key: &PolicyIdV2) -> Option<MapAssetNameV2ToMetadataDetails> {
        self.0.get(&key.0).map(|v| v.clone().into())
    }

    pub fn keys(&self) -> PolicyIdV2s {
        PolicyIdV2s(self.0.iter().map(|(k, _v)| k.clone()).collect::<Vec<_>>())
    }
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
        use core::prelude::FromBytes;
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
pub struct LabelMetadata(pub(crate) core::LabelMetadata);

#[wasm_bindgen]
impl LabelMetadata {
    pub fn to_bytes(&self) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<LabelMetadata, JsValue> {
        use core::prelude::FromBytes;
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

    pub fn from_json(json: &str) -> Result<LabelMetadata, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn new_label_metadata_v1(label_metadata_v1: &LabelMetadataV1) -> Self {
        Self(core::LabelMetadata::new_label_metadata_v1(
            label_metadata_v1.clone().into(),
        ))
    }

    pub fn new_label_metadata_v2(label_metadata_v2: &LabelMetadataV2) -> Self {
        Self(core::LabelMetadata::new_label_metadata_v2(
            label_metadata_v2.clone().into(),
        ))
    }

    pub fn kind(&self) -> LabelMetadataKind {
        match &self.0 {
            core::LabelMetadata::LabelMetadataV1(_) => LabelMetadataKind::LabelMetadataV1,
            core::LabelMetadata::LabelMetadataV2(_) => LabelMetadataKind::LabelMetadataV2,
        }
    }

    pub fn as_label_metadata_v1(&self) -> Option<LabelMetadataV1> {
        match &self.0 {
            core::LabelMetadata::LabelMetadataV1(label_metadata_v1) => {
                Some(label_metadata_v1.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_label_metadata_v2(&self) -> Option<LabelMetadataV2> {
        match &self.0 {
            core::LabelMetadata::LabelMetadataV2(label_metadata_v2) => {
                Some(label_metadata_v2.clone().into())
            }
            _ => None,
        }
    }
}

impl From<core::LabelMetadata> for LabelMetadata {
    fn from(native: core::LabelMetadata) -> Self {
        Self(native)
    }
}

impl From<LabelMetadata> for core::LabelMetadata {
    fn from(wasm: LabelMetadata) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]
pub enum LabelMetadataKind {
    LabelMetadataV1,
    LabelMetadataV2,
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct LabelMetadataV1(pub(crate) core::LabelMetadataV1);

impl From<core::LabelMetadataV1> for LabelMetadataV1 {
    fn from(native: core::LabelMetadataV1) -> Self {
        Self(native)
    }
}

impl From<LabelMetadataV1> for core::LabelMetadataV1 {
    fn from(wrapper: LabelMetadataV1) -> Self {
        wrapper.0
    }
}

#[wasm_bindgen]
impl LabelMetadataV1 {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(
        &mut self,
        key: &PolicyIdV1,
        value: &MapAssetNameV1ToMetadataDetails,
    ) -> Option<MapAssetNameV1ToMetadataDetails> {
        self.0
            .insert(key.clone().into(), value.clone().into())
            .map(Into::into)
    }

    pub fn get(&self, key: &PolicyIdV1) -> Option<MapAssetNameV1ToMetadataDetails> {
        self.0.get(&key.0).map(|v| v.clone().into())
    }

    pub fn keys(&self) -> PolicyIdV1s {
        PolicyIdV1s(self.0.iter().map(|(k, _v)| k.clone()).collect::<Vec<_>>())
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct LabelMetadataV2(pub(crate) core::LabelMetadataV2);

#[wasm_bindgen]
impl LabelMetadataV2 {
    pub fn to_bytes(&self) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<LabelMetadataV2, JsValue> {
        use core::prelude::FromBytes;
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

    pub fn from_json(json: &str) -> Result<LabelMetadataV2, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn data(&self) -> Data {
        self.0.data.clone().into()
    }

    pub fn new(data: &Data) -> Self {
        Self(core::LabelMetadataV2::new(data.clone().into()))
    }
}

impl From<core::LabelMetadataV2> for LabelMetadataV2 {
    fn from(native: core::LabelMetadataV2) -> Self {
        Self(native)
    }
}

impl From<LabelMetadataV2> for core::LabelMetadataV2 {
    fn from(wasm: LabelMetadataV2) -> Self {
        wasm.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MapAssetNameV1ToMetadataDetails(
    pub(crate) BTreeMap<core::AssetNameV1, core::MetadataDetails>,
);

impl From<BTreeMap<core::AssetNameV1, core::MetadataDetails>> for MapAssetNameV1ToMetadataDetails {
    fn from(native: BTreeMap<core::AssetNameV1, core::MetadataDetails>) -> Self {
        Self(native)
    }
}

impl From<MapAssetNameV1ToMetadataDetails> for BTreeMap<core::AssetNameV1, core::MetadataDetails> {
    fn from(wrapper: MapAssetNameV1ToMetadataDetails) -> Self {
        wrapper.0
    }
}

#[wasm_bindgen]
impl MapAssetNameV1ToMetadataDetails {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(
        &mut self,
        key: &AssetNameV1,
        value: &MetadataDetails,
    ) -> Option<MetadataDetails> {
        self.0
            .insert(key.clone().into(), value.clone().into())
            .map(Into::into)
    }

    pub fn get(&self, key: &AssetNameV1) -> Option<MetadataDetails> {
        self.0.get(&key.0).map(|v| v.clone().into())
    }

    pub fn keys(&self) -> AssetNameV1s {
        AssetNameV1s(self.0.iter().map(|(k, _v)| k.clone()).collect::<Vec<_>>())
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MapAssetNameV2ToMetadataDetails(
    pub(crate) BTreeMap<core::AssetNameV2, core::MetadataDetails>,
);

impl From<BTreeMap<core::AssetNameV2, core::MetadataDetails>> for MapAssetNameV2ToMetadataDetails {
    fn from(native: BTreeMap<core::AssetNameV2, core::MetadataDetails>) -> Self {
        Self(native)
    }
}

impl From<MapAssetNameV2ToMetadataDetails> for BTreeMap<core::AssetNameV2, core::MetadataDetails> {
    fn from(wrapper: MapAssetNameV2ToMetadataDetails) -> Self {
        wrapper.0
    }
}

#[wasm_bindgen]
impl MapAssetNameV2ToMetadataDetails {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(
        &mut self,
        key: &AssetNameV2,
        value: &MetadataDetails,
    ) -> Option<MetadataDetails> {
        self.0
            .insert(key.clone().into(), value.clone().into())
            .map(Into::into)
    }

    pub fn get(&self, key: &AssetNameV2) -> Option<MetadataDetails> {
        self.0.get(&key.0).map(|v| v.clone().into())
    }

    pub fn keys(&self) -> AssetNameV2s {
        AssetNameV2s(self.0.iter().map(|(k, _v)| k.clone()).collect::<Vec<_>>())
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
        use core::prelude::FromBytes;
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

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct PolicyIdV1(pub(crate) core::PolicyIdV1);

#[wasm_bindgen]
impl PolicyIdV1 {
    pub fn to_bytes(&self) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<PolicyIdV1, JsValue> {
        use core::prelude::FromBytes;
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

    pub fn from_json(json: &str) -> Result<PolicyIdV1, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn get(&self) -> String64 {
        self.0.get().clone().into()
    }
}

impl From<core::PolicyIdV1> for PolicyIdV1 {
    fn from(native: core::PolicyIdV1) -> Self {
        Self(native)
    }
}

impl From<PolicyIdV1> for core::PolicyIdV1 {
    fn from(wasm: PolicyIdV1) -> Self {
        wasm.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct PolicyIdV1s(pub(crate) Vec<core::PolicyIdV1>);

#[wasm_bindgen]
impl PolicyIdV1s {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> PolicyIdV1 {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &PolicyIdV1) {
        self.0.push(elem.clone().into());
    }
}

impl From<Vec<core::PolicyIdV1>> for PolicyIdV1s {
    fn from(native: Vec<core::PolicyIdV1>) -> Self {
        Self(native)
    }
}

impl From<PolicyIdV1s> for Vec<core::PolicyIdV1> {
    fn from(wrapper: PolicyIdV1s) -> Self {
        wrapper.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct PolicyIdV2(pub(crate) core::PolicyIdV2);

#[wasm_bindgen]
impl PolicyIdV2 {
    pub fn to_bytes(&self) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<PolicyIdV2, JsValue> {
        use core::prelude::FromBytes;
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

    pub fn from_json(json: &str) -> Result<PolicyIdV2, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn get(&self) -> Vec<u8> {
        self.0.get().clone()
    }
}

impl From<core::PolicyIdV2> for PolicyIdV2 {
    fn from(native: core::PolicyIdV2) -> Self {
        Self(native)
    }
}

impl From<PolicyIdV2> for core::PolicyIdV2 {
    fn from(wasm: PolicyIdV2) -> Self {
        wasm.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct PolicyIdV2s(pub(crate) Vec<core::PolicyIdV2>);

#[wasm_bindgen]
impl PolicyIdV2s {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> PolicyIdV2 {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &PolicyIdV2) {
        self.0.push(elem.clone().into());
    }
}

impl From<Vec<core::PolicyIdV2>> for PolicyIdV2s {
    fn from(native: Vec<core::PolicyIdV2>) -> Self {
        Self(native)
    }
}

impl From<PolicyIdV2s> for Vec<core::PolicyIdV2> {
    fn from(wrapper: PolicyIdV2s) -> Self {
        wrapper.0
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
        use core::prelude::FromBytes;
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
