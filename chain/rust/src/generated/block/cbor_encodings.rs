// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use cml_core::serialization::{LenEncoding, StringEncoding};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default)]
pub struct BlockEncoding {
    pub len_encoding: LenEncoding,
    pub transaction_bodies_encoding: LenEncoding,
    pub transaction_witness_sets_encoding: LenEncoding,
    pub auxiliary_data_set_encoding: LenEncoding,
    pub auxiliary_data_set_key_encodings: BTreeMap<u16, Option<cbor_event::Sz>>,
    pub invalid_transactions_encoding: LenEncoding,
    pub invalid_transactions_elem_encodings: Vec<Option<cbor_event::Sz>>,
}

#[derive(Clone, Debug, Default)]
pub struct HeaderBodyEncoding {
    pub len_encoding: LenEncoding,
    pub block_number_encoding: Option<cbor_event::Sz>,
    pub slot_encoding: Option<cbor_event::Sz>,
    pub prev_hash_encoding: StringEncoding,
    pub issuer_vkey_encoding: StringEncoding,
    pub vrf_vkey_encoding: StringEncoding,
    pub block_body_size_encoding: Option<cbor_event::Sz>,
    pub block_body_hash_encoding: StringEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct HeaderEncoding {
    pub len_encoding: LenEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct OperationalCertEncoding {
    pub len_encoding: LenEncoding,
    pub hot_vkey_encoding: StringEncoding,
    pub sequence_number_encoding: Option<cbor_event::Sz>,
    pub kes_period_encoding: Option<cbor_event::Sz>,
    pub sigma_encoding: StringEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct ProtocolVersionEncoding {
    pub len_encoding: LenEncoding,
    pub major_encoding: Option<cbor_event::Sz>,
    pub minor_encoding: Option<cbor_event::Sz>,
}
