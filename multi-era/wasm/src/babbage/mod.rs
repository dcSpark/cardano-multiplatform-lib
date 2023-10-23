// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use crate::shelley::ProtocolVersionStruct;
use crate::{
    AllegraCertificateList, BabbageTransactionBodyList, BabbageTransactionOutputList,
    BabbageTransactionWitnessSetList, GenesisHashList, MapTransactionIndexToBabbageAuxiliaryData,
};
use cml_chain_wasm::address::Address;
use cml_chain_wasm::assets::{Coin, Mint, Value};
use cml_chain_wasm::auxdata::{ShelleyFormatAuxData, ShelleyMaFormatAuxData};
use cml_chain_wasm::block::Header;
use cml_chain_wasm::crypto::{AuxiliaryDataHash, GenesisHash, ScriptDataHash};
use cml_chain_wasm::plutus::{ExUnitPrices, ExUnits, PlutusV1Script, PlutusV2Script};
use cml_chain_wasm::transaction::{AlonzoFormatTxOut, DatumOption, NativeScript, RequiredSigners};
use cml_chain_wasm::{
    BootstrapWitnessList, IntList, NativeScriptList, NetworkId, PlutusDataList, PlutusV1ScriptList,
    PlutusV2ScriptList, RedeemerList, TransactionInputList, VkeywitnessList,
};
use cml_chain_wasm::{Epoch, Rational, UnitInterval, Withdrawals};
use cml_core::ordered_hash_map::OrderedHashMap;
use cml_core::TransactionIndex;
use cml_core_wasm::{impl_wasm_cbor_json_api, impl_wasm_conversions, metadata::Metadata};
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct BabbageAuxiliaryData(cml_multi_era::babbage::BabbageAuxiliaryData);

impl_wasm_cbor_json_api!(BabbageAuxiliaryData);

impl_wasm_conversions!(
    cml_multi_era::babbage::BabbageAuxiliaryData,
    BabbageAuxiliaryData
);

#[wasm_bindgen]
impl BabbageAuxiliaryData {
    pub fn new_shelley(shelley: &ShelleyFormatAuxData) -> Self {
        Self(cml_multi_era::babbage::BabbageAuxiliaryData::new_shelley(
            shelley.clone().into(),
        ))
    }

    pub fn new_shelley_m_a(shelley_m_a: &ShelleyMaFormatAuxData) -> Self {
        Self(
            cml_multi_era::babbage::BabbageAuxiliaryData::new_shelley_m_a(
                shelley_m_a.clone().into(),
            ),
        )
    }

    pub fn new_babbage(babbage: &BabbageFormatAuxData) -> Self {
        Self(cml_multi_era::babbage::BabbageAuxiliaryData::new_babbage(
            babbage.clone().into(),
        ))
    }

    pub fn kind(&self) -> BabbageAuxiliaryDataKind {
        match &self.0 {
            cml_multi_era::babbage::BabbageAuxiliaryData::Shelley(_) => {
                BabbageAuxiliaryDataKind::Shelley
            }
            cml_multi_era::babbage::BabbageAuxiliaryData::ShelleyMA(_) => {
                BabbageAuxiliaryDataKind::ShelleyMA
            }
            cml_multi_era::babbage::BabbageAuxiliaryData::Babbage(_) => {
                BabbageAuxiliaryDataKind::Babbage
            }
        }
    }

