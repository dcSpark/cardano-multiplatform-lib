// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use cml_chain::{address::RewardAccount, certs::StakeCredential, crypto::GenesisHash};
use cml_core::serialization::{LenEncoding, StringEncoding};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default)]
pub struct GenesisKeyDelegationEncoding {
    pub len_encoding: LenEncoding,
    pub tag_encoding: Option<cbor_event::Sz>,
    pub genesis_hash_encoding: StringEncoding,
    pub genesis_delegate_hash_encoding: StringEncoding,
    pub vrf_key_hash_encoding: StringEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct MultisigAllEncoding {
    pub len_encoding: LenEncoding,
    pub tag_encoding: Option<cbor_event::Sz>,
    pub multisig_scripts_encoding: LenEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct MultisigAnyEncoding {
    pub len_encoding: LenEncoding,
    pub tag_encoding: Option<cbor_event::Sz>,
    pub multisig_scripts_encoding: LenEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct MultisigNOfKEncoding {
    pub len_encoding: LenEncoding,
    pub tag_encoding: Option<cbor_event::Sz>,
    pub n_encoding: Option<cbor_event::Sz>,
    pub multisig_scripts_encoding: LenEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct MultisigPubkeyEncoding {
    pub len_encoding: LenEncoding,
    pub tag_encoding: Option<cbor_event::Sz>,
    pub ed25519_key_hash_encoding: StringEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct ProtocolVersionStructEncoding {
    pub len_encoding: LenEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct ShelleyBlockEncoding {
    pub len_encoding: LenEncoding,
    pub transaction_bodies_encoding: LenEncoding,
    pub transaction_witness_sets_encoding: LenEncoding,
    pub transaction_metadata_set_encoding: LenEncoding,
    pub transaction_metadata_set_key_encodings: BTreeMap<u16, Option<cbor_event::Sz>>,
}

#[derive(Clone, Debug, Default)]
pub struct ShelleyDNSNameEncoding {
    pub inner_encoding: StringEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct ShelleyHeaderBodyEncoding {
    pub len_encoding: LenEncoding,
    pub block_number_encoding: Option<cbor_event::Sz>,
    pub slot_encoding: Option<cbor_event::Sz>,
    pub prev_hash_encoding: StringEncoding,
    pub issuer_vkey_encoding: StringEncoding,
    pub vrf_vkey_encoding: StringEncoding,
    pub block_body_size_encoding: Option<cbor_event::Sz>,
    pub block_body_hash_encoding: StringEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct ShelleyHeaderEncoding {
    pub len_encoding: LenEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct ShelleyMoveInstantaneousRewardEncoding {
    pub len_encoding: LenEncoding,
    pub pot_encoding: Option<cbor_event::Sz>,
    pub to_stake_credentials_encoding: LenEncoding,
    pub to_stake_credentials_value_encodings: BTreeMap<StakeCredential, Option<cbor_event::Sz>>,
}

#[derive(Clone, Debug, Default)]
pub struct ShelleyMoveInstantaneousRewardsCertEncoding {
    pub len_encoding: LenEncoding,
    pub tag_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct ShelleyMultiHostNameEncoding {
    pub len_encoding: LenEncoding,
    pub tag_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct ShelleyPoolParamsEncoding {
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
pub struct ShelleyPoolRegistrationEncoding {
    pub len_encoding: LenEncoding,
    pub tag_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct ShelleyProtocolParamUpdateEncoding {
    pub len_encoding: LenEncoding,
    pub orig_deser_order: Vec<usize>,
    pub minfee_a_encoding: Option<cbor_event::Sz>,
    pub minfee_a_key_encoding: Option<cbor_event::Sz>,
    pub minfee_b_encoding: Option<cbor_event::Sz>,
    pub minfee_b_key_encoding: Option<cbor_event::Sz>,
    pub max_block_body_size_encoding: Option<cbor_event::Sz>,
    pub max_block_body_size_key_encoding: Option<cbor_event::Sz>,
    pub max_transaction_size_encoding: Option<cbor_event::Sz>,
    pub max_transaction_size_key_encoding: Option<cbor_event::Sz>,
    pub max_block_header_size_encoding: Option<cbor_event::Sz>,
    pub max_block_header_size_key_encoding: Option<cbor_event::Sz>,
    pub key_deposit_encoding: Option<cbor_event::Sz>,
    pub key_deposit_key_encoding: Option<cbor_event::Sz>,
    pub pool_deposit_encoding: Option<cbor_event::Sz>,
    pub pool_deposit_key_encoding: Option<cbor_event::Sz>,
    pub maximum_epoch_encoding: Option<cbor_event::Sz>,
    pub maximum_epoch_key_encoding: Option<cbor_event::Sz>,
    pub n_opt_encoding: Option<cbor_event::Sz>,
    pub n_opt_key_encoding: Option<cbor_event::Sz>,
    pub pool_pledge_influence_key_encoding: Option<cbor_event::Sz>,
    pub expansion_rate_key_encoding: Option<cbor_event::Sz>,
    pub treasury_growth_rate_key_encoding: Option<cbor_event::Sz>,
    pub decentralization_constant_key_encoding: Option<cbor_event::Sz>,
    pub extra_entropy_key_encoding: Option<cbor_event::Sz>,
    pub protocol_version_key_encoding: Option<cbor_event::Sz>,
    pub min_utxo_value_encoding: Option<cbor_event::Sz>,
    pub min_utxo_value_key_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct ShelleySingleHostNameEncoding {
    pub len_encoding: LenEncoding,
    pub tag_encoding: Option<cbor_event::Sz>,
    pub port_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct ShelleyTransactionBodyEncoding {
    pub len_encoding: LenEncoding,
    pub orig_deser_order: Vec<usize>,
    pub inputs_encoding: LenEncoding,
    pub inputs_key_encoding: Option<cbor_event::Sz>,
    pub outputs_encoding: LenEncoding,
    pub outputs_key_encoding: Option<cbor_event::Sz>,
    pub fee_encoding: Option<cbor_event::Sz>,
    pub fee_key_encoding: Option<cbor_event::Sz>,
    pub ttl_encoding: Option<cbor_event::Sz>,
    pub ttl_key_encoding: Option<cbor_event::Sz>,
    pub certs_encoding: LenEncoding,
    pub certs_key_encoding: Option<cbor_event::Sz>,
    pub withdrawals_encoding: LenEncoding,
    pub withdrawals_value_encodings: BTreeMap<RewardAccount, Option<cbor_event::Sz>>,
    pub withdrawals_key_encoding: Option<cbor_event::Sz>,
    pub update_key_encoding: Option<cbor_event::Sz>,
    pub auxiliary_data_hash_encoding: StringEncoding,
    pub auxiliary_data_hash_key_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct ShelleyTransactionEncoding {
    pub len_encoding: LenEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct ShelleyTransactionOutputEncoding {
    pub len_encoding: LenEncoding,
    pub amount_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct ShelleyTransactionWitnessSetEncoding {
    pub len_encoding: LenEncoding,
    pub orig_deser_order: Vec<usize>,
    pub vkeywitnesses_encoding: LenEncoding,
    pub vkeywitnesses_key_encoding: Option<cbor_event::Sz>,
    pub native_scripts_encoding: LenEncoding,
    pub native_scripts_key_encoding: Option<cbor_event::Sz>,
    pub bootstrap_witnesses_encoding: LenEncoding,
    pub bootstrap_witnesses_key_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct ShelleyUpdateEncoding {
    pub len_encoding: LenEncoding,
    pub shelley_proposed_protocol_parameter_updates_encoding: LenEncoding,
    pub shelley_proposed_protocol_parameter_updates_key_encodings:
        BTreeMap<GenesisHash, StringEncoding>,
    pub epoch_encoding: Option<cbor_event::Sz>,
}
