// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

pub mod cbor_encodings;
pub mod serialization;

use crate::shelley::{ShelleyHeader, ShelleyTransactionOutput, ShelleyUpdate};
use cbor_encodings::{
    AllegraBlockEncoding, AllegraTransactionBodyEncoding, AllegraTransactionEncoding,
    AllegraTransactionWitnessSetEncoding,
};
use cml_chain::assets::Coin;
use cml_chain::auxdata::{ShelleyAuxData, ShelleyMaAuxData};
use cml_chain::certs::Certificate;
use cml_chain::crypto::{AuxiliaryDataHash, BootstrapWitness, Vkeywitness};
use cml_chain::transaction::{NativeScript, TransactionInput};
use cml_chain::TransactionIndex;
use cml_chain::Withdrawals;
use cml_core::ordered_hash_map::OrderedHashMap;
use std::collections::BTreeMap;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub enum AllegraAuxiliaryData {
    ShelleyAuxData(ShelleyAuxData),
    ShelleyMaAuxData(ShelleyMaAuxData),
}

impl AllegraAuxiliaryData {
    pub fn new_shelley_aux_data(shelley_aux_data: ShelleyAuxData) -> Self {
        Self::ShelleyAuxData(shelley_aux_data)
    }

    pub fn new_shelley_ma_aux_data(shelley_ma_aux_data: ShelleyMaAuxData) -> Self {
        Self::ShelleyMaAuxData(shelley_ma_aux_data)
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct AllegraBlock {
    pub header: ShelleyHeader,
    pub transaction_bodies: Vec<AllegraTransactionBody>,
    pub transaction_witness_sets: Vec<AllegraTransactionWitnessSet>,
    pub auxiliary_data_set: OrderedHashMap<TransactionIndex, AllegraAuxiliaryData>,
    #[serde(skip)]
    pub encodings: Option<AllegraBlockEncoding>,
}

impl AllegraBlock {
    pub fn new(
        header: ShelleyHeader,
        transaction_bodies: Vec<AllegraTransactionBody>,
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
pub struct AllegraTransaction {
    pub body: AllegraTransactionBody,
    pub witness_set: AllegraTransactionWitnessSet,
    pub auxiliary_data: Option<AllegraAuxiliaryData>,
    #[serde(skip)]
    pub encodings: Option<AllegraTransactionEncoding>,
}

impl AllegraTransaction {
    pub fn new(
        body: AllegraTransactionBody,
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
pub struct AllegraTransactionBody {
    pub inputs: Vec<TransactionInput>,
    pub outputs: Vec<ShelleyTransactionOutput>,
    pub fee: Coin,
    pub ttl: Option<u64>,
    pub certs: Option<Vec<Certificate>>,
    pub withdrawals: Option<Withdrawals>,
    pub update: Option<ShelleyUpdate>,
    pub auxiliary_data_hash: Option<AuxiliaryDataHash>,
    pub validity_interval_start: Option<u64>,
    #[serde(skip)]
    pub encodings: Option<AllegraTransactionBodyEncoding>,
}

impl AllegraTransactionBody {
    pub fn new(
        inputs: Vec<TransactionInput>,
        outputs: Vec<ShelleyTransactionOutput>,
        fee: Coin,
    ) -> Self {
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
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct AllegraTransactionWitnessSet {
    pub vkeywitnesses: Option<Vec<Vkeywitness>>,
    pub native_scripts: Option<Vec<NativeScript>>,
    pub bootstrap_witnesses: Option<Vec<BootstrapWitness>>,
    #[serde(skip)]
    pub encodings: Option<AllegraTransactionWitnessSetEncoding>,
}

impl AllegraTransactionWitnessSet {
    pub fn new() -> Self {
        Self {
            vkeywitnesses: None,
            native_scripts: None,
            bootstrap_witnesses: None,
            encodings: None,
        }
    }
}

impl Default for AllegraTransactionWitnessSet {
    fn default() -> Self {
        Self::new()
    }
}
