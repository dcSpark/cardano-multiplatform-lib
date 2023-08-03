// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use cml_chain_wasm::assets::Coin;
use cml_chain_wasm::auxdata::{ShelleyAuxData, ShelleyMaAuxData};
use cml_crypto_wasm::AuxiliaryDataHash;
use cml_chain_wasm::Withdrawals;
use crate::shelley::{ShelleyHeader, ShelleyUpdate};
use cml_chain_wasm::{
    BootstrapWitnessList, NativeScriptList, VkeywitnessList, TransactionInputList, CertificateList, 
};
use cml_core_wasm::{impl_wasm_cbor_json_api, impl_wasm_conversions};
use crate::{
    AllegraTransactionBodyList, AllegraTransactionWitnessSetList,
    MapTransactionIndexToAllegraAuxiliaryData, ShelleyTransactionOutputList,
};
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct AllegraAuxiliaryData(cml_multi_era::allegra::AllegraAuxiliaryData);

impl_wasm_cbor_json_api!(AllegraAuxiliaryData);

impl_wasm_conversions!(
    cml_multi_era::allegra::AllegraAuxiliaryData,
    AllegraAuxiliaryData
);

#[wasm_bindgen]
impl AllegraAuxiliaryData {
    pub fn new_shelley_aux_data(shelley_aux_data: &ShelleyAuxData) -> Self {
        Self(
            cml_multi_era::allegra::AllegraAuxiliaryData::new_shelley_aux_data(
                shelley_aux_data.clone().into(),
            ),
        )
    }

    pub fn new_shelley_ma_aux_data(shelley_ma_aux_data: &ShelleyMaAuxData) -> Self {
        Self(
            cml_multi_era::allegra::AllegraAuxiliaryData::new_shelley_ma_aux_data(
                shelley_ma_aux_data.clone().into(),
            ),
        )
    }

    pub fn kind(&self) -> AllegraAuxiliaryDataKind {
        match &self.0 {
            cml_multi_era::allegra::AllegraAuxiliaryData::ShelleyAuxData(_) => {
                AllegraAuxiliaryDataKind::ShelleyAuxData
            }
            cml_multi_era::allegra::AllegraAuxiliaryData::ShelleyMaAuxData(_) => {
                AllegraAuxiliaryDataKind::ShelleyMaAuxData
            }
        }
    }

