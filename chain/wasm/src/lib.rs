pub mod address;
pub mod crypto;

use address::*;
use crypto::*;

use wasm_bindgen::prelude::{wasm_bindgen, JsError, JsValue};

// this is actually chain pulling in the core definition.
// when we regenerate we might want to change the module naming
use core::DeserializeError;

#[wasm_bindgen]
pub struct StakeCredential(core::StakeCredential);

#[wasm_bindgen]
impl StakeCredential {
    pub fn new_key(addr_keyhash: &Ed25519KeyHashOnChain) -> Self {
        core::StakeCredential::new_key(addr_keyhash.clone().into()).into()
    }

    pub fn new_script(scripthash: &ScriptHashOnChain) -> Self {
        core::StakeCredential::new_script(scripthash.clone().into()).into()
    }
}

impl From<core::StakeCredential> for StakeCredential {
    fn from(native: core::StakeCredential) -> Self {
        Self(native)
    }
}

impl From<StakeCredential> for core::StakeCredential {
    fn from(wasm: StakeCredential) -> Self {
        wasm.0
    }
}

// TODO: regenerate
