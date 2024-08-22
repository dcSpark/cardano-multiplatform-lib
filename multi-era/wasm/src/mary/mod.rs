// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use crate::allegra::{AllegraAuxiliaryData, AllegraTransactionWitnessSet};
use crate::shelley::{ShelleyHeader, ShelleyUpdate};
use crate::{
    AllegraCertificateList, AllegraTransactionWitnessSetList,
    MapTransactionIndexToAllegraAuxiliaryData, MaryTransactionBodyList, MaryTransactionOutputList,
};
use cml_chain_wasm::assets::{Coin, Mint};
use cml_chain_wasm::TransactionInputList;
use cml_chain_wasm::{address::Address, Value, Withdrawals};
use cml_core_wasm::{impl_wasm_cbor_json_api, impl_wasm_conversions};
use cml_crypto_wasm::AuxiliaryDataHash;
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MaryBlock(cml_multi_era::mary::MaryBlock);

impl_wasm_cbor_json_api!(MaryBlock);

impl_wasm_conversions!(cml_multi_era::mary::MaryBlock, MaryBlock);

#[wasm_bindgen]
impl MaryBlock {
    pub fn header(&self) -> ShelleyHeader {
        self.0.header.clone().into()
    }

    pub fn transaction_bodies(&self) -> MaryTransactionBodyList {
        self.0.transaction_bodies.clone().into()
    }

    pub fn transaction_witness_sets(&self) -> AllegraTransactionWitnessSetList {
        self.0.transaction_witness_sets.clone().into()
    }

    pub fn auxiliary_data_set(&self) -> MapTransactionIndexToAllegraAuxiliaryData {
        self.0.auxiliary_data_set.clone().into()
    }

    pub fn new(
        header: &ShelleyHeader,
        transaction_bodies: &MaryTransactionBodyList,
        transaction_witness_sets: &AllegraTransactionWitnessSetList,
        auxiliary_data_set: &MapTransactionIndexToAllegraAuxiliaryData,
    ) -> Self {
        Self(cml_multi_era::mary::MaryBlock::new(
            header.clone().into(),
            transaction_bodies.clone().into(),
            transaction_witness_sets.clone().into(),
            auxiliary_data_set.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MaryTransaction(cml_multi_era::mary::MaryTransaction);

impl_wasm_cbor_json_api!(MaryTransaction);

impl_wasm_conversions!(cml_multi_era::mary::MaryTransaction, MaryTransaction);

#[wasm_bindgen]
impl MaryTransaction {
    pub fn body(&self) -> MaryTransactionBody {
        self.0.body.clone().into()
    }

    pub fn witness_set(&self) -> AllegraTransactionWitnessSet {
        self.0.witness_set.clone().into()
    }

    pub fn auxiliary_data(&self) -> Option<AllegraAuxiliaryData> {
        self.0.auxiliary_data.clone().map(std::convert::Into::into)
    }

    pub fn new(
        body: &MaryTransactionBody,
        witness_set: &AllegraTransactionWitnessSet,
        auxiliary_data: Option<AllegraAuxiliaryData>,
    ) -> Self {
        Self(cml_multi_era::mary::MaryTransaction::new(
            body.clone().into(),
            witness_set.clone().into(),
            auxiliary_data.map(Into::into),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MaryTransactionBody(cml_multi_era::mary::MaryTransactionBody);

impl_wasm_cbor_json_api!(MaryTransactionBody);

impl_wasm_conversions!(
    cml_multi_era::mary::MaryTransactionBody,
    MaryTransactionBody
);

#[wasm_bindgen]
impl MaryTransactionBody {
    pub fn inputs(&self) -> TransactionInputList {
        self.0.inputs.clone().into()
    }

    pub fn outputs(&self) -> MaryTransactionOutputList {
        self.0.outputs.clone().into()
    }

    pub fn fee(&self) -> Coin {
        self.0.fee
    }

    pub fn set_ttl(&mut self, ttl: u64) {
        self.0.ttl = Some(ttl)
    }

    pub fn ttl(&self) -> Option<u64> {
        self.0.ttl
    }

    pub fn set_certs(&mut self, certs: &AllegraCertificateList) {
        self.0.certs = Some(certs.clone().into())
    }

    pub fn certs(&self) -> Option<AllegraCertificateList> {
        self.0.certs.clone().map(std::convert::Into::into)
    }

    pub fn set_withdrawals(&mut self, withdrawals: &Withdrawals) {
        self.0.withdrawals = Some(withdrawals.clone().into())
    }

    pub fn withdrawals(&self) -> Option<Withdrawals> {
        self.0.withdrawals.clone().map(std::convert::Into::into)
    }

    pub fn set_update(&mut self, update: &ShelleyUpdate) {
        self.0.update = Some(update.clone().into())
    }

    pub fn update(&self) -> Option<ShelleyUpdate> {
        self.0.update.clone().map(std::convert::Into::into)
    }

    pub fn set_auxiliary_data_hash(&mut self, auxiliary_data_hash: &AuxiliaryDataHash) {
        self.0.auxiliary_data_hash = Some(auxiliary_data_hash.clone().into())
    }

    pub fn auxiliary_data_hash(&self) -> Option<AuxiliaryDataHash> {
        self.0.auxiliary_data_hash.map(std::convert::Into::into)
    }

    pub fn set_validity_interval_start(&mut self, validity_interval_start: u64) {
        self.0.validity_interval_start = Some(validity_interval_start)
    }

    pub fn validity_interval_start(&self) -> Option<u64> {
        self.0.validity_interval_start
    }

    pub fn set_mint(&mut self, mint: &Mint) {
        self.0.mint = Some(mint.clone().into())
    }

    pub fn mint(&self) -> Option<Mint> {
        self.0.mint.clone().map(std::convert::Into::into)
    }

    pub fn new(
        inputs: &TransactionInputList,
        outputs: &MaryTransactionOutputList,
        fee: Coin,
    ) -> Self {
        Self(cml_multi_era::mary::MaryTransactionBody::new(
            inputs.clone().into(),
            outputs.clone().into(),
            fee,
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MaryTransactionOutput(cml_multi_era::mary::MaryTransactionOutput);

impl_wasm_cbor_json_api!(MaryTransactionOutput);

impl_wasm_conversions!(
    cml_multi_era::mary::MaryTransactionOutput,
    MaryTransactionOutput
);

#[wasm_bindgen]
impl MaryTransactionOutput {
    pub fn address(&self) -> Address {
        self.0.address.clone().into()
    }

    pub fn amount(&self) -> Value {
        self.0.amount.clone().into()
    }

    pub fn new(address: &Address, amount: &Value) -> Self {
        Self(cml_multi_era::mary::MaryTransactionOutput::new(
            address.clone().into(),
            amount.clone().into(),
        ))
    }
}
