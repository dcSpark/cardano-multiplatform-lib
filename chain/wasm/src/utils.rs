use super::Int;
use wasm_bindgen::prelude::{wasm_bindgen, JsError, JsValue};

use cml_core_wasm::{impl_wasm_cbor_json_api, impl_wasm_conversions};

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct BigInt(cml_chain::utils::BigInt);

impl_wasm_conversions!(cml_chain::utils::BigInt, BigInt);

impl_wasm_cbor_json_api!(BigInt);

#[wasm_bindgen]
impl BigInt {
    pub fn from_int(x: &Int) -> Self {
        Self(cml_chain::utils::BigInt::from_int(x.as_ref()))
    }

    #[allow(clippy::should_implement_trait)]
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
