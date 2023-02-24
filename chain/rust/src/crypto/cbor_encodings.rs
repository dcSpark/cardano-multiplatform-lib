// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use cml_core::serialization::{LenEncoding, StringEncoding};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default)]
pub struct BootstrapWitnessEncoding {
    pub len_encoding: LenEncoding,
    pub public_key_encoding: StringEncoding,
    pub signature_encoding: StringEncoding,
    pub chain_code_encoding: StringEncoding,
    pub attributes_encoding: StringEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct KESSignatureEncoding {
    pub inner_encoding: StringEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct SignkeyKESEncoding {
    pub inner_encoding: StringEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct VRFCertEncoding {
    pub len_encoding: LenEncoding,
    pub index_0_encoding: StringEncoding,
    pub bytes_encoding: StringEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct VkeywitnessEncoding {
    pub len_encoding: LenEncoding,
    pub vkey_encoding: StringEncoding,
    pub ed25519_signature_encoding: StringEncoding,
}
