use cml_core::ArithmeticError;

use crate::{
    address::Address,
    transaction::{DatumOption, ScriptRef, TransactionOutput, ShelleyTxOut, BabbageTxOut},
    plutus::PlutusData,
    assets::{MultiAsset, Value, Coin},
    crypto::hash::hash_plutus_data,
    min_ada::{min_ada_required},
};

#[derive(Debug, thiserror::Error)]
pub enum OutputBuilderError {
    #[error("Address missing")]
    AddressMissing,
    #[error("Value missing")]
    AmountMissing,
    #[error("Min ADA error: {0:?}")]
    MinAdaError(#[from] ArithmeticError),
}

/// We introduce a builder-pattern format for creating transaction outputs
/// This is because:
/// 1. Some fields (i.e. data hash) are optional, and we can't easily expose Option<> in WASM
/// 2. Some fields like amounts have many ways it could be set (some depending on other field values being known)
/// 3. Easier to adapt as the output format gets more complicated in future Cardano releases

#[derive(Clone, Debug, Default)]
pub struct TransactionOutputBuilder {
    pub address: Option<Address>,
    pub datum: Option<DatumOption>,
    pub communication_datum: Option<PlutusData>,
    pub script_ref: Option<ScriptRef>,
}

impl TransactionOutputBuilder {
    pub fn new() -> Self {
        // explicit new for consistency with WASM
        Self::default()
    }

    pub fn with_address(mut self, address: Address) -> Self {
        self.address = Some(address);
        self
    }

    /// A communication datum is one where the data hash is used in the tx output
    /// Yet the full datum is included in the witness of the same transaction
    pub fn with_communication_data(mut self, datum: PlutusData) -> Self {
        self.datum = Some(DatumOption::new_hash(hash_plutus_data(&datum)));
        self.communication_datum = Some(datum);
        self
    }
    pub fn with_data(mut self, datum: DatumOption) -> Self {
        self.datum = Some(datum);
        self.communication_datum = None;
        self
    }

    pub fn with_reference_script(mut self, script_ref: ScriptRef) -> Self {
        self.script_ref = Some(script_ref);
        self
    }

    pub fn next(self) -> Result<TransactionOutputAmountBuilder, OutputBuilderError> {
        Ok(TransactionOutputAmountBuilder {
            address: self.address.ok_or(OutputBuilderError::AddressMissing)?,
            amount: None,
            datum: self.datum,
            script_ref: self.script_ref,
            communication_datum: self.communication_datum,
        })
    }
}

#[derive(Clone, Debug)]
pub struct TransactionOutputAmountBuilder {
    address: Address,
    amount: Option<Value>,
    datum: Option<DatumOption>,
    script_ref: Option<ScriptRef>,
    communication_datum: Option<PlutusData>,
}

impl TransactionOutputAmountBuilder {
    pub fn with_value<T: Into<Value>>(mut self, amount: T) -> Self {
        self.amount = Some(amount.into());
        self
    }

    pub fn with_asset_and_min_required_coin(mut self, multiasset: MultiAsset, coins_per_utxo_byte: Coin) -> Result<Self, OutputBuilderError> {
        let mut min_output = TransactionOutput::new_babbage_tx_out(BabbageTxOut {
            address: self.address.clone(),
            amount: self.amount.clone().unwrap_or_else(|| Value::from(0)),
            datum_option: self.datum.clone(),
            script_reference: self.script_ref.clone(),
            encodings: None,
        });
        let min_possible_coin = min_ada_required(&min_output, coins_per_utxo_byte)?;

        let check_output = &mut min_output;
        check_output.set_amount(Value::new(min_possible_coin, multiasset.clone()));

        let required_coin = min_ada_required(check_output, coins_per_utxo_byte)?;

        Ok(self.with_value(Value::new(required_coin, multiasset)))
    }

    pub fn build(self) -> Result<SingleOutputBuilderResult, OutputBuilderError> {
        let output = TransactionOutput::new_babbage_tx_out(BabbageTxOut {
            address: self.address,
            amount: self.amount.ok_or(OutputBuilderError::AmountMissing)?,
            datum_option: self.datum,
            script_reference: self.script_ref,
            encodings: None,
        });
        Ok(SingleOutputBuilderResult {
            output,
            communication_datum: self.communication_datum
        })
    }
}

#[derive(Clone, Debug)]
pub struct SingleOutputBuilderResult {
    pub output: TransactionOutput,
    pub communication_datum: Option<PlutusData>,
}

impl SingleOutputBuilderResult {
    pub fn new(output: TransactionOutput) -> SingleOutputBuilderResult {
        Self {
            output,
            communication_datum: None,
        }
    }
}
