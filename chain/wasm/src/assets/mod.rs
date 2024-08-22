use wasm_bindgen::prelude::wasm_bindgen;

pub mod utils;

pub use utils::{Mint, MultiAsset, Value};

pub use cml_chain::assets::{Coin, NonZeroInt64, PositiveCoin};

use cml_core_wasm::{impl_wasm_cbor_json_api, impl_wasm_conversions};

// Code below here was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct AssetName(cml_chain::assets::AssetName);

impl_wasm_cbor_json_api!(AssetName);

impl_wasm_conversions!(cml_chain::assets::AssetName, AssetName);
