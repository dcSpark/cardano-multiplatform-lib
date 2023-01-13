use wasm_bindgen::prelude::{wasm_bindgen, JsError, JsValue};

use cardano_multiplatform_lib_core_wasm::metadata::Metadata;

// TODO: remove after regen
use core::{ToBytes, FromBytes};

use std::collections::BTreeMap;

pub type AssetNameV1 = String64;

pub type AssetNameV2 = Vec<u8>;

pub type PolicyIdV1 = String64;

pub type PolicyIdV2 = Vec<u8>;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct MapAssetNameV2ToMetadataDetails(BTreeMap<core::AssetNameV2, core::MetadataDetails>);

#[wasm_bindgen]
impl MapAssetNameV2ToMetadataDetails {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, key: AssetNameV2, value: &MetadataDetails) -> Option<MetadataDetails> {
        self.0.insert(key, value.clone().into()).map(|v| v.clone().into())
    }

    pub fn get(&self, key: AssetNameV2) -> Option<MetadataDetails> {
        self.0.get(&key).map(|v| v.clone().into())
    }

    pub fn keys(&self) -> AssetNameV2s {
        AssetNameV2s(self.0.iter().map(|(k, _v)| k.clone()).collect::<Vec<_>>())
    }
}

impl From<BTreeMap<core::AssetNameV2, core::MetadataDetails>> for MapAssetNameV2ToMetadataDetails {
    fn from(native: BTreeMap<core::AssetNameV2, core::MetadataDetails>) -> Self {
        Self(native)
    }
}

