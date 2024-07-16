// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

pub mod cbor_encodings;
pub mod serialization;
pub mod utils;

use super::{NetworkId, Value};
use crate::address::Address;
use crate::assets::{Coin, Mint, PositiveCoin};
use crate::auxdata::AuxiliaryData;
use crate::crypto::{
    AuxiliaryDataHash, DatumHash, Ed25519KeyHash, ScriptDataHash, TransactionHash,
};
use crate::governance::VotingProcedures;
use crate::plutus::{PlutusData, Redeemers};
use crate::{
    NonemptySetBootstrapWitness, NonemptySetCertificate, NonemptySetNativeScript,
    NonemptySetPlutusData, NonemptySetPlutusV1Script, NonemptySetPlutusV2Script,
    NonemptySetPlutusV3Script, NonemptySetProposalProcedure, NonemptySetTransactionInput,
    NonemptySetVkeywitness, RequiredSigners, Script, SetTransactionInput, Slot, Withdrawals,
};
use cbor_encodings::{
    AlonzoFormatTxOutEncoding, ConwayFormatTxOutEncoding, ScriptAllEncoding, ScriptAnyEncoding,
    ScriptInvalidBeforeEncoding, ScriptInvalidHereafterEncoding, ScriptNOfKEncoding,
    ScriptPubkeyEncoding, TransactionBodyEncoding, TransactionEncoding, TransactionInputEncoding,
    TransactionWitnessSetEncoding,
};
use cml_core::ordered_hash_map::OrderedHashMap;
use cml_core::serialization::{LenEncoding, StringEncoding};
use std::collections::BTreeMap;

#[derive(
    Clone, Debug, derivative::Derivative, serde::Deserialize, serde::Serialize, schemars::JsonSchema,
)]
#[derivative(Hash, Eq, PartialEq)]
pub struct AlonzoFormatTxOut {
    pub address: Address,
    pub amount: Value,
    pub datum_hash: Option<DatumHash>,
    #[serde(skip)]
    #[derivative(PartialEq = "ignore", Hash = "ignore")]
    pub encodings: Option<AlonzoFormatTxOutEncoding>,
}

impl AlonzoFormatTxOut {
    pub fn new(address: Address, amount: Value) -> Self {
        Self {
            address,
            amount,
            datum_hash: None,
            encodings: None,
        }
    }
}

#[derive(
    Clone, Debug, derivative::Derivative, serde::Deserialize, serde::Serialize, schemars::JsonSchema,
)]
#[derivative(Hash, Eq, PartialEq)]
pub struct ConwayFormatTxOut {
    pub address: Address,
    pub amount: Value,
    pub datum_option: Option<DatumOption>,
    pub script_reference: Option<ScriptRef>,
    #[serde(skip)]
    #[derivative(PartialEq = "ignore", Hash = "ignore")]
    pub encodings: Option<ConwayFormatTxOutEncoding>,
}

impl ConwayFormatTxOut {
    pub fn new(address: Address, amount: Value) -> Self {
        Self {
            address,
            amount,
            datum_option: None,
            script_reference: None,
            encodings: None,
        }
    }
}

#[derive(
    Clone, Debug, derivative::Derivative, serde::Deserialize, serde::Serialize, schemars::JsonSchema,
)]
#[derivative(Eq, PartialEq, Hash)]
pub enum DatumOption {
    Hash {
        datum_hash: DatumHash,
        #[serde(skip)]
        #[derivative(PartialEq = "ignore", Hash = "ignore")]
        len_encoding: LenEncoding,
        #[serde(skip)]
        #[derivative(PartialEq = "ignore", Hash = "ignore")]
        tag_encoding: Option<cbor_event::Sz>,
        #[serde(skip)]
        #[derivative(PartialEq = "ignore", Hash = "ignore")]
        datum_hash_encoding: StringEncoding,
    },
    Datum {
        datum: PlutusData,
        #[serde(skip)]
        #[derivative(PartialEq = "ignore", Hash = "ignore")]
        len_encoding: LenEncoding,
        #[serde(skip)]
        #[derivative(PartialEq = "ignore", Hash = "ignore")]
        tag_encoding: Option<cbor_event::Sz>,
        #[serde(skip)]
        #[derivative(PartialEq = "ignore", Hash = "ignore")]
        datum_tag_encoding: Option<cbor_event::Sz>,
        #[serde(skip)]
        #[derivative(PartialEq = "ignore", Hash = "ignore")]
        datum_bytes_encoding: StringEncoding,
    },
}

