// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use cml_core::serialization::{LenEncoding, StringEncoding};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default)]
pub struct DnsNameEncoding {
    pub inner_encoding: StringEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct GenesisKeyDelegationEncoding {
    pub len_encoding: LenEncoding,
    pub tag_encoding: Option<cbor_event::Sz>,
    pub genesis_hash_encoding: StringEncoding,
    pub genesis_delegate_hash_encoding: StringEncoding,
    pub v_r_f_key_hash_encoding: StringEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct Ipv4Encoding {
    pub inner_encoding: StringEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct Ipv6Encoding {
    pub inner_encoding: StringEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct MoveInstantaneousRewardEncoding {
    pub len_encoding: LenEncoding,
    pub pot_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct MoveInstantaneousRewardsCertEncoding {
    pub len_encoding: LenEncoding,
    pub tag_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct MultiHostNameEncoding {
    pub len_encoding: LenEncoding,
    pub tag_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct PoolMetadataEncoding {
    pub len_encoding: LenEncoding,
    pub pool_metadata_hash_encoding: StringEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct PoolParamsEncoding {
    pub len_encoding: LenEncoding,
    pub operator_encoding: StringEncoding,
    pub vrf_keyhash_encoding: StringEncoding,
    pub pledge_encoding: Option<cbor_event::Sz>,
    pub cost_encoding: Option<cbor_event::Sz>,
    pub pool_owners_encoding: LenEncoding,
    pub pool_owners_elem_encodings: Vec<StringEncoding>,
    pub relays_encoding: LenEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct PoolRegistrationEncoding {
    pub len_encoding: LenEncoding,
    pub tag_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct PoolRetirementEncoding {
    pub len_encoding: LenEncoding,
    pub tag_encoding: Option<cbor_event::Sz>,
    pub ed25519_key_hash_encoding: StringEncoding,
    pub epoch_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct SingleHostAddrEncoding {
    pub len_encoding: LenEncoding,
    pub tag_encoding: Option<cbor_event::Sz>,
    pub port_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct SingleHostNameEncoding {
    pub len_encoding: LenEncoding,
    pub tag_encoding: Option<cbor_event::Sz>,
    pub port_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct StakeDelegationEncoding {
    pub len_encoding: LenEncoding,
    pub tag_encoding: Option<cbor_event::Sz>,
    pub ed25519_key_hash_encoding: StringEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct StakeDeregistrationEncoding {
    pub len_encoding: LenEncoding,
    pub tag_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct StakeRegistrationEncoding {
    pub len_encoding: LenEncoding,
    pub tag_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct UrlEncoding {
    pub inner_encoding: StringEncoding,
}
