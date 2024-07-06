use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

use cml_core::serialization::{Deserialize, Serialize};

// re-export to make macros easier to use
pub use cml_core::serialization::RawBytesEncoding;

#[macro_use]
pub mod wasm_wrappers;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Int(cml_core::Int);

#[wasm_bindgen]

impl Int {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<Int, JsValue> {
        Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_bytes: {e}")))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_json: {e}")))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_js_value: {e}")))
    }

    pub fn from_json(json: &str) -> Result<Int, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {e}")))
    }

    pub fn new(x: i64) -> Self {
        if x >= 0 {
            Self(cml_core::Int::new_uint(x as u64))
        } else {
            Self(cml_core::Int::new_nint((x + 1).unsigned_abs()))
        }
    }

    pub fn to_str(&self) -> String {
        self.0.to_string()
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(string: &str) -> Result<Int, JsValue> {
        // have to redefine so it's visible in WASM
        std::str::FromStr::from_str(string)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("Int.from_str({string}): {e:?}")))
    }
}

impl From<cml_core::Int> for Int {
    fn from(native: cml_core::Int) -> Self {
        Self(native)
    }
}

impl From<Int> for cml_core::Int {
    fn from(wasm: Int) -> Self {
        wasm.0
    }
}

impl AsRef<cml_core::Int> for Int {
    fn as_ref(&self) -> &cml_core::Int {
        &self.0
    }
}
