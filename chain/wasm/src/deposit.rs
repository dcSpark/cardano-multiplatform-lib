use wasm_bindgen::prelude::{wasm_bindgen, JsError};

use crate::{
    assets::{Coin, Value},
    transaction::TransactionBody,
};

#[wasm_bindgen]
pub fn get_implicit_input(
    txbody: &TransactionBody,
    pool_deposit: Coin, // // protocol parameter
    key_deposit: Coin,  // protocol parameter
) -> Result<Value, JsError> {
    cml_chain::deposit::get_implicit_input(txbody.as_ref(), pool_deposit, key_deposit)
        .map(Into::into)
        .map_err(Into::into)
}

#[wasm_bindgen]
pub fn get_deposit(
    txbody: &TransactionBody,
    pool_deposit: Coin, // // protocol parameter
    key_deposit: Coin,  // protocol parameter
) -> Result<Coin, JsError> {
    cml_chain::deposit::get_deposit(txbody.as_ref(), pool_deposit, key_deposit).map_err(Into::into)
}
