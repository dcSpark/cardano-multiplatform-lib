// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use cml_core::serialization::{LenEncoding, StringEncoding};

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
pub struct VRFCertEncoding {
    pub len_encoding: LenEncoding,
    pub output_encoding: StringEncoding,
    pub proof_encoding: StringEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct VkeywitnessEncoding {
    pub len_encoding: LenEncoding,
    pub vkey_encoding: StringEncoding,
    pub ed25519_signature_encoding: StringEncoding,
}
