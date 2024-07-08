// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

pub mod cbor_encodings;
pub mod serialization;
pub mod utils;

use crate::allegra::AllegraCertificate;
use crate::alonzo::AlonzoRedeemer;
use crate::shelley::ProtocolVersionStruct;
use cbor_encodings::{
    BabbageBlockEncoding, BabbageFormatAuxDataEncoding, BabbageFormatTxOutEncoding,
    BabbageProtocolParamUpdateEncoding, BabbageTransactionBodyEncoding, BabbageTransactionEncoding,
    BabbageTransactionWitnessSetEncoding, BabbageUpdateEncoding,
};
use cml_chain::address::Address;
use cml_chain::assets::{Coin, Value};
use cml_chain::auxdata::{Metadata, ShelleyFormatAuxData, ShelleyMAFormatAuxData};
use cml_chain::block::Header;
use cml_chain::crypto::{
    AuxiliaryDataHash, BootstrapWitness, GenesisHash, ScriptDataHash, Vkeywitness,
};
use cml_chain::plutus::{
    CostModels, ExUnitPrices, ExUnits, PlutusData, PlutusV1Script, PlutusV2Script,
};
use cml_chain::transaction::{AlonzoFormatTxOut, DatumOption, NativeScript, TransactionInput};
use cml_chain::{Epoch, NetworkId, Rational, RequiredSigners, UnitInterval, Withdrawals};

use cml_core::ordered_hash_map::OrderedHashMap;
use cml_core::serialization::LenEncoding;
use cml_core::TransactionIndex;

use std::collections::BTreeMap;

use self::utils::BabbageMint;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub enum BabbageAuxiliaryData {
    Shelley(ShelleyFormatAuxData),
    ShelleyMA(ShelleyMAFormatAuxData),
    Babbage(BabbageFormatAuxData),
}

impl BabbageAuxiliaryData {
    pub fn new_shelley(shelley: ShelleyFormatAuxData) -> Self {
        Self::Shelley(shelley)
    }

    pub fn new_shelley_ma(shelley_ma: ShelleyMAFormatAuxData) -> Self {
        Self::ShelleyMA(shelley_ma)
    }

