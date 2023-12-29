use wasm_bindgen::prelude::{wasm_bindgen, JsError};

use crate::{byron::AddressContent, crypto::BootstrapWitness, Vkeywitness};

use cml_crypto_wasm::{PrivateKey, TransactionHash};

#[wasm_bindgen]
impl BootstrapWitness {
    pub fn to_address(&self) -> Result<AddressContent, JsError> {
        self.0.to_address().map(Into::into).map_err(Into::into)
    }
}

#[wasm_bindgen]
pub fn make_vkey_witness(tx_body_hash: &TransactionHash, sk: &PrivateKey) -> Vkeywitness {
    cml_chain::crypto::utils::make_vkey_witness(tx_body_hash.as_ref(), sk.as_ref()).into()
}
