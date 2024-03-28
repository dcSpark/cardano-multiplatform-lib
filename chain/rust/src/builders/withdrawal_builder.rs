use crate::builders::witness_builder::{InputAggregateWitnessData, PartialPlutusWitness};
use crate::*;

use super::witness_builder::{NativeScriptWitnessInfo, RequiredWitnessSet};

use crate::{address::RewardAddress, certs::StakeCredential, RequiredSigners};

#[derive(Debug, thiserror::Error)]
pub enum WithdrawalBuilderError {
    #[error("Missing the following witnesses for the withdrawal: {0:?}. ")]
    MissingWitnesses(Box<RequiredWitnessSet>),
    //#[error("Withdrawal required a script, not a payment key: {}", .to_address().to_bech32(None))]
    #[error("Withdrawal required a script, not a payment key")]
    RequiredScript(Box<RewardAddress>),
}

// comes from witsVKeyNeeded in the Ledger spec
pub fn withdrawal_required_wits(
    address: &RewardAddress,
    required_witnesses: &mut RequiredWitnessSet,
) {
    match &address.payment {
        StakeCredential::PubKey { hash, .. } => {
            required_witnesses.add_vkey_key_hash(*hash);
        }
        StakeCredential::Script { hash, .. } => {
            required_witnesses.add_script_hash(*hash);
            // recall: no datum hash for reward withdrawals
        }
    }
}

#[derive(Clone)]
pub struct WithdrawalBuilderResult {
    pub address: RewardAddress,
    pub amount: Coin,
    pub aggregate_witness: Option<InputAggregateWitnessData>,
    pub required_wits: RequiredWitnessSet,
}

#[derive(Clone)]
pub struct SingleWithdrawalBuilder {
    address: RewardAddress,
    amount: Coin,
}

impl SingleWithdrawalBuilder {
    pub fn new(address: RewardAddress, amount: Coin) -> Self {
        Self { address, amount }
    }

    pub fn payment_key(self) -> Result<WithdrawalBuilderResult, WithdrawalBuilderError> {
        let mut required_wits = RequiredWitnessSet::default();
        withdrawal_required_wits(&self.address, &mut required_wits);

        if !required_wits.scripts.is_empty() {
            return Err(WithdrawalBuilderError::RequiredScript(Box::new(
                self.address.clone(),
            )));
        }

        Ok(WithdrawalBuilderResult {
            address: self.address,
            amount: self.amount,
            aggregate_witness: None,
            required_wits,
        })
    }

    pub fn native_script(
        self,
        native_script: &NativeScript,
        witness_info: &NativeScriptWitnessInfo,
    ) -> Result<WithdrawalBuilderResult, WithdrawalBuilderError> {
        let mut required_wits = RequiredWitnessSet::default();
        withdrawal_required_wits(&self.address, &mut required_wits);
        let mut required_wits_left = required_wits.clone();

        // check the user provided all the required witnesses
        required_wits_left.scripts.remove(&native_script.hash());

        if !required_wits_left.scripts.is_empty() {
            return Err(WithdrawalBuilderError::MissingWitnesses(Box::new(
                required_wits_left,
            )));
        }

        Ok(WithdrawalBuilderResult {
            address: self.address,
            amount: self.amount,
            aggregate_witness: Some(InputAggregateWitnessData::NativeScript(
                native_script.clone(),
                witness_info.clone(),
            )),
            required_wits,
        })
    }

    pub fn plutus_script(
        self,
        partial_witness: PartialPlutusWitness,
        required_signers: RequiredSigners,
    ) -> Result<WithdrawalBuilderResult, WithdrawalBuilderError> {
        let mut required_wits = RequiredWitnessSet::default();
        required_signers
            .iter()
            .for_each(|required_signer| required_wits.add_vkey_key_hash(*required_signer));
        withdrawal_required_wits(&self.address, &mut required_wits);
        let mut required_wits_left = required_wits.clone();

        // no way to know these at this time
        required_wits_left.vkeys.clear();

        let script_hash = partial_witness.script.hash();

        // check the user provided all the required witnesses
        required_wits_left.scripts.remove(&script_hash);

        if required_wits_left.len() > 0 {
            return Err(WithdrawalBuilderError::MissingWitnesses(Box::new(
                required_wits_left,
            )));
        }

        Ok(WithdrawalBuilderResult {
            address: self.address,
            amount: self.amount,
            aggregate_witness: Some(InputAggregateWitnessData::PlutusScript(
                partial_witness,
                required_signers,
                None,
            )),
            required_wits,
        })
    }
}
