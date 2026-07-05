use wasm_bindgen::prelude::wasm_bindgen;
use super::{Ipv4, Ipv6, StakeCredential};

#[wasm_bindgen]
impl StakeCredential {
    // we don't implement RawBytesEncoding as from_raw_bytes() would be unable to distinguish
    pub fn to_raw_bytes(&self) -> Vec<u8> {
        self.0.to_raw_bytes().into()
    }
}

#[wasm_bindgen]
impl Ipv4 {
    pub fn to_str(&self) -> String {
        self.0.to_string()
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Result<Ipv4, JsError> {
        cml_chain::certs::Ipv4::from_str(s).map(Into::into).map_err(Into::into)
    }
}

#[wasm_bindgen]
impl Ipv6 {
    pub fn to_str(&self) -> String {
        self.0.to_string()
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Result<Ipv6, JsError> {
        cml_chain::certs::Ipv6::from_str(s).map(Into::into).map_err(Into::into)
    }
}