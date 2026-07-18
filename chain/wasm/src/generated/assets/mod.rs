// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

pub use crate::Value;
// cddl-codegen:insert-start
// Deref is used by the hand-augmented Mint/MultiAsset class methods below.
use std::ops::Deref;
// cddl-codegen:insert-end

use crate::generated::crypto::ScriptHash;
use crate::generated::{MapAssetNameToNonZeroInt64, MapAssetNameToU64, PolicyId, PolicyIdList};
use cml_core::ordered_hash_map::OrderedHashMap;
use cml_core_wasm::{impl_wasm_cbor_json_api, impl_wasm_conversions, impl_wasm_list_needs_into};
use wasm_bindgen::prelude::{JsError, wasm_bindgen};

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct AssetName(pub(crate) cml_chain::assets::AssetName);

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

pub type Coin = u64;

pub type MapPolicyIdToMapAssetNameToNonZeroInt64 = Mint;

pub type MapPolicyIdToMapAssetNameToU64 = MultiAsset;

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct Mint(pub(crate) cml_chain::assets::Mint);

impl_wasm_conversions!(cml_chain::assets::Mint, Mint);

#[wasm_bindgen]
impl Mint {
    pub fn new() -> Self {
        // cddl-codegen:replace-start
        Self(cml_chain::assets::Mint::new())
        // cddl-codegen:replaces
        // Self(OrderedHashMap::new())
        // cddl-codegen:replace-end
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
        // cddl-codegen:replace-start
        self.0.deref().get(key.as_ref()).map(|v| v.clone().into())
        // cddl-codegen:replaces
        // self.0.get(key.as_ref()).map(|v| v.clone().into())
        // cddl-codegen:replace-end
    }

    pub fn keys(&self) -> PolicyIdList {
        PolicyIdList(self.0.keys().cloned().collect::<Vec<_>>())
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MultiAsset(pub(crate) cml_chain::assets::MultiAsset);

impl_wasm_conversions!(cml_chain::assets::MultiAsset, MultiAsset);

#[wasm_bindgen]
impl MultiAsset {
    pub fn new() -> Self {
        // cddl-codegen:replace-start
        Self(cml_chain::assets::MultiAsset::new())
        // cddl-codegen:replaces
        // Self(OrderedHashMap::new())
        // cddl-codegen:replace-end
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
        // cddl-codegen:replace-start
        self.0.deref().get(key.as_ref()).map(|v| v.clone().into())
        // cddl-codegen:replaces
        // self.0.get(key.as_ref()).map(|v| v.clone().into())
        // cddl-codegen:replace-end
    }

    pub fn keys(&self) -> PolicyIdList {
        PolicyIdList(self.0.keys().cloned().collect::<Vec<_>>())
    }
}

pub type NonZeroInt64 = i64;

pub type PositiveCoin = u64;
