//! Same keygen runtime assertions as keygen_runtime.rs, but run in a real (headless) browser:
//!
//!   wasm-pack test --headless --chrome crypto/wasm
//!   wasm-pack test --headless --firefox crypto/wasm
//!
//! `run_in_browser` makes this target browser-only, so native/node runners skip it (and the
//! file compiles to nothing off wasm32). It proves the getrandom `wasm_js` backend
//! (crypto.getRandomValues) works in an actual browser, not just Node.

#![cfg(target_arch = "wasm32")]

mod common;

use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn ed25519_keygen_is_random() {
    common::ed25519_keygen_is_random();
}

#[wasm_bindgen_test]
fn bip32_keygen_is_random() {
    common::bip32_keygen_is_random();
}
