use std::io::{BufRead, Write};

pub use cardano_multiplatform_lib_core::{
    serialization::*,
    error::*,
    metadata::{Metadata, TransactionMetadatum},
};

// This library was code-generated using an experimental CDDL to rust tool:
// https://github.com/Emurgo/cddl-codegen

use cbor_event::{self, de::Deserializer, se::Serializer};

use cbor_event::Type as CBORType;

use cbor_event::Special as CBORSpecial;


pub mod serialization;

use std::collections::BTreeMap;

use std::convert::{From, TryFrom};

pub type AssetNameV1 = String64;

pub type AssetNameV2 = Vec<u8>;

pub type Data = BTreeMap<PolicyIdV2, BTreeMap<AssetNameV2, MetadataDetails>>;

pub type LabelMetadataV1 = BTreeMap<PolicyIdV1, BTreeMap<AssetNameV1, MetadataDetails>>;

pub type PolicyIdV1 = String64;

pub type PolicyIdV1s = Vec<PolicyIdV1>;

pub type PolicyIdV2 = Vec<u8>;

pub type PolicyIdV2s = Vec<PolicyIdV2>;

pub static CIP25_METADATA_LABEL: u64 = 721;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct FilesDetails {
    pub name: String64,
    pub media_type: String64,
    pub src: ChunkableString,
}