    pub fn as_shelley_aux_data(&self) -> Option<ShelleyAuxData> {
        match &self.0 {
            cml_multi_era::allegra::AllegraAuxiliaryData::ShelleyAuxData(shelley_aux_data) => {
                Some(shelley_aux_data.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_shelley_ma_aux_data(&self) -> Option<ShelleyMaAuxData> {
        match &self.0 {
            cml_multi_era::allegra::AllegraAuxiliaryData::ShelleyMaAuxData(shelley_ma_aux_data) => {
                Some(shelley_ma_aux_data.clone().into())
            }
            _ => None,
        }
    }
}

#[wasm_bindgen]
pub enum AllegraAuxiliaryDataKind {
    ShelleyAuxData,
    ShelleyMaAuxData,
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct AllegraBlock(cml_multi_era::allegra::AllegraBlock);

impl_wasm_cbor_json_api!(AllegraBlock);

impl_wasm_conversions!(cml_multi_era::allegra::AllegraBlock, AllegraBlock);

#[wasm_bindgen]
impl AllegraBlock {
    pub fn header(&self) -> ShelleyHeader {
        self.0.header.clone().into()
    }

    pub fn transaction_bodies(&self) -> AllegraTransactionBodyList {
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
        transaction_bodies: &AllegraTransactionBodyList,
        transaction_witness_sets: &AllegraTransactionWitnessSetList,
        auxiliary_data_set: &MapTransactionIndexToAllegraAuxiliaryData,
    ) -> Self {
        Self(cml_multi_era::allegra::AllegraBlock::new(
            header.clone().into(),
            transaction_bodies.clone().into(),
            transaction_witness_sets.clone().into(),
            auxiliary_data_set.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct AllegraTransaction(cml_multi_era::allegra::AllegraTransaction);

impl_wasm_cbor_json_api!(AllegraTransaction);

impl_wasm_conversions!(
    cml_multi_era::allegra::AllegraTransaction,
    AllegraTransaction
);

#[wasm_bindgen]
impl AllegraTransaction {
    pub fn body(&self) -> AllegraTransactionBody {
        self.0.body.clone().into()
    }

    pub fn witness_set(&self) -> AllegraTransactionWitnessSet {
        self.0.witness_set.clone().into()
    }

    pub fn auxiliary_data(&self) -> Option<AllegraAuxiliaryData> {
        self.0.auxiliary_data.clone().map(std::convert::Into::into)
    }

    pub fn new(
        body: &AllegraTransactionBody,
        witness_set: &AllegraTransactionWitnessSet,
        auxiliary_data: Option<AllegraAuxiliaryData>,
    ) -> Self {
        Self(cml_multi_era::allegra::AllegraTransaction::new(
            body.clone().into(),
            witness_set.clone().into(),
            auxiliary_data.map(Into::into),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct AllegraTransactionBody(cml_multi_era::allegra::AllegraTransactionBody);

impl_wasm_cbor_json_api!(AllegraTransactionBody);

impl_wasm_conversions!(
    cml_multi_era::allegra::AllegraTransactionBody,
    AllegraTransactionBody
);

#[wasm_bindgen]
impl AllegraTransactionBody {
    pub fn inputs(&self) -> TransactionInputList {
        self.0.inputs.clone().into()
    }

    pub fn outputs(&self) -> ShelleyTransactionOutputList {
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

    pub fn set_certs(&mut self, certs: &CertificateList) {
        self.0.certs = Some(certs.clone().into())
    }

    pub fn certs(&self) -> Option<CertificateList> {
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
        self.0
            .auxiliary_data_hash
            .clone()
            .map(std::convert::Into::into)
    }

    pub fn set_validity_interval_start(&mut self, validity_interval_start: u64) {
        self.0.validity_interval_start = Some(validity_interval_start)
    }

    pub fn validity_interval_start(&self) -> Option<u64> {
        self.0.validity_interval_start
    }

    pub fn new(
        inputs: &TransactionInputList,
        outputs: &ShelleyTransactionOutputList,
        fee: Coin,
    ) -> Self {
        Self(cml_multi_era::allegra::AllegraTransactionBody::new(
            inputs.clone().into(),
            outputs.clone().into(),
            fee,
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct AllegraTransactionWitnessSet(cml_multi_era::allegra::AllegraTransactionWitnessSet);

impl_wasm_cbor_json_api!(AllegraTransactionWitnessSet);

impl_wasm_conversions!(
    cml_multi_era::allegra::AllegraTransactionWitnessSet,
    AllegraTransactionWitnessSet
);

#[wasm_bindgen]
impl AllegraTransactionWitnessSet {
    pub fn set_vkeywitnesses(&mut self, vkeywitnesses: &VkeywitnessList) {
        self.0.vkeywitnesses = Some(vkeywitnesses.clone().into())
    }

    pub fn vkeywitnesses(&self) -> Option<VkeywitnessList> {
        self.0.vkeywitnesses.clone().map(std::convert::Into::into)
    }

    pub fn set_native_scripts(&mut self, native_scripts: &NativeScriptList) {
        self.0.native_scripts = Some(native_scripts.clone().into())
    }

    pub fn native_scripts(&self) -> Option<NativeScriptList> {
        self.0.native_scripts.clone().map(std::convert::Into::into)
    }

    pub fn set_bootstrap_witnesses(&mut self, bootstrap_witnesses: &BootstrapWitnessList) {
        self.0.bootstrap_witnesses = Some(bootstrap_witnesses.clone().into())
    }

    pub fn bootstrap_witnesses(&self) -> Option<BootstrapWitnessList> {
        self.0
            .bootstrap_witnesses
            .clone()
            .map(std::convert::Into::into)
    }

    pub fn new() -> Self {
        Self(cml_multi_era::allegra::AllegraTransactionWitnessSet::new())
    }
}
