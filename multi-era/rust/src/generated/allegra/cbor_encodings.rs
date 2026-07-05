// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use cml_chain::address::RewardAccount;
use cml_core::serialization::{LenEncoding, StringEncoding};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default)]
pub struct AllegraBlockEncoding {
    pub len_encoding: LenEncoding,
    pub transaction_bodies_encoding: LenEncoding,
    pub transaction_witness_sets_encoding: LenEncoding,
    pub auxiliary_data_set_encoding: LenEncoding,
    pub auxiliary_data_set_key_encodings: BTreeMap<u16, Option<cbor_event::Sz>>,
}

#[derive(Clone, Debug, Default)]
pub struct AllegraTransactionBodyEncoding {
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
}

#[derive(Clone, Debug, Default)]
pub struct AllegraTransactionEncoding {
    pub len_encoding: LenEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct AllegraTransactionWitnessSetEncoding {
    pub len_encoding: LenEncoding,
    pub orig_deser_order: Vec<usize>,
    pub vkeywitnesses_encoding: LenEncoding,
    pub vkeywitnesses_key_encoding: Option<cbor_event::Sz>,
    pub native_scripts_encoding: LenEncoding,
    pub native_scripts_key_encoding: Option<cbor_event::Sz>,
    pub bootstrap_witnesses_encoding: LenEncoding,
    pub bootstrap_witnesses_key_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct MoveInstantaneousRewardEncoding {
    pub len_encoding: LenEncoding,
    pub pot_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct MoveInstantaneousRewardsCertEncoding {
    pub len_encoding: LenEncoding,
    pub tag_encoding: Option<cbor_event::Sz>,
}