impl FilesDetails {
    pub fn new(name: String64, media_type: String64, src: ChunkableString) -> Self {
        Self {
            name,
            media_type,
            src,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub enum LabelMetadata {
    LabelMetadataV1(LabelMetadataV1),
    LabelMetadataV2(LabelMetadataV2),
}

impl LabelMetadata {
    pub fn new_label_metadata_v1(label_metadata_v1: LabelMetadataV1) -> Self {
        Self::LabelMetadataV1(label_metadata_v1)
    }

    pub fn new_label_metadata_v2(label_metadata_v2: LabelMetadataV2) -> Self {
        Self::LabelMetadataV2(label_metadata_v2)
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct LabelMetadataV2 {
    pub data: Data,
}

impl LabelMetadataV2 {
    pub fn new(data: Data) -> Self {
        Self {
            data,
        }
    }
}

/// This is the entire metadata schema for CIP-25
/// It can be parsed by passing in the CBOR bytes of the entire transaction metadata
/// or by passing in an existing Metadata struct.
/// Parsing from CBOR bytes should be marginally faster.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct CIP25Metadata {
    /// The core details of the CIP25 spec
    pub key_721: LabelMetadata,
}

impl CIP25Metadata {
    pub fn new(key_721: LabelMetadata) -> Self {
        Self {
            key_721,
        }
    }

    /// Create a Metadata containing only the CIP25 schema
    pub fn to_metadata(&self) -> Result<Metadata, DeserializeError> {
        use std::convert::TryInto;
        self.try_into()
    }

    /// Read the CIP25 schema from a Metadata. Ignores all other data besides CIP25
    /// Can fail if the Metadata does not conform to CIP25
    pub fn from_metadata(metadata: &Metadata) -> Result<CIP25Metadata, DeserializeError> {
        Self::try_from(metadata)
    }

    /// Add to an existing metadata (could be empty) the full CIP25 metadata
    pub fn add_to_metadata(&self, metadata: &mut Metadata) -> Result<(), DeserializeError> {
        let cip25_metadatum = TransactionMetadatum::from_cbor_bytes(&self.key_721.to_bytes())?;
        metadata.insert(CIP25_METADATA_LABEL, cip25_metadatum);
        Ok(())
    }
}

impl std::convert::TryFrom<&Metadata> for CIP25Metadata {
    type Error = DeserializeError;

    fn try_from(metadata: &Metadata) -> Result<Self, Self::Error> {
        let cip25_metadatum = metadata
            .get(&CIP25_METADATA_LABEL)
            .ok_or_else(|| DeserializeFailure::MandatoryFieldMissing(Key::Uint(CIP25_METADATA_LABEL)))?;
        Ok(Self {
            key_721: LabelMetadata::from_cbor_bytes(&cip25_metadatum.to_original_cbor_bytes())?,
        })
    }
}

impl std::convert::TryInto<Metadata> for &CIP25Metadata {
    type Error = DeserializeError;

    fn try_into(self) -> Result<Metadata, Self::Error> {
        let mut metadata = Metadata::new();
        self.add_to_metadata(&mut metadata)?;
        Ok(metadata)
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct MetadataDetails {
    pub name: String64,
    pub image: ChunkableString,
    pub media_type: Option<String64>,
    pub description: Option<ChunkableString>,
    pub files: Option<Vec<FilesDetails>>,
}

impl MetadataDetails {
    pub fn new(name: String64, image: ChunkableString) -> Self {
        Self {
            name,
            image,
            media_type: None,
            description: None,
            files: None,
        }
    }
}

/// A String of at most 64 bytes.
/// This is to conform with Cardano metadata restrictions.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema, Eq, PartialEq, Ord, PartialOrd)]
pub struct String64(String);

impl String64 {
    pub fn new(inner: &str) -> Result<Self, DeserializeError> {
        if inner.len() > 64 {
            return Err(DeserializeError::new("String64", DeserializeFailure::RangeCheck{ found: inner.len(), min: Some(0), max: Some(64) }));
        }
        Ok(Self(inner.to_owned()))
    }

    pub fn to_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<&str> for String64 {
    type Error = DeserializeError;

    fn try_from(inner: &str) -> Result<Self, Self::Error> {
        String64::new(inner)
    }
}

impl From<String64> for String {
    fn from(wrapper: String64) -> Self {
        wrapper.0
    }
}

/// A String that may or may not be chunked into 64-byte chunks to be able
/// to conform to Cardano TX Metadata limitations.
/// Unless you have good reasons, you should be using the From<&str> trait to construct this:
/// ```
/// use cardano_multiplatform_lib_cip25::ChunkableString;
/// // automatically chunks this too long string into two chunks:
/// let chunkable_string = ChunkableString::from("this can be any length and will automatically be chunked into 64-byte pieces when/if needed");
/// match chunkable_string {
///     ChunkableString::Single(_) => panic!(),
///     ChunkableString::Chunked(chunks) => {
///         assert_eq!(chunks[0].to_str(), "this can be any length and will automatically be chunked into 64");
///         assert_eq!(chunks[1].to_str(), "-byte pieces when/if needed");
///     },
/// }
/// ```
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub enum ChunkableString {
    Single(String64),
    Chunked(Vec<String64>),
}

impl ChunkableString {
    /// Construct from a single <=64 byte string chunk.
    /// If size is not known or for simplicity use From<&str> instead
    pub fn new_single(string64: String64) -> Self {
        Self::Single(string64)
    }

    /// Construct from an explicit list of chunks
    /// If size is not known or for simplicity use From<&str> instead
    pub fn new_chunked(arr_string64: Vec<String64>) -> Self {
        Self::Chunked(arr_string64)
    }
}

impl From<&str> for ChunkableString {
    fn from(s: &str) -> Self {
        String64::new(s)
            .map(Self::Single)
            .unwrap_or_else(|_err| {
                let mut chunks = Vec::with_capacity(s.len() / 64);
                for i in (0..s.len()).step_by(64) {
                    let j = std::cmp::min(s.len(), i + 64);
                    chunks.push(String64::new(&s[i..j]).unwrap());
                }
                Self::Chunked(chunks)
            })
    }
}

impl From<&ChunkableString> for String {
    fn from(chunkable: &ChunkableString) -> Self {
        match chunkable {
            ChunkableString::Single(chunk) => chunk.to_str().to_owned(),
            ChunkableString::Chunked(chunks) => chunks.iter().map(|chunk| chunk.to_str().to_owned()).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let mut details = MetadataDetails::new(
            String64::try_from("Metadata Name").unwrap(),
            ChunkableString::from("htts://some.website.com/image.png"));
        details.description = Some(ChunkableString::from("description of this NFT"));
        details.media_type = Some(String64::try_from("image/*").unwrap());
        details.files = Some(vec![
            FilesDetails::new(
                String64::new("filename1").unwrap(),
                String64::new("filetype1").unwrap(),
                ChunkableString::from("src1")),
            FilesDetails::new(
                String64::new("filename2").unwrap(),
                String64::new("filetype2").unwrap(),
                ChunkableString::from("src2")),
        ]);
        let mut v2 = Data::new();
        let mut v2_inner = BTreeMap::new();
        v2_inner.insert(vec![0xCA, 0xFE, 0xD0, 0x0D], details);
        v2.insert(vec![0xBA, 0xAD, 0xF0, 0x0D], v2_inner);
        let metadata = CIP25Metadata::new(LabelMetadata::new_label_metadata_v2(LabelMetadataV2::new(v2)));
        let metadata_bytes = metadata.to_bytes();
        let roundtrip = CIP25Metadata::from_cbor_bytes(&metadata_bytes).unwrap();
        assert_eq!(metadata_bytes, roundtrip.to_bytes());
        let as_metadata = metadata.to_metadata().unwrap();
        let from_metadata = CIP25Metadata::from_metadata(&as_metadata).unwrap();
        assert_eq!(metadata_bytes, from_metadata.to_bytes());
    }
}