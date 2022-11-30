use std::io::{BufRead, Seek, Write};
use prelude::*;

// This library was code-generated using an experimental CDDL to rust tool:
// https://github.com/Emurgo/cddl-codegen

use cbor_event::{self, de::Deserializer, se::{Serialize, Serializer}};

use cbor_event::Type as CBORType;

use cbor_event::Special as CBORSpecial;

use serialization::*;

pub mod prelude;

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

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct FilesDetails {
    pub name: String64,
    pub media_type: String64,
    pub src: String64OrArrString64,
}

impl FilesDetails {
    pub fn new(name: String64, media_type: String64, src: String64OrArrString64) -> Self {
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

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct Metadata {
    pub key_721: LabelMetadata,
}

impl Metadata {
    pub fn new(key_721: LabelMetadata) -> Self {
        Self {
            key_721,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct MetadataDetails {
    pub name: String64,
    pub image: String64OrArrString64,
    pub media_type: Option<String64>,
    pub description: Option<String64OrArrString64>,
    pub files: Option<Vec<FilesDetails>>,
}

impl MetadataDetails {
    pub fn new(name: String64, image: String64OrArrString64) -> Self {
        Self {
            name,
            image,
            media_type: None,
            description: None,
            files: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema, Eq, PartialEq, Ord, PartialOrd)]
pub struct String64(String);

impl String64 {
    pub fn get(&self) -> &String {
        &self.0
    }

    pub fn new(inner: String) -> Result<Self, DeserializeError> {
        if inner.len() > 64 {
            return Err(DeserializeError::new("String64", DeserializeFailure::RangeCheck{ found: inner.len(), min: Some(0), max: Some(64) }));
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

impl From<String64> for String {
    fn from(wrapper: String64) -> Self {
        wrapper.0
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub enum String64OrArrString64 {
    String64(String64),
    ArrString64(Vec<String64>),
}

impl String64OrArrString64 {
    pub fn new_string64(string64: String64) -> Self {
        Self::String64(string64)
    }

    pub fn new_arr_string64(arr_string64: Vec<String64>) -> Self {
        Self::ArrString64(arr_string64)
    }
}