#[cfg(not(all(target_arch = "wasm32", not(target_os = "wasi"), not(target_os = "emscripten"))))]
use noop_proc_macro::wasm_bindgen;

#[cfg(all(target_arch = "wasm32", not(target_os = "wasi"), not(target_os = "emscripten")))]
use wasm_bindgen::prelude::*;

use crate::crypto::{TransactionHash, PrivateKey, Vkeywitness, Vkey};

#[wasm_bindgen]
pub fn make_vkey_witness(
    tx_body_hash: &TransactionHash,
    sk: &PrivateKey
) -> Vkeywitness {
    let sig = sk.sign(tx_body_hash.0.as_ref());
    Vkeywitness::new(&Vkey::new(&sk.to_public()), &sig)
}
