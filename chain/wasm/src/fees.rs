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
    pub fn new(coefficient: Coin, constant: Coin) -> Self {
        cml_chain::fees::LinearFee::new(coefficient, constant).into()
    }

    pub fn constant(&self) -> Coin {
        self.0.constant
    }

    pub fn coefficient(&self) -> Coin {
        self.0.coefficient
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
