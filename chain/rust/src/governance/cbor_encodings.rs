// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use crate::address::RewardAccount;
use crate::certs::Credential;
use cml_core::serialization::{LenEncoding, StringEncoding};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default)]
pub struct AnchorEncoding {
    pub len_encoding: LenEncoding,
    pub anchor_doc_hash_encoding: StringEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct ConstitutionEncoding {
    pub len_encoding: LenEncoding,
    pub script_hash_encoding: StringEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct GovActionIdEncoding {
    pub len_encoding: LenEncoding,
    pub transaction_id_encoding: StringEncoding,
    pub gov_action_index_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct HardForkInitiationActionEncoding {
    pub len_encoding: LenEncoding,
    pub tag_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct NewConstitutionEncoding {
    pub len_encoding: LenEncoding,
    pub tag_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct NoConfidenceEncoding {
    pub len_encoding: LenEncoding,
    pub tag_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct ParameterChangeActionEncoding {
    pub len_encoding: LenEncoding,
    pub tag_encoding: Option<cbor_event::Sz>,
    pub policy_hash_encoding: StringEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct ProposalProcedureEncoding {
    pub len_encoding: LenEncoding,
    pub deposit_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct TreasuryWithdrawalsActionEncoding {
    pub len_encoding: LenEncoding,
    pub tag_encoding: Option<cbor_event::Sz>,
    pub withdrawal_encoding: LenEncoding,
    pub withdrawal_value_encodings: BTreeMap<RewardAccount, Option<cbor_event::Sz>>,
    pub policy_hash_encoding: StringEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct UpdateCommitteeEncoding {
    pub len_encoding: LenEncoding,
    pub tag_encoding: Option<cbor_event::Sz>,
    pub credentials_encoding: LenEncoding,
    pub credentials_value_encodings: BTreeMap<Credential, Option<cbor_event::Sz>>,
}

#[derive(Clone, Debug, Default)]
pub struct VotingProcedureEncoding {
    pub len_encoding: LenEncoding,
    pub vote_encoding: Option<cbor_event::Sz>,
}
