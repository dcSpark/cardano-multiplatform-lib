// Seeded by cddl-codegen on first export; never overwritten after that.
// All regenerated code lives in the `generated` module. Add your own
// modules/re-exports/attrs here freely (e.g. `pub mod utils;`).
#![allow(
    clippy::len_without_is_empty,
    clippy::too_many_arguments,
    clippy::new_without_default
)]

mod generated;
pub use generated::*;

pub mod utils;

pub use utils::CIP25LabelMetadata;

pub use cml_cip25::CIP25Version;
