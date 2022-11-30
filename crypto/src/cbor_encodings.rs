pub use cardano_multiplatform_lib_core::serialization::{LenEncoding, StringEncoding};

#[derive(Clone, Debug, Default)]
pub struct BootstrapWitnessEncoding {
    pub len_encoding: LenEncoding,
    pub chain_code_encoding: StringEncoding,
    pub attributes_encoding: StringEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct KesSignatureEncoding {
    pub inner_encoding: StringEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct KesVkeyEncoding {
    pub inner_encoding: StringEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct Nonce1Encoding {
    pub len_encoding: LenEncoding,
    pub index_0_encoding: Option<cbor_event::Sz>,
    pub bytes_encoding: StringEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct SignatureEncoding {
    pub inner_encoding: StringEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct SignkeyKESEncoding {
    pub inner_encoding: StringEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct VkeyEncoding {
    pub pubkey_bytes_encoding: StringEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct VkeywitnessEncoding {
    pub len_encoding: LenEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct VrfCertEncoding {
    pub len_encoding: LenEncoding,
    pub index_0_encoding: StringEncoding,
    pub bytes_encoding: StringEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct VrfVkeyEncoding {
    pub inner_encoding: StringEncoding,
}