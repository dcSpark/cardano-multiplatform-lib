// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use crate::address::Address;
use crate::assets::{Coin, Mint, PositiveCoin, Value};
use crate::auxdata::AuxiliaryData;
use crate::governance::VotingProcedures;
use crate::plutus::PlutusData;
use crate::Script;
use crate::{
    BootstrapWitnessList, CertificateList, NativeScriptList, NetworkId, PlutusDataList,
    PlutusV1ScriptList, PlutusV2ScriptList, PlutusV3ScriptList, ProposalProcedureList,
    RedeemerList, Slot, TransactionInputList, TransactionOutputList, VkeywitnessList, Withdrawals,
};
use cml_core_wasm::{impl_wasm_cbor_json_api, impl_wasm_conversions};
use cml_crypto_wasm::{
    AuxiliaryDataHash, DatumHash, Ed25519KeyHash, ScriptDataHash, TransactionHash,
};
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

pub mod utils;

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct AlonzoFormatTxOut(cml_chain::transaction::AlonzoFormatTxOut);

impl_wasm_cbor_json_api!(AlonzoFormatTxOut);

impl_wasm_conversions!(cml_chain::transaction::AlonzoFormatTxOut, AlonzoFormatTxOut);

#[wasm_bindgen]
impl AlonzoFormatTxOut {
    pub fn address(&self) -> Address {
        self.0.address.clone().into()
    }

    pub fn amount(&self) -> Value {
        self.0.amount.clone().into()
    }

    pub fn set_datum_hash(&mut self, datum_hash: &DatumHash) {
        self.0.datum_hash = Some(datum_hash.clone().into())
    }

    pub fn datum_hash(&self) -> Option<DatumHash> {
        self.0.datum_hash.map(std::convert::Into::into)
    }

