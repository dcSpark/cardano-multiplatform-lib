// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

pub mod block;
pub mod delegation;
pub mod mpc;
pub mod serialization;
pub mod transaction;
pub mod update;
pub mod utils;

use cml_crypto::TransactionHash;
pub use utils::*;

pub type ByronBlockId = Blake2b256;

pub type ByronPubKey = Vec<u8>;

pub type ByronSignature = Vec<u8>;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ByronSlotId {
    pub epoch: EpochId,
    pub slot: u64,
}

impl ByronSlotId {
    pub fn new(epoch: EpochId, slot: u64) -> Self {
        Self { epoch, slot }
    }
}

pub type ByronTxId = TransactionHash;

pub type ByronUpdateId = Blake2b256;

pub type EpochId = u64;
