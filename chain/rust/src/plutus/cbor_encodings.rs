// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use cml_core::serialization::{LenEncoding, StringEncoding};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default)]
pub struct ConstrPlutusDataEncoding {
    pub len_encoding: LenEncoding,
    pub tag_encoding: Option<cbor_event::Sz>,
    pub constructor_encoding: Option<cbor_event::Sz>,
    pub fields_encoding: LenEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct CostModelsEncoding {
    pub len_encoding: LenEncoding,
    pub orig_deser_order: Vec<usize>,
    pub plutus_v1_encoding: LenEncoding,
    pub plutus_v1_key_encoding: Option<cbor_event::Sz>,
    pub plutus_v2_encoding: LenEncoding,
    pub plutus_v2_key_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct ExUnitPricesEncoding {
    pub len_encoding: LenEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct ExUnitsEncoding {
    pub len_encoding: LenEncoding,
    pub mem_encoding: Option<cbor_event::Sz>,
    pub steps_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct PlutusV1ScriptEncoding {
    pub inner_encoding: StringEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct PlutusV2ScriptEncoding {
    pub inner_encoding: StringEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct RedeemerEncoding {
    pub len_encoding: LenEncoding,
    pub tag_encoding: Option<cbor_event::Sz>,
    pub index_encoding: Option<cbor_event::Sz>,
}
