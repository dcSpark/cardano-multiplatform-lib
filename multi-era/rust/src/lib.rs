#![allow(clippy::too_many_arguments)]

// Seeded by cddl-codegen on first export; never overwritten after that.
// All regenerated code lives in the `generated` module. Add your own
// modules/re-exports/attrs here freely (e.g. `pub mod utils;`).
mod generated;
pub use generated::*;

pub mod byron;
pub mod utils;

// Facade module (see cddl-codegen docs "Per-scope hand modules: the facade pattern"):
// shadows the glob-imported generated `babbage` scope, merging the generated items with
// the hand file at src/babbage/utils.rs under the same public path.
pub mod babbage {
    pub use crate::generated::babbage::*;
    pub mod utils;
}

// Extern-type re-exports so the generated glue (`pub use crate::X;`) resolves.
pub use babbage::utils::BabbageMint;
pub use byron::block::ByronBlock;
pub use byron::transaction::ByronTx;
pub use cml_chain::block::Block;
