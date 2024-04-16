// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

pub mod serialization;
pub mod utils;

use crate::byron::{Blake2b256, ByronPubKey, ByronSignature, ByronTxId};

use cml_chain::byron::ByronTxOut;
use std::collections::BTreeMap;

use super::ByronAny;

pub type ByronAttributes = BTreeMap<ByronAny, ByronAny>;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ByronPkWitness {
    pub index_1: ByronPkWitnessEntry,
}

impl ByronPkWitness {
    pub fn new(index_1: ByronPkWitnessEntry) -> Self {
        Self { index_1 }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ByronPkWitnessEntry {
    pub byron_pub_key: ByronPubKey,
    pub byron_signature: ByronSignature,
}

impl ByronPkWitnessEntry {
    pub fn new(byron_pub_key: ByronPubKey, byron_signature: ByronSignature) -> Self {
        Self {
            byron_pub_key,
            byron_signature,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ByronRedeemWitness {
    pub index_1: ByronRedeemerWitnessEntry,
}

impl ByronRedeemWitness {
    pub fn new(index_1: ByronRedeemerWitnessEntry) -> Self {
        Self { index_1 }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ByronRedeemerScript {
    pub u16: u16,
    pub index_1: Vec<u8>,
}

impl ByronRedeemerScript {
    pub fn new(u16: u16, index_1: Vec<u8>) -> Self {
        Self { u16, index_1 }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ByronRedeemerWitnessEntry {
    pub byron_pub_key: ByronPubKey,
    pub byron_signature: ByronSignature,
}

impl ByronRedeemerWitnessEntry {
    pub fn new(byron_pub_key: ByronPubKey, byron_signature: ByronSignature) -> Self {
        Self {
            byron_pub_key,
            byron_signature,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ByronScriptWitness {
    pub index_1: ByronScriptWitnessEntry,
}

impl ByronScriptWitness {
    pub fn new(index_1: ByronScriptWitnessEntry) -> Self {
        Self { index_1 }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ByronScriptWitnessEntry {
    pub byron_validator_script: ByronValidatorScript,
    pub byron_redeemer_script: ByronRedeemerScript,
}

impl ByronScriptWitnessEntry {
    pub fn new(
        byron_validator_script: ByronValidatorScript,
        byron_redeemer_script: ByronRedeemerScript,
    ) -> Self {
        Self {
            byron_validator_script,
            byron_redeemer_script,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ByronTx {
    pub inputs: Vec<ByronTxIn>,
    pub outputs: Vec<ByronTxOut>,
    pub attrs: ByronAttributes,
}

impl ByronTx {
    pub fn new(inputs: Vec<ByronTxIn>, outputs: Vec<ByronTxOut>, attrs: ByronAttributes) -> Self {
        Self {
            inputs,
            outputs,
            attrs,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub enum ByronTxIn {
    ByronTxInRegular(ByronTxInRegular),
    ByronTxInGenesis(ByronTxInGenesis),
}

impl ByronTxIn {
    pub fn new_byron_tx_in_regular(byron_tx_in_regular: ByronTxInRegular) -> Self {
        Self::ByronTxInRegular(byron_tx_in_regular)
    }

    pub fn new_byron_tx_in_genesis(byron_tx_in_genesis: ByronTxInGenesis) -> Self {
        Self::ByronTxInGenesis(byron_tx_in_genesis)
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ByronTxInGenesis {
    pub u8: u8,
    pub index_1: Vec<u8>,
}

impl ByronTxInGenesis {
    pub fn new(u8: u8, index_1: Vec<u8>) -> Self {
        Self { u8, index_1 }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ByronTxInRegular {
    pub index_1: ByronTxOutPtr,
}

impl ByronTxInRegular {
    pub fn new(index_1: ByronTxOutPtr) -> Self {
        Self { index_1 }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ByronTxOutPtr {
    pub byron_tx_id: ByronTxId,
    pub u32: u32,
}

impl ByronTxOutPtr {
    pub fn new(byron_tx_id: ByronTxId, u32: u32) -> Self {
        Self { byron_tx_id, u32 }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ByronTxProof {
    pub u32: u32,
    pub blake2b256: Blake2b256,
    pub blake2b2562: Blake2b256,
}

impl ByronTxProof {
    pub fn new(u32: u32, blake2b256: Blake2b256, blake2b2562: Blake2b256) -> Self {
        Self {
            u32,
            blake2b256,
            blake2b2562,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub enum ByronTxWitness {
    ByronPkWitness(ByronPkWitness),
    ByronScriptWitness(ByronScriptWitness),
    ByronRedeemWitness(ByronRedeemWitness),
}

impl ByronTxWitness {
    pub fn new_byron_pk_witness(index_1: ByronPkWitnessEntry) -> Self {
        Self::ByronPkWitness(ByronPkWitness::new(index_1))
    }

    pub fn new_byron_script_witness(index_1: ByronScriptWitnessEntry) -> Self {
        Self::ByronScriptWitness(ByronScriptWitness::new(index_1))
    }

    pub fn new_byron_redeem_witness(index_1: ByronRedeemerWitnessEntry) -> Self {
        Self::ByronRedeemWitness(ByronRedeemWitness::new(index_1))
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ByronValidatorScript {
    pub u16: u16,
    pub index_1: Vec<u8>,
}

impl ByronValidatorScript {
    pub fn new(u16: u16, index_1: Vec<u8>) -> Self {
        Self { u16, index_1 }
    }
}
