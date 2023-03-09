
use wasm_bindgen::prelude::{wasm_bindgen, JsError, JsValue};
use super::Int;
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct BigInt(cml_chain::utils::BigInt);

#[wasm_bindgen]
impl BigInt {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<BigInt, JsValue> {
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

    pub fn from_json(json: &str) -> Result<BigInt, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn from_int(x: &Int) -> Self {
        Self(cml_chain::utils::BigInt::from_int(x.as_ref()))
    }

    pub fn from_str(s: &str) -> Result<BigInt, JsError> {
        use std::str::FromStr;
        cml_chain::utils::BigInt::from_str(s)
            .map(Self)
            .map_err(Into::into)
    }

    pub fn to_str(&self) -> String {
        self.0.to_string()
    }

    /// Converts to a u64
    /// Returns None if the number was negative or too big for a u64
    pub fn as_u64(&self) -> Option<u64> {
        self.0.as_u64()
    }

    /// Converts to an Int
    /// Returns None when the number is too big for an Int (outside +/- 64-bit unsigned)
    /// Retains encoding info if the original was encoded as an Int
    pub fn as_int(&self) -> Option<Int> {
        self.0.as_int().map(Into::into)
    }
}

impl From<cml_chain::utils::BigInt> for BigInt {
    fn from(native: cml_chain::utils::BigInt) -> Self {
        Self(native)
    }
}

impl From<BigInt> for cml_chain::utils::BigInt {
    fn from(wasm: BigInt) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::utils::BigInt> for BigInt {
    fn as_ref(&self) -> &cml_chain::utils::BigInt {
        &self.0
    }
}