    pub fn new(address: &Address, amount: &Value) -> Self {
        Self(cml_chain::transaction::AlonzoFormatTxOut::new(
            address.clone().into(),
            amount.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ConwayFormatTxOut(cml_chain::transaction::ConwayFormatTxOut);

impl_wasm_cbor_json_api!(ConwayFormatTxOut);

impl_wasm_conversions!(cml_chain::transaction::ConwayFormatTxOut, ConwayFormatTxOut);

#[wasm_bindgen]
impl ConwayFormatTxOut {
    pub fn address(&self) -> Address {
        self.0.address.clone().into()
    }

    pub fn amount(&self) -> Value {
        self.0.amount.clone().into()
    }

    pub fn set_datum_option(&mut self, datum_option: &DatumOption) {
        self.0.datum_option = Some(datum_option.clone().into())
    }

    pub fn datum_option(&self) -> Option<DatumOption> {
        self.0.datum_option.clone().map(std::convert::Into::into)
    }

    pub fn set_script_reference(&mut self, script_reference: &ScriptRef) {
        self.0.script_reference = Some(script_reference.clone().into())
    }

    pub fn script_reference(&self) -> Option<ScriptRef> {
        self.0
            .script_reference
            .clone()
            .map(std::convert::Into::into)
    }

    pub fn new(address: &Address, amount: &Value) -> Self {
        Self(cml_chain::transaction::ConwayFormatTxOut::new(
            address.clone().into(),
            amount.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct DatumOption(cml_chain::transaction::DatumOption);

impl_wasm_cbor_json_api!(DatumOption);

impl_wasm_conversions!(cml_chain::transaction::DatumOption, DatumOption);

#[wasm_bindgen]
impl DatumOption {
    pub fn new_hash(datum_hash: &DatumHash) -> Self {
        Self(cml_chain::transaction::DatumOption::new_hash(
            datum_hash.clone().into(),
        ))
    }

    pub fn new_datum(datum: &PlutusData) -> Self {
        Self(cml_chain::transaction::DatumOption::new_datum(
            datum.clone().into(),
        ))
    }

    pub fn kind(&self) -> DatumOptionKind {
        match &self.0 {
            cml_chain::transaction::DatumOption::Hash { .. } => DatumOptionKind::Hash,
            cml_chain::transaction::DatumOption::Datum { .. } => DatumOptionKind::Datum,
        }
    }

    pub fn as_hash(&self) -> Option<DatumHash> {
        match &self.0 {
            cml_chain::transaction::DatumOption::Hash { datum_hash, .. } => {
                Some((*datum_hash).into())
            }
            _ => None,
        }
    }

    pub fn as_datum(&self) -> Option<PlutusData> {
        match &self.0 {
            cml_chain::transaction::DatumOption::Datum { datum, .. } => Some(datum.clone().into()),
            _ => None,
        }
    }
}

#[wasm_bindgen]
pub enum DatumOptionKind {
    Hash,
    Datum,
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct NativeScript(cml_chain::transaction::NativeScript);

impl_wasm_cbor_json_api!(NativeScript);

impl_wasm_conversions!(cml_chain::transaction::NativeScript, NativeScript);

#[wasm_bindgen]
impl NativeScript {
    pub fn new_script_pubkey(ed25519_key_hash: &Ed25519KeyHash) -> Self {
        Self(cml_chain::transaction::NativeScript::new_script_pubkey(
            ed25519_key_hash.clone().into(),
        ))
    }

    pub fn new_script_all(native_scripts: &NativeScriptList) -> Self {
        Self(cml_chain::transaction::NativeScript::new_script_all(
            native_scripts.clone().into(),
        ))
    }

    pub fn new_script_any(native_scripts: &NativeScriptList) -> Self {
        Self(cml_chain::transaction::NativeScript::new_script_any(
            native_scripts.clone().into(),
        ))
    }

    pub fn new_script_n_of_k(n: u64, native_scripts: &NativeScriptList) -> Self {
        Self(cml_chain::transaction::NativeScript::new_script_n_of_k(
            n,
            native_scripts.clone().into(),
        ))
    }

    pub fn new_script_invalid_before(before: Slot) -> Self {
        Self(cml_chain::transaction::NativeScript::new_script_invalid_before(before))
    }

    pub fn new_script_invalid_hereafter(after: Slot) -> Self {
        Self(cml_chain::transaction::NativeScript::new_script_invalid_hereafter(after))
    }

    pub fn kind(&self) -> NativeScriptKind {
        match &self.0 {
            cml_chain::transaction::NativeScript::ScriptPubkey(_) => NativeScriptKind::ScriptPubkey,
            cml_chain::transaction::NativeScript::ScriptAll(_) => NativeScriptKind::ScriptAll,
            cml_chain::transaction::NativeScript::ScriptAny(_) => NativeScriptKind::ScriptAny,
            cml_chain::transaction::NativeScript::ScriptNOfK(_) => NativeScriptKind::ScriptNOfK,
            cml_chain::transaction::NativeScript::ScriptInvalidBefore(_) => {
                NativeScriptKind::ScriptInvalidBefore
            }
            cml_chain::transaction::NativeScript::ScriptInvalidHereafter(_) => {
                NativeScriptKind::ScriptInvalidHereafter
            }
        }
    }

    pub fn as_script_pubkey(&self) -> Option<ScriptPubkey> {
        match &self.0 {
            cml_chain::transaction::NativeScript::ScriptPubkey(script_pubkey) => {
                Some(script_pubkey.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_script_all(&self) -> Option<ScriptAll> {
        match &self.0 {
            cml_chain::transaction::NativeScript::ScriptAll(script_all) => {
                Some(script_all.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_script_any(&self) -> Option<ScriptAny> {
        match &self.0 {
            cml_chain::transaction::NativeScript::ScriptAny(script_any) => {
                Some(script_any.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_script_n_of_k(&self) -> Option<ScriptNOfK> {
        match &self.0 {
            cml_chain::transaction::NativeScript::ScriptNOfK(script_n_of_k) => {
                Some(script_n_of_k.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_script_invalid_before(&self) -> Option<ScriptInvalidBefore> {
        match &self.0 {
            cml_chain::transaction::NativeScript::ScriptInvalidBefore(script_invalid_before) => {
                Some(script_invalid_before.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_script_invalid_hereafter(&self) -> Option<ScriptInvalidHereafter> {
        match &self.0 {
            cml_chain::transaction::NativeScript::ScriptInvalidHereafter(
                script_invalid_hereafter,
            ) => Some(script_invalid_hereafter.clone().into()),
            _ => None,
        }
    }
}

#[wasm_bindgen]
pub enum NativeScriptKind {
    ScriptPubkey,
    ScriptAll,
    ScriptAny,
    ScriptNOfK,
    ScriptInvalidBefore,
    ScriptInvalidHereafter,
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct RequiredSigners(Vec<cml_chain::crypto::Ed25519KeyHash>);

impl_wasm_conversions!(Vec<cml_chain::crypto::Ed25519KeyHash>, RequiredSigners);

#[wasm_bindgen]
impl RequiredSigners {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Ed25519KeyHash {
        self.0[index].into()
    }

    pub fn add(&mut self, elem: &Ed25519KeyHash) {
        self.0.push(elem.clone().into());
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ScriptAll(cml_chain::transaction::ScriptAll);

impl_wasm_cbor_json_api!(ScriptAll);

impl_wasm_conversions!(cml_chain::transaction::ScriptAll, ScriptAll);

#[wasm_bindgen]
impl ScriptAll {
    pub fn native_scripts(&self) -> NativeScriptList {
        self.0.native_scripts.clone().into()
    }

    pub fn new(native_scripts: &NativeScriptList) -> Self {
        Self(cml_chain::transaction::ScriptAll::new(
            native_scripts.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ScriptAny(cml_chain::transaction::ScriptAny);

impl_wasm_cbor_json_api!(ScriptAny);

impl_wasm_conversions!(cml_chain::transaction::ScriptAny, ScriptAny);

#[wasm_bindgen]
impl ScriptAny {
    pub fn native_scripts(&self) -> NativeScriptList {
        self.0.native_scripts.clone().into()
    }

    pub fn new(native_scripts: &NativeScriptList) -> Self {
        Self(cml_chain::transaction::ScriptAny::new(
            native_scripts.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ScriptInvalidBefore(cml_chain::transaction::ScriptInvalidBefore);

impl_wasm_cbor_json_api!(ScriptInvalidBefore);

impl_wasm_conversions!(
    cml_chain::transaction::ScriptInvalidBefore,
    ScriptInvalidBefore
);

#[wasm_bindgen]
impl ScriptInvalidBefore {
    pub fn before(&self) -> Slot {
        self.0.before
    }

    pub fn new(before: Slot) -> Self {
        Self(cml_chain::transaction::ScriptInvalidBefore::new(before))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ScriptInvalidHereafter(cml_chain::transaction::ScriptInvalidHereafter);

impl_wasm_cbor_json_api!(ScriptInvalidHereafter);

impl_wasm_conversions!(
    cml_chain::transaction::ScriptInvalidHereafter,
    ScriptInvalidHereafter
);

#[wasm_bindgen]
impl ScriptInvalidHereafter {
    pub fn after(&self) -> Slot {
        self.0.after
    }

    pub fn new(after: Slot) -> Self {
        Self(cml_chain::transaction::ScriptInvalidHereafter::new(after))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ScriptNOfK(cml_chain::transaction::ScriptNOfK);

impl_wasm_cbor_json_api!(ScriptNOfK);

impl_wasm_conversions!(cml_chain::transaction::ScriptNOfK, ScriptNOfK);

#[wasm_bindgen]
impl ScriptNOfK {
    pub fn n(&self) -> u64 {
        self.0.n
    }

    pub fn native_scripts(&self) -> NativeScriptList {
        self.0.native_scripts.clone().into()
    }

    pub fn new(n: u64, native_scripts: &NativeScriptList) -> Self {
        Self(cml_chain::transaction::ScriptNOfK::new(
            n,
            native_scripts.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ScriptPubkey(cml_chain::transaction::ScriptPubkey);

impl_wasm_cbor_json_api!(ScriptPubkey);

impl_wasm_conversions!(cml_chain::transaction::ScriptPubkey, ScriptPubkey);

#[wasm_bindgen]
impl ScriptPubkey {
    pub fn ed25519_key_hash(&self) -> Ed25519KeyHash {
        self.0.ed25519_key_hash.into()
    }

    pub fn new(ed25519_key_hash: &Ed25519KeyHash) -> Self {
        Self(cml_chain::transaction::ScriptPubkey::new(
            ed25519_key_hash.clone().into(),
        ))
    }
}

pub type ScriptRef = Script;

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct Transaction(cml_chain::transaction::Transaction);

impl_wasm_cbor_json_api!(Transaction);

impl_wasm_conversions!(cml_chain::transaction::Transaction, Transaction);

#[wasm_bindgen]
impl Transaction {
    pub fn body(&self) -> TransactionBody {
        self.0.body.clone().into()
    }

    pub fn witness_set(&self) -> TransactionWitnessSet {
        self.0.witness_set.clone().into()
    }

    pub fn is_valid(&self) -> bool {
        self.0.is_valid
    }

    pub fn auxiliary_data(&self) -> Option<AuxiliaryData> {
        self.0.auxiliary_data.clone().map(std::convert::Into::into)
    }

    pub fn new(
        body: &TransactionBody,
        witness_set: &TransactionWitnessSet,
        is_valid: bool,
        auxiliary_data: Option<AuxiliaryData>,
    ) -> Self {
        Self(cml_chain::transaction::Transaction::new(
            body.clone().into(),
            witness_set.clone().into(),
            is_valid,
            auxiliary_data.map(Into::into),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct TransactionBody(cml_chain::transaction::TransactionBody);

impl_wasm_cbor_json_api!(TransactionBody);

impl_wasm_conversions!(cml_chain::transaction::TransactionBody, TransactionBody);

#[wasm_bindgen]
impl TransactionBody {
    pub fn inputs(&self) -> TransactionInputList {
        self.0.inputs.clone().into()
    }

    pub fn outputs(&self) -> TransactionOutputList {
        self.0.outputs.clone().into()
    }

    pub fn fee(&self) -> Coin {
        self.0.fee
    }

    pub fn set_ttl(&mut self, ttl: u64) {
        self.0.ttl = Some(ttl)
    }

    pub fn ttl(&self) -> Option<u64> {
        self.0.ttl
    }

    pub fn set_certs(&mut self, certs: &CertificateList) {
        self.0.certs = Some(certs.clone().into())
    }

    pub fn certs(&self) -> Option<CertificateList> {
        self.0.certs.clone().map(std::convert::Into::into)
    }

    pub fn set_withdrawals(&mut self, withdrawals: &Withdrawals) {
        self.0.withdrawals = Some(withdrawals.clone().into())
    }

    pub fn withdrawals(&self) -> Option<Withdrawals> {
        self.0.withdrawals.clone().map(std::convert::Into::into)
    }

    pub fn set_auxiliary_data_hash(&mut self, auxiliary_data_hash: &AuxiliaryDataHash) {
        self.0.auxiliary_data_hash = Some(auxiliary_data_hash.clone().into())
    }

    pub fn auxiliary_data_hash(&self) -> Option<AuxiliaryDataHash> {
        self.0
            .auxiliary_data_hash
            .map(std::convert::Into::into)
    }

    pub fn set_validity_interval_start(&mut self, validity_interval_start: u64) {
        self.0.validity_interval_start = Some(validity_interval_start)
    }

    pub fn validity_interval_start(&self) -> Option<u64> {
        self.0.validity_interval_start
    }

    pub fn set_mint(&mut self, mint: &Mint) {
        self.0.mint = Some(mint.clone().into())
    }

    pub fn mint(&self) -> Option<Mint> {
        self.0.mint.clone().map(std::convert::Into::into)
    }

    pub fn set_script_data_hash(&mut self, script_data_hash: &ScriptDataHash) {
        self.0.script_data_hash = Some(script_data_hash.clone().into())
    }

    pub fn script_data_hash(&self) -> Option<ScriptDataHash> {
        self.0
            .script_data_hash
            .map(std::convert::Into::into)
    }

    pub fn set_collateral_inputs(&mut self, collateral_inputs: &TransactionInputList) {
        self.0.collateral_inputs = Some(collateral_inputs.clone().into())
    }

    pub fn collateral_inputs(&self) -> Option<TransactionInputList> {
        self.0
            .collateral_inputs
            .clone()
            .map(std::convert::Into::into)
    }

    pub fn set_required_signers(&mut self, required_signers: &RequiredSigners) {
        self.0.required_signers = Some(required_signers.clone().into())
    }

    pub fn required_signers(&self) -> Option<RequiredSigners> {
        self.0
            .required_signers
            .clone()
            .map(std::convert::Into::into)
    }

    pub fn set_network_id(&mut self, network_id: &NetworkId) {
        self.0.network_id = Some(network_id.clone().into())
    }

    pub fn network_id(&self) -> Option<NetworkId> {
        self.0.network_id.map(std::convert::Into::into)
    }

    pub fn set_collateral_return(&mut self, collateral_return: &TransactionOutput) {
        self.0.collateral_return = Some(collateral_return.clone().into())
    }

    pub fn collateral_return(&self) -> Option<TransactionOutput> {
        self.0
            .collateral_return
            .clone()
            .map(std::convert::Into::into)
    }

    pub fn set_total_collateral(&mut self, total_collateral: Coin) {
        self.0.total_collateral = Some(total_collateral)
    }

    pub fn total_collateral(&self) -> Option<Coin> {
        self.0.total_collateral
    }

    pub fn set_reference_inputs(&mut self, reference_inputs: &TransactionInputList) {
        self.0.reference_inputs = Some(reference_inputs.clone().into())
    }

    pub fn reference_inputs(&self) -> Option<TransactionInputList> {
        self.0
            .reference_inputs
            .clone()
            .map(std::convert::Into::into)
    }

    pub fn set_voting_procedures(&mut self, voting_procedures: &VotingProcedures) {
        self.0.voting_procedures = Some(voting_procedures.clone().into())
    }

    pub fn voting_procedures(&self) -> Option<VotingProcedures> {
        self.0
            .voting_procedures
            .clone()
            .map(std::convert::Into::into)
    }

    pub fn set_proposal_procedures(&mut self, proposal_procedures: &ProposalProcedureList) {
        self.0.proposal_procedures = Some(proposal_procedures.clone().into())
    }

    pub fn proposal_procedures(&self) -> Option<ProposalProcedureList> {
        self.0
            .proposal_procedures
            .clone()
            .map(std::convert::Into::into)
    }

    pub fn set_current_treasury_value(&mut self, current_treasury_value: Coin) {
        self.0.current_treasury_value = Some(current_treasury_value)
    }

    pub fn current_treasury_value(&self) -> Option<Coin> {
        self.0.current_treasury_value
    }

    pub fn set_donation(&mut self, donation: PositiveCoin) {
        self.0.donation = Some(donation)
    }

    pub fn donation(&self) -> Option<PositiveCoin> {
        self.0.donation
    }

    pub fn new(inputs: &TransactionInputList, outputs: &TransactionOutputList, fee: Coin) -> Self {
        Self(cml_chain::transaction::TransactionBody::new(
            inputs.clone().into(),
            outputs.clone().into(),
            fee,
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct TransactionInput(cml_chain::transaction::TransactionInput);

impl_wasm_cbor_json_api!(TransactionInput);

impl_wasm_conversions!(cml_chain::transaction::TransactionInput, TransactionInput);

#[wasm_bindgen]
impl TransactionInput {
    pub fn transaction_id(&self) -> TransactionHash {
        self.0.transaction_id.into()
    }

    pub fn index(&self) -> u64 {
        self.0.index
    }

    pub fn new(transaction_id: &TransactionHash, index: u64) -> Self {
        Self(cml_chain::transaction::TransactionInput::new(
            transaction_id.clone().into(),
            index,
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct TransactionOutput(cml_chain::transaction::TransactionOutput);

impl_wasm_cbor_json_api!(TransactionOutput);

impl_wasm_conversions!(cml_chain::transaction::TransactionOutput, TransactionOutput);

#[wasm_bindgen]
impl TransactionOutput {
    pub fn new_alonzo_format_tx_out(alonzo_format_tx_out: &AlonzoFormatTxOut) -> Self {
        Self(
            cml_chain::transaction::TransactionOutput::new_alonzo_format_tx_out(
                alonzo_format_tx_out.clone().into(),
            ),
        )
    }

    pub fn new_conway_format_tx_out(conway_format_tx_out: &ConwayFormatTxOut) -> Self {
        Self(
            cml_chain::transaction::TransactionOutput::new_conway_format_tx_out(
                conway_format_tx_out.clone().into(),
            ),
        )
    }

    pub fn kind(&self) -> TransactionOutputKind {
        match &self.0 {
            cml_chain::transaction::TransactionOutput::AlonzoFormatTxOut(_) => {
                TransactionOutputKind::AlonzoFormatTxOut
            }
            cml_chain::transaction::TransactionOutput::ConwayFormatTxOut(_) => {
                TransactionOutputKind::ConwayFormatTxOut
            }
        }
    }

    pub fn as_alonzo_format_tx_out(&self) -> Option<AlonzoFormatTxOut> {
        match &self.0 {
            cml_chain::transaction::TransactionOutput::AlonzoFormatTxOut(alonzo_format_tx_out) => {
                Some(alonzo_format_tx_out.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_conway_format_tx_out(&self) -> Option<ConwayFormatTxOut> {
        match &self.0 {
            cml_chain::transaction::TransactionOutput::ConwayFormatTxOut(conway_format_tx_out) => {
                Some(conway_format_tx_out.clone().into())
            }
            _ => None,
        }
    }
}

#[wasm_bindgen]
pub enum TransactionOutputKind {
    AlonzoFormatTxOut,
    ConwayFormatTxOut,
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct TransactionWitnessSet(cml_chain::transaction::TransactionWitnessSet);

impl_wasm_cbor_json_api!(TransactionWitnessSet);

impl_wasm_conversions!(
    cml_chain::transaction::TransactionWitnessSet,
    TransactionWitnessSet
);

#[wasm_bindgen]
impl TransactionWitnessSet {
    pub fn set_vkeywitnesses(&mut self, vkeywitnesses: &VkeywitnessList) {
        self.0.vkeywitnesses = Some(vkeywitnesses.clone().into())
    }

    pub fn vkeywitnesses(&self) -> Option<VkeywitnessList> {
        self.0.vkeywitnesses.clone().map(std::convert::Into::into)
    }

    pub fn set_native_scripts(&mut self, native_scripts: &NativeScriptList) {
        self.0.native_scripts = Some(native_scripts.clone().into())
    }

    pub fn native_scripts(&self) -> Option<NativeScriptList> {
        self.0.native_scripts.clone().map(std::convert::Into::into)
    }

    pub fn set_bootstrap_witnesses(&mut self, bootstrap_witnesses: &BootstrapWitnessList) {
        self.0.bootstrap_witnesses = Some(bootstrap_witnesses.clone().into())
    }

    pub fn bootstrap_witnesses(&self) -> Option<BootstrapWitnessList> {
        self.0
            .bootstrap_witnesses
            .clone()
            .map(std::convert::Into::into)
    }

    pub fn set_plutus_v1_scripts(&mut self, plutus_v1_scripts: &PlutusV1ScriptList) {
        self.0.plutus_v1_scripts = Some(plutus_v1_scripts.clone().into())
    }

    pub fn plutus_v1_scripts(&self) -> Option<PlutusV1ScriptList> {
        self.0
            .plutus_v1_scripts
            .clone()
            .map(std::convert::Into::into)
    }

    pub fn set_plutus_datums(&mut self, plutus_datums: &PlutusDataList) {
        self.0.plutus_datums = Some(plutus_datums.clone().into())
    }

    pub fn plutus_datums(&self) -> Option<PlutusDataList> {
        self.0.plutus_datums.clone().map(std::convert::Into::into)
    }

    pub fn set_redeemers(&mut self, redeemers: &RedeemerList) {
        self.0.redeemers = Some(redeemers.clone().into())
    }

    pub fn redeemers(&self) -> Option<RedeemerList> {
        self.0.redeemers.clone().map(std::convert::Into::into)
    }

    pub fn set_plutus_v2_scripts(&mut self, plutus_v2_scripts: &PlutusV2ScriptList) {
        self.0.plutus_v2_scripts = Some(plutus_v2_scripts.clone().into())
    }

    pub fn plutus_v2_scripts(&self) -> Option<PlutusV2ScriptList> {
        self.0
            .plutus_v2_scripts
            .clone()
            .map(std::convert::Into::into)
    }

    pub fn set_plutus_v3_scripts(&mut self, plutus_v3_scripts: &PlutusV3ScriptList) {
        self.0.plutus_v3_scripts = Some(plutus_v3_scripts.clone().into())
    }

    pub fn plutus_v3_scripts(&self) -> Option<PlutusV3ScriptList> {
        self.0
            .plutus_v3_scripts
            .clone()
            .map(std::convert::Into::into)
    }

    pub fn new() -> Self {
        Self(cml_chain::transaction::TransactionWitnessSet::new())
    }
}
