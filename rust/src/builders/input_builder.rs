use crate::*;
use crate::builders::witness_builder::{InputAggregateWitnessData, PartialPlutusWitness};

use super::witness_builder::RequiredWitnessSet;

// comes from witsVKeyNeeded in the Ledger spec
pub fn input_required_wits(utxo_info: &TransactionOutput, required_witnesses: &mut RequiredWitnessSet) -> () {
    // TODO: script hash, plutus script, plutus data
    if let Some(cred) = &utxo_info.address().payment_cred() {
        if let Some(keyhash) = &cred.to_keyhash() {
            required_witnesses.add_vkey_key_hash(&keyhash);
        }
        if let Some(script_hash) = &cred.to_scripthash() {
            required_witnesses.add_script_hash(&script_hash);
            if let Some(data_hash) = utxo_info.data_hash() {
                required_witnesses.add_plutus_datum_hash(&data_hash);
                // note: redeemer is required as well
                // but we can't know the index, so we rely on the tx builder to satisfy this requirement
            }
        }
    };
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct InputBuilderResult {
    input: TransactionInput,
    utxo_info: TransactionOutput,
    aggregate_witness: Option<InputAggregateWitnessData>,
    required_wits: RequiredWitnessSet,
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct SingleInputBuilder {
    input: TransactionInput,
    utxo_info: TransactionOutput,
}

#[wasm_bindgen]
impl SingleInputBuilder {
    pub fn new(input: &TransactionInput, utxo_info: &TransactionOutput,) -> Self {
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
            required_wits: required_wits.clone(),
        })
    }

    pub fn vkey(&self, vkey: &Vkey) -> Result<InputBuilderResult, JsError> {
        let mut required_wits = RequiredWitnessSet::default();
        input_required_wits(&self.utxo_info,&mut required_wits);
        let mut required_wits_left = required_wits.clone();

        let keyhash = &vkey.public_key().hash();

        // the user may have provided more witnesses than required. Strip it down to just the required wits
        let contains = required_wits_left.vkeys.contains(&keyhash);
        
        // check the user provided all the required witnesses
        required_wits_left.vkeys.remove(&keyhash);

        if required_wits_left.len() > 0 {
            return Err(JsError::from_str(&format!("Missing the following witnesses for the certificate: \n{:#?}", required_wits_left.to_str()))); 
        }

        Ok(InputBuilderResult {
            input: self.input.clone(),
            utxo_info: self.utxo_info.clone(),
            aggregate_witness: if contains { Some(InputAggregateWitnessData::Vkeys(vec![vkey.clone()])) } else { None },
            required_wits: required_wits.clone(),
        })
    }

    pub fn bootstrap(&self, bootstrap: &BootstrapWitness) -> Result<InputBuilderResult, JsError> {
        let mut required_wits = RequiredWitnessSet::default();
        input_required_wits(&self.utxo_info,&mut required_wits);
        let mut required_wits_left = required_wits.clone();

        let keyhash = &bootstrap.vkey().public_key().hash();

        // the user may have provided more witnesses than required. Strip it down to just the required wits
        let contains = required_wits_left.bootstraps.contains(&keyhash);
        
        // check the user provided all the required witnesses
        required_wits_left.bootstraps.remove(&keyhash);

        if required_wits_left.len() > 0 {
            return Err(JsError::from_str(&format!("Missing the following witnesses for the certificate: \n{:#?}", required_wits_left.to_str()))); 
        }

        Ok(InputBuilderResult {
            input: self.input.clone(),
            utxo_info: self.utxo_info.clone(),
            aggregate_witness: if contains { Some(InputAggregateWitnessData::Bootstraps(vec![bootstrap.clone()])) } else { None },
            required_wits: required_wits.clone(),
        })
    }

    pub fn native_script(&self, native_script: &NativeScript) -> Result<InputBuilderResult, JsError> {
        let mut required_wits = RequiredWitnessSet::default();
        input_required_wits(&self.utxo_info,&mut required_wits);
        let mut required_wits_left = required_wits.clone();

        let script_hash = &native_script.hash(ScriptHashNamespace::NativeScript);

        // the user may have provided more witnesses than required. Strip it down to just the required wits
        let contains = required_wits_left.scripts.contains(script_hash);
        
        // check the user provided all the required witnesses
        required_wits_left.scripts.remove(script_hash);

        if required_wits_left.len() > 0 {
            return Err(JsError::from_str(&format!("Missing the following witnesses for the certificate: \n{:#?}", required_wits_left.to_str()))); 
        }

        Ok(InputBuilderResult {
            input: self.input.clone(),
            utxo_info: self.utxo_info.clone(),
            aggregate_witness: if contains { Some(InputAggregateWitnessData::NativeScript(native_script.clone())) } else { None },
            required_wits: required_wits.clone(),
        })
    }

    pub fn plutus_script(&self, partial_witness: &PartialPlutusWitness, datum: &PlutusData) -> Result<InputBuilderResult, JsError> {
        let mut required_wits = RequiredWitnessSet::default();
        input_required_wits(&self.utxo_info,&mut required_wits);
        let mut required_wits_left = required_wits.clone();

        // TODO: Plutus V2
        let script_hash = &partial_witness.script().hash(ScriptHashNamespace::PlutusV1);

        // the user may have provided more witnesses than required. Strip it down to just the required wits
        let contains = required_wits_left.scripts.contains(script_hash);
        
        // check the user provided all the required witnesses
        required_wits_left.scripts.remove(script_hash);
        required_wits_left.plutus_data.remove(&hash_plutus_data(datum));

        if required_wits_left.len() > 0 {
            return Err(JsError::from_str(&format!("Missing the following witnesses for the certificate: \n{:#?}", required_wits_left.to_str()))); 
        }

        Ok(InputBuilderResult {
            input: self.input.clone(),
            utxo_info: self.utxo_info.clone(),
            aggregate_witness: if contains { Some(InputAggregateWitnessData::PlutusScriptWithDatum(partial_witness.clone(), datum.clone())) } else { None },
            required_wits: required_wits.clone(),
        })
    }
}
