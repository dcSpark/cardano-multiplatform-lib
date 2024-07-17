// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

pub mod cbor_encodings;
pub mod serialization;
pub mod utils;

#[cfg(not(feature = "used_from_wasm"))]
use noop_proc_macro::wasm_bindgen;
#[cfg(feature = "used_from_wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

use crate::allegra::AllegraCertificate;
use crate::shelley::{ProtocolVersionStruct, ShelleyHeader};
use cbor_encodings::{
    AlonzoBlockEncoding, AlonzoFormatAuxDataEncoding, AlonzoProtocolParamUpdateEncoding,
    AlonzoTransactionBodyEncoding, AlonzoTransactionEncoding, AlonzoTransactionWitnessSetEncoding,
    AlonzoUpdateEncoding,
};
use cml_chain::assets::{Coin, Mint};
use cml_chain::auxdata::{Metadata, ShelleyFormatAuxData, ShelleyMAFormatAuxData};
use cml_chain::crypto::{
    AuxiliaryDataHash, BootstrapWitness, GenesisHash, Nonce, ScriptDataHash, Vkeywitness,
};
use cml_chain::plutus::{CostModels, ExUnitPrices, ExUnits, PlutusData, PlutusV1Script};
use cml_chain::transaction::{AlonzoFormatTxOut, NativeScript, TransactionInput};
use cml_chain::TransactionIndex;
use cml_chain::{Epoch, NetworkId, Rational, RequiredSigners, UnitInterval, Withdrawals};
use cml_core::ordered_hash_map::OrderedHashMap;
use std::collections::BTreeMap;

use self::cbor_encodings::AlonzoRedeemerEncoding;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub enum AlonzoAuxiliaryData {
    Shelley(ShelleyFormatAuxData),
    ShelleyMA(ShelleyMAFormatAuxData),
    Alonzo(AlonzoFormatAuxData),
}

impl AlonzoAuxiliaryData {
    pub fn new_shelley(shelley: ShelleyFormatAuxData) -> Self {
        Self::Shelley(shelley)
    }

    pub fn new_shelley_ma(shelley_ma: ShelleyMAFormatAuxData) -> Self {
        Self::ShelleyMA(shelley_ma)
    }

