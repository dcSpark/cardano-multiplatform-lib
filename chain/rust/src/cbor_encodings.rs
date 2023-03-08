// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use crate::crypto::{GenesisHash, ScriptHash};
use crate::AssetName;
use cml_core::serialization::{LenEncoding, StringEncoding};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default)]
pub struct AssetNameEncoding {
    pub inner_encoding: StringEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct BootstrapWitnessEncoding {
    pub len_encoding: LenEncoding,
    pub public_key_encoding: StringEncoding,
    pub signature_encoding: StringEncoding,
    pub chain_code_encoding: StringEncoding,
    pub attributes_encoding: StringEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct PositiveIntervalEncoding {
    pub len_encoding: LenEncoding,
    pub tag_encoding: Option<cbor_event::Sz>,
    pub strart_encoding: Option<cbor_event::Sz>,
    pub end_encoding: Option<cbor_event::Sz>,
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
pub struct ProtocolVersionStructEncoding {
    pub len_encoding: LenEncoding,
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

#[derive(Clone, Debug, Default)]
pub struct UpdateEncoding {
    pub len_encoding: LenEncoding,
    pub proposed_protocol_parameter_updates_encoding: LenEncoding,
    pub proposed_protocol_parameter_updates_key_encodings: BTreeMap<GenesisHash, StringEncoding>,
    pub epoch_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct ValueEncoding {
    pub len_encoding: LenEncoding,
    pub coin_encoding: Option<cbor_event::Sz>,
    pub multiasset_encoding: LenEncoding,
    pub multiasset_key_encodings: BTreeMap<ScriptHash, StringEncoding>,
    pub multiasset_value_encodings:
        BTreeMap<ScriptHash, (LenEncoding, BTreeMap<AssetName, Option<cbor_event::Sz>>)>,
}

#[derive(Clone, Debug, Default)]
pub struct VkeywitnessEncoding {
    pub len_encoding: LenEncoding,
    pub vkey_encoding: StringEncoding,
    pub ed25519_signature_encoding: StringEncoding,
}
