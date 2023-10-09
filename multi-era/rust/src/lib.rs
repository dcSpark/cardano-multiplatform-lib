#![allow(clippy::too_many_arguments)]

extern crate derivative;
pub mod allegra;
pub mod alonzo;
pub mod babbage;
pub mod byron;
pub mod mary;
pub mod serialization;
pub mod shelley;
pub mod utils;
// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use crate::{
    allegra::AllegraBlock, alonzo::AlonzoBlock, babbage::BabbageBlock, byron::block::ByronBlock,
    mary::MaryBlock, shelley::ShelleyBlock,
};
use cml_chain::address::RewardAccount;
use cml_chain::block::Block;
use cml_chain::crypto::GenesisHash;

pub type GenesisHashList = Vec<GenesisHash>;

pub type RewardAccountList = Vec<RewardAccount>;

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
