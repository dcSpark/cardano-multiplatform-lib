use crate::{*, ledger::babbage::min_ada::{min_ada_required, compatible_min_ada_required}};

/// We introduce a builder-pattern format for creating transaction outputs
/// This is because:
/// 1. Some fields (i.e. data hash) are optional, and we can't easily expose Option<> in WASM
/// 2. Some fields like amounts have many ways it could be set (some depending on other field values being known)
/// 3. Easier to adapt as the output format gets more complicated in future Cardano releases

#[wasm_bindgen]
#[derive(Clone, Debug, Default)]
pub struct TransactionOutputBuilder {
    pub(crate) address: Option<Address>,
    pub(crate) datum: Option<DatumEnum>,
    pub(crate) script_ref: Option<ScriptRef>,
}

#[wasm_bindgen]
impl TransactionOutputBuilder {
    pub fn new() -> Self {
        // explicit add new so that it's visible from WASM
        Self::default()
    }

    pub fn with_address(&self, address: &Address) -> Self {
        let mut cfg = self.clone();
        cfg.address = Some(address.clone());
        cfg
    }

    pub fn with_data(&self, datum: &Datum) -> Self {
        let mut cfg = self.clone();
        cfg.datum = Some(datum.0.clone());
        cfg
    }

    pub fn with_reference_script(&self, script_ref: &ScriptRef) -> Self {
        let mut cfg = self.clone();
        cfg.script_ref = Some(script_ref.clone());
        cfg
    }

    pub fn next(&self) -> Result<TransactionOutputAmountBuilder, JsError> {
        Ok(TransactionOutputAmountBuilder {
            address: self.address.clone().ok_or_else(|| JsError::from_str("TransactionOutputBaseBuilder: Address missing"))?,
            amount: None,
            datum: self.datum.clone(),
            script_ref: self.script_ref.clone(),
        })
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct TransactionOutputAmountBuilder {
    address: Address,
    amount: Option<Value>,
    datum: Option<DatumEnum>,
    script_ref: Option<ScriptRef>,
}

#[wasm_bindgen]
impl TransactionOutputAmountBuilder {

    pub fn with_value(&self, amount: &Value) -> Self {
        let mut cfg = self.clone();
        cfg.amount = Some(amount.clone());
        cfg
    }

    pub fn with_coin(&self, coin: &Coin) -> Self {
        let mut cfg = self.clone();

        cfg.amount = Some(Value::new(coin));
        cfg
    }

    pub fn with_coin_and_asset(&self, coin: &Coin, multiasset: &MultiAsset) -> Self {
        let mut cfg = self.clone();

        let mut val = Value::new(coin);
        val.set_multiasset(multiasset);
        cfg.amount = Some(val.clone());
        cfg
    }

    pub fn with_asset_and_min_required_coin(&self, multiasset: &MultiAsset, coins_per_utxo_byte: &Coin, coins_per_utxo_word: Option<Coin>) -> Result<TransactionOutputAmountBuilder, JsError> {
        let mut min_output = TransactionOutput::new(
            &self.address,
            &self.amount.clone().unwrap_or_else(|| Value::new(&to_bignum(0))),
        );
        min_output.datum_option = self.datum.clone();
        min_output.script_ref = self.script_ref.clone();
        let min_possible_coin = calc_min_ada(&min_output, coins_per_utxo_byte, coins_per_utxo_word.as_ref())?;
        
        let mut value = Value::new(&min_possible_coin);
        value.set_multiasset(multiasset);

        let mut check_output = TransactionOutput::new(&self.address, &value);
        check_output.datum_option = self.datum.clone();
        check_output.script_ref = self.script_ref.clone();

        let required_coin = calc_min_ada(&check_output, coins_per_utxo_byte, coins_per_utxo_word.as_ref())?;

        Ok(self.with_coin_and_asset(&required_coin, multiasset))
    }

    pub fn build(&self) -> Result<TransactionOutput, JsError> {
        Ok(TransactionOutput {
            address: self.address.clone(),
            amount: self.amount.clone().ok_or_else(|| JsError::from_str("TransactionOutputAmountBuilder: amount missing"))?,
            datum_option: self.datum.clone(),
            script_ref: self.script_ref.clone(),
        })
    }
}

#[deprecated(
    since = "1.0.0",
    note = "If you don't need to support Alonzo, you don't need this function"
)]
pub(crate) fn calc_min_ada(output: &TransactionOutput, coins_per_utxo_byte: &BigNum, coins_per_utxo_word: Option<&BigNum>) -> Result<BigNum, JsError> {
    match coins_per_utxo_word {
        Some(coins_per_utxo_word) => compatible_min_ada_required(
            output,
            coins_per_utxo_byte,
            coins_per_utxo_word
        ),
        None => min_ada_required(
            output,
            coins_per_utxo_byte,
        )
    }
}
