use std::ops::Deref;

use crate::{assets::AssetName, AssetNameList, MapAssetNameToNonZeroInt64, PolicyId, PolicyIdList};
use wasm_bindgen::{prelude::wasm_bindgen, JsError};

use cml_core_wasm::{impl_wasm_conversions, impl_wasm_map};

use super::Coin;

impl_wasm_map!(
    cml_chain::assets::AssetName,
    Coin,
    AssetName,
    Coin,
    AssetNameList,
    MapAssetNameToCoin,
    false,
    true,
    false,
    true
);

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MultiAsset(cml_chain::assets::MultiAsset);

#[wasm_bindgen]
impl MultiAsset {
    pub fn new() -> Self {
        Self(cml_chain::assets::MultiAsset::default())
    }

    pub fn policy_count(&self) -> usize {
        self.0.len()
    }

    pub fn insert_assets(
        &mut self,
        policy_id: &PolicyId,
        assets: &MapAssetNameToCoin,
    ) -> Option<MapAssetNameToCoin> {
        self.0
            .insert(policy_id.clone().into(), assets.clone().into())
            .map(Into::into)
    }

    pub fn get_assets(&self, key: &PolicyId) -> Option<MapAssetNameToCoin> {
        self.0.deref().get(key.as_ref()).map(|v| v.clone().into())
    }

    /// Get the value of policy_id:asset_name if it exists.
    pub fn get(&self, policy_id: &PolicyId, asset: &AssetName) -> Option<Coin> {
        self.0.get(policy_id.as_ref(), asset.as_ref())
    }

    /// Set the value of policy_id:asset_name to value.
    /// Returns the previous value, or None if it didn't exist
    pub fn set(&mut self, policy_id: &PolicyId, asset: &AssetName, value: Coin) -> Option<Coin> {
        self.0
            .set(policy_id.clone().into(), asset.clone().into(), value)
    }

    pub fn keys(&self) -> PolicyIdList {
        PolicyIdList(self.0.iter().map(|(k, _v)| *k).collect::<Vec<_>>())
    }

    /// Adds to multiassets together, checking value bounds.
    /// Does not modify self, and instead returns the result.
    pub fn checked_add(&self, rhs: &MultiAsset) -> Result<MultiAsset, JsError> {
        self.0
            .checked_add(rhs.as_ref())
            .map(Into::into)
            .map_err(Into::into)
    }

    /// Subtracts rhs from this multiasset.
    /// This does not modify self, and instead returns the result.
    /// If this would cause there to be fewer than 0 of a given asset
    /// an error will be returned.
    /// Use clamped_sub if you need to only try to remove assets when they exist
    /// and ignore them when they don't.
    pub fn checked_sub(&self, rhs: &MultiAsset) -> Result<MultiAsset, JsError> {
        self.0
            .checked_sub(rhs.as_ref())
            .map(Into::into)
            .map_err(Into::into)
    }

    /// Sybtracts rhs from this multiasset.
    /// If this would cause there to be 0 or fewer of a given asset
    /// it will simply be removed entirely from the result.
    pub fn clamped_sub(&self, rhs: &MultiAsset) -> Self {
        use cml_chain::assets::ClampedSub;
        self.0.clamped_sub(rhs.as_ref()).into()
    }
}

impl_wasm_conversions!(cml_chain::assets::MultiAsset, MultiAsset);

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct Mint(cml_chain::assets::Mint);

#[wasm_bindgen]
impl Mint {
    pub fn new() -> Self {
        Self(cml_chain::assets::Mint::default())
    }

    pub fn policy_count(&self) -> usize {
        self.0.len()
    }

    pub fn insert_assets(
        &mut self,
        policy_id: &PolicyId,
        assets: &MapAssetNameToNonZeroInt64,
    ) -> Option<MapAssetNameToNonZeroInt64> {
        self.0
            .insert(policy_id.clone().into(), assets.clone().into())
            .map(Into::into)
    }

    pub fn get_assets(&self, key: &PolicyId) -> Option<MapAssetNameToNonZeroInt64> {
        self.0.deref().get(key.as_ref()).map(|v| v.clone().into())
    }

    /// Get the value of policy_id:asset_name if it exists.
    pub fn get(&self, policy_id: &PolicyId, asset: &AssetName) -> Option<i64> {
        self.0.get(policy_id.as_ref(), asset.as_ref())
    }

    /// Set the value of policy_id:asset_name to value.
    /// Returns the previous value, or None if it didn't exist
    pub fn set(&mut self, policy_id: &PolicyId, asset: &AssetName, value: i64) -> Option<i64> {
        self.0
            .set(policy_id.clone().into(), asset.clone().into(), value)
    }

    pub fn keys(&self) -> PolicyIdList {
        PolicyIdList(self.0.iter().map(|(k, _v)| *k).collect::<Vec<_>>())
    }

    /// Adds two mints together, checking value bounds.
    /// Does not modify self, and instead returns the result.
    pub fn checked_add(&self, rhs: &Mint) -> Result<Mint, JsError> {
        self.0
            .checked_add(rhs.as_ref())
            .map(Into::into)
            .map_err(Into::into)
    }

    /// Subtracts rhs from this mint.
    /// This does not modify self, and instead returns the result.
    pub fn checked_sub(&self, rhs: &Mint) -> Result<Mint, JsError> {
        self.0
            .checked_sub(rhs.as_ref())
            .map(Into::into)
            .map_err(Into::into)
    }
}

impl_wasm_conversions!(cml_chain::assets::Mint, Mint);

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Value(cml_chain::assets::Value);

#[wasm_bindgen]
impl Value {
    pub fn from_coin(coin: Coin) -> Self {
        cml_chain::assets::Value::from(coin).into()
    }

    pub fn new(coin: Coin, multiasset: &MultiAsset) -> Self {
        cml_chain::assets::Value::new(coin, multiasset.clone().into()).into()
    }

    pub fn coin(&self) -> Coin {
        self.0.coin
    }

    pub fn multi_asset(&self) -> MultiAsset {
        self.0.multiasset.clone().into()
    }

    pub fn zero() -> Value {
        cml_chain::assets::Value::zero().into()
    }

    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    pub fn has_multiassets(&self) -> bool {
        self.0.has_multiassets()
    }

    pub fn checked_add(&self, rhs: &Value) -> Result<Value, JsError> {
        self.0
            .checked_add(rhs.as_ref())
            .map(Into::into)
            .map_err(Into::into)
    }

    /// Subtract ADA and/or assets
    /// Removes an asset from the list if the result is 0 or less
    /// Does not modify this object, instead the result is returned
    /// None is returned if there would be integer underflow
    pub fn checked_sub(&self, rhs: &Value) -> Result<Value, JsError> {
        self.0
            .checked_sub(rhs.as_ref())
            .map(Into::into)
            .map_err(Into::into)
    }

    pub fn clamped_sub(&self, rhs: &Value) -> Value {
        self.0.clamped_sub(rhs.as_ref()).into()
    }
}

impl_wasm_conversions!(cml_chain::assets::Value, Value);
