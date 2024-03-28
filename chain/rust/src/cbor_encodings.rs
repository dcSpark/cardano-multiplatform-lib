// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use cml_core::serialization::LenEncoding;

#[derive(Clone, Debug, Default)]
pub struct DRepVotingThresholdsEncoding {
    pub len_encoding: LenEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct PoolVotingThresholdsEncoding {
    pub len_encoding: LenEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct ProtocolParamUpdateEncoding {
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
    pub pool_voting_thresholds_key_encoding: Option<cbor_event::Sz>,
    pub d_rep_voting_thresholds_key_encoding: Option<cbor_event::Sz>,
    pub min_committee_size_encoding: Option<cbor_event::Sz>,
    pub min_committee_size_key_encoding: Option<cbor_event::Sz>,
    pub committee_term_limit_encoding: Option<cbor_event::Sz>,
    pub committee_term_limit_key_encoding: Option<cbor_event::Sz>,
    pub governance_action_validity_period_encoding: Option<cbor_event::Sz>,
    pub governance_action_validity_period_key_encoding: Option<cbor_event::Sz>,
    pub governance_action_deposit_encoding: Option<cbor_event::Sz>,
    pub governance_action_deposit_key_encoding: Option<cbor_event::Sz>,
    pub d_rep_deposit_encoding: Option<cbor_event::Sz>,
    pub d_rep_deposit_key_encoding: Option<cbor_event::Sz>,
    pub d_rep_inactivity_period_encoding: Option<cbor_event::Sz>,
    pub d_rep_inactivity_period_key_encoding: Option<cbor_event::Sz>,
    pub min_fee_ref_script_cost_per_byte_key_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct RationalEncoding {
    pub len_encoding: LenEncoding,
    pub tag_encoding: Option<cbor_event::Sz>,
    pub numerator_encoding: Option<cbor_event::Sz>,
    pub denominator_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct UnitIntervalEncoding {
    pub len_encoding: LenEncoding,
    pub tag_encoding: Option<cbor_event::Sz>,
    pub start_encoding: Option<cbor_event::Sz>,
    pub end_encoding: Option<cbor_event::Sz>,
}
