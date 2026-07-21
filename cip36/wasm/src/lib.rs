// Seeded by cddl-codegen on first export; never overwritten after that.
// All regenerated code lives in the `generated` module. Add your own
// modules/re-exports/attrs here freely (e.g. `pub mod utils;`).
mod generated;
pub use generated::*;

pub mod utils;

// Convenience/API-compat re-exports. These types are dep-owned (declared in
// specs/cip36/_CDDL_CODEGEN_EXTERN_DEPS_DIR_), so the generated code qualifies them through
// their crates directly; these re-exports only preserve the historical cml_cip36_wasm-root API.
pub use cml_chain_wasm::address::Address as PaymentAddress;
pub use cml_crypto_wasm::{Ed25519Signature, PublicKey};
