#![allow(clippy::too_many_arguments)]

// This library was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

pub mod serialization;
pub mod utils;

pub use utils::{CIP25Version, LabelMetadata};

use cbor_event;
use cbor_event::de::Deserializer;
use cbor_event::se::{Serializer};
use cbor_event::Special as CBORSpecial;
use cbor_event::Type as CBORType;
pub use cml_core::error::*;
use std::convert::{From, TryFrom};
use std::io::{BufRead, Write};

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
        Self { key_721 }
    }
}

/// A String that may or may not be chunked into 64-byte chunks to be able
/// to conform to Cardano TX Metadata limitations.
/// Unless you have good reasons, you should be using the From<&str> trait to construct this:
/// ```
/// use cml_cip25::ChunkableString;
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
    pub fn new_single(single: String64) -> Self {
        Self::Single(single)
    }

    /// Construct from an explicit list of chunks
    /// If size is not known or for simplicity use From<&str> instead
    pub fn new_chunked(chunked: Vec<String64>) -> Self {
        Self::Chunked(chunked)
    }
}

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

impl From<String64> for String {
    fn from(wrapper: String64) -> Self {
        wrapper.0
    }
}

#[derive(
    Clone,
    Debug,
    serde::Deserialize,
    serde::Serialize,
    schemars::JsonSchema,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
)]
pub struct String64(pub String);

impl String64 {
    pub fn get(&self) -> &String {
        &self.0
    }

    pub fn new(inner: String) -> Result<Self, DeserializeError> {
        if inner.len() > 64 {
            return Err(DeserializeError::new(
                "String64",
                DeserializeFailure::RangeCheck {
                    found: inner.len(),
                    min: Some(0),
                    max: Some(64),
                },
            ));
        }
        Ok(Self(inner))
    }
}

impl TryFrom<String> for String64 {
    type Error = DeserializeError;

    fn try_from(inner: String) -> Result<Self, Self::Error> {
        String64::new(inner)
    }
}
