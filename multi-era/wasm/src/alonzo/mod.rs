// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use crate::shelley::{ProtocolVersionStruct, ShelleyHeader};
use crate::{
    AllegraCertificateList, AlonzoFormatTxOutList, AlonzoRedeemerList, AlonzoTransactionBodyList,
    AlonzoTransactionWitnessSetList, GenesisHashList, MapTransactionIndexToAlonzoAuxiliaryData,
};
use cml_chain_wasm::assets::{Coin, Mint};
use cml_chain_wasm::auxdata::{Metadata, ShelleyFormatAuxData, ShelleyMAFormatAuxData};
use cml_chain_wasm::crypto::Nonce;
use cml_chain_wasm::plutus::{CostModels, ExUnitPrices, ExUnits, PlutusData};
use cml_chain_wasm::RequiredSigners;
use cml_chain_wasm::TransactionIndex;
use cml_chain_wasm::{
    BootstrapWitnessList, NativeScriptList, PlutusDataList, PlutusV1ScriptList,
    TransactionInputList, VkeywitnessList,
};
use cml_chain_wasm::{Epoch, NetworkId, Rational, UnitInterval, Withdrawals};
use cml_core::ordered_hash_map::OrderedHashMap;
use cml_core_wasm::{impl_wasm_cbor_json_api, impl_wasm_conversions};
use cml_crypto_wasm::{AuxiliaryDataHash, GenesisHash, ScriptDataHash};
use cml_multi_era::alonzo::AlonzoRedeemerTag;
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct AlonzoAuxiliaryData(cml_multi_era::alonzo::AlonzoAuxiliaryData);

impl_wasm_cbor_json_api!(AlonzoAuxiliaryData);

impl_wasm_conversions!(
    cml_multi_era::alonzo::AlonzoAuxiliaryData,
    AlonzoAuxiliaryData
);

#[wasm_bindgen]
impl AlonzoAuxiliaryData {
    pub fn new_shelley(shelley: &ShelleyFormatAuxData) -> Self {
        Self(cml_multi_era::alonzo::AlonzoAuxiliaryData::new_shelley(
            shelley.clone().into(),
        ))
    }

    pub fn new_shelley_ma(shelley_ma: &ShelleyMAFormatAuxData) -> Self {
        Self(cml_multi_era::alonzo::AlonzoAuxiliaryData::new_shelley_ma(
            shelley_ma.clone().into(),
        ))
    }

    pub fn new_alonzo(alonzo: &AlonzoFormatAuxData) -> Self {
        Self(cml_multi_era::alonzo::AlonzoAuxiliaryData::new_alonzo(
            alonzo.clone().into(),
        ))
    }

    pub fn kind(&self) -> AlonzoAuxiliaryDataKind {
        match &self.0 {
            cml_multi_era::alonzo::AlonzoAuxiliaryData::Shelley(_) => {
                AlonzoAuxiliaryDataKind::Shelley
            }
            cml_multi_era::alonzo::AlonzoAuxiliaryData::ShelleyMA(_) => {
                AlonzoAuxiliaryDataKind::ShelleyMA
            }
            cml_multi_era::alonzo::AlonzoAuxiliaryData::Alonzo(_) => {
                AlonzoAuxiliaryDataKind::Alonzo
            }
        }
    }