impl DatumOption {
    pub fn new_hash(datum_hash: DatumHash) -> Self {
        Self::Hash {
            datum_hash,
            len_encoding: LenEncoding::default(),
            tag_encoding: None,
            datum_hash_encoding: StringEncoding::default(),
        }
    }

    pub fn new_datum(datum: PlutusData) -> Self {
        Self::Datum {
            datum,
            len_encoding: LenEncoding::default(),
            tag_encoding: None,
            datum_tag_encoding: None,
            datum_bytes_encoding: StringEncoding::default(),
        }
    }
}

#[derive(
    Clone, Debug, Hash, PartialEq, Eq, serde::Deserialize, serde::Serialize, schemars::JsonSchema,
)]
pub enum NativeScript {
    ScriptPubkey(ScriptPubkey),
    ScriptAll(ScriptAll),
    ScriptAny(ScriptAny),
    ScriptNOfK(ScriptNOfK),
    /// Timelock validity intervals are half-open intervals [a, b). This field specifies the left (included) endpoint a.
    ScriptInvalidBefore(ScriptInvalidBefore),
    /// Timelock validity intervals are half-open intervals [a, b). This field specifies the right (excluded) endpoint b.
    ScriptInvalidHereafter(ScriptInvalidHereafter),
}

impl NativeScript {
    pub fn new_script_pubkey(ed25519_key_hash: Ed25519KeyHash) -> Self {
        Self::ScriptPubkey(ScriptPubkey::new(ed25519_key_hash))
    }

    pub fn new_script_all(native_scripts: Vec<NativeScript>) -> Self {
        Self::ScriptAll(ScriptAll::new(native_scripts))
    }

    pub fn new_script_any(native_scripts: Vec<NativeScript>) -> Self {
        Self::ScriptAny(ScriptAny::new(native_scripts))
    }

    pub fn new_script_n_of_k(n: u64, native_scripts: Vec<NativeScript>) -> Self {
        Self::ScriptNOfK(ScriptNOfK::new(n, native_scripts))
    }

    /// Timelock validity intervals are half-open intervals [a, b). This field specifies the left (included) endpoint a.
    pub fn new_script_invalid_before(before: Slot) -> Self {
        Self::ScriptInvalidBefore(ScriptInvalidBefore::new(before))
    }

    /// Timelock validity intervals are half-open intervals [a, b). This field specifies the right (excluded) endpoint b.
    pub fn new_script_invalid_hereafter(after: Slot) -> Self {
        Self::ScriptInvalidHereafter(ScriptInvalidHereafter::new(after))
    }
}

#[derive(
    Clone, Debug, derivative::Derivative, serde::Deserialize, serde::Serialize, schemars::JsonSchema,
)]
#[derivative(Hash, PartialEq, Eq)]
pub struct ScriptAll {
    pub native_scripts: Vec<NativeScript>,
    #[serde(skip)]
    #[derivative(PartialEq = "ignore", Hash = "ignore")]
    pub encodings: Option<ScriptAllEncoding>,
}

impl ScriptAll {
    pub fn new(native_scripts: Vec<NativeScript>) -> Self {
        Self {
            native_scripts,
            encodings: None,
        }
    }
}

#[derive(
    Clone, Debug, derivative::Derivative, serde::Deserialize, serde::Serialize, schemars::JsonSchema,
)]
#[derivative(Hash, PartialEq, Eq)]
pub struct ScriptAny {
    pub native_scripts: Vec<NativeScript>,
    #[serde(skip)]
    #[derivative(PartialEq = "ignore", Hash = "ignore")]
    pub encodings: Option<ScriptAnyEncoding>,
}