    pub fn as_shelley(&self) -> Option<ShelleyFormatAuxData> {
        match &self.0 {
            cml_multi_era::babbage::BabbageAuxiliaryData::Shelley(shelley) => {
                Some(shelley.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_shelley_m_a(&self) -> Option<ShelleyMaFormatAuxData> {
        match &self.0 {
            cml_multi_era::babbage::BabbageAuxiliaryData::ShelleyMA(shelley_m_a) => {
                Some(shelley_m_a.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_babbage(&self) -> Option<BabbageFormatAuxData> {
        match &self.0 {
            cml_multi_era::babbage::BabbageAuxiliaryData::Babbage(babbage) => {
                Some(babbage.clone().into())
            }
            _ => None,
        }
    }
}

#[wasm_bindgen]
pub enum BabbageAuxiliaryDataKind {
    Shelley,
    ShelleyMA,
    Babbage,
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct BabbageBlock(cml_multi_era::babbage::BabbageBlock);

impl_wasm_cbor_json_api!(BabbageBlock);

impl_wasm_conversions!(cml_multi_era::babbage::BabbageBlock, BabbageBlock);

#[wasm_bindgen]
impl BabbageBlock {
    pub fn header(&self) -> Header {
        self.0.header.clone().into()
    }

    pub fn transaction_bodies(&self) -> BabbageTransactionBodyList {
        self.0.transaction_bodies.clone().into()
    }

    pub fn transaction_witness_sets(&self) -> BabbageTransactionWitnessSetList {
        self.0.transaction_witness_sets.clone().into()
    }

    pub fn auxiliary_data_set(&self) -> MapTransactionIndexToBabbageAuxiliaryData {
        self.0.auxiliary_data_set.clone().into()
    }

    pub fn invalid_transactions(&self) -> Vec<TransactionIndex> {
        self.0.invalid_transactions.clone()
    }

    pub fn new(
        header: &Header,
        transaction_bodies: &BabbageTransactionBodyList,
        transaction_witness_sets: &BabbageTransactionWitnessSetList,
        auxiliary_data_set: &MapTransactionIndexToBabbageAuxiliaryData,
        invalid_transactions: Vec<TransactionIndex>,
    ) -> Self {
        Self(cml_multi_era::babbage::BabbageBlock::new(
            header.clone().into(),
            transaction_bodies.clone().into(),
            transaction_witness_sets.clone().into(),
            auxiliary_data_set.clone().into(),
            invalid_transactions,
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct BabbageCostModels(cml_multi_era::babbage::BabbageCostModels);

impl_wasm_cbor_json_api!(BabbageCostModels);

impl_wasm_conversions!(cml_multi_era::babbage::BabbageCostModels, BabbageCostModels);

#[wasm_bindgen]
impl BabbageCostModels {
    pub fn set_plutus_v1(&mut self, plutus_v1: &IntList) {
        self.0.plutus_v1 = Some(plutus_v1.clone().into())
    }

    pub fn plutus_v1(&self) -> Option<IntList> {
        self.0.plutus_v1.clone().map(std::convert::Into::into)
    }

    pub fn set_plutus_v2(&mut self, plutus_v2: &IntList) {
        self.0.plutus_v2 = Some(plutus_v2.clone().into())
    }

    pub fn plutus_v2(&self) -> Option<IntList> {
        self.0.plutus_v2.clone().map(std::convert::Into::into)
    }

    pub fn new() -> Self {
        Self(cml_multi_era::babbage::BabbageCostModels::new())
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct BabbageFormatAuxData(cml_multi_era::babbage::BabbageFormatAuxData);

impl_wasm_cbor_json_api!(BabbageFormatAuxData);

impl_wasm_conversions!(
    cml_multi_era::babbage::BabbageFormatAuxData,
    BabbageFormatAuxData
);

#[wasm_bindgen]
impl BabbageFormatAuxData {
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

    pub fn set_plutus_v2_scripts(&mut self, plutus_v2_scripts: &PlutusV2ScriptList) {
        self.0.plutus_v2_scripts = Some(plutus_v2_scripts.clone().into())
    }

    pub fn plutus_v2_scripts(&self) -> Option<PlutusV2ScriptList> {
        self.0
            .plutus_v2_scripts
            .clone()
            .map(std::convert::Into::into)
    }

    pub fn new() -> Self {
        Self(cml_multi_era::babbage::BabbageFormatAuxData::new())
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct BabbageFormatTxOut(cml_multi_era::babbage::BabbageFormatTxOut);

impl_wasm_cbor_json_api!(BabbageFormatTxOut);

impl_wasm_conversions!(
    cml_multi_era::babbage::BabbageFormatTxOut,
    BabbageFormatTxOut
);

#[wasm_bindgen]
impl BabbageFormatTxOut {
    pub fn address(&self) -> Address {
        self.0.address.clone().into()
    }

    pub fn amount(&self) -> Value {
        self.0.amount.clone().into()
    }

    pub fn set_datum_option(&mut self, datum_option: &DatumOption) {
        self.0.datum_option = Some(datum_option.clone().into())
    }

    pub fn datum_option(&self) -> Option<DatumOption> {
        self.0.datum_option.clone().map(std::convert::Into::into)
    }

    pub fn set_script_reference(&mut self, script_reference: &BabbageScriptRef) {
        self.0.script_reference = Some(script_reference.clone().into())
    }

    pub fn script_reference(&self) -> Option<BabbageScriptRef> {
        self.0
            .script_reference
            .clone()
            .map(std::convert::Into::into)
    }

    pub fn new(address: &Address, amount: &Value) -> Self {
        Self(cml_multi_era::babbage::BabbageFormatTxOut::new(
            address.clone().into(),
            amount.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct BabbageProposedProtocolParameterUpdates(
    cml_multi_era::babbage::BabbageProposedProtocolParameterUpdates,
);

impl_wasm_conversions!(
    cml_multi_era::babbage::BabbageProposedProtocolParameterUpdates,
    BabbageProposedProtocolParameterUpdates
);

#[wasm_bindgen]
impl BabbageProposedProtocolParameterUpdates {
    pub fn new() -> Self {
        Self(OrderedHashMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(
        &mut self,
        key: &GenesisHash,
        value: &BabbageProtocolParamUpdate,
    ) -> Option<BabbageProtocolParamUpdate> {
        self.0
            .insert(key.clone().into(), value.clone().into())
            .map(Into::into)
    }

    pub fn get(&self, key: &GenesisHash) -> Option<BabbageProtocolParamUpdate> {
        self.0.get(key.as_ref()).map(|v| v.clone().into())
    }

    pub fn keys(&self) -> GenesisHashList {
        self.0.iter().map(|(k, _v)| *k).collect::<Vec<_>>().into()
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct BabbageProtocolParamUpdate(cml_multi_era::babbage::BabbageProtocolParamUpdate);

impl_wasm_cbor_json_api!(BabbageProtocolParamUpdate);

impl_wasm_conversions!(
    cml_multi_era::babbage::BabbageProtocolParamUpdate,
    BabbageProtocolParamUpdate
);

#[wasm_bindgen]
impl BabbageProtocolParamUpdate {
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
        cost_models_for_script_languages: &BabbageCostModels,
    ) {
        self.0.cost_models_for_script_languages =
            Some(cost_models_for_script_languages.clone().into())
    }

    pub fn cost_models_for_script_languages(&self) -> Option<BabbageCostModels> {
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
        Self(cml_multi_era::babbage::BabbageProtocolParamUpdate::new())
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct BabbageScript(cml_multi_era::babbage::BabbageScript);

impl_wasm_cbor_json_api!(BabbageScript);

impl_wasm_conversions!(cml_multi_era::babbage::BabbageScript, BabbageScript);

#[wasm_bindgen]
impl BabbageScript {
    pub fn new_native(script: &NativeScript) -> Self {
        Self(cml_multi_era::babbage::BabbageScript::new_native(
            script.clone().into(),
        ))
    }

    pub fn new_plutus_v1(script: &PlutusV1Script) -> Self {
        Self(cml_multi_era::babbage::BabbageScript::new_plutus_v1(
            script.clone().into(),
        ))
    }

    pub fn new_plutus_v2(script: &PlutusV2Script) -> Self {
        Self(cml_multi_era::babbage::BabbageScript::new_plutus_v2(
            script.clone().into(),
        ))
    }

    pub fn kind(&self) -> BabbageScriptKind {
        match &self.0 {
            cml_multi_era::babbage::BabbageScript::Native { .. } => BabbageScriptKind::Native,
            cml_multi_era::babbage::BabbageScript::PlutusV1 { .. } => BabbageScriptKind::PlutusV1,
            cml_multi_era::babbage::BabbageScript::PlutusV2 { .. } => BabbageScriptKind::PlutusV2,
        }
    }

    pub fn as_native(&self) -> Option<NativeScript> {
        match &self.0 {
            cml_multi_era::babbage::BabbageScript::Native { script, .. } => {
                Some(script.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_plutus_v1(&self) -> Option<PlutusV1Script> {
        match &self.0 {
            cml_multi_era::babbage::BabbageScript::PlutusV1 { script, .. } => {
                Some(script.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_plutus_v2(&self) -> Option<PlutusV2Script> {
        match &self.0 {
            cml_multi_era::babbage::BabbageScript::PlutusV2 { script, .. } => {
                Some(script.clone().into())
            }
            _ => None,
        }
    }
}

#[wasm_bindgen]
pub enum BabbageScriptKind {
    Native,
    PlutusV1,
    PlutusV2,
}

pub type BabbageScriptRef = BabbageScript;

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct BabbageTransaction(cml_multi_era::babbage::BabbageTransaction);

impl_wasm_cbor_json_api!(BabbageTransaction);

impl_wasm_conversions!(
    cml_multi_era::babbage::BabbageTransaction,
    BabbageTransaction
);

#[wasm_bindgen]
impl BabbageTransaction {
    pub fn body(&self) -> BabbageTransactionBody {
        self.0.body.clone().into()
    }

    pub fn witness_set(&self) -> BabbageTransactionWitnessSet {
        self.0.witness_set.clone().into()
    }

    pub fn is_valid(&self) -> bool {
        self.0.is_valid
    }

    pub fn auxiliary_data(&self) -> Option<BabbageAuxiliaryData> {
        self.0.auxiliary_data.clone().map(std::convert::Into::into)
    }

    pub fn new(
        body: &BabbageTransactionBody,
        witness_set: &BabbageTransactionWitnessSet,
        is_valid: bool,
        auxiliary_data: Option<BabbageAuxiliaryData>,
    ) -> Self {
        Self(cml_multi_era::babbage::BabbageTransaction::new(
            body.clone().into(),
            witness_set.clone().into(),
            is_valid,
            auxiliary_data.map(Into::into),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct BabbageTransactionBody(cml_multi_era::babbage::BabbageTransactionBody);

impl_wasm_cbor_json_api!(BabbageTransactionBody);

impl_wasm_conversions!(
    cml_multi_era::babbage::BabbageTransactionBody,
    BabbageTransactionBody
);

#[wasm_bindgen]
impl BabbageTransactionBody {
    pub fn inputs(&self) -> TransactionInputList {
        self.0.inputs.clone().into()
    }

    pub fn outputs(&self) -> BabbageTransactionOutputList {
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

    pub fn set_update(&mut self, update: &BabbageUpdate) {
        self.0.update = Some(update.clone().into())
    }

    pub fn update(&self) -> Option<BabbageUpdate> {
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

    pub fn set_collateral_return(&mut self, collateral_return: &BabbageTransactionOutput) {
        self.0.collateral_return = Some(collateral_return.clone().into())
    }

    pub fn collateral_return(&self) -> Option<BabbageTransactionOutput> {
        self.0
            .collateral_return
            .clone()
            .map(std::convert::Into::into)
    }

    pub fn set_total_collateral(&mut self, total_collateral: Coin) {
        self.0.total_collateral = Some(total_collateral)
    }

    pub fn total_collateral(&self) -> Option<Coin> {
        self.0.total_collateral
    }

    pub fn set_reference_inputs(&mut self, reference_inputs: &TransactionInputList) {
        self.0.reference_inputs = Some(reference_inputs.clone().into())
    }

    pub fn reference_inputs(&self) -> Option<TransactionInputList> {
        self.0
            .reference_inputs
            .clone()
            .map(std::convert::Into::into)
    }

    pub fn new(
        inputs: &TransactionInputList,
        outputs: &BabbageTransactionOutputList,
        fee: Coin,
    ) -> Self {
        Self(cml_multi_era::babbage::BabbageTransactionBody::new(
            inputs.clone().into(),
            outputs.clone().into(),
            fee,
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct BabbageTransactionOutput(cml_multi_era::babbage::BabbageTransactionOutput);

impl_wasm_cbor_json_api!(BabbageTransactionOutput);

impl_wasm_conversions!(
    cml_multi_era::babbage::BabbageTransactionOutput,
    BabbageTransactionOutput
);

#[wasm_bindgen]
impl BabbageTransactionOutput {
    pub fn new_alonzo_format_tx_out(alonzo_format_tx_out: &AlonzoFormatTxOut) -> Self {
        Self(
            cml_multi_era::babbage::BabbageTransactionOutput::new_alonzo_format_tx_out(
                alonzo_format_tx_out.clone().into(),
            ),
        )
    }

    pub fn new_babbage_format_tx_out(babbage_format_tx_out: &BabbageFormatTxOut) -> Self {
        Self(
            cml_multi_era::babbage::BabbageTransactionOutput::new_babbage_format_tx_out(
                babbage_format_tx_out.clone().into(),
            ),
        )
    }

    pub fn kind(&self) -> BabbageTransactionOutputKind {
        match &self.0 {
            cml_multi_era::babbage::BabbageTransactionOutput::AlonzoFormatTxOut(_) => {
                BabbageTransactionOutputKind::AlonzoFormatTxOut
            }
            cml_multi_era::babbage::BabbageTransactionOutput::BabbageFormatTxOut(_) => {
                BabbageTransactionOutputKind::BabbageFormatTxOut
            }
        }
    }

    pub fn as_alonzo_format_tx_out(&self) -> Option<AlonzoFormatTxOut> {
        match &self.0 {
            cml_multi_era::babbage::BabbageTransactionOutput::AlonzoFormatTxOut(
                alonzo_format_tx_out,
            ) => Some(alonzo_format_tx_out.clone().into()),
            _ => None,
        }
    }

    pub fn as_babbage_format_tx_out(&self) -> Option<BabbageFormatTxOut> {
        match &self.0 {
            cml_multi_era::babbage::BabbageTransactionOutput::BabbageFormatTxOut(
                babbage_format_tx_out,
            ) => Some(babbage_format_tx_out.clone().into()),
            _ => None,
        }
    }
}

#[wasm_bindgen]
pub enum BabbageTransactionOutputKind {
    AlonzoFormatTxOut,
    BabbageFormatTxOut,
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct BabbageTransactionWitnessSet(cml_multi_era::babbage::BabbageTransactionWitnessSet);

impl_wasm_cbor_json_api!(BabbageTransactionWitnessSet);

impl_wasm_conversions!(
    cml_multi_era::babbage::BabbageTransactionWitnessSet,
    BabbageTransactionWitnessSet
);

#[wasm_bindgen]
impl BabbageTransactionWitnessSet {
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

    pub fn set_redeemers(&mut self, redeemers: &RedeemerList) {
        self.0.redeemers = Some(redeemers.clone().into())
    }

    pub fn redeemers(&self) -> Option<RedeemerList> {
        self.0.redeemers.clone().map(std::convert::Into::into)
    }

    pub fn set_plutus_v2_scripts(&mut self, plutus_v2_scripts: &PlutusV2ScriptList) {
        self.0.plutus_v2_scripts = Some(plutus_v2_scripts.clone().into())
    }

    pub fn plutus_v2_scripts(&self) -> Option<PlutusV2ScriptList> {
        self.0
            .plutus_v2_scripts
            .clone()
            .map(std::convert::Into::into)
    }

    pub fn new() -> Self {
        Self(cml_multi_era::babbage::BabbageTransactionWitnessSet::new())
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct BabbageUpdate(cml_multi_era::babbage::BabbageUpdate);

impl_wasm_cbor_json_api!(BabbageUpdate);

impl_wasm_conversions!(cml_multi_era::babbage::BabbageUpdate, BabbageUpdate);

#[wasm_bindgen]
impl BabbageUpdate {
    pub fn updates(&self) -> BabbageProposedProtocolParameterUpdates {
        self.0.updates.clone().into()
    }

    pub fn epoch(&self) -> Epoch {
        self.0.epoch
    }

    pub fn new(updates: &BabbageProposedProtocolParameterUpdates, epoch: Epoch) -> Self {
        Self(cml_multi_era::babbage::BabbageUpdate::new(
            updates.clone().into(),
            epoch,
        ))
    }
}
