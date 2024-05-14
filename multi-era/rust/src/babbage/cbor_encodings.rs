// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use cml_chain::address::RewardAccount;
use cml_core::serialization::{LenEncoding, StringEncoding};
use cml_crypto::GenesisHash;
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default)]
pub struct BabbageBlockEncoding {
    pub len_encoding: LenEncoding,
    pub transaction_bodies_encoding: LenEncoding,
    pub transaction_witness_sets_encoding: LenEncoding,
    pub auxiliary_data_set_encoding: LenEncoding,
    pub auxiliary_data_set_key_encodings: BTreeMap<u16, Option<cbor_event::Sz>>,
    pub invalid_transactions_encoding: LenEncoding,
    pub invalid_transactions_elem_encodings: Vec<Option<cbor_event::Sz>>,
}

#[derive(Clone, Debug, Default)]
pub struct BabbageFormatAuxDataEncoding {
    pub len_encoding: LenEncoding,
    pub tag_encoding: Option<cbor_event::Sz>,
    pub orig_deser_order: Vec<usize>,
    pub metadata_key_encoding: Option<cbor_event::Sz>,
    pub native_scripts_encoding: LenEncoding,
    pub native_scripts_key_encoding: Option<cbor_event::Sz>,
    pub plutus_v1_scripts_encoding: LenEncoding,
    pub plutus_v1_scripts_key_encoding: Option<cbor_event::Sz>,
    pub plutus_v2_scripts_encoding: LenEncoding,
    pub plutus_v2_scripts_key_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct BabbageFormatTxOutEncoding {
    pub len_encoding: LenEncoding,
    pub orig_deser_order: Vec<usize>,
    pub address_key_encoding: Option<cbor_event::Sz>,
    pub amount_key_encoding: Option<cbor_event::Sz>,
    pub datum_option_key_encoding: Option<cbor_event::Sz>,
    pub script_reference_tag_encoding: Option<cbor_event::Sz>,
    pub script_reference_bytes_encoding: StringEncoding,
    pub script_reference_key_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct BabbageProtocolParamUpdateEncoding {
    pub len_encoding: LenEncoding,
    pub orig_deser_order: Vec<usize>,
    pub minfee_a_encoding: Option<cbor_event::Sz>,
    pub minfee_a_key_encoding: Option<cbor_event::Sz>,
    pub minfee_b_encoding: Option<cbor_event::Sz>,
    pub minfee_b_key_encoding: Option<cbor_event::Sz>,
    pub max_block_body_size_encoding: Option<cbor_event::Sz>,
    pub max_block_body_size_key_encoding: Option<cbor_event::Sz>,
    pub max_transaction_size_encoding: Option<cbor_event::Sz>,
    pub max_transaction_size_key_encoding: Option<cbor_event::Sz>,
    pub max_block_header_size_encoding: Option<cbor_event::Sz>,
    pub max_block_header_size_key_encoding: Option<cbor_event::Sz>,
    pub key_deposit_encoding: Option<cbor_event::Sz>,
    pub key_deposit_key_encoding: Option<cbor_event::Sz>,
    pub pool_deposit_encoding: Option<cbor_event::Sz>,
    pub pool_deposit_key_encoding: Option<cbor_event::Sz>,
    pub maximum_epoch_encoding: Option<cbor_event::Sz>,
    pub maximum_epoch_key_encoding: Option<cbor_event::Sz>,
    pub n_opt_encoding: Option<cbor_event::Sz>,
    pub n_opt_key_encoding: Option<cbor_event::Sz>,
    pub pool_pledge_influence_key_encoding: Option<cbor_event::Sz>,
    pub expansion_rate_key_encoding: Option<cbor_event::Sz>,
    pub treasury_growth_rate_key_encoding: Option<cbor_event::Sz>,
    pub protocol_version_key_encoding: Option<cbor_event::Sz>,
    pub min_pool_cost_encoding: Option<cbor_event::Sz>,
    pub min_pool_cost_key_encoding: Option<cbor_event::Sz>,
    pub ada_per_utxo_byte_encoding: Option<cbor_event::Sz>,
    pub ada_per_utxo_byte_key_encoding: Option<cbor_event::Sz>,
    pub cost_models_for_script_languages_key_encoding: Option<cbor_event::Sz>,
    pub execution_costs_key_encoding: Option<cbor_event::Sz>,
    pub max_tx_ex_units_key_encoding: Option<cbor_event::Sz>,
    pub max_block_ex_units_key_encoding: Option<cbor_event::Sz>,
    pub max_value_size_encoding: Option<cbor_event::Sz>,
    pub max_value_size_key_encoding: Option<cbor_event::Sz>,
    pub collateral_percentage_encoding: Option<cbor_event::Sz>,
    pub collateral_percentage_key_encoding: Option<cbor_event::Sz>,
    pub max_collateral_inputs_encoding: Option<cbor_event::Sz>,
    pub max_collateral_inputs_key_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct BabbageTransactionBodyEncoding {
    pub len_encoding: LenEncoding,
    pub orig_deser_order: Vec<usize>,
    pub inputs_encoding: LenEncoding,
    pub inputs_key_encoding: Option<cbor_event::Sz>,
    pub outputs_encoding: LenEncoding,
    pub outputs_key_encoding: Option<cbor_event::Sz>,
    pub fee_encoding: Option<cbor_event::Sz>,
    pub fee_key_encoding: Option<cbor_event::Sz>,
    pub ttl_encoding: Option<cbor_event::Sz>,
    pub ttl_key_encoding: Option<cbor_event::Sz>,
    pub certs_encoding: LenEncoding,
    pub certs_key_encoding: Option<cbor_event::Sz>,
    pub withdrawals_encoding: LenEncoding,
    pub withdrawals_value_encodings: BTreeMap<RewardAccount, Option<cbor_event::Sz>>,
    pub withdrawals_key_encoding: Option<cbor_event::Sz>,
    pub update_key_encoding: Option<cbor_event::Sz>,
    pub auxiliary_data_hash_encoding: StringEncoding,
    pub auxiliary_data_hash_key_encoding: Option<cbor_event::Sz>,
    pub validity_interval_start_encoding: Option<cbor_event::Sz>,
    pub validity_interval_start_key_encoding: Option<cbor_event::Sz>,
    pub mint_key_encoding: Option<cbor_event::Sz>,
    pub script_data_hash_encoding: StringEncoding,
    pub script_data_hash_key_encoding: Option<cbor_event::Sz>,
    pub collateral_inputs_encoding: LenEncoding,
    pub collateral_inputs_key_encoding: Option<cbor_event::Sz>,
    pub required_signers_encoding: LenEncoding,
    pub required_signers_elem_encodings: Vec<StringEncoding>,
    pub required_signers_key_encoding: Option<cbor_event::Sz>,
    pub network_id_key_encoding: Option<cbor_event::Sz>,
    pub collateral_return_key_encoding: Option<cbor_event::Sz>,
    pub total_collateral_encoding: Option<cbor_event::Sz>,
    pub total_collateral_key_encoding: Option<cbor_event::Sz>,
    pub reference_inputs_encoding: LenEncoding,
    pub reference_inputs_key_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct BabbageTransactionEncoding {
    pub len_encoding: LenEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct BabbageTransactionWitnessSetEncoding {
    pub len_encoding: LenEncoding,
    pub orig_deser_order: Vec<usize>,
    pub vkeywitnesses_encoding: LenEncoding,
    pub vkeywitnesses_key_encoding: Option<cbor_event::Sz>,
    pub native_scripts_encoding: LenEncoding,
    pub native_scripts_key_encoding: Option<cbor_event::Sz>,
    pub bootstrap_witnesses_encoding: LenEncoding,
    pub bootstrap_witnesses_key_encoding: Option<cbor_event::Sz>,
    pub plutus_v1_scripts_encoding: LenEncoding,
    pub plutus_v1_scripts_key_encoding: Option<cbor_event::Sz>,
    pub plutus_datums_encoding: LenEncoding,
    pub plutus_datums_key_encoding: Option<cbor_event::Sz>,
    pub redeemers_encoding: LenEncoding,
    pub redeemers_key_encoding: Option<cbor_event::Sz>,
    pub plutus_v2_scripts_encoding: LenEncoding,
    pub plutus_v2_scripts_key_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct BabbageUpdateEncoding {
    pub len_encoding: LenEncoding,
    pub updates_encoding: LenEncoding,
    pub updates_key_encodings: BTreeMap<GenesisHash, StringEncoding>,
    pub epoch_encoding: Option<cbor_event::Sz>,
}
