// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use super::*;
use cml_core::serialization::{LenEncoding, StringEncoding};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default)]
pub struct AssetNameEncoding {
    pub inner_encoding: StringEncoding,
}