impl ScriptAny {
    pub fn new(native_scripts: Vec<NativeScript>) -> Self {
        Self {
            native_scripts,
            encodings: None,
        }
    }
}

#[derive(
    Clone, Debug, derivative::Derivative, serde::Deserialize, serde::Serialize, schemars::JsonSchema,
)]
#[derivative(Hash, PartialEq, Eq)]
pub struct ScriptInvalidBefore {
    pub before: Slot,
    #[serde(skip)]
    #[derivative(PartialEq = "ignore", Hash = "ignore")]
    pub encodings: Option<ScriptInvalidBeforeEncoding>,
}

impl ScriptInvalidBefore {
    pub fn new(before: Slot) -> Self {
        Self {
            before,
            encodings: None,
        }
    }
}

#[derive(
    Clone, Debug, derivative::Derivative, serde::Deserialize, serde::Serialize, schemars::JsonSchema,
)]
#[derivative(Hash, PartialEq, Eq)]
pub struct ScriptInvalidHereafter {
    pub after: Slot,
    #[serde(skip)]
    #[derivative(PartialEq = "ignore", Hash = "ignore")]
    pub encodings: Option<ScriptInvalidHereafterEncoding>,
}

impl ScriptInvalidHereafter {
    pub fn new(after: Slot) -> Self {
        Self {
            after,
            encodings: None,
        }
    }
}

#[derive(
    Clone, Debug, derivative::Derivative, serde::Deserialize, serde::Serialize, schemars::JsonSchema,
)]
#[derivative(Hash, PartialEq, Eq)]
pub struct ScriptNOfK {
    pub n: u64,
    pub native_scripts: Vec<NativeScript>,
    #[serde(skip)]
    #[derivative(PartialEq = "ignore", Hash = "ignore")]
    pub encodings: Option<ScriptNOfKEncoding>,
}

impl ScriptNOfK {
    pub fn new(n: u64, native_scripts: Vec<NativeScript>) -> Self {
        Self {
            n,
            native_scripts,
            encodings: None,
        }
    }
}

#[derive(
    Clone, Debug, derivative::Derivative, serde::Deserialize, serde::Serialize, schemars::JsonSchema,
)]
#[derivative(Hash, PartialEq, Eq)]
pub struct ScriptPubkey {
    pub ed25519_key_hash: Ed25519KeyHash,
    #[serde(skip)]
    #[derivative(PartialEq = "ignore", Hash = "ignore")]
    pub encodings: Option<ScriptPubkeyEncoding>,
}

impl ScriptPubkey {
    pub fn new(ed25519_key_hash: Ed25519KeyHash) -> Self {
        Self {
            ed25519_key_hash,
            encodings: None,
        }
    }
}

pub type ScriptRef = Script;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct Transaction {
    pub body: TransactionBody,
    pub witness_set: TransactionWitnessSet,
    pub is_valid: bool,
    pub auxiliary_data: Option<AuxiliaryData>,
    #[serde(skip)]
    pub encodings: Option<TransactionEncoding>,
}

