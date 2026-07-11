// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

pub mod utils;

pub use crate::Value;
pub use cml_chain::assets::{Coin, NonZeroInt64, PositiveCoin};

use crate::generated::{MapAssetNameToNonZeroInt64, MapAssetNameToU64, PolicyId, PolicyIdList};
use std::ops::Deref;

use cml_core_wasm::{impl_wasm_cbor_json_api, impl_wasm_conversions};
use wasm_bindgen::prelude::{JsError, wasm_bindgen};

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct AssetName(cml_chain::assets::AssetName);

impl_wasm_cbor_json_api!(AssetName);

impl_wasm_conversions!(cml_chain::assets::AssetName, AssetName);

#[wasm_bindgen]
impl AssetName {
    pub fn new(inner: Vec<u8>) -> Result<AssetName, JsError> {
        cml_chain::assets::AssetName::new(inner)
            .map(Into::into)
            .map_err(Into::into)
    }

    pub fn get(&self) -> Vec<u8> {
        self.0.get().clone()
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct Mint(cml_chain::assets::Mint);

impl_wasm_conversions!(cml_chain::assets::Mint, Mint);

#[wasm_bindgen]
impl Mint {
    pub fn new() -> Self {
        Self(cml_chain::assets::Mint::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(
        &mut self,
        key: &PolicyId,
        value: &MapAssetNameToNonZeroInt64,
    ) -> Option<MapAssetNameToNonZeroInt64> {
        self.0
            .insert(key.clone().into(), value.clone().into())
            .map(Into::into)
    }

    pub fn get(&self, key: &PolicyId) -> Option<MapAssetNameToNonZeroInt64> {
        self.0.deref().get(key.as_ref()).map(|v| v.clone().into())
    }

    pub fn keys(&self) -> PolicyIdList {
        PolicyIdList(self.0.keys().cloned().collect::<Vec<_>>())
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MultiAsset(cml_chain::assets::MultiAsset);

impl_wasm_conversions!(cml_chain::assets::MultiAsset, MultiAsset);

#[wasm_bindgen]
impl MultiAsset {
    pub fn new() -> Self {
        Self(cml_chain::assets::MultiAsset::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(
        &mut self,
        key: &PolicyId,
        value: &MapAssetNameToU64,
    ) -> Option<MapAssetNameToU64> {
        self.0
            .insert(key.clone().into(), value.clone().into())
            .map(Into::into)
    }

    pub fn get(&self, key: &PolicyId) -> Option<MapAssetNameToU64> {
        self.0.deref().get(key.as_ref()).map(|v| v.clone().into())
    }

    pub fn keys(&self) -> PolicyIdList {
        PolicyIdList(self.0.keys().cloned().collect::<Vec<_>>())
    }
}
