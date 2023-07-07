// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

pub mod cbor_encodings;
pub mod serialization;

use crate::allegra::{AllegraAuxiliaryData, AllegraTransactionWitnessSet};
use cml_chain::TransactionIndex;
use cml_chain::assets::{Coin, Mint};
use cml_chain::certs::Certificate;
use cml_chain::crypto::AuxiliaryDataHash;
use cml_chain::transaction::{ShelleyTxOut, TransactionInput};
use cml_chain::Withdrawals;
use crate::shelley::{ShelleyHeader, ShelleyUpdate};
use cbor_encodings::{MaryBlockEncoding, MaryTransactionBodyEncoding, MaryTransactionEncoding};
use cml_core::ordered_hash_map::OrderedHashMap;
use std::collections::BTreeMap;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct MaryBlock {
    pub header: ShelleyHeader,
    pub transaction_bodies: Vec<MaryTransactionBody>,
    pub transaction_witness_sets: Vec<AllegraTransactionWitnessSet>,
    pub auxiliary_data_set: OrderedHashMap<TransactionIndex, AllegraAuxiliaryData>,
    #[serde(skip)]
    pub encodings: Option<MaryBlockEncoding>,
}

impl MaryBlock {
    pub fn new(
        header: ShelleyHeader,
        transaction_bodies: Vec<MaryTransactionBody>,
        transaction_witness_sets: Vec<AllegraTransactionWitnessSet>,
        auxiliary_data_set: OrderedHashMap<TransactionIndex, AllegraAuxiliaryData>,
    ) -> Self {
        Self {
            header,
            transaction_bodies,
            transaction_witness_sets,
            auxiliary_data_set,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct MaryTransaction {
    pub body: MaryTransactionBody,
    pub witness_set: AllegraTransactionWitnessSet,
    pub auxiliary_data: Option<AllegraAuxiliaryData>,
    #[serde(skip)]
    pub encodings: Option<MaryTransactionEncoding>,
}

impl MaryTransaction {
    pub fn new(
        body: MaryTransactionBody,
        witness_set: AllegraTransactionWitnessSet,
        auxiliary_data: Option<AllegraAuxiliaryData>,
    ) -> Self {
        Self {
            body,
            witness_set,
            auxiliary_data,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct MaryTransactionBody {
    pub inputs: Vec<TransactionInput>,
    pub outputs: Vec<ShelleyTxOut>,
    pub fee: Coin,
    pub ttl: Option<u64>,
    pub certs: Option<Vec<Certificate>>,
    pub withdrawals: Option<Withdrawals>,
    pub update: Option<ShelleyUpdate>,
    pub auxiliary_data_hash: Option<AuxiliaryDataHash>,
    pub validity_interval_start: Option<u64>,
    pub mint: Option<Mint>,
    #[serde(skip)]
    pub encodings: Option<MaryTransactionBodyEncoding>,
}

impl MaryTransactionBody {
    pub fn new(inputs: Vec<TransactionInput>, outputs: Vec<ShelleyTxOut>, fee: Coin) -> Self {
        Self {
            inputs,
            outputs,
            fee,
            ttl: None,
            certs: None,
            withdrawals: None,
            update: None,
            auxiliary_data_hash: None,
            validity_interval_start: None,
            mint: None,
            encodings: None,
        }
    }
}
