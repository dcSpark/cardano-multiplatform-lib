// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use cml_core::serialization::{LenEncoding, StringEncoding};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default)]
pub struct CostModelsEncoding {
    pub inner_encoding: LenEncoding,
    pub inner_key_encodings: BTreeMap<u64, Option<cbor_event::Sz>>,
    pub inner_value_encodings: BTreeMap<u64, (LenEncoding, Vec<Option<cbor_event::Sz>>)>,
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
pub struct LegacyRedeemerEncoding {
    pub len_encoding: LenEncoding,
    pub tag_encoding: Option<cbor_event::Sz>,
    pub index_encoding: Option<cbor_event::Sz>,
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
pub struct PlutusV3ScriptEncoding {
    pub inner_encoding: StringEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct RedeemerKeyEncoding {
    pub len_encoding: LenEncoding,
    pub tag_encoding: Option<cbor_event::Sz>,
    pub index_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct RedeemerValEncoding {
    pub len_encoding: LenEncoding,
}
