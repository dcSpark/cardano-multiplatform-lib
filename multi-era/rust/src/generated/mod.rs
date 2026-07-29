// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

#![allow(
    clippy::too_many_arguments,
    clippy::large_enum_variant,
    clippy::result_large_err
)]

extern crate alloc;
use alloc::vec::Vec;
extern crate derivative;
pub mod allegra;
pub mod alonzo;
pub mod babbage;
mod borrowed_key_types;
mod extern_interface_check;
pub mod mary;
pub mod shelley;
pub use cml_core::{Int, IntError};
pub mod serialization;
// cddl-codegen extern re-export contract: this crate's hand-written root lib.rs must re-export
// each name below (`pub use <your_module>::<Name>;`) so the generated glue resolves against the
// user-owned definition. See the extern types section of docs/output_format.
pub use crate::Block;
pub use crate::ByronBlock;
pub use crate::ByronTx;

use allegra::{AllegraBlock, AllegraTransactionBody};
use alonzo::{AlonzoBlock, AlonzoTransactionBody};
use babbage::{BabbageBlock, BabbageTransactionBody};
use cml_chain::crypto::GenesisHash;
use cml_chain::transaction::TransactionBody;
use mary::{MaryBlock, MaryTransactionBody};
use shelley::{ShelleyBlock, ShelleyTransactionBody};

pub type GenesisHashList = Vec<GenesisHash>;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub enum MultiEraBlock {
    Byron(ByronBlock),
    Shelley(ShelleyBlock),
    Allegra(AllegraBlock),
    Mary(MaryBlock),
    Alonzo(AlonzoBlock),
    Babbage(BabbageBlock),
    Conway(Block),
}

impl MultiEraBlock {
    pub fn new_byron(byron: ByronBlock) -> Self {
        Self::Byron(byron)
    }

    pub fn new_shelley(shelley: ShelleyBlock) -> Self {
        Self::Shelley(shelley)
    }

    pub fn new_allegra(allegra: AllegraBlock) -> Self {
        Self::Allegra(allegra)
    }

    pub fn new_mary(mary: MaryBlock) -> Self {
        Self::Mary(mary)
    }

    pub fn new_alonzo(alonzo: AlonzoBlock) -> Self {
        Self::Alonzo(alonzo)
    }

    pub fn new_babbage(babbage: BabbageBlock) -> Self {
        Self::Babbage(babbage)
    }

    pub fn new_conway(conway: Block) -> Self {
        Self::Conway(conway)
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub enum MultiEraTransactionBody {
    Byron(ByronTx),
    Shelley(ShelleyTransactionBody),
    Allegra(AllegraTransactionBody),
    Mary(MaryTransactionBody),
    Alonzo(AlonzoTransactionBody),
    Babbage(BabbageTransactionBody),
    Conway(TransactionBody),
}

impl MultiEraTransactionBody {
    pub fn new_byron(byron: ByronTx) -> Self {
        Self::Byron(byron)
    }

    pub fn new_shelley(shelley: ShelleyTransactionBody) -> Self {
        Self::Shelley(shelley)
    }

    pub fn new_allegra(allegra: AllegraTransactionBody) -> Self {
        Self::Allegra(allegra)
    }

    pub fn new_mary(mary: MaryTransactionBody) -> Self {
        Self::Mary(mary)
    }

    pub fn new_alonzo(alonzo: AlonzoTransactionBody) -> Self {
        Self::Alonzo(alonzo)
    }

    pub fn new_babbage(babbage: BabbageTransactionBody) -> Self {
        Self::Babbage(babbage)
    }

    pub fn new_conway(conway: TransactionBody) -> Self {
        Self::Conway(conway)
    }
}
