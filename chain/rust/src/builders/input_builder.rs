use crate::builders::witness_builder::{InputAggregateWitnessData, PartialPlutusWitness};

use super::witness_builder::{NativeScriptWitnessInfo, RequiredWitnessSet};

use crate::{
    address::Address,
    certs::StakeCredential,
    crypto::hash::hash_plutus_data,
    plutus::PlutusData,
    transaction::{RequiredSigners, TransactionInput, TransactionOutput},
    NativeScript,
};

#[derive(Debug, thiserror::Error)]
pub enum InputBuilderError {
    #[error("UTXO address was not a payment key: {0:?}")]
    UTXOAddressNotPayment(Box<Address>),
    #[error("Missing the following witnesses for the input: {0:?}")]
    MissingWitnesses(Box<RequiredWitnessSet>),
}

pub fn input_required_wits(
    utxo_info: &TransactionOutput,
    required_witnesses: &mut RequiredWitnessSet,
) {
    if let Some(cred) = utxo_info.address().payment_cred() {
        match cred {
            StakeCredential::PubKey { hash, .. } => {
                required_witnesses.add_vkey_key_hash(*hash);
            }
            StakeCredential::Script { hash, .. } => {
                required_witnesses.add_script_hash(*hash);
                if let Some(data_hash) = utxo_info.datum_hash() {
                    required_witnesses.add_plutus_datum_hash(*data_hash);
                    // note: redeemer is required as well
                    // but we can't know the index, so we rely on the tx builder to satisfy this requirement
                }
            }
        }
    };
    if let Address::Byron(byron) = utxo_info.address() {
        required_witnesses.add_bootstrap(byron.clone());
    }
}

#[derive(Clone, Debug)]
pub struct InputBuilderResult {
    pub input: TransactionInput,
    pub utxo_info: TransactionOutput,
    pub aggregate_witness: Option<InputAggregateWitnessData>,
    pub required_wits: RequiredWitnessSet,
}

#[derive(Clone)]
pub struct SingleInputBuilder {
    input: TransactionInput,
    utxo_info: TransactionOutput,
}

impl SingleInputBuilder {
    pub fn new(input: TransactionInput, utxo_info: TransactionOutput) -> Self {
        Self { input, utxo_info }
    }

    pub fn payment_key(self) -> Result<InputBuilderResult, InputBuilderError> {
        let mut required_wits = RequiredWitnessSet::default();
        input_required_wits(&self.utxo_info, &mut required_wits);
        let required_wits_left = required_wits.clone();

        if !required_wits_left.scripts.is_empty() {
            return Err(InputBuilderError::UTXOAddressNotPayment(Box::new(
                self.utxo_info.address().clone(),
            )));
        }

        Ok(InputBuilderResult {
            input: self.input,
            utxo_info: self.utxo_info,
            aggregate_witness: None,
            required_wits,
        })
    }

    pub fn native_script(
        self,
        native_script: NativeScript,
        witness_info: NativeScriptWitnessInfo,
    ) -> Result<InputBuilderResult, InputBuilderError> {
        let mut required_wits = RequiredWitnessSet::default();
        input_required_wits(&self.utxo_info, &mut required_wits);
        let mut required_wits_left = required_wits.clone();

        let script_hash = &native_script.hash();

        // check the user provided all the required witnesses
        required_wits_left.scripts.remove(script_hash);

        if !required_wits_left.scripts.is_empty() {
            return Err(InputBuilderError::MissingWitnesses(Box::new(
                required_wits_left,
            )));
        }

        Ok(InputBuilderResult {
            input: self.input,
            utxo_info: self.utxo_info,
            aggregate_witness: Some(InputAggregateWitnessData::NativeScript(
                native_script,
                witness_info,
            )),
            required_wits,
        })
    }

    pub fn plutus_script(
        self,
        partial_witness: PartialPlutusWitness,
        required_signers: RequiredSigners,
        datum: PlutusData,
    ) -> Result<InputBuilderResult, InputBuilderError> {
        self.plutus_script_inner(partial_witness, required_signers, Some(datum))
    }

    pub fn plutus_script_inline_datum(
        self,
        partial_witness: PartialPlutusWitness,
        required_signers: RequiredSigners,
    ) -> Result<InputBuilderResult, InputBuilderError> {
        self.plutus_script_inner(partial_witness, required_signers, None)
    }

    fn plutus_script_inner(
        self,
        partial_witness: PartialPlutusWitness,
        required_signers: RequiredSigners,
        datum: Option<PlutusData>,
    ) -> Result<InputBuilderResult, InputBuilderError> {
        let mut required_wits = RequiredWitnessSet::default();
        required_signers
            .iter()
            .for_each(|required_signer| required_wits.add_vkey_key_hash(*required_signer));
        input_required_wits(&self.utxo_info, &mut required_wits);
        let mut required_wits_left = required_wits.clone();

        // no way to know these at this time
        required_wits_left.vkeys.clear();

        let script_hash = partial_witness.script.hash();

        // check the user provided all the required witnesses
        required_wits_left.scripts.remove(&script_hash);
        if let Some(datum) = &datum {
            required_wits_left
                .plutus_data
                .remove(&hash_plutus_data(datum));
        }

        if required_wits_left.len() > 0 {
            return Err(InputBuilderError::MissingWitnesses(Box::new(
                required_wits_left,
            )));
        }

        Ok(InputBuilderResult {
            input: self.input,
            utxo_info: self.utxo_info,
            aggregate_witness: Some(InputAggregateWitnessData::PlutusScript(
                partial_witness,
                required_signers,
                datum,
            )),
            required_wits,
        })
    }
}
