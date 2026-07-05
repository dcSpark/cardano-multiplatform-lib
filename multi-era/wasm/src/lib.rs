#![allow(
    clippy::len_without_is_empty,
    clippy::too_many_arguments,
    clippy::new_without_default
)]

// Seeded by cddl-codegen on first export; never overwritten after that.
// All regenerated code lives in the `generated` module. Add your own
// modules/re-exports/attrs here freely (e.g. `pub mod utils;`).
mod generated;
pub use generated::*;

pub mod byron;
pub mod utils;

// We export types from cip25/cip36 in order to have them be exported to WASM
// to allow this crate to be like cml + multi-era (e.g. all functionality).
// See comment in cml/wasm/src/lib.rs
pub use cml_cip25_wasm::CIP25Metadata;
pub use cml_cip36_wasm::CIP36DeregistrationCbor;

// Extern-type re-exports so the generated glue (`pub use crate::X;`) resolves.
pub use byron::block::ByronBlock;
pub use cml_chain_wasm::block::Block;
