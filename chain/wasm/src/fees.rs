use cml_core_wasm::impl_wasm_conversions;
use wasm_bindgen::prelude::{wasm_bindgen, JsError};

use crate::{plutus::ExUnitPrices, transaction::Transaction, Coin};

/// Careful: although the linear fee is the same for Byron & Shelley
/// The value of the parameters and how fees are computed is not the same
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct LinearFee(cml_chain::fees::LinearFee);

impl_wasm_conversions!(cml_chain::fees::LinearFee, LinearFee);

#[wasm_bindgen]
impl LinearFee {
    /**
     * * `coefficient` - minfee_a from protocol params
     * * `constant` - minfee_b from protocol params
     * * `ref_script_cost_per_bytes` - min_fee_ref_script_cost_per_byte from protocol params. New in Conway
     */
    pub fn new(coefficient: Coin, constant: Coin, ref_script_cost_per_byte: Coin) -> Self {
        cml_chain::fees::LinearFee::new(coefficient, constant, ref_script_cost_per_byte).into()
    }

    /// minfee_a
    pub fn coefficient(&self) -> Coin {
        self.0.coefficient
    }

    /// minfee_b
    pub fn constant(&self) -> Coin {
        self.0.constant
    }

    // minfee_ref_script_cost_per_byte
    pub fn ref_script_cost_per_byte(&self) -> Coin {
        self.0.ref_script_cost_per_byte
    }
}

/**
 * Min fee for JUST the script
 */
#[wasm_bindgen]
pub fn min_script_fee(tx: &Transaction, ex_unit_prices: &ExUnitPrices) -> Result<Coin, JsError> {
    cml_chain::fees::min_script_fee(tx.as_ref(), ex_unit_prices.as_ref()).map_err(Into::into)
}

#[wasm_bindgen]
pub fn min_no_script_fee(tx: &Transaction, linear_fee: &LinearFee) -> Result<Coin, JsError> {
    cml_chain::fees::min_no_script_fee(tx.as_ref(), linear_fee.as_ref()).map_err(Into::into)
}

#[wasm_bindgen]
pub fn min_fee(
    tx: &Transaction,
    linear_fee: &LinearFee,
    ex_unit_prices: &ExUnitPrices,
) -> Result<Coin, JsError> {
    cml_chain::fees::min_fee(tx.as_ref(), linear_fee.as_ref(), ex_unit_prices.as_ref())
        .map_err(Into::into)
}
