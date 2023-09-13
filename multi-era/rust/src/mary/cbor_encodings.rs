// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use cml_chain::{address::RewardAccount, assets::AssetName, PolicyId};
use cml_core::serialization::{LenEncoding, StringEncoding};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default)]
pub struct MaryBlockEncoding {
    pub len_encoding: LenEncoding,
    pub transaction_bodies_encoding: LenEncoding,
    pub transaction_witness_sets_encoding: LenEncoding,
    pub auxiliary_data_set_encoding: LenEncoding,
    pub auxiliary_data_set_key_encodings: BTreeMap<u16, Option<cbor_event::Sz>>,
}

#[derive(Clone, Debug, Default)]
pub struct MaryTransactionBodyEncoding {
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
    pub mint_encoding: LenEncoding,
    pub mint_key_encodings: BTreeMap<PolicyId, StringEncoding>,
    pub mint_value_encodings:
        BTreeMap<PolicyId, (LenEncoding, BTreeMap<AssetName, Option<cbor_event::Sz>>)>,
    pub mint_key_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct MaryTransactionEncoding {
    pub len_encoding: LenEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct MaryTransactionOutputEncoding {
    pub len_encoding: LenEncoding,
}