impl Transaction {
    pub fn new(
        body: TransactionBody,
        witness_set: TransactionWitnessSet,
        is_valid: bool,
        auxiliary_data: Option<AuxiliaryData>,
    ) -> Self {
        Self {
            body,
            witness_set,
            is_valid,
            auxiliary_data,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct TransactionBody {
    pub inputs: SetTransactionInput,
    pub outputs: Vec<TransactionOutput>,
    pub fee: Coin,
    pub ttl: Option<u64>,
    pub certs: Option<NonemptySetCertificate>,
    pub withdrawals: Option<Withdrawals>,
    pub auxiliary_data_hash: Option<AuxiliaryDataHash>,
    pub validity_interval_start: Option<u64>,
    pub mint: Option<Mint>,
    pub script_data_hash: Option<ScriptDataHash>,
    pub collateral_inputs: Option<NonemptySetTransactionInput>,
    pub required_signers: Option<RequiredSigners>,
    pub network_id: Option<NetworkId>,
    pub collateral_return: Option<TransactionOutput>,
    pub total_collateral: Option<Coin>,
    pub reference_inputs: Option<NonemptySetTransactionInput>,
    pub voting_procedures: Option<VotingProcedures>,
    pub proposal_procedures: Option<NonemptySetProposalProcedure>,
    pub current_treasury_value: Option<Coin>,
    pub donation: Option<PositiveCoin>,
    #[serde(skip)]
    pub encodings: Option<TransactionBodyEncoding>,
}

impl TransactionBody {
    pub fn new(inputs: SetTransactionInput, outputs: Vec<TransactionOutput>, fee: Coin) -> Self {
        Self {
            inputs,
            outputs,
            fee,
            ttl: None,
            certs: None,
            withdrawals: None,
            auxiliary_data_hash: None,
            validity_interval_start: None,
            mint: None,
            script_data_hash: None,
            collateral_inputs: None,
            required_signers: None,
            network_id: None,
            collateral_return: None,
            total_collateral: None,
            reference_inputs: None,
            voting_procedures: None,
            proposal_procedures: None,
            current_treasury_value: None,
            donation: None,
            encodings: None,
        }
    }
}

#[derive(
    Clone, Debug, derivative::Derivative, serde::Deserialize, serde::Serialize, schemars::JsonSchema,
)]
#[derivative(Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct TransactionInput {
    pub transaction_id: TransactionHash,
    pub index: u64,
    #[serde(skip)]
    #[derivative(
        PartialEq = "ignore",
        Ord = "ignore",
        PartialOrd = "ignore",
        Hash = "ignore"
    )]
    pub encodings: Option<TransactionInputEncoding>,
}

impl TransactionInput {
    pub fn new(transaction_id: TransactionHash, index: u64) -> Self {
        Self {
            transaction_id,
            index,
            encodings: None,
        }
    }
}

#[derive(
    Clone, Debug, Hash, Eq, PartialEq, serde::Deserialize, serde::Serialize, schemars::JsonSchema,
)]
pub enum TransactionOutput {
    AlonzoFormatTxOut(AlonzoFormatTxOut),
    ConwayFormatTxOut(ConwayFormatTxOut),
}

impl TransactionOutput {
    pub fn new_alonzo_format_tx_out(alonzo_format_tx_out: AlonzoFormatTxOut) -> Self {
        Self::AlonzoFormatTxOut(alonzo_format_tx_out)
    }

    pub fn new_conway_format_tx_out(conway_format_tx_out: ConwayFormatTxOut) -> Self {
        Self::ConwayFormatTxOut(conway_format_tx_out)
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct TransactionWitnessSet {
    pub vkeywitnesses: Option<NonemptySetVkeywitness>,
    pub native_scripts: Option<NonemptySetNativeScript>,
    pub bootstrap_witnesses: Option<NonemptySetBootstrapWitness>,
    pub plutus_v1_scripts: Option<NonemptySetPlutusV1Script>,
    pub plutus_datums: Option<NonemptySetPlutusData>,
    pub redeemers: Option<Redeemers>,
    pub plutus_v2_scripts: Option<NonemptySetPlutusV2Script>,
    pub plutus_v3_scripts: Option<NonemptySetPlutusV3Script>,
    #[serde(skip)]
    pub encodings: Option<TransactionWitnessSetEncoding>,
}

impl TransactionWitnessSet {
    pub fn new() -> Self {
        Self {
            vkeywitnesses: None,
            native_scripts: None,
            bootstrap_witnesses: None,
            plutus_v1_scripts: None,
            plutus_datums: None,
            redeemers: None,
            plutus_v2_scripts: None,
            plutus_v3_scripts: None,
            encodings: None,
        }
    }
}

impl Default for TransactionWitnessSet {
    fn default() -> Self {
        Self::new()
    }
}
