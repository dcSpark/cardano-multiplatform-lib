// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use crate::{address::RewardAccount, assets::AssetName, Voter};
use cml_core::serialization::{LenEncoding, StringEncoding};
use cml_crypto::ScriptHash;
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default)]
pub struct AlonzoFormatTxOutEncoding {
    pub len_encoding: LenEncoding,
    pub datum_hash_encoding: StringEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct ConwayFormatTxOutEncoding {
    pub len_encoding: LenEncoding,
    pub orig_deser_order: Vec<usize>,
    pub address_key_encoding: Option<cbor_event::Sz>,
    pub amount_key_encoding: Option<cbor_event::Sz>,
    pub datum_option_key_encoding: Option<cbor_event::Sz>,
    pub script_reference_tag_encoding: Option<cbor_event::Sz>,
    pub script_reference_bytes_encoding: StringEncoding,
    pub script_reference_key_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct ScriptAllEncoding {
    pub len_encoding: LenEncoding,
    pub tag_encoding: Option<cbor_event::Sz>,
    pub native_scripts_encoding: LenEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct ScriptAnyEncoding {
    pub len_encoding: LenEncoding,
    pub tag_encoding: Option<cbor_event::Sz>,
    pub native_scripts_encoding: LenEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct ScriptInvalidBeforeEncoding {
    pub len_encoding: LenEncoding,
    pub tag_encoding: Option<cbor_event::Sz>,
    pub before_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct ScriptInvalidHereafterEncoding {
    pub len_encoding: LenEncoding,
    pub tag_encoding: Option<cbor_event::Sz>,
    pub after_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct ScriptNOfKEncoding {
    pub len_encoding: LenEncoding,
    pub tag_encoding: Option<cbor_event::Sz>,
    pub n_encoding: Option<cbor_event::Sz>,
    pub native_scripts_encoding: LenEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct ScriptPubkeyEncoding {
    pub len_encoding: LenEncoding,
    pub tag_encoding: Option<cbor_event::Sz>,
    pub ed25519_key_hash_encoding: StringEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct TransactionBodyEncoding {
    pub len_encoding: LenEncoding,
    pub orig_deser_order: Vec<usize>,
    pub inputs_key_encoding: Option<cbor_event::Sz>,
    pub outputs_encoding: LenEncoding,
    pub outputs_key_encoding: Option<cbor_event::Sz>,
    pub fee_encoding: Option<cbor_event::Sz>,
    pub fee_key_encoding: Option<cbor_event::Sz>,
    pub ttl_encoding: Option<cbor_event::Sz>,
    pub ttl_key_encoding: Option<cbor_event::Sz>,
    pub certs_key_encoding: Option<cbor_event::Sz>,
    pub withdrawals_encoding: LenEncoding,
    pub withdrawals_value_encodings: BTreeMap<RewardAccount, Option<cbor_event::Sz>>,
    pub withdrawals_key_encoding: Option<cbor_event::Sz>,
    pub auxiliary_data_hash_encoding: StringEncoding,
    pub auxiliary_data_hash_key_encoding: Option<cbor_event::Sz>,
    pub validity_interval_start_encoding: Option<cbor_event::Sz>,
    pub validity_interval_start_key_encoding: Option<cbor_event::Sz>,
    pub mint_encoding: LenEncoding,
    pub mint_key_encodings: BTreeMap<ScriptHash, StringEncoding>,
    pub mint_value_encodings:
        BTreeMap<ScriptHash, (LenEncoding, BTreeMap<AssetName, Option<cbor_event::Sz>>)>,
    pub mint_key_encoding: Option<cbor_event::Sz>,
    pub script_data_hash_encoding: StringEncoding,
    pub script_data_hash_key_encoding: Option<cbor_event::Sz>,
    pub collateral_inputs_key_encoding: Option<cbor_event::Sz>,
    pub required_signers_key_encoding: Option<cbor_event::Sz>,
    pub network_id_key_encoding: Option<cbor_event::Sz>,
    pub collateral_return_key_encoding: Option<cbor_event::Sz>,
    pub total_collateral_encoding: Option<cbor_event::Sz>,
    pub total_collateral_key_encoding: Option<cbor_event::Sz>,
    pub reference_inputs_key_encoding: Option<cbor_event::Sz>,
    pub voting_procedures_encoding: LenEncoding,
    pub voting_procedures_value_encodings: BTreeMap<Voter, LenEncoding>,
    pub voting_procedures_key_encoding: Option<cbor_event::Sz>,
    pub proposal_procedures_key_encoding: Option<cbor_event::Sz>,
    pub current_treasury_value_encoding: Option<cbor_event::Sz>,
    pub current_treasury_value_key_encoding: Option<cbor_event::Sz>,
    pub donation_encoding: Option<cbor_event::Sz>,
    pub donation_key_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct TransactionEncoding {
    pub len_encoding: LenEncoding,
}

#[derive(Clone, Debug, Default)]
pub struct TransactionInputEncoding {
    pub len_encoding: LenEncoding,
    pub transaction_id_encoding: StringEncoding,
    pub index_encoding: Option<cbor_event::Sz>,
}

#[derive(Clone, Debug, Default)]
pub struct TransactionWitnessSetEncoding {
    pub len_encoding: LenEncoding,
    pub orig_deser_order: Vec<usize>,
    pub vkeywitnesses_key_encoding: Option<cbor_event::Sz>,
    pub native_scripts_key_encoding: Option<cbor_event::Sz>,
    pub bootstrap_witnesses_key_encoding: Option<cbor_event::Sz>,
    pub plutus_v1_scripts_key_encoding: Option<cbor_event::Sz>,
    pub plutus_datums_key_encoding: Option<cbor_event::Sz>,
    pub redeemers_key_encoding: Option<cbor_event::Sz>,
    pub plutus_v2_scripts_key_encoding: Option<cbor_event::Sz>,
    pub plutus_v3_scripts_key_encoding: Option<cbor_event::Sz>,
}
