use crate::*;
use crate::builders::witness_builder::{InputAggregateWitnessData, PartialPlutusWitness};
use crate::byron::ByronAddress;
use crate::ledger::common::hash::hash_plutus_data;

use super::witness_builder::{RequiredWitnessSet, NativeScriptWitnessInfo, PlutusScriptWitnessInfo};

pub fn input_required_wits(utxo_info: &TransactionOutput, required_witnesses: &mut RequiredWitnessSet) {
    if let Some(cred) = &utxo_info.address().payment_cred() {
        if let Some(keyhash) = &cred.to_keyhash() {
            required_witnesses.add_vkey_key_hash(keyhash);
        }
        if let Some(script_hash) = &cred.to_scripthash() {
            required_witnesses.add_script_hash(script_hash);
            if let Some(data_hash) = &utxo_info.datum().and_then(|datum| datum.as_data_hash()) {
                required_witnesses.add_plutus_datum_hash(data_hash);
                // note: redeemer is required as well
                // but we can't know the index, so we rely on the tx builder to satisfy this requirement
            }
        }
    };
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct InputBuilderResult {
    pub(crate) input: TransactionInput,
    pub(crate) utxo_info: TransactionOutput,
    pub(crate) aggregate_witness: Option<InputAggregateWitnessData>,
    pub(crate) required_wits: RequiredWitnessSet,
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct SingleInputBuilder {
    input: TransactionInput,
    utxo_info: TransactionOutput,
}

#[wasm_bindgen]
impl SingleInputBuilder {
    pub fn new(input: &TransactionInput, utxo_info: &TransactionOutput) -> Self {
        Self {
            input: input.clone(),
            utxo_info: utxo_info.clone(),
        }
    }

    pub fn skip_witness(&self) -> Result<InputBuilderResult, JsError> {
        let mut required_wits = RequiredWitnessSet::default();
        input_required_wits(&self.utxo_info, &mut required_wits);

        Ok(InputBuilderResult {
            input: self.input.clone(),
            utxo_info: self.utxo_info.clone(),
            aggregate_witness: None,
            required_wits,
        })
    }

    pub fn vkey(&self, vkey: &Vkey) -> Result<InputBuilderResult, JsError> {
        let mut required_wits = RequiredWitnessSet::default();
        input_required_wits(&self.utxo_info,&mut required_wits);
        let mut required_wits_left = required_wits.clone();

        let keyhash = vkey.public_key().hash();

        // check the user provided all the required witnesses
        required_wits_left.vkeys.remove(&keyhash);

        if required_wits_left.len() > 0 {
            return Err(JsError::from_str(&format!("Missing the following witnesses for the input: \n{:#?}", required_wits_left.to_str())));
        }

        Ok(InputBuilderResult {
            input: self.input.clone(),
            utxo_info: self.utxo_info.clone(),
            aggregate_witness: Some(InputAggregateWitnessData::Vkeys(vec![vkey.clone()])),
            required_wits,
        })
    }

    pub fn bootstrap(&self, vkey: &Vkey, address: &ByronAddress) -> Result<InputBuilderResult, JsError> {
        let mut required_wits = RequiredWitnessSet::default();
        input_required_wits(&self.utxo_info,&mut required_wits);
        let mut required_wits_left = required_wits.clone();

        let keyhash = &vkey.public_key().hash();

        // check the user provided all the required witnesses
        required_wits_left.bootstraps.remove(keyhash);

        if required_wits_left.len() > 0 {
            return Err(JsError::from_str(&format!("Missing the following witnesses for the input: \n{:#?}", required_wits_left.to_str())));
        }

        Ok(InputBuilderResult {
            input: self.input.clone(),
            utxo_info: self.utxo_info.clone(),
            aggregate_witness: Some(InputAggregateWitnessData::Bootstraps(vec![(vkey.clone(), address.clone())])),
            required_wits,
        })
    }

    pub fn native_script(&self, native_script: &NativeScript, witness_info: &NativeScriptWitnessInfo) -> Result<InputBuilderResult, JsError> {
        let mut required_wits = RequiredWitnessSet::default();
        input_required_wits(&self.utxo_info,&mut required_wits);
        let mut required_wits_left = required_wits.clone();

        let script_hash = &native_script.hash(ScriptHashNamespace::NativeScript);

        // check the user provided all the required witnesses
        required_wits_left.scripts.remove(script_hash);

        if required_wits_left.len() > 0 {
            return Err(JsError::from_str(&format!("Missing the following witnesses for the input: \n{:#?}", required_wits_left.to_str())));
        }

        Ok(InputBuilderResult {
            input: self.input.clone(),
            utxo_info: self.utxo_info.clone(),
            aggregate_witness:Some(InputAggregateWitnessData::NativeScript(native_script.clone(), witness_info.clone())),
            required_wits,
        })
    }

    pub fn plutus_script(&self, partial_witness: &PartialPlutusWitness, witness_info: &PlutusScriptWitnessInfo, datum: &PlutusData) -> Result<InputBuilderResult, JsError> {
        let mut required_wits = RequiredWitnessSet::default();
        witness_info.missing_signers.0.iter().for_each(|required_signer| required_wits.add_vkey_key_hash(required_signer));
        input_required_wits(&self.utxo_info,&mut required_wits);
        let mut required_wits_left = required_wits.clone();

        let script_hash = partial_witness.script.hash();

        // check the user provided all the required witnesses
        required_wits_left.scripts.remove(&script_hash);
        required_wits_left.plutus_data.remove(&hash_plutus_data(datum));

        if required_wits_left.len() > 0 {
            return Err(JsError::from_str(&format!("Missing the following witnesses for the input: \n{:#?}", required_wits_left.to_str())));
        }

        Ok(InputBuilderResult {
            input: self.input.clone(),
            utxo_info: self.utxo_info.clone(),
            aggregate_witness: Some(InputAggregateWitnessData::PlutusScript(partial_witness.clone(), witness_info.clone(), Some(datum.clone()))),
            required_wits,
        })
    }
}
