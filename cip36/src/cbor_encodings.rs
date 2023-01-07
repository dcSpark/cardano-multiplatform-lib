use super::*;

#[derive(Clone, Debug, Default)]
pub struct DelegationEncoding {
    pub len_encoding: LenEncoding,
    pub weight_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct DeregistrationWitnessEncoding {
    pub len_encoding: LenEncoding,
    pub orig_deser_order: Vec<usize>,
    pub stake_witness_key_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct Ed25519SignatureEncoding {
    pub inner_encoding: StringEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct KeyDeregistrationEncoding {
    pub len_encoding: LenEncoding,
    pub orig_deser_order: Vec<usize>,
    pub stake_credential_key_encoding: Option<cbor_event::Sz>,
    pub nonce_encoding: Option<cbor_event::Sz>,
    pub nonce_key_encoding: Option<cbor_event::Sz>,
    pub voting_purpose_encoding: Option<cbor_event::Sz>,
    pub voting_purpose_default_present: bool,
    pub voting_purpose_key_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct KeyRegistrationEncoding {
    pub len_encoding: LenEncoding,
    pub orig_deser_order: Vec<usize>,
    pub delegation_key_encoding: Option<cbor_event::Sz>,
    pub stake_credential_key_encoding: Option<cbor_event::Sz>,
    pub reward_address_key_encoding: Option<cbor_event::Sz>,
    pub nonce_encoding: Option<cbor_event::Sz>,
    pub nonce_key_encoding: Option<cbor_event::Sz>,
    pub voting_purpose_encoding: Option<cbor_event::Sz>,
    pub voting_purpose_default_present: bool,
    pub voting_purpose_key_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct RegistrationWitnessEncoding {
    pub len_encoding: LenEncoding,
    pub orig_deser_order: Vec<usize>,
    pub stake_witness_key_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct StakingPubKeyEncoding {
    pub inner_encoding: StringEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct VotingPubKeyEncoding {
    pub inner_encoding: StringEncoding,
}