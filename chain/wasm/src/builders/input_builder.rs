use crate::builders::{
    tx_builder::TransactionUnspentOutput,
    witness_builder::{NativeScriptWitnessInfo, PartialPlutusWitness},
};
use crate::{
    plutus::PlutusData,
    transaction::{TransactionInput, TransactionOutput},
    NativeScript, RequiredSigners,
};
use cml_core_wasm::impl_wasm_conversions;
use wasm_bindgen::prelude::{wasm_bindgen, JsError};

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct InputBuilderResult(cml_chain::builders::input_builder::InputBuilderResult);

impl_wasm_conversions!(
    cml_chain::builders::input_builder::InputBuilderResult,
    InputBuilderResult
);

#[wasm_bindgen]
#[derive(Clone)]
pub struct SingleInputBuilder(cml_chain::builders::input_builder::SingleInputBuilder);

impl_wasm_conversions!(
    cml_chain::builders::input_builder::SingleInputBuilder,
    SingleInputBuilder
);

#[wasm_bindgen]
impl SingleInputBuilder {
    pub fn new(input: &TransactionInput, utxo_info: &TransactionOutput) -> Self {
        cml_chain::builders::input_builder::SingleInputBuilder::new(
            input.clone().into(),
            utxo_info.clone().into(),
        )
        .into()
    }

    pub fn from_transaction_unspent_output(utxo: &TransactionUnspentOutput) -> Self {
        cml_chain::builders::input_builder::SingleInputBuilder::from(Into::<
            cml_chain::builders::tx_builder::TransactionUnspentOutput,
        >::into(utxo.clone()))
        .into()
    }

    pub fn payment_key(&self) -> Result<InputBuilderResult, JsError> {
        self.0
            .clone()
            .payment_key()
            .map(Into::into)
            .map_err(Into::into)
    }

    pub fn native_script(
        &self,
        native_script: &NativeScript,
        witness_info: &NativeScriptWitnessInfo,
    ) -> Result<InputBuilderResult, JsError> {
        self.0
            .clone()
            .native_script(native_script.clone().into(), witness_info.clone().into())
            .map(Into::into)
            .map_err(Into::into)
    }

    pub fn plutus_script(
        &self,
        partial_witness: &PartialPlutusWitness,
        required_signers: &RequiredSigners,
        datum: &PlutusData,
    ) -> Result<InputBuilderResult, JsError> {
        self.0
            .clone()
            .plutus_script(
                partial_witness.clone().into(),
                required_signers.clone().into(),
                datum.clone().into(),
            )
            .map(Into::into)
            .map_err(Into::into)
    }

    pub fn plutus_script_inline_datum(
        &self,
        partial_witness: &PartialPlutusWitness,
        required_signers: &RequiredSigners,
    ) -> Result<InputBuilderResult, JsError> {
        self.0
            .clone()
            .plutus_script_inline_datum(
                partial_witness.clone().into(),
                required_signers.clone().into(),
            )
            .map(Into::into)
            .map_err(Into::into)
    }
}
