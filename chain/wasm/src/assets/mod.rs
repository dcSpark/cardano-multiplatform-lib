use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

pub mod utils;

pub use utils::{Mint, MultiAsset, Value};

pub use cml_chain::assets::Coin;

// Code below here was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct AssetName(cml_chain::assets::AssetName);

#[wasm_bindgen]
impl AssetName {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<AssetName, JsValue> {
        cml_chain::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<AssetName, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn get(&self) -> Vec<u8> {
        self.0.get().clone()
    }
}

impl From<cml_chain::assets::AssetName> for AssetName {
    fn from(native: cml_chain::assets::AssetName) -> Self {
        Self(native)
    }
}

impl From<AssetName> for cml_chain::assets::AssetName {
    fn from(wasm: AssetName) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::assets::AssetName> for AssetName {
    fn as_ref(&self) -> &cml_chain::assets::AssetName {
        &self.0
    }
}
