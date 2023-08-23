use wasm_bindgen::{prelude::wasm_bindgen, JsError};

use cml_core_wasm::impl_wasm_conversions;

use crate::{
    address::Address,
    assets::{Coin, MultiAsset, Value},
    plutus::PlutusData,
    transaction::{DatumOption, ScriptRef, TransactionOutput},
};

/// We introduce a builder-pattern format for creating transaction outputs
/// This is because:
/// 1. Some fields (i.e. data hash) are optional, and we can't easily expose Option<> in WASM
/// 2. Some fields like amounts have many ways it could be set (some depending on other field values being known)
/// 3. Easier to adapt as the output format gets more complicated in future Cardano releases
#[derive(Clone, Debug, Default)]
#[wasm_bindgen]
pub struct TransactionOutputBuilder(cml_chain::builders::output_builder::TransactionOutputBuilder);

#[wasm_bindgen]
impl TransactionOutputBuilder {
    pub fn new() -> Self {
        cml_chain::builders::output_builder::TransactionOutputBuilder::new().into()
    }

    pub fn with_address(&self, address: &Address) -> Self {
        self.0.clone().with_address(address.clone().into()).into()
    }

    /// A communication datum is one where the data hash is used in the tx output
    /// Yet the full datum is included in the witness of the same transaction
    pub fn with_communication_data(&self, datum: &PlutusData) -> Self {
        self.0
            .clone()
            .with_communication_data(datum.clone().into())
            .into()
    }
    pub fn with_data(&self, datum: &DatumOption) -> Self {
        self.0.clone().with_data(datum.clone().into()).into()
    }

    pub fn with_reference_script(&self, script_ref: &ScriptRef) -> Self {
        self.0
            .clone()
            .with_reference_script(script_ref.clone().into())
            .into()
    }

    pub fn next(&self) -> Result<TransactionOutputAmountBuilder, JsError> {
        self.0.clone().next().map(Into::into).map_err(Into::into)
    }
}

impl_wasm_conversions!(
    cml_chain::builders::output_builder::TransactionOutputBuilder,
    TransactionOutputBuilder
);

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct TransactionOutputAmountBuilder(
    cml_chain::builders::output_builder::TransactionOutputAmountBuilder,
);

#[wasm_bindgen]
impl TransactionOutputAmountBuilder {
    pub fn with_value(&self, amount: &Value) -> Self {
        self.0
            .clone()
            .with_value::<cml_chain::Value>(amount.clone().into())
            .into()
    }

    pub fn with_asset_and_min_required_coin(
        &self,
        multiasset: &MultiAsset,
        coins_per_utxo_byte: Coin,
    ) -> Result<TransactionOutputAmountBuilder, JsError> {
        self.0
            .clone()
            .with_asset_and_min_required_coin(multiasset.clone().into(), coins_per_utxo_byte)
            .map(Into::into)
            .map_err(Into::into)
    }

    pub fn build(&self) -> Result<SingleOutputBuilderResult, JsError> {
        self.0.clone().build().map(Into::into).map_err(Into::into)
    }
}

impl_wasm_conversions!(
    cml_chain::builders::output_builder::TransactionOutputAmountBuilder,
    TransactionOutputAmountBuilder
);

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct SingleOutputBuilderResult(
    cml_chain::builders::output_builder::SingleOutputBuilderResult,
);

#[wasm_bindgen]
impl SingleOutputBuilderResult {
    pub fn new(output: &TransactionOutput) -> SingleOutputBuilderResult {
        cml_chain::builders::output_builder::SingleOutputBuilderResult::new(output.clone().into())
            .into()
    }

    pub fn output(&self) -> TransactionOutput {
        self.0.output.clone().into()
    }

    pub fn communication_datum(&self) -> Option<PlutusData> {
        self.0.communication_datum.clone().map(Into::into)
    }
}

impl_wasm_conversions!(
    cml_chain::builders::output_builder::SingleOutputBuilderResult,
    SingleOutputBuilderResult
);
