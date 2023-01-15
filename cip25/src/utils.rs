use std::convert::TryFrom;

pub use cardano_multiplatform_lib_core::{
    serialization::*,
    error::*,
    metadata::{Metadata, TransactionMetadatum},
};

use crate::{String64, ChunkableString, LabelMetadata, CIP25Metadata};

pub static CIP25_METADATA_LABEL: u64 = 721;

impl CIP25Metadata {

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

impl String64 {
    pub fn new_str(inner: &str) -> Result<Self, DeserializeError> {
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
        String64::new_str(inner)
    }
}

impl From<&str> for ChunkableString {
    fn from(s: &str) -> Self {
        String64::new_str(s)
            .map(Self::Single)
            .unwrap_or_else(|_err| {
                let mut chunks = Vec::with_capacity(s.len() / 64);
                for i in (0..s.len()).step_by(64) {
                    let j = std::cmp::min(s.len(), i + 64);
                    chunks.push(String64::new_str(&s[i..j]).unwrap());
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
    use std::collections::BTreeMap;

    use crate::{LabelMetadataV2, Data, MetadataDetails, FilesDetails, AssetNameV2, PolicyIdV2};

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
                String64::new_str("filename1").unwrap(),
                String64::new_str("filetype1").unwrap(),
                ChunkableString::from("src1")),
            FilesDetails::new(
                String64::new_str("filename2").unwrap(),
                String64::new_str("filetype2").unwrap(),
                ChunkableString::from("src2")),
        ]);
        let mut v2 = Data::new();
        let mut v2_inner = BTreeMap::new();
        v2_inner.insert(AssetNameV2::from(vec![0xCA, 0xFE, 0xD0, 0x0D]), details);
        v2.insert(PolicyIdV2::from(vec![0xBA, 0xAD, 0xF0, 0x0D]), v2_inner);
        let metadata = CIP25Metadata::new(LabelMetadata::new_label_metadata_v2(LabelMetadataV2::new(v2)));
        let metadata_bytes = metadata.to_bytes();
        let roundtrip = CIP25Metadata::from_cbor_bytes(&metadata_bytes).unwrap();
        assert_eq!(metadata_bytes, roundtrip.to_bytes());
        let as_metadata = metadata.to_metadata().unwrap();
        let from_metadata = CIP25Metadata::from_metadata(&as_metadata).unwrap();
        assert_eq!(metadata_bytes, from_metadata.to_bytes());
    }
}
