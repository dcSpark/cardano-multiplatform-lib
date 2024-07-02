// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use cml_core::serialization::{LenEncoding, StringEncoding};

#[derive(Clone, Debug, Default)]
pub struct AuthCommitteeHotCertEncoding {
    pub len_encoding: LenEncoding,
    pub index_0_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct DNSNameEncoding {
    pub inner_encoding: StringEncoding,
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
    pub pool_encoding: StringEncoding,
    pub epoch_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct RegCertEncoding {
    pub len_encoding: LenEncoding,
    pub tag_encoding: Option<cbor_event::Sz>,
    pub deposit_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct RegDrepCertEncoding {
    pub len_encoding: LenEncoding,
    pub index_0_encoding: Option<cbor_event::Sz>,
    pub deposit_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct ResignCommitteeColdCertEncoding {
    pub len_encoding: LenEncoding,
    pub index_0_encoding: Option<cbor_event::Sz>,
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
    pub pool_encoding: StringEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct StakeDeregistrationEncoding {
    pub len_encoding: LenEncoding,
    pub tag_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct StakeRegDelegCertEncoding {
    pub len_encoding: LenEncoding,
    pub tag_encoding: Option<cbor_event::Sz>,
    pub pool_encoding: StringEncoding,
    pub deposit_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct StakeRegistrationEncoding {
    pub len_encoding: LenEncoding,
    pub tag_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct StakeVoteDelegCertEncoding {
    pub len_encoding: LenEncoding,
    pub tag_encoding: Option<cbor_event::Sz>,
    pub pool_encoding: StringEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct StakeVoteRegDelegCertEncoding {
    pub len_encoding: LenEncoding,
    pub tag_encoding: Option<cbor_event::Sz>,
    pub pool_encoding: StringEncoding,
    pub deposit_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct UnregCertEncoding {
    pub len_encoding: LenEncoding,
    pub tag_encoding: Option<cbor_event::Sz>,
    pub deposit_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct UnregDrepCertEncoding {
    pub len_encoding: LenEncoding,
    pub index_0_encoding: Option<cbor_event::Sz>,
    pub deposit_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct UpdateDrepCertEncoding {
    pub len_encoding: LenEncoding,
    pub index_0_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct UrlEncoding {
    pub inner_encoding: StringEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct VoteDelegCertEncoding {
    pub len_encoding: LenEncoding,
    pub tag_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct VoteRegDelegCertEncoding {
    pub len_encoding: LenEncoding,
    pub tag_encoding: Option<cbor_event::Sz>,
    pub deposit_encoding: Option<cbor_event::Sz>,
}
