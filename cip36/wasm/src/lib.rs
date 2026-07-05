// Seeded by cddl-codegen on first export; never overwritten after that.
// All regenerated code lives in the `generated` module. Add your own
// modules/re-exports/attrs here freely (e.g. `pub mod utils;`).
mod generated;
pub use generated::*;

pub mod utils;

// Extern-type re-export so the generated glue (`pub use crate::PaymentAddress;`) resolves.
pub use cml_chain_wasm::address::Address as PaymentAddress;
