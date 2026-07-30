#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

// Seeded by cddl-codegen on first export; never overwritten after that.
// All regenerated code lives in the `generated` module. Add your own
// modules/re-exports/attrs here freely (e.g. `pub mod utils;`).
mod generated;
pub use generated::*;

pub mod utils;

pub use utils::{CIP25LabelMetadata, CIP25Version};
