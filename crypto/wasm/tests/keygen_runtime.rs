//! Keygen runtime check on desktop (native) and web (wasm32 in Node).
//!
//! Exercises the RNG path end to end: `generate_*` -> cml-crypto -> `rand` -> getrandom
//! (OS backend natively; `wasm_js` / `crypto.getRandomValues` on wasm32). The wasm case is
//! the one that previously failed at runtime when the backend wasn't wired up.
//!
//!   cargo test -p cml-crypto-wasm --test keygen_runtime                                 # desktop
//!   wasm-pack test --node crypto/wasm                                                   # web (Node)
//!
//! Browser execution of the same assertions lives in keygen_browser.rs.

mod common;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::wasm_bindgen_test;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn ed25519_keygen_is_random() {
    common::ed25519_keygen_is_random();
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn bip32_keygen_is_random() {
    common::bip32_keygen_is_random();
}
