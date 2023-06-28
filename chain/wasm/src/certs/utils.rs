use wasm_bindgen::prelude::wasm_bindgen;
use super::StakeCredential;

#[wasm_bindgen]
impl StakeCredential {
    // we don't implement RawBytesEncoding as from_raw_bytes() would be unable to distinguish
    pub fn to_raw_bytes(&self) -> Vec<u8> {
        self.0.to_raw_bytes().into()
    }
}
