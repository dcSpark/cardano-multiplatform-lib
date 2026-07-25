// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

// Collection-wrapper index for this crate: one `pub use` re-export per collection
// wrapper class defined here (list/map wrappers minted from `[* T]` / `{* K => V}`
// shapes, including their NonEmpty variants). Compiled as part of this crate, so a
// line naming a removed wrapper fails this crate's own build — the index cannot
// drift. Downstream crates point `--extern-wrapper-index <dep>=<this file>` here to
// avoid re-minting these wrappers (a wasm duplicate-symbol link error otherwise).
pub use crate::generated::CIP36DelegationList;
pub use crate::generated::MapTransactionMetadatumLabelToTransactionMetadatum;
pub use crate::generated::NonEmptyCIP36DelegationList;
