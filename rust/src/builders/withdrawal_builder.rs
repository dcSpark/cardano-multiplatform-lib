use crate::*;
use crate::builders::witness_builder::{InputAggregateWitnessData, PartialPlutusWitness};
use crate::ledger::common::hash::ScriptHashNamespace;

use super::witness_builder::{RequiredWitnessSet, NativeScriptWitnessInfo, PlutusScriptWitnessInfo};

// comes from witsVKeyNeeded in the Ledger spec
pub fn withdrawal_required_wits(address: &RewardAddress, required_witnesses: &mut RequiredWitnessSet) {
    let cred = &address.payment_cred();
    if let Some(keyhash) = &cred.to_keyhash() {
        required_witnesses.add_vkey_key_hash(keyhash);
    }
    if let Some(script_hash) = &cred.to_scripthash() {
        required_witnesses.add_script_hash(script_hash);
        // recall: no datum hash for reward withdrawals
    };
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct WithdrawalBuilderResult {
    pub(crate) address: RewardAddress,
    pub(crate) amount: Coin,
    pub(crate) aggregate_witness: Option<InputAggregateWitnessData>,
    pub(crate) required_wits: RequiredWitnessSet,
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct SingleWithdrawalBuilder {
    address: RewardAddress,
    amount: Coin,
}

#[wasm_bindgen]
impl SingleWithdrawalBuilder {
    pub fn new(address: &RewardAddress, amount: &Coin) -> Self {
        Self {
            address: address.clone(),
            amount: *amount,
        }
    }

    pub fn payment_key(&self) -> Result<WithdrawalBuilderResult, JsError> {
        let mut required_wits = RequiredWitnessSet::default();
        withdrawal_required_wits(&self.address, &mut required_wits);
        let mut required_wits_left = required_wits.clone();

        if required_wits_left.scripts.len() > 0 {
            return Err(JsError::from_str(&format!("Withdrawal required a script, not a payment key: \n{:#?}", self.address.to_address().to_bech32(None))));
        }

        Ok(WithdrawalBuilderResult {
            address: self.address.clone(),
            amount: self.amount,
            aggregate_witness: None,
            required_wits,
        })
    }

    pub fn native_script(&self, native_script: &NativeScript, witness_info: &NativeScriptWitnessInfo) -> Result<WithdrawalBuilderResult, JsError> {
        let mut required_wits = RequiredWitnessSet::default();
        withdrawal_required_wits(&self.address, &mut required_wits);
        let mut required_wits_left = required_wits.clone();

        // check the user provided all the required witnesses
        required_wits_left.scripts.remove(&native_script.hash(ScriptHashNamespace::NativeScript));

        if required_wits_left.len() > 0 {
            return Err(JsError::from_str(&format!("Missing the following witnesses for the withdrawal: \n{:#?}", required_wits_left.to_str())));
        }

        Ok(WithdrawalBuilderResult {
            address: self.address.clone(),
            amount: self.amount,
            aggregate_witness: Some(InputAggregateWitnessData::NativeScript(native_script.clone(), witness_info.clone())),
            required_wits,
        })
    }

    pub fn plutus_script(&self, partial_witness: &PartialPlutusWitness, witness_info: &PlutusScriptWitnessInfo) -> Result<WithdrawalBuilderResult, JsError> {
        let mut required_wits = RequiredWitnessSet::default();
        witness_info.missing_signers.0.iter().for_each(|required_signer| required_wits.add_vkey_key_hash(&required_signer));
        withdrawal_required_wits(&self.address, &mut required_wits);
        let mut required_wits_left = required_wits.clone();

        let script_hash = partial_witness.script.hash();

        // check the user provided all the required witnesses
        required_wits_left.scripts.remove(&script_hash);

        if required_wits_left.len() > 0 {
            return Err(JsError::from_str(&format!("Missing the following witnesses for the withdrawal: \n{:#?}", required_wits_left.to_str())));
        }

        Ok(WithdrawalBuilderResult {
            address: self.address.clone(),
            amount: self.amount,
            aggregate_witness:  Some(InputAggregateWitnessData::PlutusScript(partial_witness.clone(), witness_info.clone(), None)),
            required_wits,
        })
    }
}