    pub fn new_babbage(babbage: BabbageFormatAuxData) -> Self {
        Self::Babbage(babbage)
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct BabbageBlock {
    pub header: Header,
    pub transaction_bodies: Vec<BabbageTransactionBody>,
    pub transaction_witness_sets: Vec<BabbageTransactionWitnessSet>,
    pub auxiliary_data_set: OrderedHashMap<TransactionIndex, BabbageAuxiliaryData>,
    pub invalid_transactions: Vec<TransactionIndex>,
    #[serde(skip)]
    pub encodings: Option<BabbageBlockEncoding>,
}

impl BabbageBlock {
    pub fn new(
        header: Header,
        transaction_bodies: Vec<BabbageTransactionBody>,
        transaction_witness_sets: Vec<BabbageTransactionWitnessSet>,
        auxiliary_data_set: OrderedHashMap<TransactionIndex, BabbageAuxiliaryData>,
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

pub type BabbageCostModels = CostModels;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct BabbageFormatAuxData {
    pub metadata: Option<Metadata>,
    pub native_scripts: Option<Vec<NativeScript>>,
    pub plutus_v1_scripts: Option<Vec<PlutusV1Script>>,
    pub plutus_v2_scripts: Option<Vec<PlutusV2Script>>,
    #[serde(skip)]
    pub encodings: Option<BabbageFormatAuxDataEncoding>,
}

impl BabbageFormatAuxData {
    pub fn new() -> Self {
        Self {
            metadata: None,
            native_scripts: None,
            plutus_v1_scripts: None,
            plutus_v2_scripts: None,
            encodings: None,
        }
    }
}

impl Default for BabbageFormatAuxData {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct BabbageFormatTxOut {
    pub address: Address,
    pub amount: Value,
    pub datum_option: Option<DatumOption>,
    pub script_reference: Option<BabbageScriptRef>,
    #[serde(skip)]
    pub encodings: Option<BabbageFormatTxOutEncoding>,
}

impl BabbageFormatTxOut {
    pub fn new(address: Address, amount: Value) -> Self {
        Self {
            address,
            amount,
            datum_option: None,
            script_reference: None,
            encodings: None,
        }
    }
}

pub type BabbageProposedProtocolParameterUpdates =
    OrderedHashMap<GenesisHash, BabbageProtocolParamUpdate>;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct BabbageProtocolParamUpdate {
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
    pub protocol_version: Option<ProtocolVersionStruct>,
    pub min_pool_cost: Option<Coin>,
    pub ada_per_utxo_byte: Option<Coin>,
    pub cost_models_for_script_languages: Option<BabbageCostModels>,
    pub execution_costs: Option<ExUnitPrices>,
    pub max_tx_ex_units: Option<ExUnits>,
    pub max_block_ex_units: Option<ExUnits>,
    pub max_value_size: Option<u64>,
    pub collateral_percentage: Option<u64>,
    pub max_collateral_inputs: Option<u64>,
    #[serde(skip)]
    pub encodings: Option<BabbageProtocolParamUpdateEncoding>,
}

impl BabbageProtocolParamUpdate {
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

impl Default for BabbageProtocolParamUpdate {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub enum BabbageScript {
    Native {
        script: NativeScript,
        #[serde(skip)]
        len_encoding: LenEncoding,
        #[serde(skip)]
        tag_encoding: Option<cbor_event::Sz>,
    },
    PlutusV1 {
        script: PlutusV1Script,
        #[serde(skip)]
        len_encoding: LenEncoding,
        #[serde(skip)]
        tag_encoding: Option<cbor_event::Sz>,
    },
    PlutusV2 {
        script: PlutusV2Script,
        #[serde(skip)]
        len_encoding: LenEncoding,
        #[serde(skip)]
        tag_encoding: Option<cbor_event::Sz>,
    },
}

impl BabbageScript {
    pub fn new_native(script: NativeScript) -> Self {
        Self::Native {
            script,
            len_encoding: LenEncoding::default(),
            tag_encoding: None,
        }
    }

    pub fn new_plutus_v1(script: PlutusV1Script) -> Self {
        Self::PlutusV1 {
            script,
            len_encoding: LenEncoding::default(),
            tag_encoding: None,
        }
    }

    pub fn new_plutus_v2(script: PlutusV2Script) -> Self {
        Self::PlutusV2 {
            script,
            len_encoding: LenEncoding::default(),
            tag_encoding: None,
        }
    }
}

pub type BabbageScriptRef = BabbageScript;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct BabbageTransaction {
    pub body: BabbageTransactionBody,
    pub witness_set: BabbageTransactionWitnessSet,
    pub is_valid: bool,
    pub auxiliary_data: Option<BabbageAuxiliaryData>,
    #[serde(skip)]
    pub encodings: Option<BabbageTransactionEncoding>,
}

impl BabbageTransaction {
    pub fn new(
        body: BabbageTransactionBody,
        witness_set: BabbageTransactionWitnessSet,
        is_valid: bool,
        auxiliary_data: Option<BabbageAuxiliaryData>,
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
pub struct BabbageTransactionBody {
    pub inputs: Vec<TransactionInput>,
    pub outputs: Vec<BabbageTransactionOutput>,
    pub fee: Coin,
    pub ttl: Option<u64>,
    pub certs: Option<Vec<AllegraCertificate>>,
    pub withdrawals: Option<Withdrawals>,
    pub update: Option<BabbageUpdate>,
    pub auxiliary_data_hash: Option<AuxiliaryDataHash>,
    pub validity_interval_start: Option<u64>,
    pub mint: Option<BabbageMint>,
    pub script_data_hash: Option<ScriptDataHash>,
    pub collateral_inputs: Option<Vec<TransactionInput>>,
    pub required_signers: Option<RequiredSigners>,
    pub network_id: Option<NetworkId>,
    pub collateral_return: Option<BabbageTransactionOutput>,
    pub total_collateral: Option<Coin>,
    pub reference_inputs: Option<Vec<TransactionInput>>,
    #[serde(skip)]
    pub encodings: Option<BabbageTransactionBodyEncoding>,
}

impl BabbageTransactionBody {
    pub fn new(
        inputs: Vec<TransactionInput>,
        outputs: Vec<BabbageTransactionOutput>,
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
            mint: None,
            script_data_hash: None,
            collateral_inputs: None,
            required_signers: None,
            network_id: None,
            collateral_return: None,
            total_collateral: None,
            reference_inputs: None,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub enum BabbageTransactionOutput {
    AlonzoFormatTxOut(AlonzoFormatTxOut),
    BabbageFormatTxOut(BabbageFormatTxOut),
}

impl BabbageTransactionOutput {
    pub fn new_alonzo_format_tx_out(alonzo_format_tx_out: AlonzoFormatTxOut) -> Self {
        Self::AlonzoFormatTxOut(alonzo_format_tx_out)
    }

    pub fn new_babbage_format_tx_out(babbage_format_tx_out: BabbageFormatTxOut) -> Self {
        Self::BabbageFormatTxOut(babbage_format_tx_out)
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct BabbageTransactionWitnessSet {
    pub vkeywitnesses: Option<Vec<Vkeywitness>>,
    pub native_scripts: Option<Vec<NativeScript>>,
    pub bootstrap_witnesses: Option<Vec<BootstrapWitness>>,
    pub plutus_v1_scripts: Option<Vec<PlutusV1Script>>,
    pub plutus_datums: Option<Vec<PlutusData>>,
    pub redeemers: Option<Vec<AlonzoRedeemer>>,
    pub plutus_v2_scripts: Option<Vec<PlutusV2Script>>,
    #[serde(skip)]
    pub encodings: Option<BabbageTransactionWitnessSetEncoding>,
}

impl BabbageTransactionWitnessSet {
    pub fn new() -> Self {
        Self {
            vkeywitnesses: None,
            native_scripts: None,
            bootstrap_witnesses: None,
            plutus_v1_scripts: None,
            plutus_datums: None,
            redeemers: None,
            plutus_v2_scripts: None,
            encodings: None,
        }
    }
}

impl Default for BabbageTransactionWitnessSet {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct BabbageUpdate {
    pub updates: BabbageProposedProtocolParameterUpdates,
    pub epoch: Epoch,
    #[serde(skip)]
    pub encodings: Option<BabbageUpdateEncoding>,
}

impl BabbageUpdate {
    pub fn new(updates: BabbageProposedProtocolParameterUpdates, epoch: Epoch) -> Self {
        Self {
            updates,
            epoch,
            encodings: None,
        }
    }
}
