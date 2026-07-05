// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use cml_core::serialization::{LenEncoding, StringEncoding};

#[derive(Clone, Debug, Default)]
pub struct CIP36DelegationEncoding {
    pub len_encoding: LenEncoding,
    pub voting_pub_key_encoding: StringEncoding,
    pub weight_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct CIP36DeregistrationCborEncoding {
    pub len_encoding: LenEncoding,
    pub orig_deser_order: Vec<usize>,
    pub key_deregistration_key_encoding: Option<cbor_event::Sz>,
    pub deregistration_witness_key_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct CIP36DeregistrationWitnessEncoding {
    pub len_encoding: LenEncoding,
    pub orig_deser_order: Vec<usize>,
    pub stake_witness_encoding: StringEncoding,
    pub stake_witness_key_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct CIP36KeyDeregistrationEncoding {
    pub len_encoding: LenEncoding,
    pub orig_deser_order: Vec<usize>,
    pub stake_credential_encoding: StringEncoding,
    pub stake_credential_key_encoding: Option<cbor_event::Sz>,
    pub nonce_encoding: Option<cbor_event::Sz>,
    pub nonce_key_encoding: Option<cbor_event::Sz>,
    pub voting_purpose_encoding: Option<cbor_event::Sz>,
    pub voting_purpose_default_present: bool,
    pub voting_purpose_key_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct CIP36KeyRegistrationEncoding {
    pub len_encoding: LenEncoding,
    pub orig_deser_order: Vec<usize>,
    pub delegation_key_encoding: Option<cbor_event::Sz>,
    pub stake_credential_encoding: StringEncoding,
    pub stake_credential_key_encoding: Option<cbor_event::Sz>,
    pub address_key_encoding: Option<cbor_event::Sz>,
    pub nonce_encoding: Option<cbor_event::Sz>,
    pub nonce_key_encoding: Option<cbor_event::Sz>,
    pub voting_purpose_encoding: Option<cbor_event::Sz>,
    pub voting_purpose_default_present: bool,
    pub voting_purpose_key_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct CIP36RegistrationCborEncoding {
    pub len_encoding: LenEncoding,
    pub orig_deser_order: Vec<usize>,
    pub key_registration_key_encoding: Option<cbor_event::Sz>,
    pub registration_witness_key_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct CIP36RegistrationWitnessEncoding {
    pub len_encoding: LenEncoding,
    pub orig_deser_order: Vec<usize>,
    pub stake_witness_encoding: StringEncoding,
    pub stake_witness_key_encoding: Option<cbor_event::Sz>,
}
