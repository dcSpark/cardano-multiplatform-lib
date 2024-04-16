use cml_chain_wasm::{
    assets::AssetName,
    auxdata::{Metadata, TransactionMetadatum},
    PolicyId,
};

use crate::*;

use wasm_bindgen::prelude::JsError;

use cml_core_wasm::impl_wasm_json_api;

#[wasm_bindgen]
impl CIP25Metadata {
    /// Create a Metadata containing only the CIP25 schema
    pub fn to_metadata(&self) -> Result<Metadata, JsError> {
        self.0.to_metadata().map(Metadata::from).map_err(Into::into)
    }

    /// Read the CIP25 schema from a Metadata. Ignores all other data besides CIP25
    /// Can fail if the Metadata does not conform to CIP25
    pub fn from_metadata(metadata: &Metadata) -> Result<CIP25Metadata, JsError> {
        cml_cip25::CIP25Metadata::from_metadata(metadata.as_ref())
            .map(Self)
            .map_err(Into::into)
    }

    /// Add to an existing metadata (could be empty) the full CIP25 metadata
    pub fn add_to_metadata(&self, metadata: &mut Metadata) -> Result<(), JsError> {
        self.0
            .add_to_metadata(metadata.as_mut())
            .map_err(Into::into)
    }
}

#[wasm_bindgen]
impl CIP25String64 {
    pub fn new(s: &str) -> Result<CIP25String64, JsError> {
        cml_cip25::CIP25String64::new_str(s)
            .map(Self)
            .map_err(Into::into)
    }

    pub fn to_str(&self) -> String {
        self.0.to_str().to_owned()
    }

    pub fn get_str(&self) -> String {
        self.0.get().clone()
    }
}

#[wasm_bindgen]

impl CIP25ChunkableString {
    pub fn from_string(str: &str) -> Self {
        Self(cml_cip25::CIP25ChunkableString::from(str))
    }

    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        String::from(&self.0)
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct CIP25MiniMetadataDetails(cml_cip25::utils::CIP25MiniMetadataDetails);

impl_wasm_conversions!(
    cml_cip25::utils::CIP25MiniMetadataDetails,
    CIP25MiniMetadataDetails
);

impl_wasm_json_api!(CIP25MiniMetadataDetails);

#[wasm_bindgen]
impl CIP25MiniMetadataDetails {
    pub fn new() -> Self {
        CIP25MiniMetadataDetails(cml_cip25::utils::CIP25MiniMetadataDetails {
            name: None,
            image: None,
        })
    }

    pub fn set_name(&mut self, name: &CIP25String64) {
        self.0.name = Some(name.clone().into())
    }

    pub fn name(&self) -> Option<CIP25String64> {
        self.0.name.clone().map(CIP25String64)
    }

    pub fn set_image(&mut self, image: &CIP25ChunkableString) {
        self.0.image = Some(image.clone().into())
    }

    pub fn image(&self) -> Option<CIP25ChunkableString> {
        self.0.image.clone().map(CIP25ChunkableString)
    }

    /// loose parsing of CIP25 metadata to allow for common exceptions to the format
    /// `metadatum` should represent the data where the `CIP25MetadataDetails` is in the cip25 structure
    pub fn loose_parse(
        metadatum: &TransactionMetadatum,
    ) -> Result<CIP25MiniMetadataDetails, JsValue> {
        let parsed_data =
            cml_cip25::utils::CIP25MiniMetadataDetails::loose_parse(&metadatum.clone().into())
                .map_err(|e| JsValue::from_str(&format!("loose_parse: {e}")))?;
        Ok(CIP25MiniMetadataDetails(parsed_data))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct CIP25LabelMetadata(cml_cip25::CIP25LabelMetadata);

impl_wasm_conversions!(cml_cip25::CIP25LabelMetadata, CIP25LabelMetadata);

impl_wasm_cbor_json_api_cbor_event_serialize!(CIP25LabelMetadata);

#[wasm_bindgen]
impl CIP25LabelMetadata {
    /// Note that Version 1 can only support utf8 string asset names.
    /// Version 2 can support any asset name.
    pub fn new(version: CIP25Version) -> Self {
        Self(cml_cip25::CIP25LabelMetadata::new(version))
    }

    /// If this is version 1 and the asset name is not a utf8 asset name
    /// then this will return an error.
    /// This function will never return an error for version 2.
    /// On success, returns the previous details that were overwritten, or None otherwise.
    pub fn set(
        &mut self,
        policy_id: &PolicyId,
        asset_name: &AssetName,
        details: &CIP25MetadataDetails,
    ) -> Result<Option<CIP25MetadataDetails>, JsError> {
        self.0
            .set(
                policy_id.clone().into(),
                asset_name.clone().into(),
                details.clone().into(),
            )
            .map(|old| old.map(Into::into))
            .map_err(Into::into)
    }

    pub fn get(
        &self,
        policy_id: &PolicyId,
        asset_name: &AssetName,
    ) -> Option<CIP25MetadataDetails> {
        self.0
            .get(policy_id.as_ref(), asset_name.as_ref())
            .map(|details| details.clone().into())
    }

    pub fn version(&self) -> CIP25Version {
        self.0.version()
    }
}
