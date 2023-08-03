use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

pub mod utils;

pub use utils::{Mint, MultiAsset, Value};

pub use cml_chain::assets::Coin;

use cml_core_wasm::{impl_wasm_cbor_json_api, impl_wasm_conversions};

// Code below here was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct AssetName(cml_chain::assets::AssetName);

impl_wasm_cbor_json_api!(AssetName);

impl_wasm_conversions!(cml_chain::assets::AssetName, AssetName);

#[wasm_bindgen]
impl AssetName {
    pub fn get(&self) -> Vec<u8> {
        self.0.get().clone()
    }
}