impl std::convert::Into<BTreeMap<core::AssetNameV2, core::MetadataDetails>> for MapAssetNameV2ToMetadataDetails {
    fn into(self) -> BTreeMap<core::AssetNameV2, core::MetadataDetails> {
        self.0
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct AssetNameV2s(Vec<core::AssetNameV2>);

#[wasm_bindgen]
impl AssetNameV2s {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> AssetNameV2 {
        self.0[index].clone()
    }

    pub fn add(&mut self, elem: AssetNameV2) {
        self.0.push(elem);
    }
}

impl From<Vec<core::AssetNameV2>> for AssetNameV2s {
    fn from(native: Vec<core::AssetNameV2>) -> Self {
        Self(native)
    }
}

impl std::convert::Into<Vec<core::AssetNameV2>> for AssetNameV2s {
    fn into(self) -> Vec<core::AssetNameV2> {
        self.0
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct String64s(Vec<core::String64>);

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

impl std::convert::Into<Vec<core::String64>> for String64s {
    fn into(self) -> Vec<core::String64> {
        self.0
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct FilesDetailss(Vec<core::FilesDetails>);

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

impl std::convert::Into<Vec<core::FilesDetails>> for FilesDetailss {
    fn into(self) -> Vec<core::FilesDetails> {
        self.0
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Data(BTreeMap<core::PolicyIdV2, BTreeMap<core::AssetNameV2, core::MetadataDetails>>);

#[wasm_bindgen]
impl Data {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, key: PolicyIdV2, value: &MapAssetNameV2ToMetadataDetails) -> Option<MapAssetNameV2ToMetadataDetails> {
        self.0.insert(key, value.clone().into()).map(|v| v.clone().into())
    }

    pub fn get(&self, key: PolicyIdV2) -> Option<MapAssetNameV2ToMetadataDetails> {
        self.0.get(&key).map(|v| v.clone().into())
    }

    pub fn keys(&self) -> PolicyIdV2s {
        PolicyIdV2s(self.0.iter().map(|(k, _v)| k.clone()).collect::<Vec<_>>())
    }
}

impl From<BTreeMap<core::PolicyIdV2, BTreeMap<core::AssetNameV2, core::MetadataDetails>>> for Data {
    fn from(native: BTreeMap<core::PolicyIdV2, BTreeMap<core::AssetNameV2, core::MetadataDetails>>) -> Self {
        Self(native)
    }
}

impl std::convert::Into<BTreeMap<core::PolicyIdV2, BTreeMap<core::AssetNameV2, core::MetadataDetails>>> for Data {
    fn into(self) -> BTreeMap<core::PolicyIdV2, BTreeMap<core::AssetNameV2, core::MetadataDetails>> {
        self.0
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct FilesDetails(core::FilesDetails);

#[wasm_bindgen]
impl FilesDetails {
    pub fn to_bytes(&self) -> Vec<u8> {
        ToBytes::to_bytes(&self.0)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<FilesDetails, JsError> {
        FromBytes::from_bytes(data).map(Self).map_err(Into::into)
    }

    pub fn to_json(&self) -> Result<String, JsError> {
        serde_json::to_string_pretty(&self.0).map_err(Into::into)
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsError> {
        serde_wasm_bindgen::to_value(&self.0).map_err(Into::into)
    }

    pub fn from_json(json: &str) -> Result<FilesDetails, JsError> {
        serde_json::from_str(json).map(Self).map_err(Into::into)
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
        Self(core::FilesDetails::new(name.clone().into(), media_type.clone().into(), src.clone().into()))
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

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct LabelMetadataV1(BTreeMap<core::PolicyIdV1, BTreeMap<core::AssetNameV1, core::MetadataDetails>>);

#[wasm_bindgen]
impl LabelMetadataV1 {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, key: &PolicyIdV1, value: &MapAssetNameV1ToMetadataDetails) -> Option<MapAssetNameV1ToMetadataDetails> {
        self.0.insert(key.clone().into(), value.clone().into()).map(|v| v.clone().into())
    }

    pub fn get(&self, key: &PolicyIdV1) -> Option<MapAssetNameV1ToMetadataDetails> {
        self.0.get(&key.0).map(|v| v.clone().into())
    }

    pub fn keys(&self) -> PolicyIdV1s {
        PolicyIdV1s(self.0.iter().map(|(k, _v)| k.clone()).collect::<Vec<_>>())
    }
}

impl From<BTreeMap<core::PolicyIdV1, BTreeMap<core::AssetNameV1, core::MetadataDetails>>> for LabelMetadataV1 {
    fn from(native: BTreeMap<core::PolicyIdV1, BTreeMap<core::AssetNameV1, core::MetadataDetails>>) -> Self {
        Self(native)
    }
}

impl std::convert::Into<BTreeMap<core::PolicyIdV1, BTreeMap<core::AssetNameV1, core::MetadataDetails>>> for LabelMetadataV1 {
    fn into(self) -> BTreeMap<core::PolicyIdV1, BTreeMap<core::AssetNameV1, core::MetadataDetails>> {
        self.0
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct PolicyIdV1s(Vec<core::PolicyIdV1>);

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

impl std::convert::Into<Vec<core::PolicyIdV1>> for PolicyIdV1s {
    fn into(self) -> Vec<core::PolicyIdV1> {
        self.0
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct MapAssetNameV1ToMetadataDetails(BTreeMap<core::AssetNameV1, core::MetadataDetails>);

#[wasm_bindgen]
impl MapAssetNameV1ToMetadataDetails {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, key: &AssetNameV1, value: &MetadataDetails) -> Option<MetadataDetails> {
        self.0.insert(key.clone().into(), value.clone().into()).map(|v| v.clone().into())
    }

    pub fn get(&self, key: &AssetNameV1) -> Option<MetadataDetails> {
        self.0.get(&key.0).map(|v| v.clone().into())
    }

    pub fn keys(&self) -> AssetNameV1s {
        AssetNameV1s(self.0.iter().map(|(k, _v)| k.clone()).collect::<Vec<_>>())
    }
}

impl From<BTreeMap<core::AssetNameV1, core::MetadataDetails>> for MapAssetNameV1ToMetadataDetails {
    fn from(native: BTreeMap<core::AssetNameV1, core::MetadataDetails>) -> Self {
        Self(native)
    }
}

impl std::convert::Into<BTreeMap<core::AssetNameV1, core::MetadataDetails>> for MapAssetNameV1ToMetadataDetails {
    fn into(self) -> BTreeMap<core::AssetNameV1, core::MetadataDetails> {
        self.0
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct AssetNameV1s(Vec<core::AssetNameV1>);

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

impl std::convert::Into<Vec<core::AssetNameV1>> for AssetNameV1s {
    fn into(self) -> Vec<core::AssetNameV1> {
        self.0
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct PolicyIdV2s(Vec<core::PolicyIdV2>);

#[wasm_bindgen]
impl PolicyIdV2s {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> PolicyIdV2 {
        self.0[index].clone()
    }

    pub fn add(&mut self, elem: PolicyIdV2) {
        self.0.push(elem);
    }
}

impl From<Vec<core::PolicyIdV2>> for PolicyIdV2s {
    fn from(native: Vec<core::PolicyIdV2>) -> Self {
        Self(native)
    }
}

impl std::convert::Into<Vec<core::PolicyIdV2>> for PolicyIdV2s {
    fn into(self) -> Vec<core::PolicyIdV2> {
        self.0
    }
}

#[wasm_bindgen]
pub enum LabelMetadataKind {
    LabelMetadataV1,
    LabelMetadataV2,
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct LabelMetadata(core::LabelMetadata);

#[wasm_bindgen]
impl LabelMetadata {
    pub fn to_bytes(&self) -> Vec<u8> {
        ToBytes::to_bytes(&self.0)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<LabelMetadata, JsError> {
        FromBytes::from_bytes(data).map(Self).map_err(Into::into)
    }

    pub fn to_json(&self) -> Result<String, JsError> {
        serde_json::to_string_pretty(&self.0).map_err(Into::into)
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsError> {
        serde_wasm_bindgen::to_value(&self.0).map_err(Into::into)
    }

    pub fn from_json(json: &str) -> Result<LabelMetadata, JsError> {
        serde_json::from_str(json).map(Self).map_err(Into::into)
    }

    pub fn new_label_metadata_v1(label_metadata_v1: LabelMetadataV1) -> Self {
        Self(core::LabelMetadata::new_label_metadata_v1(label_metadata_v1.clone().into()))
    }

    pub fn new_label_metadata_v2(label_metadata_v2: &LabelMetadataV2) -> Self {
        Self(core::LabelMetadata::new_label_metadata_v2(label_metadata_v2.clone().into()))
    }

    pub fn kind(&self) -> LabelMetadataKind {
        match &self.0 {
            core::LabelMetadata::LabelMetadataV1(_) => LabelMetadataKind::LabelMetadataV1,
            core::LabelMetadata::LabelMetadataV2(_) => LabelMetadataKind::LabelMetadataV2,
        }
    }

    pub fn as_label_metadata_v1(&self) -> Option<LabelMetadataV1> {
        match &self.0 {
            core::LabelMetadata::LabelMetadataV1(label_metadata_v1) => Some(label_metadata_v1.clone().into()),
            _ => None,
        }
    }

    pub fn as_label_metadata_v2(&self) -> Option<LabelMetadataV2> {
        match &self.0 {
            core::LabelMetadata::LabelMetadataV2(label_metadata_v2) => Some(label_metadata_v2.clone().into()),
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
#[derive(Clone, Debug)]
pub struct LabelMetadataV2(core::LabelMetadataV2);

#[wasm_bindgen]
impl LabelMetadataV2 {
    pub fn to_bytes(&self) -> Vec<u8> {
        ToBytes::to_bytes(&self.0)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<LabelMetadataV2, JsError> {
        FromBytes::from_bytes(data).map(Self).map_err(Into::into)
    }

    pub fn to_json(&self) -> Result<String, JsError> {
        serde_json::to_string_pretty(&self.0).map_err(Into::into)
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsError> {
        serde_wasm_bindgen::to_value(&self.0).map_err(Into::into)
    }

    pub fn from_json(json: &str) -> Result<LabelMetadataV2, JsError> {
        serde_json::from_str(json).map(Self).map_err(Into::into)
    }

    pub fn data(&self) -> Data {
        self.0.data.clone().into()
    }

    pub fn new(data: Data) -> Self {
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

/// This is the entire metadata schema for CIP-25
/// It can be parsed by passing in the CBOR bytes of the entire transaction metadata
/// or by passing in an existing Metadata struct.
/// Parsing from CBOR bytes should be marginally faster.
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct CIP25Metadata(core::CIP25Metadata);

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

    /// Serialize to CBOR bytes compatible with tx metadata
    /// Does not guarantee any specific type of CBOR format and should NOT
    /// be used with round-tripping. It will ignore all non-CIP25 keys.
    /// Use core::metadate crate for round-tripping metadata.
    pub fn to_bytes(&self) -> Vec<u8> {
        ToBytes::to_bytes(&self.0)
    }

    /// Deserialize from CBOR bytes compatible with tx metadata
    /// Does not guarantee any specific type of CBOR format and should NOT
    /// be used with round-tripping. It will ignore all non-CIP25 keys.
    /// Use core::metadate crate for round-tripping metadata.
    pub fn from_bytes(data: Vec<u8>) -> Result<CIP25Metadata, JsError> {
        FromBytes::from_bytes(data).map(Self).map_err(Into::into)
    }

    pub fn to_json(&self) -> Result<String, JsError> {
        serde_json::to_string_pretty(&self.0).map_err(Into::into)
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsError> {
        serde_wasm_bindgen::to_value(&self.0).map_err(Into::into)
    }

    pub fn from_json(json: &str) -> Result<CIP25Metadata, JsError> {
        serde_json::from_str(json).map(Self).map_err(Into::into)
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

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct MetadataDetails(core::MetadataDetails);

#[wasm_bindgen]
impl MetadataDetails {
    pub fn to_bytes(&self) -> Vec<u8> {
        ToBytes::to_bytes(&self.0)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<MetadataDetails, JsError> {
        FromBytes::from_bytes(data).map(Self).map_err(Into::into)
    }

    pub fn to_json(&self) -> Result<String, JsError> {
        serde_json::to_string_pretty(&self.0).map_err(Into::into)
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsError> {
        serde_wasm_bindgen::to_value(&self.0).map_err(Into::into)
    }

    pub fn from_json(json: &str) -> Result<MetadataDetails, JsError> {
        serde_json::from_str(json).map(Self).map_err(Into::into)
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
        Self(core::MetadataDetails::new(name.clone().into(), image.clone().into()))
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
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct String64(core::String64);

#[wasm_bindgen]
impl String64 {
    pub fn new(s: &str) -> Result<String64, JsError> {
        core::String64::new(s).map(Self).map_err(Into::into)
    }

    pub fn to_str(&self) -> String {
        self.0.to_str().to_owned()
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        ToBytes::to_bytes(&self.0)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<String64, JsError> {
        FromBytes::from_bytes(data).map(Self).map_err(Into::into)
    }

    pub fn to_json(&self) -> Result<String, JsError> {
        serde_json::to_string_pretty(&self.0).map_err(Into::into)
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsError> {
        serde_wasm_bindgen::to_value(&self.0).map_err(Into::into)
    }

    pub fn from_json(json: &str) -> Result<String64, JsError> {
        serde_json::from_str(json).map(Self).map_err(Into::into)
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

#[wasm_bindgen]
pub enum ChunkableStringKind {
    Single,
    Chunked,
}

/// A String that may or may not be chunked into 64-byte chunks to be able
/// to conform to Cardano TX Metadata limitations.
/// Most users should simply use ChunkableString::from_string() and ChunkableString::to_string()
/// and avoid the explicit single/chunk interface:
/// ```javascript
/// let chunkableString = CIP25.ChunkableString.from_string("this can be any length and will automatically be chunked if needed");
/// ```
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct ChunkableString(core::ChunkableString);

#[wasm_bindgen]
impl ChunkableString {
    pub fn from_string(str: &str) -> Self {
        Self(core::ChunkableString::from(str))
    }

    pub fn to_string(&self) -> String {
        String::from(&self.0)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        ToBytes::to_bytes(&self.0)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<ChunkableString, JsError> {
        FromBytes::from_bytes(data).map(Self).map_err(Into::into)
    }

    pub fn to_json(&self) -> Result<String, JsError> {
        serde_json::to_string_pretty(&self.0).map_err(Into::into)
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsError> {
        serde_wasm_bindgen::to_value(&self.0).map_err(Into::into)
    }

    pub fn from_json(json: &str) -> Result<ChunkableString, JsError> {
        serde_json::from_str(json).map(Self).map_err(Into::into)
    }

    pub fn new_single(string64: &String64) -> Self {
        Self(core::ChunkableString::new_single(string64.clone().into()))
    }

    pub fn new_chunked(arr_string64: &String64s) -> Self {
        Self(core::ChunkableString::new_chunked(arr_string64.clone().into()))
    }

    pub fn kind(&self) -> ChunkableStringKind {
        match &self.0 {
            core::ChunkableString::Single(_) => ChunkableStringKind::Single,
            core::ChunkableString::Chunked(_) => ChunkableStringKind::Chunked,
        }
    }

    pub fn as_string64(&self) -> Option<String64> {
        match &self.0 {
            core::ChunkableString::Single(string64) => Some(string64.clone().into()),
            _ => None,
        }
    }

    pub fn as_chunks(&self) -> Option<String64s> {
        match &self.0 {
            core::ChunkableString::Chunked(arr_string64) => Some(arr_string64.clone().into()),
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