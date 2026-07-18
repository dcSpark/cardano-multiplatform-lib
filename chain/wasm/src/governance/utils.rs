use wasm_bindgen::prelude::wasm_bindgen;

use cml_crypto_wasm::{Ed25519KeyHash, ScriptHash};

use super::{GovAction, Voter};

#[wasm_bindgen]
impl GovAction {
    pub fn script_hash(&self) -> Option<ScriptHash> {
        self.0.script_hash().map(|hash| (*hash).into())
    }
}

#[wasm_bindgen]
impl Voter {
    pub fn key_hash(&self) -> Option<Ed25519KeyHash> {
        self.0.key_hash().map(|hash| (*hash).into())
    }

    pub fn script_hash(&self) -> Option<ScriptHash> {
        self.0.script_hash().map(|hash| (*hash).into())
    }
}
