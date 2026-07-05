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

pub mod address;
pub mod builders;
pub mod byron;
pub mod deposit;
pub mod fees;
pub mod genesis;
pub mod json;
pub mod min_ada;
pub mod utils;

// Imports/aliases the hand-written modules pull in via `use super::*` /
// `use crate::*` or reference bare as `crate::X` (these were private root
// imports in the pre-thin-root lib.rs).
use crate::assets::AssetName;
use crate::crypto::{ScriptHash, Vkeywitness};
use crate::transaction::NativeScript;
use ::wasm_bindgen::prelude::{JsError, wasm_bindgen};

// Generic-extern list aliases. The generator cannot emit these itself; it emits
// `pub use crate::X;` glue in the generated module that resolves back to here.
pub type SetTransactionInput = TransactionInputList;
pub type NonemptySetBootstrapWitness = BootstrapWitnessList;
pub type NonemptySetCertificate = CertificateList;
pub type NonemptySetNativeScript = NativeScriptList;
pub type NonemptySetPlutusData = PlutusDataList;
pub type NonemptySetPlutusV1Script = PlutusV1ScriptList;
pub type NonemptySetPlutusV2Script = PlutusV2ScriptList;
pub type NonemptySetPlutusV3Script = PlutusV3ScriptList;
pub type NonemptySetProposalProcedure = ProposalProcedureList;
pub type NonemptySetTransactionInput = TransactionInputList;
pub type NonemptySetVkeywitness = VkeywitnessList;
pub type RequiredSigners = Ed25519KeyHashList;
pub type SetCommitteeColdCredential = CommitteeColdCredentialList;
pub type SetEd25519KeyHash = Ed25519KeyHashList;