    pub fn new_alonzo(alonzo: AlonzoFormatAuxData) -> Self {
        Self::Alonzo(alonzo)
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct AlonzoBlock {
    pub header: ShelleyHeader,
    pub transaction_bodies: Vec<AlonzoTransactionBody>,
    pub transaction_witness_sets: Vec<AlonzoTransactionWitnessSet>,
    pub auxiliary_data_set: OrderedHashMap<TransactionIndex, AlonzoAuxiliaryData>,
    pub invalid_transactions: Vec<TransactionIndex>,
    #[serde(skip)]
    pub encodings: Option<AlonzoBlockEncoding>,
}

impl AlonzoBlock {
    pub fn new(
        header: ShelleyHeader,
        transaction_bodies: Vec<AlonzoTransactionBody>,
        transaction_witness_sets: Vec<AlonzoTransactionWitnessSet>,
        auxiliary_data_set: OrderedHashMap<TransactionIndex, AlonzoAuxiliaryData>,
        invalid_transactions: Vec<TransactionIndex>,
    ) -> Self {
        Self {
            header,
            transaction_bodies,
            transaction_witness_sets,
            auxiliary_data_set,
            invalid_transactions,
            encodings: None,
        }
    }
}

pub type AlonzoCostModels = CostModels;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct AlonzoFormatAuxData {
    pub metadata: Option<Metadata>,
    pub native_scripts: Option<Vec<NativeScript>>,
    pub plutus_v1_scripts: Option<Vec<PlutusV1Script>>,
    #[serde(skip)]
    pub encodings: Option<AlonzoFormatAuxDataEncoding>,
}

impl AlonzoFormatAuxData {
    pub fn new() -> Self {
        Self {
            metadata: None,
            native_scripts: None,
            plutus_v1_scripts: None,
            encodings: None,
        }
    }
}

impl Default for AlonzoFormatAuxData {
    fn default() -> Self {
        Self::new()
    }
}

pub type AlonzoProposedProtocolParameterUpdates =
    OrderedHashMap<GenesisHash, AlonzoProtocolParamUpdate>;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct AlonzoProtocolParamUpdate {
    pub minfee_a: Option<u64>,
    pub minfee_b: Option<u64>,
    pub max_block_body_size: Option<u64>,
    pub max_transaction_size: Option<u64>,
    pub max_block_header_size: Option<u64>,
    pub key_deposit: Option<Coin>,
    pub pool_deposit: Option<Coin>,
    pub maximum_epoch: Option<Epoch>,
    pub n_opt: Option<u64>,
    pub pool_pledge_influence: Option<Rational>,
    pub expansion_rate: Option<UnitInterval>,
    pub treasury_growth_rate: Option<UnitInterval>,
    pub decentralization_constant: Option<UnitInterval>,
    pub extra_entropy: Option<Nonce>,
    pub protocol_version: Option<ProtocolVersionStruct>,
    pub min_pool_cost: Option<Coin>,
    pub ada_per_utxo_byte: Option<Coin>,
    pub cost_models_for_script_languages: Option<AlonzoCostModels>,
    pub execution_costs: Option<ExUnitPrices>,
    pub max_tx_ex_units: Option<ExUnits>,
    pub max_block_ex_units: Option<ExUnits>,
    pub max_value_size: Option<u64>,
    pub collateral_percentage: Option<u64>,
    pub max_collateral_inputs: Option<u64>,
    #[serde(skip)]
    pub encodings: Option<AlonzoProtocolParamUpdateEncoding>,
}

impl AlonzoProtocolParamUpdate {
    pub fn new() -> Self {
        Self {
            minfee_a: None,
            minfee_b: None,
            max_block_body_size: None,
            max_transaction_size: None,
            max_block_header_size: None,
            key_deposit: None,
            pool_deposit: None,
            maximum_epoch: None,
            n_opt: None,
            pool_pledge_influence: None,
            expansion_rate: None,
            treasury_growth_rate: None,
            decentralization_constant: None,
            extra_entropy: None,
            protocol_version: None,
            min_pool_cost: None,
            ada_per_utxo_byte: None,
            cost_models_for_script_languages: None,
            execution_costs: None,
            max_tx_ex_units: None,
            max_block_ex_units: None,
            max_value_size: None,
            collateral_percentage: None,
            max_collateral_inputs: None,
            encodings: None,
        }
    }
}

impl Default for AlonzoProtocolParamUpdate {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct AlonzoRedeemer {
    pub tag: AlonzoRedeemerTag,
    pub index: u64,
    pub data: PlutusData,
    pub ex_units: ExUnits,
    #[serde(skip)]
    pub encodings: Option<AlonzoRedeemerEncoding>,
}

impl AlonzoRedeemer {
    pub fn new(tag: AlonzoRedeemerTag, index: u64, data: PlutusData, ex_units: ExUnits) -> Self {
        Self {
            tag,
            index,
            data,
            ex_units,
            encodings: None,
        }
    }
}

#[derive(
    Copy,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Clone,
    Debug,
    serde::Deserialize,
    serde::Serialize,
    schemars::JsonSchema,
)]
#[wasm_bindgen]
pub enum AlonzoRedeemerTag {
    Spend,
    Mint,
    Cert,
    Reward,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct AlonzoTransaction {
    pub body: AlonzoTransactionBody,
    pub witness_set: AlonzoTransactionWitnessSet,
    pub is_valid: bool,
    pub auxiliary_data: Option<AlonzoAuxiliaryData>,
    #[serde(skip)]
    pub encodings: Option<AlonzoTransactionEncoding>,
}

impl AlonzoTransaction {
    pub fn new(
        body: AlonzoTransactionBody,
        witness_set: AlonzoTransactionWitnessSet,
        is_valid: bool,
        auxiliary_data: Option<AlonzoAuxiliaryData>,
    ) -> Self {
        Self {
            body,
            witness_set,
            is_valid,
            auxiliary_data,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct AlonzoTransactionBody {
    pub inputs: Vec<TransactionInput>,
    pub outputs: Vec<AlonzoFormatTxOut>,
    pub fee: Coin,
    pub ttl: Option<u64>,
    pub certs: Option<Vec<AllegraCertificate>>,
    pub withdrawals: Option<Withdrawals>,
    pub update: Option<AlonzoUpdate>,
    pub auxiliary_data_hash: Option<AuxiliaryDataHash>,
    pub validity_interval_start: Option<u64>,
    pub mint: Option<Mint>,
    pub script_data_hash: Option<ScriptDataHash>,
    pub collateral_inputs: Option<Vec<TransactionInput>>,
    pub required_signers: Option<RequiredSigners>,
    pub network_id: Option<NetworkId>,
    #[serde(skip)]
    pub encodings: Option<AlonzoTransactionBodyEncoding>,
}

impl AlonzoTransactionBody {
    pub fn new(inputs: Vec<TransactionInput>, outputs: Vec<AlonzoFormatTxOut>, fee: Coin) -> Self {
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
            script_data_hash: None,
            collateral_inputs: None,
            required_signers: None,
            network_id: None,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct AlonzoTransactionWitnessSet {
    pub vkeywitnesses: Option<Vec<Vkeywitness>>,
    pub native_scripts: Option<Vec<NativeScript>>,
    pub bootstrap_witnesses: Option<Vec<BootstrapWitness>>,
    pub plutus_v1_scripts: Option<Vec<PlutusV1Script>>,
    pub plutus_datums: Option<Vec<PlutusData>>,
    pub redeemers: Option<Vec<AlonzoRedeemer>>,
    #[serde(skip)]
    pub encodings: Option<AlonzoTransactionWitnessSetEncoding>,
}

impl AlonzoTransactionWitnessSet {
    pub fn new() -> Self {
        Self {
            vkeywitnesses: None,
            native_scripts: None,
            bootstrap_witnesses: None,
            plutus_v1_scripts: None,
            plutus_datums: None,
            redeemers: None,
            encodings: None,
        }
    }
}

impl Default for AlonzoTransactionWitnessSet {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct AlonzoUpdate {
    pub proposed_protocol_parameter_updates: AlonzoProposedProtocolParameterUpdates,
    pub epoch: Epoch,
    #[serde(skip)]
    pub encodings: Option<AlonzoUpdateEncoding>,
}

impl AlonzoUpdate {
    pub fn new(
        proposed_protocol_parameter_updates: AlonzoProposedProtocolParameterUpdates,
        epoch: Epoch,
    ) -> Self {
        Self {
            proposed_protocol_parameter_updates,
            epoch,
            encodings: None,
        }
    }
}
