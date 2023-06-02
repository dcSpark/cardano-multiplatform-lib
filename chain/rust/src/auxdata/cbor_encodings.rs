// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use cml_core::serialization::{LenEncoding, StringEncoding};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default)]
pub struct AlonzoAuxDataEncoding {
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
pub struct ShelleyMaAuxDataEncoding {
    pub len_encoding: LenEncoding,
    pub auxiliary_scripts_encoding: LenEncoding,
}