    pub fn as_shelley(&self) -> Option<ShelleyFormatAuxData> {
        match &self.0 {
            cml_multi_era::alonzo::AlonzoAuxiliaryData::Shelley(shelley) => {
                Some(shelley.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_shelley_ma(&self) -> Option<ShelleyMAFormatAuxData> {
        match &self.0 {
            cml_multi_era::alonzo::AlonzoAuxiliaryData::ShelleyMA(shelley_ma) => {
                Some(shelley_ma.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_alonzo(&self) -> Option<AlonzoFormatAuxData> {
        match &self.0 {
            cml_multi_era::alonzo::AlonzoAuxiliaryData::Alonzo(alonzo) => {
                Some(alonzo.clone().into())
            }
            _ => None,
        }
    }
}

#[wasm_bindgen]
pub enum AlonzoAuxiliaryDataKind {
    Shelley,
    ShelleyMA,
    Alonzo,
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct AlonzoBlock(cml_multi_era::alonzo::AlonzoBlock);

impl_wasm_cbor_json_api!(AlonzoBlock);

impl_wasm_conversions!(cml_multi_era::alonzo::AlonzoBlock, AlonzoBlock);

#[wasm_bindgen]
impl AlonzoBlock {
    pub fn header(&self) -> ShelleyHeader {
        self.0.header.clone().into()
    }

    pub fn transaction_bodies(&self) -> AlonzoTransactionBodyList {
        self.0.transaction_bodies.clone().into()
    }

    pub fn transaction_witness_sets(&self) -> AlonzoTransactionWitnessSetList {
        self.0.transaction_witness_sets.clone().into()
    }

    pub fn auxiliary_data_set(&self) -> MapTransactionIndexToAlonzoAuxiliaryData {
        self.0.auxiliary_data_set.clone().into()
    }

    pub fn invalid_transactions(&self) -> Vec<TransactionIndex> {
        self.0.invalid_transactions.clone()
    }

    pub fn new(
        header: &ShelleyHeader,
        transaction_bodies: &AlonzoTransactionBodyList,
        transaction_witness_sets: &AlonzoTransactionWitnessSetList,
        auxiliary_data_set: &MapTransactionIndexToAlonzoAuxiliaryData,
        invalid_transactions: Vec<TransactionIndex>,
    ) -> Self {
        Self(cml_multi_era::alonzo::AlonzoBlock::new(
            header.clone().into(),
            transaction_bodies.clone().into(),
            transaction_witness_sets.clone().into(),
            auxiliary_data_set.clone().into(),
            invalid_transactions,
        ))
    }
}

pub type AlonzoCostModels = CostModels;

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct AlonzoFormatAuxData(cml_multi_era::alonzo::AlonzoFormatAuxData);

impl_wasm_cbor_json_api!(AlonzoFormatAuxData);

impl_wasm_conversions!(
    cml_multi_era::alonzo::AlonzoFormatAuxData,
    AlonzoFormatAuxData
);

#[wasm_bindgen]
impl AlonzoFormatAuxData {
    pub fn set_metadata(&mut self, metadata: &Metadata) {
        self.0.metadata = Some(metadata.clone().into())
    }

    pub fn metadata(&self) -> Option<Metadata> {
        self.0.metadata.clone().map(std::convert::Into::into)
    }

    pub fn set_native_scripts(&mut self, native_scripts: &NativeScriptList) {
        self.0.native_scripts = Some(native_scripts.clone().into())
    }

    pub fn native_scripts(&self) -> Option<NativeScriptList> {
        self.0.native_scripts.clone().map(std::convert::Into::into)
    }

    pub fn set_plutus_v1_scripts(&mut self, plutus_v1_scripts: &PlutusV1ScriptList) {
        self.0.plutus_v1_scripts = Some(plutus_v1_scripts.clone().into())
    }

    pub fn plutus_v1_scripts(&self) -> Option<PlutusV1ScriptList> {
        self.0
            .plutus_v1_scripts
            .clone()
            .map(std::convert::Into::into)
    }

    pub fn new() -> Self {
        Self(cml_multi_era::alonzo::AlonzoFormatAuxData::new())
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct AlonzoProposedProtocolParameterUpdates(
    cml_multi_era::alonzo::AlonzoProposedProtocolParameterUpdates,
);

impl_wasm_conversions!(
    cml_multi_era::alonzo::AlonzoProposedProtocolParameterUpdates,
    AlonzoProposedProtocolParameterUpdates
);

#[wasm_bindgen]
impl AlonzoProposedProtocolParameterUpdates {
    pub fn new() -> Self {
        Self(OrderedHashMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(
        &mut self,
        key: &GenesisHash,
        value: &AlonzoProtocolParamUpdate,
    ) -> Option<AlonzoProtocolParamUpdate> {
        self.0
            .insert(key.clone().into(), value.clone().into())
            .map(Into::into)
    }

    pub fn get(&self, key: &GenesisHash) -> Option<AlonzoProtocolParamUpdate> {
        self.0.get(key.as_ref()).map(|v| v.clone().into())
    }

    pub fn keys(&self) -> GenesisHashList {
        self.0.iter().map(|(k, _v)| *k).collect::<Vec<_>>().into()
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct AlonzoProtocolParamUpdate(cml_multi_era::alonzo::AlonzoProtocolParamUpdate);

impl_wasm_cbor_json_api!(AlonzoProtocolParamUpdate);

impl_wasm_conversions!(
    cml_multi_era::alonzo::AlonzoProtocolParamUpdate,
    AlonzoProtocolParamUpdate
);

#[wasm_bindgen]
impl AlonzoProtocolParamUpdate {
    pub fn set_minfee_a(&mut self, minfee_a: u64) {
        self.0.minfee_a = Some(minfee_a)
    }

    pub fn minfee_a(&self) -> Option<u64> {
        self.0.minfee_a
    }

    pub fn set_minfee_b(&mut self, minfee_b: u64) {
        self.0.minfee_b = Some(minfee_b)
    }

    pub fn minfee_b(&self) -> Option<u64> {
        self.0.minfee_b
    }

    pub fn set_max_block_body_size(&mut self, max_block_body_size: u64) {
        self.0.max_block_body_size = Some(max_block_body_size)
    }

    pub fn max_block_body_size(&self) -> Option<u64> {
        self.0.max_block_body_size
    }

    pub fn set_max_transaction_size(&mut self, max_transaction_size: u64) {
        self.0.max_transaction_size = Some(max_transaction_size)
    }

    pub fn max_transaction_size(&self) -> Option<u64> {
        self.0.max_transaction_size
    }

    pub fn set_max_block_header_size(&mut self, max_block_header_size: u64) {
        self.0.max_block_header_size = Some(max_block_header_size)
    }

    pub fn max_block_header_size(&self) -> Option<u64> {
        self.0.max_block_header_size
    }

    pub fn set_key_deposit(&mut self, key_deposit: Coin) {
        self.0.key_deposit = Some(key_deposit)
    }

    pub fn key_deposit(&self) -> Option<Coin> {
        self.0.key_deposit
    }

    pub fn set_pool_deposit(&mut self, pool_deposit: Coin) {
        self.0.pool_deposit = Some(pool_deposit)
    }

    pub fn pool_deposit(&self) -> Option<Coin> {
        self.0.pool_deposit
    }

    pub fn set_maximum_epoch(&mut self, maximum_epoch: Epoch) {
        self.0.maximum_epoch = Some(maximum_epoch)
    }

    pub fn maximum_epoch(&self) -> Option<Epoch> {
        self.0.maximum_epoch
    }

    pub fn set_n_opt(&mut self, n_opt: u64) {
        self.0.n_opt = Some(n_opt)
    }

    pub fn n_opt(&self) -> Option<u64> {
        self.0.n_opt
    }

    pub fn set_pool_pledge_influence(&mut self, pool_pledge_influence: &Rational) {
        self.0.pool_pledge_influence = Some(pool_pledge_influence.clone().into())
    }

    pub fn pool_pledge_influence(&self) -> Option<Rational> {
        self.0
            .pool_pledge_influence
            .clone()
            .map(std::convert::Into::into)
    }

    pub fn set_expansion_rate(&mut self, expansion_rate: &UnitInterval) {
        self.0.expansion_rate = Some(expansion_rate.clone().into())
    }

    pub fn expansion_rate(&self) -> Option<UnitInterval> {
        self.0.expansion_rate.clone().map(std::convert::Into::into)
    }

    pub fn set_treasury_growth_rate(&mut self, treasury_growth_rate: &UnitInterval) {
        self.0.treasury_growth_rate = Some(treasury_growth_rate.clone().into())
    }

    pub fn treasury_growth_rate(&self) -> Option<UnitInterval> {
        self.0
            .treasury_growth_rate
            .clone()
            .map(std::convert::Into::into)
    }

    pub fn set_decentralization_constant(&mut self, decentralization_constant: &UnitInterval) {
        self.0.decentralization_constant = Some(decentralization_constant.clone().into())
    }

    pub fn decentralization_constant(&self) -> Option<UnitInterval> {
        self.0
            .decentralization_constant
            .clone()
            .map(std::convert::Into::into)
    }

    pub fn set_extra_entropy(&mut self, extra_entropy: &Nonce) {
        self.0.extra_entropy = Some(extra_entropy.clone().into())
    }

    pub fn extra_entropy(&self) -> Option<Nonce> {
        self.0.extra_entropy.clone().map(std::convert::Into::into)
    }

    pub fn set_protocol_version(&mut self, protocol_version: &ProtocolVersionStruct) {
        self.0.protocol_version = Some(protocol_version.clone().into())
    }

    pub fn protocol_version(&self) -> Option<ProtocolVersionStruct> {
        self.0
            .protocol_version
            .clone()
            .map(std::convert::Into::into)
    }

    pub fn set_min_pool_cost(&mut self, min_pool_cost: Coin) {
        self.0.min_pool_cost = Some(min_pool_cost)
    }

    pub fn min_pool_cost(&self) -> Option<Coin> {
        self.0.min_pool_cost
    }

    pub fn set_ada_per_utxo_byte(&mut self, ada_per_utxo_byte: Coin) {
        self.0.ada_per_utxo_byte = Some(ada_per_utxo_byte)
    }

    pub fn ada_per_utxo_byte(&self) -> Option<Coin> {
        self.0.ada_per_utxo_byte
    }

    pub fn set_cost_models_for_script_languages(
        &mut self,
        cost_models_for_script_languages: &AlonzoCostModels,
    ) {
        self.0.cost_models_for_script_languages =
            Some(cost_models_for_script_languages.clone().into())
    }

    pub fn cost_models_for_script_languages(&self) -> Option<AlonzoCostModels> {
        self.0
            .cost_models_for_script_languages
            .clone()
            .map(std::convert::Into::into)
    }

    pub fn set_execution_costs(&mut self, execution_costs: &ExUnitPrices) {
        self.0.execution_costs = Some(execution_costs.clone().into())
    }

    pub fn execution_costs(&self) -> Option<ExUnitPrices> {
        self.0.execution_costs.clone().map(std::convert::Into::into)
    }

    pub fn set_max_tx_ex_units(&mut self, max_tx_ex_units: &ExUnits) {
        self.0.max_tx_ex_units = Some(max_tx_ex_units.clone().into())
    }

    pub fn max_tx_ex_units(&self) -> Option<ExUnits> {
        self.0.max_tx_ex_units.clone().map(std::convert::Into::into)
    }

    pub fn set_max_block_ex_units(&mut self, max_block_ex_units: &ExUnits) {
        self.0.max_block_ex_units = Some(max_block_ex_units.clone().into())
    }

    pub fn max_block_ex_units(&self) -> Option<ExUnits> {
        self.0
            .max_block_ex_units
            .clone()
            .map(std::convert::Into::into)
    }

    pub fn set_max_value_size(&mut self, max_value_size: u64) {
        self.0.max_value_size = Some(max_value_size)
    }

    pub fn max_value_size(&self) -> Option<u64> {
        self.0.max_value_size
    }

    pub fn set_collateral_percentage(&mut self, collateral_percentage: u64) {
        self.0.collateral_percentage = Some(collateral_percentage)
    }

    pub fn collateral_percentage(&self) -> Option<u64> {
        self.0.collateral_percentage
    }

    pub fn set_max_collateral_inputs(&mut self, max_collateral_inputs: u64) {
        self.0.max_collateral_inputs = Some(max_collateral_inputs)
    }

    pub fn max_collateral_inputs(&self) -> Option<u64> {
        self.0.max_collateral_inputs
    }

    pub fn new() -> Self {
        Self(cml_multi_era::alonzo::AlonzoProtocolParamUpdate::new())
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct AlonzoRedeemer(cml_multi_era::alonzo::AlonzoRedeemer);

impl_wasm_cbor_json_api!(AlonzoRedeemer);

impl_wasm_conversions!(cml_multi_era::alonzo::AlonzoRedeemer, AlonzoRedeemer);

#[wasm_bindgen]
impl AlonzoRedeemer {
    pub fn tag(&self) -> AlonzoRedeemerTag {
        self.0.tag
    }

    pub fn index(&self) -> u64 {
        self.0.index
    }

    pub fn data(&self) -> PlutusData {
        self.0.data.clone().into()
    }

    pub fn ex_units(&self) -> ExUnits {
        self.0.ex_units.clone().into()
    }

    pub fn new(tag: AlonzoRedeemerTag, index: u64, data: &PlutusData, ex_units: &ExUnits) -> Self {
        Self(cml_multi_era::alonzo::AlonzoRedeemer::new(
            tag,
            index,
            data.clone().into(),
            ex_units.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct AlonzoTransaction(cml_multi_era::alonzo::AlonzoTransaction);

impl_wasm_cbor_json_api!(AlonzoTransaction);

impl_wasm_conversions!(cml_multi_era::alonzo::AlonzoTransaction, AlonzoTransaction);

#[wasm_bindgen]
impl AlonzoTransaction {
    pub fn body(&self) -> AlonzoTransactionBody {
        self.0.body.clone().into()
    }

    pub fn witness_set(&self) -> AlonzoTransactionWitnessSet {
        self.0.witness_set.clone().into()
    }

    pub fn is_valid(&self) -> bool {
        self.0.is_valid
    }

    pub fn auxiliary_data(&self) -> Option<AlonzoAuxiliaryData> {
        self.0.auxiliary_data.clone().map(std::convert::Into::into)
    }

    pub fn new(
        body: &AlonzoTransactionBody,
        witness_set: &AlonzoTransactionWitnessSet,
        is_valid: bool,
        auxiliary_data: Option<AlonzoAuxiliaryData>,
    ) -> Self {
        Self(cml_multi_era::alonzo::AlonzoTransaction::new(
            body.clone().into(),
            witness_set.clone().into(),
            is_valid,
            auxiliary_data.map(Into::into),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct AlonzoTransactionBody(cml_multi_era::alonzo::AlonzoTransactionBody);

impl_wasm_cbor_json_api!(AlonzoTransactionBody);

impl_wasm_conversions!(
    cml_multi_era::alonzo::AlonzoTransactionBody,
    AlonzoTransactionBody
);

#[wasm_bindgen]
impl AlonzoTransactionBody {
    pub fn inputs(&self) -> TransactionInputList {
        self.0.inputs.clone().into()
    }

    pub fn outputs(&self) -> AlonzoFormatTxOutList {
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

    pub fn set_update(&mut self, update: &AlonzoUpdate) {
        self.0.update = Some(update.clone().into())
    }

    pub fn update(&self) -> Option<AlonzoUpdate> {
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

    pub fn set_script_data_hash(&mut self, script_data_hash: &ScriptDataHash) {
        self.0.script_data_hash = Some(script_data_hash.clone().into())
    }

    pub fn script_data_hash(&self) -> Option<ScriptDataHash> {
        self.0.script_data_hash.map(std::convert::Into::into)
    }

    pub fn set_collateral_inputs(&mut self, collateral_inputs: &TransactionInputList) {
        self.0.collateral_inputs = Some(collateral_inputs.clone().into())
    }

    pub fn collateral_inputs(&self) -> Option<TransactionInputList> {
        self.0
            .collateral_inputs
            .clone()
            .map(std::convert::Into::into)
    }

    pub fn set_required_signers(&mut self, required_signers: &RequiredSigners) {
        self.0.required_signers = Some(required_signers.clone().into())
    }

    pub fn required_signers(&self) -> Option<RequiredSigners> {
        self.0
            .required_signers
            .clone()
            .map(std::convert::Into::into)
    }

    pub fn set_network_id(&mut self, network_id: &NetworkId) {
        self.0.network_id = Some(network_id.clone().into())
    }

    pub fn network_id(&self) -> Option<NetworkId> {
        self.0.network_id.map(std::convert::Into::into)
    }

    pub fn new(inputs: &TransactionInputList, outputs: &AlonzoFormatTxOutList, fee: Coin) -> Self {
        Self(cml_multi_era::alonzo::AlonzoTransactionBody::new(
            inputs.clone().into(),
            outputs.clone().into(),
            fee,
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct AlonzoTransactionWitnessSet(cml_multi_era::alonzo::AlonzoTransactionWitnessSet);

impl_wasm_cbor_json_api!(AlonzoTransactionWitnessSet);

impl_wasm_conversions!(
    cml_multi_era::alonzo::AlonzoTransactionWitnessSet,
    AlonzoTransactionWitnessSet
);

#[wasm_bindgen]
impl AlonzoTransactionWitnessSet {
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

    pub fn set_plutus_v1_scripts(&mut self, plutus_v1_scripts: &PlutusV1ScriptList) {
        self.0.plutus_v1_scripts = Some(plutus_v1_scripts.clone().into())
    }

    pub fn plutus_v1_scripts(&self) -> Option<PlutusV1ScriptList> {
        self.0
            .plutus_v1_scripts
            .clone()
            .map(std::convert::Into::into)
    }

    pub fn set_plutus_datums(&mut self, plutus_datums: &PlutusDataList) {
        self.0.plutus_datums = Some(plutus_datums.clone().into())
    }

    pub fn plutus_datums(&self) -> Option<PlutusDataList> {
        self.0.plutus_datums.clone().map(std::convert::Into::into)
    }

    pub fn set_redeemers(&mut self, redeemers: &AlonzoRedeemerList) {
        self.0.redeemers = Some(redeemers.clone().into())
    }

    pub fn redeemers(&self) -> Option<AlonzoRedeemerList> {
        self.0.redeemers.clone().map(std::convert::Into::into)
    }

    pub fn new() -> Self {
        Self(cml_multi_era::alonzo::AlonzoTransactionWitnessSet::new())
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct AlonzoUpdate(cml_multi_era::alonzo::AlonzoUpdate);

impl_wasm_cbor_json_api!(AlonzoUpdate);

impl_wasm_conversions!(cml_multi_era::alonzo::AlonzoUpdate, AlonzoUpdate);

#[wasm_bindgen]
impl AlonzoUpdate {
    pub fn proposed_protocol_parameter_updates(&self) -> AlonzoProposedProtocolParameterUpdates {
        self.0.proposed_protocol_parameter_updates.clone().into()
    }

    pub fn epoch(&self) -> Epoch {
        self.0.epoch
    }

    pub fn new(
        proposed_protocol_parameter_updates: &AlonzoProposedProtocolParameterUpdates,
        epoch: Epoch,
    ) -> Self {
        Self(cml_multi_era::alonzo::AlonzoUpdate::new(
            proposed_protocol_parameter_updates.clone().into(),
            epoch,
        ))
    }
}
