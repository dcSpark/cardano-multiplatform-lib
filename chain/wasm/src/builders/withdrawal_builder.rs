use crate::builders::witness_builder::PartialPlutusWitness;
use crate::*;
use cml_core_wasm::impl_wasm_conversions;
use wasm_bindgen::prelude::{wasm_bindgen, JsError};

use super::witness_builder::NativeScriptWitnessInfo;

use crate::{address::RewardAddress, RequiredSigners};

#[wasm_bindgen]
#[derive(Clone)]
pub struct WithdrawalBuilderResult(
    cml_chain::builders::withdrawal_builder::WithdrawalBuilderResult,
);

impl_wasm_conversions!(
    cml_chain::builders::withdrawal_builder::WithdrawalBuilderResult,
    WithdrawalBuilderResult
);

#[wasm_bindgen]
#[derive(Clone)]
pub struct SingleWithdrawalBuilder(
    cml_chain::builders::withdrawal_builder::SingleWithdrawalBuilder,
);

impl_wasm_conversions!(
    cml_chain::builders::withdrawal_builder::SingleWithdrawalBuilder,
    SingleWithdrawalBuilder
);

#[wasm_bindgen]
impl SingleWithdrawalBuilder {
    pub fn new(address: &RewardAddress, amount: Coin) -> Self {
        cml_chain::builders::withdrawal_builder::SingleWithdrawalBuilder::new(
            address.clone().into(),
            amount,
        )
        .into()
    }

    pub fn payment_key(&self) -> Result<WithdrawalBuilderResult, JsError> {
        self.0
            .clone()
            .payment_key()
            .map(Into::into)
            .map_err(Into::into)
    }

    pub fn native_script(
        self,
        native_script: &NativeScript,
        witness_info: &NativeScriptWitnessInfo,
    ) -> Result<WithdrawalBuilderResult, JsError> {
        self.0
            .native_script(native_script.as_ref(), witness_info.as_ref())
            .map(Into::into)
            .map_err(Into::into)
    }

    pub fn plutus_script(
        self,
        partial_witness: PartialPlutusWitness,
        required_signers: RequiredSigners,
    ) -> Result<WithdrawalBuilderResult, JsError> {
        self.0
            .plutus_script(partial_witness.into(), required_signers.into())
            .map(Into::into)
            .map_err(Into::into)
    }
}
