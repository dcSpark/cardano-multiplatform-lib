# Core

This crate is for core features and traits common to all CML crates. Most users likely won't need to directly use this module except for possibly pulling in traits used with other cml crates. If you are using CML from WASM/typescript this module will not be needed as any used types will be re-exported in the crates (e.g. cml-chain-wasm, cml-cip25-wasm, etc) that use it.